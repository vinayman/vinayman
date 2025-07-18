use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full items-center">
            <div class="flex flex-col items-start justify-start w-full px-12 md:px-0 md:w-2/5 md:max-w-2/5 break-all space-y-6">
                <h1 class="text-3xl font-bold">Blog</h1>
                <div class="flex flex-col items-start justify-start w-full space-y-6">
                    <A href="hello-world-post">First Blog Post</A>
                </div>
            </div>
        </div>
    }
}
