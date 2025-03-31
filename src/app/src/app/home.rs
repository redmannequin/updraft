use actix_web::HttpResponse;
use leptos::prelude::*;

use super::component::MyHtml;

pub async fn home() -> HttpResponse {
    let html = view! {
        <MyHtml>
            <div class="container text-light text-center pt-4">
                <h1 class="text-center">"Welcome to UPDRAFT"</h1>
            </div>
        </MyHtml>
    }
    .to_html();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf8")
        .body(html)
}
