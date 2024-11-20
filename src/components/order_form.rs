use dioxus::prelude::*;
use crate::theme::{Theme, ButtonVariant};

#[component]
pub fn OrderForm() -> Element {
    rsx! {
        div { 
            class: Theme::card("w-[300px]"),
            // Header with controls
            div { 
                class: Theme::cx(&[
                    Theme::FLEX, Theme::ITEMS_CENTER, Theme::JUSTIFY_BETWEEN,
                    Theme::BORDER_B, "p-2"
                ]),
                div { 
                    class: Theme::cx(&[Theme::FLEX, Theme::ITEMS_CENTER]),
                    span { class: Theme::cx(&[Theme::TEXT_MUTED, Theme::TEXT_SM]), "Order form" }
                    button { class: Theme::cx(&[Theme::TEXT_MUTED, "hover:text-[var(--text-primary)] text-lg"]), "×" }
                }
                button { class: Theme::cx(&[Theme::TEXT_MUTED, "hover:text-[var(--text-primary)] text-lg"]), "⤢" }
            }

            div { class: "p-2",
                // Spot/Margin tabs
                div { class: "flex mb-3",
                    button { class: Theme::button(ButtonVariant::Primary, Theme::ROUNDED_L), "Spot" }
                    button { 
                        class: Theme::cx(&[
                            "flex-1 py-1.5", Theme::BG_PRIMARY, Theme::ROUNDED_R,
                            Theme::TEXT_SM, Theme::TEXT_MUTED
                        ]), 
                        "Margin" 
                    }
                }

                // Buy/Sell tabs
                div { class: "flex mb-3",
                    button { 
                        class: Theme::cx(&[
                            "flex-1 py-1.5", Theme::TEXT_SM, Theme::TEXT_SUCCESS,
                            "border-b-2 border-[var(--text-success)]"
                        ]), 
                        "Buy" 
                    }
                    button { class: Theme::cx(&["flex-1 py-1.5", Theme::TEXT_SM, Theme::TEXT_MUTED]), "Sell" }
                }

                // Order type dropdown
                div { class: "mb-3",
                    button { 
                        class: Theme::button(ButtonVariant::Primary, "w-full text-left flex justify-between items-center"),
                        span { "Limit" }
                        span { "▼" }
                    }
                }

                // Price input
                div { class: "mb-3",
                    label { 
                        class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "mb-1 block"]), 
                        "Limit price" 
                    }
                    div { class: "relative",
                        input { class: Theme::input(""), "type": "text", value: "94319.4" }
                    }
                }

                // Amount input
                div { class: "mb-3",
                    label { 
                        class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "mb-1 block"]), 
                        "Amount" 
                    }
                    div { class: "relative",
                        input { class: Theme::input(""), "type": "text", value: "0.00005" }
                    }
                }

                // Total
                div { class: "mb-3",
                    label { 
                        class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "mb-1 block"]), 
                        "Total" 
                    }
                    div { class: Theme::FLEX,
                        input { class: Theme::input(""), "type": "text", value: "4.72" }
                        span { 
                            class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "ml-2"]), 
                            "USD" 
                        }
                    }
                }

                // Place order button
                button { 
                    class: Theme::cx(&[
                        "w-full py-2 bg-[var(--text-success)]", Theme::ROUNDED,
                        Theme::TEXT_SM, Theme::TEXT_SECONDARY, Theme::FONT_MEDIUM
                    ]),
                    "Place buy order"
                }
            }
        }
    }
}
