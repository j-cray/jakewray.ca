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
use leptonic::components::button::{Button, ButtonSize, ButtonVariant};
use leptonic::components::card::Card;
use leptonic::components::grid::{Col, Grid, Row};
use leptonic::components::root::Root;
use leptonic::components::stack::{Stack, StackOrientation};
use leptonic::components::theme::LeptonicTheme;
use leptonic::components::typography::{H1, H3, P};
use leptonic::prelude::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

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
            <Root default_theme=LeptonicTheme::Dark>
                <Router>
                    <div class="min-h-screen flex flex-col bg-gray-50/50">
                        <Navbar/>
                        <main class="flex-grow container mx-auto px-4 py-8">
                            <Routes fallback=|| view! { <NotFound/> }>
                                <Route path="/" view=HomePage/>
                                <Route path="/about" view=AboutPage/>
                                <Route path="/contact" view=ContactPage/>
                                <Route path="/admin" view=AdminLoginPage/>
                                <ParentRoute path="/admin" view=crate::pages::admin::AdminProtectedLayout>
                                    <Route path="dashboard" view=AdminDashboard/>
                                    <Route path="composer" view=AdminComposer/>
                                    <Route path="sync" view=AdminSyncManager/>
                                    <Route path="media" view=MediaLibraryPlaceholder/>
                                </ParentRoute>
                                <Route path="/journalism" view=JournalismPage/>
                                <Route path="/personal" view=PersonalPage/>
                                <Route path="/creative-writing" view=CreativeWritingPage/>
                                <Route path="/music" view=MusicPage/>
                                <Route path="/visual-art" view=VisualArtPage/>
                                <Route path="/programming" view=ProgrammingPage/>
                            </Routes>
                        </main>
                        <Footer/>
                    </div>
                </Router>
            </Root>
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
        <div class="p-8 max-w-[1400px] mx-auto flex flex-col gap-16">
            <header class="text-center py-16">
                <H1 class="mb-2 text-6xl font-extrabold tracking-tighter">
                    <span class="text-gradient">"JAKE WRAY"</span>
                </H1>
                <P class="text-2xl text-gray-400 max-w-[600px] mx-auto">
                    "Journalist. Developer. Photographer. Creating extensive archives of the present."
                </P>
                <div class="mt-8 flex gap-4 justify-center">
                    <Button variant=ButtonVariant::Filled size=ButtonSize::Big>
                        "Read Journal"
                    </Button>
                    <Button variant=ButtonVariant::Outlined size=ButtonSize::Big>
                        "View Portfolio"
                    </Button>
                </div>
            </header>

            <div class="grid grid-cols-1 sm:grid-cols-3 gap-8">
                <div class="h-full">
                    <Card class="glass h-full transition-transform hover:scale-105">
                        <H3 class="font-bold text-brand">"Latest Articles"</H3>
                        <P>"Deep dives into technology, culture, and the intersection of both."</P>
                    </Card>
                </div>
                <div class="h-full">
                    <Card class="glass h-full transition-transform hover:scale-105">
                        <H3 class="font-bold text-brand">"Recent Projects"</H3>
                        <P>"Software engineering experiments, open source contributions, and more."</P>
                    </Card>
                </div>
                <div class="h-full">
                    <Card class="glass h-full transition-transform hover:scale-105">
                        <H3 class="font-bold text-brand">"Visuals"</H3>
                        <P>"A collection of photography and digital art capturing moments in time."</P>
                    </Card>
                </div>
            </div>
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
