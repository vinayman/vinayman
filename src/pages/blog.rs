use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full items-center">
            <div class="flex flex-col items-start justify-start w-full px-12 md:px-0 md:w-2/5 md:max-w-2/5 break-all space-y-6">
                <h1 class="text-3xl font-bold">Blog</h1>
                <div class="flex flex-col items-start justify-start w-full space-y-6">
                    <A href="hello-world-post">
                        <div class="flex flex-col items-start justify-start w-full space-y-2">
                            <div class="flex flex-row items-center justify-between w-full">
                                <h2 class="text-2xl font-bold">Hello World</h2>
                                <p class="text-sm text-gray-500">"2025-07-19"</p>
                            </div>
                            <p class="text-sm text-gray-500">
                                "This is the first blog post. It's a simple post about the first blog post."
                            </p>
                        </div>
                    </A>
                </div>
            </div>
        </div>
    }
}
