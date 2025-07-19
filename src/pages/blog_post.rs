use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use gloo_net::http::Request;
use pulldown_cmark::{Parser, Options, html};
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default().to_string();

    // State for the HTML content
    let (html_content, set_html_content) = signal("<p>Loading...</p>".to_string());

    // Effect to fetch and parse markdown when id changes
    Effect::new(move |_| {
        let id = id();
        let url = format!("/{}.md", id);
        let set_html = set_html_content.clone();

        spawn_local(async move {
            match Request::get(&url).send().await {
                Ok(resp) => {
                    if let Ok(content) = resp.text().await {
                        let mut html_output = String::new();
                        let parser = Parser::new_ext(&content, Options::all());
                        html::push_html(&mut html_output, parser);
                        set_html.set(html_output);
                    } else {
                        set_html.set("<p>Error loading content.</p>".to_string());
                    }
                }
                Err(_) => {
                    set_html.set("<p>Blog post not found.</p>".to_string());
                }
            }
        });
    });

    view! {
        <div class="flex flex-col w-full items-center">
            <div class="flex flex-col items-start justify-start w-full px-12 md:px-0 md:w-2/5 md:max-w-2/5 break-all space-y-6">
                <h1 class="text-3xl font-bold">Blog</h1>
                <div
                    class="prose prose-blog mx-auto md:prose-lg prose-pre:m-0 prose-pre:rounded-none break-normal"
                    inner_html=move || html_content.get()
                ></div>
            </div>
        </div>
    }
}
