use dioxus::prelude::*;
use crate::theme::{Theme, ButtonVariant, ThemeVariant};
use crate::Route;

#[component]
pub fn Navigation(theme: Signal<ThemeVariant>, route: Signal<Route>) -> Element {
    rsx! {
        nav { 
            class: Theme::cx(&[Theme::BG_PRIMARY, "p-2 border-[var(--border)]"]),
            div { 
                class: Theme::cx(&[Theme::FLEX, Theme::ITEMS_CENTER, "container mx-auto px-2"]),
                // Left section - Logo and main nav
                div { 
                    class: Theme::cx(&[Theme::FLEX, Theme::ITEMS_CENTER, "flex-1"]),
                    span { 
                        class: Theme::cx(&["text-[var(--brand-primary)]", Theme::FONT_BOLD, "mr-6", Theme::TEXT_BASE]), 
                        "PRO" 
                    }
                    div { 
                        class: "space-x-1",
                        button { 
                            class: Theme::button(ButtonVariant::Primary, Theme::TEXT_SM), 
                            onclick: move |_| route.set(Route::Home),
                            "Trade" 
                        }
                        button { 
                            class: Theme::button(ButtonVariant::Primary, Theme::TEXT_SM), 
                            onclick: move |_| route.set(Route::About),
                            "About" 
                        }
                        button { 
                            class: Theme::button(ButtonVariant::Primary, Theme::TEXT_SM), 
                            "Portfolio" 
                        }
                        button { 
                            class: Theme::button(ButtonVariant::Primary, Theme::TEXT_SM), 
                            "Earn" 
                        }
                        button { 
                            class: Theme::button(ButtonVariant::Primary, Theme::TEXT_SM), 
                            "History" 
                        }
                        button { 
                            class: Theme::button(ButtonVariant::Primary, Theme::TEXT_SM), 
                            "OTC" 
                        }
                    }
                }

                // Right section - Theme selector and Auth buttons
                div { 
                    class: Theme::cx(&[Theme::FLEX, Theme::ITEMS_CENTER, "space-x-4"]),
                    // Theme selector
                    div {
                        class: Theme::cx(&[Theme::FLEX, Theme::ITEMS_CENTER]),
                        div {
                            class: Theme::cx(&[
                                Theme::FLEX, Theme::ITEMS_CENTER, Theme::BG_SECONDARY,
                                Theme::ROUNDED, "px-2 py-1.5 border border-[var(--border)]",
                                "hover:border-[var(--brand-primary)]"
                            ]),
                            span { 
                                class: Theme::cx(&[Theme::TEXT_PRIMARY, Theme::TEXT_SM, "mr-2"]), 
                                "🎨" 
                            }
                            select {
                                class: Theme::cx(&[
                                    Theme::BG_SECONDARY, Theme::TEXT_PRIMARY, Theme::TEXT_SM,
                                    "outline-none cursor-pointer appearance-none pl-1 pr-6",
                                    "hover:text-[var(--brand-primary)]"
                                ]),
                                onchange: move |evt| {
                                    if let Ok(idx) = evt.value().parse::<usize>() {
                                        if let Some((variant, _)) = ThemeVariant::all().get(idx) {
                                            theme.set(*variant);
                                        }
                                    }
                                },
                                {ThemeVariant::all().iter().enumerate().map(|(idx, (variant, name))| {
                                    rsx! {
                                        option {
                                            key: "{idx}",
                                            value: "{idx}",
                                            selected: *variant == *theme.read(),
                                            {name}
                                        }
                                    }
                                })}
                            }
                        }
                    }
                    // Auth buttons
                    button { 
                        class: Theme::button(ButtonVariant::Primary, Theme::TEXT_SM), 
                        "Sign in" 
                    }
                    button { 
                        class: Theme::button(ButtonVariant::Secondary, Theme::TEXT_SM), 
                        "Sign up" 
                    }
                }
            }
        }
    }
}
