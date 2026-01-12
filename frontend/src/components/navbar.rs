use leptos::prelude::*;
use leptos::*;
use thaw::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <LayoutHeader class="p-4 border-b border-gray-200">
            <div class="container mx-auto flex flex-col md:flex-row justify-between items-center gap-4">
                <a href="/" class="text-2xl font-bold font-heading no-underline text-gray-900">
                    "Jake Wray"
                </a>

                <div class="flex gap-2 flex-wrap justify-center">
                    <Button on_click=move |_| { let _ = window().location().set_href("/journalism"); }>
                        "Journalism"
                    </Button>
                    <Button on_click=move |_| { let _ = window().location().set_href("/personal"); }>
                        "Personal"
                    </Button>
                    <Button on_click=move |_| { let _ = window().location().set_href("/creative-writing"); }>
                        "Writing"
                    </Button>
                    <Button on_click=move |_| { let _ = window().location().set_href("/music"); }>
                        "Music"
                    </Button>
                    <Button on_click=move |_| { let _ = window().location().set_href("/visual-art"); }>
                        "Art"
                    </Button>
                    <Button on_click=move |_| { let _ = window().location().set_href("/programming"); }>
                        "Code"
                    </Button>
                    <div class="w-px h-6 bg-gray-300 mx-2 hidden md:block"></div>
                    <Button on_click=move |_| { let _ = window().location().set_href("/about"); }>
                        "About"
                    </Button>
                </div>
            </div>
        </LayoutHeader>
    }
}
