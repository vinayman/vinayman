use leptos::prelude::*;

struct Project {
    name: &'static str,
    description: &'static str,
    url: &'static str,
}

const PROJECTS: &[&Project] = &[
    &Project {
        name: "C++ Cryptocurrency Trading Framework and Algorithms",
        description: "A C++ framework for cryptocurrency trading and algorithmic trading interfacing with the Binance API.",
        url: "https://github.com/vinayman/cryptalgocpp",
    },
    &Project {
        name: "Helicopter Game written in Go",
        description: "An implementation of the popular Helicopter game written in Go using the terminal-based game engine termloop. Users must navigate a helicopter through a series of obstacles. ",
        url: "https://github.com/vinayman/helicopter-go",
    },
    &Project {
        name: "Python and ElasticSearch based Search Engine",
        description: "A search engine built using Python and ElasticSearch. It allows users to search for documents using natural language queries and retrieve the most relevant results.",
        url: "https://github.com/vinayman/elasticsearch_titles_search",
    },
    &Project {
        name: "TypeScript web app for tracking your kick-ups through your webcam",
        description: "A TypeScript web app leveraging Tensorflow to track your kick-ups and body movements through your webcam, using MoveNet for pose detection and Lite MobileNet V2 for object detection.",
        url: "https://github.com/vinayman/kick-ups",
    },
    &Project {
        name: "My personal portfolio website",
        description: "A personal portfolio website written in Rust using Leptos.",
        url: "https://github.com/vinayman/vinayman",
    },
];

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full items-center">
            <div class="flex flex-col items-start justify-start w-full px-12 md:px-0 md:w-2/5 md:max-w-2/5 break-all space-y-6">
                <h1 class="text-3xl font-bold">Projects</h1>
                <ul class="space-y-6 break-normal">
                    {PROJECTS
                        .iter()
                        .map(|project| {
                            view! {
                                <li class="space-y-4">
                                    <a href=project.url class="text-blue-500">
                                        {project.name}
                                    </a>
                                    <p>{project.description}</p>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </ul>
            </div>
        </div>
    }
}
