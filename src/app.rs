use crate::{pages::dashboard::DashBoard, service::settings::Settings};
use config::ConfigError;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let settings: Result<Settings, ConfigError> = Settings::new();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/simple-dashboard-project.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/dashboard" view=DashBoard/>
                    // <Route path="/dashboard" view=move || {
                    //       view! {
                    //         <DashBoard settings=settings.as_ref().unwrap()/>
                    //       }
                    // }/>


                    <Route path="/demo" view=Demo/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
                " | Some more text"
            </button>
        </main>
    }
}

#[component]
fn Demo() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                <div class="bg-white shadow rounded p-4">
                  <h3 class="text-lg font-medium mb-2">Card Title</h3>
                  <p>Card content goes here. You can replace this with data from your backend.</p>
                </div>
                <div class="bg-white shadow rounded p-4">
                  <h3 class="text-lg font-medium mb-2">Card Title</h3>
                  <p>Card content goes here. You can replace this with data from your backend.</p>
                </div>
                <div class="bg-white shadow rounded p-4">
                  <h3 class="text-lg font-medium mb-2">Card Title</h3>
                  <p>Card content goes here. You can replace this with data from your backend.</p>
                </div>
                <div class="bg-white shadow rounded p-4">
                  <h3 class="text-lg font-medium mb-2">Card Title</h3>
                  <p>Card content goes here. You can replace this with data from your backend.</p>
                </div>
        </div>
    }
}
/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <main class="my-0 mx-auto max-w-3xl text-center bg-black">
            <div class="absolute h-full w-full -z-10 bg-black left-0"/>
            <img class="absolute h-full" src="https://http.dog/404.jpg"/>
        </main>
    }
}
