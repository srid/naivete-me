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

                    // Trying raw HTML in a few places as part of exploration
                    // For flexible visualization, I'm considering getting rid of markdown entirely. leptos-markdown rendering is bad anyway.
                    <Markdown src=indoc! {
                        r#"
                    > **Naiveté** is that intimate aspect of oneself that one usually keeps hidden away for fear of seeming foolish ... it is like being a child again, but with adult sensibilities, which means that one can separate out the distinction between being naïve and being gullible.
                    >
                    > Some synonyms of **naiveté** are: _guileless, artless, simple, ingenuous, innocuous, unsophisticated, artless, frank, open_. -[source](https://actualfreedom.com.au/richard/selectedcorrespondence/sc-naivete.htm)
                    "#
                    } />

                    <h2>"Various aspects"</h2>

                    <p>Here we explore various aspects of naiveté.</p>

                    <h3 id="open">"Naively open and sincere"</h3>

                    <p>
                        "The usual modus operandi when investigating feelings is to be closed (and cunning), by predicting a particular way of doing it and framing oneself in that closed fashion. Since this is the only moment there is — and the future cannot be predestined — one might as well remain open (be naively open) to whatever pops up. Stay fascinatedly curious as to how this moment will unfold: whenever one is no longer feeling good, one can look at "
                        <strong>"whatever"</strong>
                        " that is, without needing to maintain a premonition about it."
                    </p>

                    <p>
                        The openness of naiveté acts to counteract the self-centered outlook of the identity.
                    </p>

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
