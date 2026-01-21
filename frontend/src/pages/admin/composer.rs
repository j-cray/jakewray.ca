use leptos::prelude::*;

#[component]
pub fn AdminComposer() -> impl IntoView {
    let (content, set_content) = signal("# New Post\n\nStart writing...".to_string());

    // Simple mock markdown parsing (replace newlines) for now.
    // TODO: Use proper markdown parser like pulldown-cmark or comrak
    // WARNING: This uses inner_html and is vulnerable to XSS. Only safe because
    // this is admin-only interface. Must use proper sanitization before production.
    let preview = move || {
        content
            .get()
            .replace("\n", "<br/>")
            .replace("# ", "<h1 class='text-2xl font-bold'>")
        // Very naive, just for scaffolding visual
    };

    view! {
        <div class="container py-12 h-screen flex flex-col">
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-3xl">"Composer"</h1>
                <div class="flex gap-4">
                     <button class="btn btn-secondary">"Save Draft"</button>
                     <button class="btn btn-primary">"Publish"</button>
                </div>
            </div>

            <div class="flex-grow grid grid-cols-1 lg:grid-cols-2 gap-6 h-full">
                <div class="form-group h-full">
                    <label for="composer-content">"Content"</label>
                    <textarea
                        id="composer-content"
                        class="w-full h-full font-mono resize-none"
                        on:input=move |ev| set_content.set(event_target_value(&ev))
                        prop:value=content
                    ></textarea>
                </div>

                <div class="card h-full">
                    <h3 class="text-lg font-bold mb-4">"Preview"</h3>
                    <div class="prose overflow-y-auto" inner_html=preview></div>
                </div>
            </div>
        </div>
    }
}
