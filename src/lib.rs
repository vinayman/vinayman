use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod pages;

// Top-Level pages
use crate::pages::blog::Blog;
use crate::pages::blog_post::BlogPost;
use crate::pages::home::Home;
use crate::pages::projects::Projects;

/// A fallback component for 404 Not Found pages.
#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full items-center">
            <div class="flex flex-col items-start justify-start w-full px-12 md:px-0 md:w-2/5 md:max-w-2/5 break-all">
                <article class="w-full">
                    <div class="space-y-4 w-full flex flex-col">
                        <h1 class="text-3xl font-bold">404: Page Not Found</h1>
                        <div class="w-full flex flex-col items-start justify-start space-y-4">
                            <p class="text-lg text-gray-500">
                                "Uh oh! The page you are looking for does not exist."
                            </p>
                            <a
                                href="/"
                                class="text-center w-fit h-auto py-2 px-4 rounded-md bg-blue-400 text-white"
                            >
                                "Go to Home"
                            </a>
                        </div>
                    </div>
                </article>
            </div>
        </div>
    }
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <link data-trunk rel="tailwind-css" href="input.css" />

        // sets the document title
        <Title text="Vinay Manektalla" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <div class="z-50 min-h-16 bg-red-100 pb-4 overscroll-none">
                <nav class="flex items-center justify-center p-6 lg:px-8" aria-label="Global">
                    <div class="flex gap-x-12 border-b border-black px-12 py-4">
                        <div class="flex-1">
                            <a href="/">home</a>
                        </div>
                        <div class="flex-1">
                            <a href="/projects">projects</a>
                        </div>
                        <div class="flex-1">
                            <a href="/blog">blog</a>
                        </div>
                    </div>
                </nav>
            </div>
            <main class="bg-red-100 min-h-screen overscroll-none">
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/projects") view=Projects />
                    <Route path=path!("/blog") view=Blog />
                    <Route path=path!("/blog/:id") view=BlogPost />
                </Routes>
            </main>
        </Router>
    }
}
