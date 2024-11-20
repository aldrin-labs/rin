use dioxus::prelude::*;
use crate::theme::{Theme, ButtonVariant};

#[component]
pub fn TradingPair() -> Element {
    rsx! {
        div { 
            class: Theme::cx(&[Theme::BG_PRIMARY, Theme::BORDER_B]),
            div { class: "flex items-baseline px-4 py-2",
                // Left side - Trading pair info
                div { class: "flex space-x-6",
                    // Pair name and leverage
                    div { class: Theme::FLEX,
                        span { 
                            class: Theme::cx(&["text-lg", Theme::TEXT_PRIMARY, Theme::FONT_BOLD]), 
                            "BTC/USD" 
                        }
                        span { 
                            class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "ml-1"]), 
                            "5×" 
                        }
                    }
                    // Market stats
                    div { class: "flex space-x-6",
                        // LAST PRICE
                        div {
                            span { 
                                class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "mr-2"]), 
                                "LAST PRICE" 
                            }
                            span { class: Theme::TEXT_SUCCESS, "94,319.5" }
                            span { 
                                class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_SUCCESS, "ml-1"]), 
                                "USD" 
                            }
                        }
                        // 24H CHANGE
                        div {
                            span { 
                                class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "mr-2"]), 
                                "24H CHANGE" 
                            }
                            span { class: Theme::TEXT_SUCCESS, "2.03%" }
                        }
                        // 24H VOLUME
                        div {
                            span { 
                                class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "mr-2"]), 
                                "24H VOLUME" 
                            }
                            span { class: Theme::TEXT_PRIMARY, "4.28" }
                            span { 
                                class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "ml-1"]), 
                                "BTC" 
                            }
                        }
                    }
                }

                // Right side - Advanced button
                button { 
                    class: Theme::button(ButtonVariant::Primary, "ml-auto flex items-center"),
                    "Advanced"
                    span { class: Theme::cx(&["ml-1", Theme::TEXT_XS]), "▼" }
                }
            }
        }
    }
}
