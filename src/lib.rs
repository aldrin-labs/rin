use dioxus::prelude::*;
use dioxus::launch;
use crate::theme::ThemeVariant;

pub mod api;
pub mod auth;
pub mod components;
pub mod pages;
pub mod store;
pub mod theme;
pub mod ui;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    About,
}

#[component]
pub fn App() -> Element {
    let theme = use_signal(|| ThemeVariant::Dark);
    let route = use_signal(|| Route::Home);
    
    // Initialize theme CSS variables
    let theme_css = theme::get_css_variables(theme.read().colors());

    rsx! {
        style { "{theme_css}" }
        div {
            class: "min-h-screen bg-[var(--bg-primary)] text-[var(--text-primary)]",
            // Single Navigation instance
            components::Navigation { 
                theme: theme.clone(),
                route: route.clone()
            }

            // Main content
            main {
                class: "container mx-auto px-4",
                match *route.read() {
                    Route::Home => rsx! { pages::home::HomePage { theme: theme } },
                    Route::About => rsx! { pages::about::AboutPage { 
                        theme: theme,
                        route: route
                    } }
                }
            }
        }
    }
}

pub fn main() {
    launch(App);
}
