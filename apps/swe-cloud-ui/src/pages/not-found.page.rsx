//! NotFound page component.
//!
//! Customizable 404 page that reads configuration from routes.yaml.
//! Route: *

use rustscript::prelude::*;
use rustscript::router::use_not_found_config;

#[derive(Props, Default)]
pub struct NotFoundProps {
    /// Override heading text
    #[prop(default)]
    pub heading: Option<String>,

    /// Override message text
    #[prop(default)]
    pub message: Option<String>,

    /// Override home link visibility
    #[prop(default)]
    pub show_home_link: Option<bool>,

    /// Override back button visibility
    #[prop(default)]
    pub show_back_button: Option<bool>,

    /// Custom CSS class
    #[prop(default)]
    pub class: Option<String>,
}

#[page]
pub fn NotFound(props: NotFoundProps) -> Element {
    let config = use_not_found_config();
    let navigate = use_navigate();

    // Use props or fall back to config
    let heading = props.heading.unwrap_or_else(|| config.heading.clone());
    let message = props.message.unwrap_or_else(|| config.message.clone());
    let show_home_link = props.show_home_link.unwrap_or(config.show_home_link);
    let show_back_button = props.show_back_button.unwrap_or(config.show_back_button);
    let class = props.class.or_else(|| config.class.clone());

    let go_back = move |_| {
        navigate.back();
    };

    let go_home = move |_| {
        navigate(&config.home_link_path);
    };

    rsx! {
        <div class={format!("not-found-page {}", class.unwrap_or_default())}>
            {if let Some(image) = &config.image {
                rsx! {
                    <div class="not-found-image">
                        <img src={image.clone()} alt="Page not found" />
                    </div>
                }
            }}

            <div class="not-found-content">
                <h1 class="not-found-heading">{heading}</h1>
                <p class="not-found-message">{message}</p>

                <div class="not-found-actions">
                    {if show_back_button {
                        rsx! {
                            <button
                                class="btn btn-secondary"
                                onclick={go_back}
                            >
                                {config.back_button_text.clone()}
                            </button>
                        }
                    }}

                    {if show_home_link {
                        rsx! {
                            <button
                                class="btn btn-primary"
                                onclick={go_home}
                            >
                                {config.home_link_text.clone()}
                            </button>
                        }
                    }}
                </div>

                {if config.show_suggestions {
                    rsx! {
                        <NotFoundSuggestions />
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn NotFoundSuggestions() -> Element {
    let suggestions = use_route_suggestions();

    if suggestions.is_empty() {
        return rsx! {};
    }

    rsx! {
        <div class="not-found-suggestions">
            <p class="suggestions-label">"Maybe you were looking for:"</p>
            <ul class="suggestions-list">
                {for suggestion in suggestions.iter() {
                    rsx! {
                        <li key={suggestion.path.clone()}>
                            <a href={suggestion.path.clone()}>
                                {suggestion.title.clone()}
                            </a>
                        </li>
                    }
                }}
            </ul>
        </div>
    }
}
