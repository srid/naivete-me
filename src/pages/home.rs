use crate::components::counter_btn::Button;
use leptos::*;
use leptos_markdown::Markdown;

const CONTENT: &str = include_str!("content.md");

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">
                <h1>"Naiveté explained"</h1>

                <div style="text-align: left;">
                    <Markdown src=CONTENT />

                </div>

                <div class="buttons">
                    <Button />
                    <Button increment=5 />
                </div>

            </div>
        </ErrorBoundary>
    }
}
