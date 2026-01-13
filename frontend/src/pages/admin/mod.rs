pub mod login;
pub mod dashboard;
pub mod composer;
pub mod sync_manager;

use leptos::prelude::*;
use leptos_router::components::{Outlet, Redirect};
use gloo_net::http::Request;

#[component]
pub fn AdminProtectedLayout() -> impl IntoView {
    // Resource to check if user is authenticated
    let auth_status = Resource::new(
        || (),
        |_| async move {
            let resp = Request::get("/api/admin/me").send().await;
            match resp {
                Ok(r) => r.ok(), // True if 200 OK
                Err(_) => false,
            }
        },
    );

    view! {
        <Suspense fallback=move || view! {
            <div class="flex items-center justify-center min-h-screen bg-gray-900 text-white">
                "Verifying authentication..."
            </div>
        }>
            {move || match auth_status.get() {
                Some(true) => view! {
                    <div class="admin-layout">
                        <Outlet/>
                    </div>
                }.into_any(),
                Some(false) => view! { <Redirect path="/admin"/> }.into_any(), // Redirect to login
                None => view! { }.into_any(),
            }}
        </Suspense>
    }
}
