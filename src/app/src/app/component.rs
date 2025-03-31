use leptos::prelude::*;

#[component]
pub fn navbar() -> impl IntoView {
    view! {
        <nav class="navbar bg-success" >
            <div class="container" >
                <h1 class="navbar-brand text-light" href="#" >UPDRAFT</h1>
            </div>
        </nav>
    }
}

#[component]
pub fn my_html(children: Children) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-bs-theme="dark" >

            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <title>"UPDRAFT"</title>
                <link
                    href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"
                    rel="stylesheet"
                    integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN"
                    crossorigin="anonymous"
                />
            </head>

            <body>
                <Navbar />
                {children()}
                <script
                    src="https://cdn.jsdelivr.net/npm/@popperjs/core@2.11.8/dist/umd/popper.min.js"
                    integrity="sha384-I7E8VVD/ismYTF4hNIPjVp/Zjvgyol6VFvRkX/vR+Vc4jQkC+hVqc2pM8ODewa9r"
                    crossorigin="anonymous"
                ></script>
                <script
                    src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.min.js"
                    integrity="sha384-BBtl+eGJRgqQAUMxJ7pMwbEyER4l1g+O15P+16Ep7Q9Q+zqX6gSbd85u4mG4QzX+"
                    crossorigin="anonymous"
                ></script>
                <script
                    src="https://unpkg.com/htmx.org@1.9.6"
                    integrity="sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni"
                    crossorigin="anonymous"
                ></script>
            </body>

        </html>
    }
}
