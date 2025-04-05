use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
};

use anyhow::Context;
use convert_case::{Case, Casing};
use idl::Idl;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use sha2::{Digest, Sha256};

pub mod idl;

pub fn generate(src_path: &str, out_path: &str) -> anyhow::Result<()> {
    let mut fp = File::open(src_path)?;
    let mut src = String::new();
    fp.read_to_string(&mut src)?;
    let idl = serde_json::from_str(&src)?;
    let code = gen_client(idl);

    let code_file = syn::parse2(code).context("failed to parse token stream")?;
    let code_src = prettyplease::unparse(&code_file);

    let mut out_fp = File::create(out_path)?;
    out_fp.write_all(code_src.as_bytes())?;

    Ok(())
}

fn gen_type(ty: &idl::Type) -> TokenStream {
    match ty {
        idl::Type::Bool => quote! { bool },
        idl::Type::U8 => quote! { u8 },
        idl::Type::U16 => quote! { u16 },
        idl::Type::U32 => quote! { u32 },
        idl::Type::U64 => quote! { u64 },
        idl::Type::U128 => quote! { u128 },
        idl::Type::I8 => quote! { i8 },
        idl::Type::I16 => quote! { i16 },
        idl::Type::I32 => quote! { i32 },
        idl::Type::I64 => quote! { i64 },
        idl::Type::I128 => quote! { i128 },
        idl::Type::Bytes => quote! { Bytes },
        idl::Type::String => quote! { String },
        idl::Type::PublicKey => quote! { [u8; 32] },
        idl::Type::Option(ty) => {
            let ty = gen_type(ty);
            quote! { Option<#ty> }
        }
        idl::Type::FixedArray(ty, n) => {
            let ty = gen_type(ty);
            quote! { [#ty; #n] }
        }
        idl::Type::DynamicArray(ty) => {
            let ty = gen_type(ty);
            quote! { Vec<#ty> }
        }
        idl::Type::Defined(ty) => {
            let ty = syn::Ident::new(&ty.to_case(Case::Pascal), Span::call_site());
            quote! { #ty }
        }
    }
}

fn gen_discriminator(discriminator: &idl::Discriminator) -> TokenStream {
    let bytes = discriminator.0.as_slice();
    quote! { [#( #bytes ),*] }
}

fn gen_field(name: &str, ty: &idl::Type) -> TokenStream {
    let name = syn::Ident::new(&name.to_case(Case::Snake), Span::call_site());
    let ty = gen_type(ty);
    quote! { pub #name: #ty }
}

fn gen_struct<'src>(
    name: &str,
    fields: impl Iterator<Item = (&'src str, &'src idl::Type<'src>)>,
) -> (syn::Ident, TokenStream) {
    let name = syn::Ident::new(&name.to_case(Case::Pascal), Span::call_site());
    let fields = fields.map(|(name, ty)| gen_field(name, ty));
    (
        name.clone(),
        quote! {
            pub struct #name {
                #( #fields, )*
            }
        },
    )
}

pub fn gen_client(idl: Idl) -> TokenStream {
    let mut code = TokenStream::new();

    let program_name = syn::Ident::new(&idl.name.to_case(Case::Pascal), Span::call_site());

    let ix_names = idl
        .instructions
        .iter()
        .map(|ix| syn::Ident::new(&ix.name.to_case(Case::Pascal), Span::call_site()));

    let ix_names_2 = ix_names.clone();

    let ix_discriminator_size = {
        let mut n = idl
            .instructions
            .iter()
            .map(|ix| ix.discriminator.as_ref().map(|d| d.0.len()).unwrap_or(8))
            .collect::<HashSet<_>>();
        if n.len() != 1 {
            panic!("instructions must have the same discriminator size");
        }
        match n.drain().next() {
            Some(n) => n,
            None => unreachable!("There should be one element"),
        }
    };

    let ixs = idl.instructions.iter().map(|ix| {
        let (name, struct_def) = gen_struct(&ix.name, ix.args.iter().map(|f| (f.name, &f.r#type)));
        let discriminator = ix
            .discriminator
            .as_ref()
            .map(gen_discriminator)
            .unwrap_or_else(|| {
                let discriminator = &idl::Discriminator({
                    let hash = Sha256::digest(format!("global:{}", ix.name));
                    hash[..8].into()
                });
                gen_discriminator(discriminator)
            });
        let discriminator_size = ix.discriminator.as_ref().map(|d| d.0.len()).unwrap_or(8);

        let accounts_def = {
            let name = syn::Ident::new(
                &format!("{}Accounts", ix.name.to_case(Case::Pascal)),
                Span::call_site(),
            );
            let fiedls = ix.accounts.iter().map(|a| {
                let name = syn::Ident::new(&a.name.to_case(Case::Snake), Span::call_site());
                let name_2 = if idl.accounts.iter().find(|aa| aa.name == a.name).is_some() {
                    let name = syn::Ident::new(&a.name.to_case(Case::Pascal), Span::call_site());
                    quote! { #name }
                } else {
                    quote! { () }
                };

                let field = match (a.is_mutable, a.is_signer) {
                    (true, true) => quote! { Account<#name_2, Mutable, Signed> },
                    (true, false) => quote! { Account<#name_2, Mutable, Unsigned> },
                    (false, true) => quote! { Account<#name_2, ReadOnly, Signed> },
                    (false, false) => quote! { Account<#name_2, ReadOnly, Unsigned> },
                };

                quote! { #name: #field }
            });

            quote! {
                pub struct #name {
                    #( #fiedls, )*
                }
            }
        };

        quote! {
            #[derive(Debug, BorshSerialize, BorshDeserialize)]
            #struct_def

            impl #name {
                pub const DISCRIMINATOR: [u8; #discriminator_size] = #discriminator;
            }

            #accounts_def

        }
    });

    let tys = idl.types.iter().map(|ty| match &ty.r#type {
        idl::TypeDefKind::Struct(struct_def) => {
            let (_name, struct_def) = gen_struct(
                &ty.name,
                struct_def.fields.iter().map(|f| (f.name, &f.r#type)),
            );
            quote! {
                #[derive(Debug, BorshSerialize, BorshDeserialize)]
                #struct_def
            }
        }
        idl::TypeDefKind::Enum(enum_def) => {
            let name = syn::Ident::new(&ty.name.to_case(Case::Pascal), Span::call_site());
            let variants = enum_def.variants.iter().map(|v| {
                let name = syn::Ident::new(&v.name.to_case(Case::Pascal), Span::call_site());
                match &v.fields {
                    Some(fields) => {
                        let fields = fields.iter().map(|f| gen_field(&f.name, &f.r#type));
                        quote! { #name { #( #fields )* } }
                    }
                    None => quote! { #name },
                }
            });
            quote! {
                #[derive(Debug, BorshSerialize, BorshDeserialize)]
                pub enum #name {
                    #( #variants, )*
                }
            }
        }
    });

    let accounts = idl.accounts.iter().map(|account| {
        let (_name, struct_def) = gen_struct(
            &account.name,
            account.r#type.fields.iter().map(|f| (f.name, &f.r#type)),
        );

        quote! {
            #struct_def
        }
    });

    let events = idl.events.iter().map(|event| {
        let (_name, struct_def) = gen_struct(
            &event.name,
            event.fields.iter().map(|f| (f.name, &f.r#type)),
        );

        quote! {
            #[derive(Debug, BorshSerialize, BorshDeserialize)]
            #struct_def
        }
    });

    let version = {
        let major = idl.version.major;
        let minor = idl.version.minor;
        let patch = idl.version.patch;
        quote! { (#major, #minor, #patch) }
    };

    code.extend(quote! {
        #![allow(dead_code)]

        use borsh::{BorshDeserialize, BorshSerialize};
        use sol_ez::*;

        pub enum #program_name {
            #(#ix_names(#ix_names),)*
        }

        impl #program_name {
            pub const VERSION: (u8, u8, u8) = #version;

            pub fn parse(data: &[u8]) -> Self {
                let (discriminator, ix_data) = data.split_at(#ix_discriminator_size);
                let discriminator = {
                    let mut ix = [0; #ix_discriminator_size];
                    ix.copy_from_slice(discriminator);
                    ix
                };
                match discriminator {
                    #(#ix_names_2::DISCRIMINATOR => Self::#ix_names_2(borsh::from_slice(ix_data).unwrap()),)*
                    _ => panic!("this should be an error"),
                }
            }
        }

        #(#ixs)*
        #(#accounts)*
        #(#events)*
        #(#tys)*

    });

    code
}
