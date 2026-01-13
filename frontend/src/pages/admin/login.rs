use leptos::logging;
use leptos::prelude::*;
use leptos_router::hooks::*;
use gloo_net::http::Request;

#[component]
pub fn AdminLoginPage() -> impl IntoView {
    let (username, set_username) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (error, set_error) = signal(Option::<String>::None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);

        let user = username.get();
        let pass = password.get();

        leptos::task::spawn_local(async move {
            let resp = Request::post("/api/admin/login")
                .json(&serde_json::json!({
                    "username": user,
                    "password": pass
                }))
                .expect("Failed to create request")
                .send()
                .await;

            match resp {
                Ok(response) if response.ok() => {
                     let navigate = use_navigate();
                     navigate("/admin/dashboard", Default::default());
                }
                _ => {
                    set_error.set(Some("Invalid username or password".to_string()));
                }
            }
        });
    };

    view! {
        <div class="flex items-center justify-center min-h-screen bg-gray-900 text-white">
            <div class="card w-full max-w-md bg-white/5 p-8 rounded-lg shadow-md border border-white/10 glass">
                <h1 class="text-2xl font-bold mb-6 text-center text-brand">"Admin Login"</h1>

                {move || error.get().map(|e| view! {
                    <div class="bg-red-500/20 text-red-200 p-3 rounded mb-4 text-center border border-red-500/50">
                        {e}
                    </div>
                })}

                <form on:submit=on_submit class="flex flex-col gap-4">
                    <input
                        type="text"
                        placeholder="Username"
                        class="p-3 rounded-md bg-black/50 border border-white/10 text-white focus:border-brand focus:outline-none"
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                        prop:value=username
                    />
                    <input
                        type="password"
                        placeholder="Password"
                        class="p-3 rounded-md bg-black/50 border border-white/10 text-white focus:border-brand focus:outline-none"
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                         prop:value=password
                    />
                    <button type="submit" class="bg-brand text-black p-3 rounded-md font-bold hover:bg-brand-dim transition">
                        "Login"
                    </button>

                     <a href="/" class="text-center text-sm text-gray-500 hover:text-white transition">"Back to Home"</a>
                </form>
            </div>
        </div>
    }
}
