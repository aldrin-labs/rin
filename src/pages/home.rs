use crate::components::{TradingPair, OrderForm, OrderBook, TradingChart};
use crate::theme::{Theme, ButtonVariant, ThemeVariant};
use dioxus::prelude::*;

#[component]
pub fn HomePage(theme: Signal<ThemeVariant>) -> Element {
    rsx! {
        div { 
            class: Theme::cx(&[Theme::FLEX_COL]),
            // Trading pair info
            TradingPair {}

            // Trading interface
            div { class: "flex flex-1 p-2 space-x-2",
                // Left side - Order form and Order book
                div { class: "flex space-x-2 min-w-fit",
                    OrderForm {}
                    OrderBook {}
                }

                // Right side - Chart
                div { class: "flex-1 min-w-0",
                    TradingChart {}
                }
            }

            // Bottom status bar
            div { 
                class: Theme::cx(&[
                    Theme::BG_PRIMARY, Theme::FLEX, Theme::ITEMS_CENTER,
                    Theme::BORDER_T, "h-10 px-4"
                ]),
                // Left side - Online status and pairs
                div { class: "flex items-center space-x-6",
                    // Online status
                    div { class: Theme::FLEX,
                        div { class: "w-1.5 h-1.5 rounded-full bg-[var(--text-success)] mr-1.5" }
                        span { class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED]), "Online" }
                    }
                    // Trading pairs
                    div { class: "flex items-center space-x-4",
                        div { class: "flex items-center space-x-1",
                            span { class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED]), "MTR/USD" }
                            span { class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_SUCCESS]), "146.06%" }
                        }
                        div { class: "flex items-center space-x-1",
                            span { class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED]), "JUNO/USD" }
                            span { class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_SUCCESS]), "72.43%" }
                        }
                        div { class: "flex items-center space-x-1",
                            span { class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED]), "GARI/USD" }
                            span { class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_SUCCESS]), "63.95%" }
                        }
                    }
                }

                // Right side - Links
                div { class: "flex items-center space-x-2 ml-auto",
                    button { class: Theme::button(ButtonVariant::Primary, Theme::TEXT_XS), "API Docs" }
                    button { class: Theme::button(ButtonVariant::Primary, Theme::TEXT_XS), "Important information" }
                    button { 
                        class: Theme::button(ButtonVariant::Primary, &Theme::cx(&[Theme::TEXT_XS, "flex items-center"])),
                        "Chat with us"
                        span { class: "ml-1", "↗" }
                    }
                }
            }
        }
    }
}
