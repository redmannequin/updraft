fn main() -> Result<(), Box<dyn std::error::Error>> {
    sol_gen::generate("idls/raydium.json", "src/program/raydium_2.rs")?;
    Ok(())
}
