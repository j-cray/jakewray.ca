use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Navbar() -> impl IntoView {
    let navigate = use_navigate();
    
    view! {
        <header class="site-header">
            <div class="container nav-container">
                <a href="/" class="site-brand" on:click=move |ev| {
                    ev.prevent_default();
                    navigate("/", Default::default());
                }>
                    "Jake Wray"
                </a>

                <nav class="nav-links">
                    <a class="nav-link" href="/journalism" on:click=move |ev| {
                        ev.prevent_default();
                        navigate("/journalism", Default::default());
                    }>"Journalism"</a>
                    <a class="nav-link" href="/personal" on:click=move |ev| {
                        ev.prevent_default();
                        navigate("/personal", Default::default());
                    }>"Personal"</a>
                    <a class="nav-link" href="/personal/blog" on:click=move |ev| {
                        ev.prevent_default();
                        navigate("/personal/blog", Default::default());
                    }>"Blog"</a>
                    <a class="nav-link" href="/programming" on:click=move |ev| {
                        ev.prevent_default();
                        navigate("/programming", Default::default());
                    }>"Code"</a>
                    <a class="nav-link nav-link-primary" href="/about" on:click=move |ev| {
                        ev.prevent_default();
                        navigate("/about", Default::default());
                    }>"About"</a>
                </nav>
            </div>
        </header>
    }
}
