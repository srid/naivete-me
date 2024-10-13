use crate::components::counter_btn::Button;
use indoc::indoc;
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
                    <Markdown src=indoc! {
                        r#"
                    > **Naiveté** is that intimate aspect of oneself that one usually keeps hidden away for fear of seeming foolish ... it is like being a child again, but with adult sensibilities, which means that one can separate out the distinction between being naïve and being gullible."
                    >
                    > Some synonyms of **naiveté** are: _guileless, artless, simple, ingenuous, innocuous, unsophisticated, artless, frank, open_." -[source](https://actualfreedom.com.au/richard/selectedcorrespondence/sc-naivete.htm)
                    "#
                    } />

                    <h2>"Various aspects"</h2>

                  <p>Here we explore various aspects of naiveté.</p>

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
