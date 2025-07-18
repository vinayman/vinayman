use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full items-center">
            <div class="flex flex-col items-start justify-start w-full px-12 md:px-0 md:w-2/5 md:max-w-2/5 break-all">
                <article class="w-full">
                    <div class="w-36 h-36 bg-red-500 rounded-full float-right" />
                    <div class="space-y-4 w-full">
                        <h1 class="text-3xl font-bold">"Hi! I'm Vinay Manektalla"</h1>
                        <p class="text-lg text-gray-500">
                            "I'm a software engineer with 10 years of experience in developing products, building and designing systems and leading teams. "
                        </p>
                        <p class="text-lg text-gray-500">
                            "I'm currently working as the Head of Development at "
                            <a href="https://eunice.ai/" class="text-blue-500">
                                eunice.ai
                            </a>
                            " where I lead our application development and function as the co-head of engineering."
                        </p>
                        <p class="text-lg text-gray-500">
                            "I'm passionate about learning new skills and constantly improving my craft. Currently in my free time, I'm learning Rust which I've used to build this website."
                        </p>
                        <p class="text-lg text-gray-500">
                            "In my free time, I love running, hiking, watching football, eating out and spending time with my family."
                        </p>
                        <p class="text-lg text-gray-500">
                            "I'm based in London. Feel free to connect with me on one of the platforms below."
                        </p>
                        <div class="flex flex-row space-x-4">
                            <a
                                href="https://www.linkedin.com/in/vinay-manektalla/"
                                class="text-blue-500"
                            >
                                LinkedIn
                            </a>
                            <a href="https://github.com/vinayman" class="text-blue-500">
                                Github
                            </a>
                        </div>
                    </div>
                </article>
            </div>
        </div>
    }
}
