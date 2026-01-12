use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::pages::about::AboutPage;
use crate::pages::admin::composer::AdminComposer;
use crate::pages::admin::dashboard::AdminDashboard;
use crate::pages::admin::login::AdminLoginPage;
use crate::pages::admin::sync_manager::AdminSyncManager;
use crate::pages::contact::ContactPage;
use crate::pages::sections::{
    CreativeWritingPage, JournalismPage, MusicPage, PersonalPage, ProgrammingPage, VisualArtPage,
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::*;

use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let theme = create_rw_signal(Theme::light());

    view! {
        <html lang="en">
        <head>
            <Title text="Jake Wray"/>
            <Meta name="description" content="Journalist, Programmer, Photographer."/>
            <Meta charset="utf-8"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1"/>
            <Stylesheet id="leptos" href="/pkg/jakewray_ca.css"/>
        </head>
        <body>
                <Router>
                    <div class="min-h-screen flex flex-col bg-gray-50/50">
                        <Navbar/>
                        <main class="flex-grow p-4">
                            <Routes fallback=|| view! { <NotFound/> }>
                                // Public Routes
                                <Route path=path!("/") view=HomePage/>
                                <Route path=path!("/about") view=AboutPage/>
                                <Route path=path!("/contact") view=ContactPage/>

                                // Portfolio
                                <Route path=path!("/journalism") view=JournalismPage/>
                                <Route path=path!("/personal") view=PersonalPage/>
                                <Route path=path!("/creative-writing") view=CreativeWritingPage/>
                                <Route path=path!("/music") view=MusicPage/>
                                <Route path=path!("/visual-art") view=VisualArtPage/>
                                <Route path=path!("/programming") view=ProgrammingPage/>

                                // Admin Routes
                                <Route path=path!("/admin") view=AdminDashboard/>
                                <Route path=path!("/admin/login") view=AdminLoginPage/>
                                <Route path=path!("/admin/compose") view=AdminComposer/>
                                <Route path=path!("/admin/sync") view=AdminSyncManager/>
                                <Route path=path!("/admin/media") view=MediaLibraryPlaceholder/>
                            </Routes>
                        </main>
                        <Footer/>
                    </div>
                </Router>
        </body>
        </html>
    }
}

#[component]
fn SectionLayout() -> impl IntoView {
    view! { <Outlet/> }
}

#[component]
fn MediaLibraryPlaceholder() -> impl IntoView {
    view! { "Media Library Placeholder" }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="container py-12">
            <header class="text-center mb-16">
                <h1 class="text-6xl mb-6 font-heading">"Jake Wray"</h1>
                <p class="text-xl text-muted max-w-2xl mx-auto">
                    "Journalist. Developer. Photographer. Creating extensive archives of the present."
                </p>
            </header>

            <Grid cols=3 x_gap=24 y_gap=24>
                <GridItem>
                    <Card>
                        <h3 class="text-xl mb-2 font-bold">"Latest Articles"</h3>
                        <p class="text-muted">"Coming soon..."</p>
                    </Card>
                </GridItem>
                <GridItem>
                    <Card>
                        <h3 class="text-xl mb-2 font-bold">"Recent Projects"</h3>
                        <p class="text-muted">"Coming soon..."</p>
                    </Card>
                </GridItem>
                <GridItem>
                    <Card>
                        <h3 class="text-xl mb-2 font-bold">"Visuals"</h3>
                        <p class="text-muted">"Coming soon..."</p>
                    </Card>
                </GridItem>
            </Grid>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="container py-24 text-center">
            <h1 class="text-4xl mb-4">"404"</h1>
            <p>"Page not found."</p>
        </div>
    }
}

#[component]
fn DummyPage() -> impl IntoView {
    view! { "Dummy" }
}
