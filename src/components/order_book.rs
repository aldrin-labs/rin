use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn OrderBook() -> Element {
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
                    span { class: Theme::cx(&[Theme::TEXT_MUTED, Theme::TEXT_SM]), "Order book" }
                    button { class: Theme::cx(&[Theme::TEXT_MUTED, "hover:text-[var(--text-primary)] text-lg"]), "×" }
                }
                div { 
                    class: Theme::cx(&[Theme::FLEX, Theme::ITEMS_CENTER]),
                    button { class: Theme::cx(&[Theme::TEXT_MUTED, "hover:text-[var(--text-primary)]"]), "⟲" }
                    button { class: Theme::cx(&[Theme::TEXT_MUTED, "hover:text-[var(--text-primary)] text-lg"]), "⤢" }
                }
            }

            div { class: "p-2",
                // Column headers
                div { 
                    class: Theme::cx(&[
                        "grid grid-cols-[2fr,2fr,1fr] gap-2",
                        Theme::TEXT_XS, Theme::TEXT_MUTED, "pb-2"
                    ]),
                    span { "Price" }
                    span { class: "text-right", "Size" }
                    span { class: "text-right", "Total" }
                }

                // Sell orders (red)
                div { class: "space-y-0.5 mb-2",
                    div { 
                        class: Theme::cx(&["grid grid-cols-[2fr,2fr,1fr] gap-2", Theme::TEXT_SM, Theme::FONT_MONO]),
                        span { class: Theme::TEXT_DANGER, "94323.7" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "0.00031220" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "4.72" }
                    }
                    div { 
                        class: Theme::cx(&["grid grid-cols-[2fr,2fr,1fr] gap-2", Theme::TEXT_SM, Theme::FONT_MONO]),
                        span { class: Theme::TEXT_DANGER, "94321.0" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "0.00031220" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "4.72" }
                    }
                    div { 
                        class: Theme::cx(&["grid grid-cols-[2fr,2fr,1fr] gap-2", Theme::TEXT_SM, Theme::FONT_MONO]),
                        span { class: Theme::TEXT_DANGER, "94320.7" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "0.00031220" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "4.72" }
                    }
                }

                // Current price
                div { 
                    class: Theme::cx(&[
                        "grid grid-cols-[2fr,2fr,1fr] gap-2 py-2",
                        Theme::BORDER, Theme::FONT_MONO
                    ]),
                    div { 
                        class: Theme::cx(&[Theme::TEXT_SUCCESS, Theme::FONT_MEDIUM, Theme::FLEX, Theme::ITEMS_CENTER]),
                        span { "94,319.4" }
                        span { class: Theme::cx(&["ml-1", Theme::TEXT_XS]), "↑" }
                    }
                    span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY, Theme::TEXT_SM]), "0.00031220" }
                    span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY, Theme::TEXT_SM]), "0.1" }
                }

                // Buy orders (green)
                div { class: "space-y-0.5 mt-2",
                    div { 
                        class: Theme::cx(&["grid grid-cols-[2fr,2fr,1fr] gap-2", Theme::TEXT_SM, Theme::FONT_MONO]),
                        span { class: Theme::TEXT_SUCCESS, "94318.9" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "0.21551196" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "4.72" }
                    }
                    div { 
                        class: Theme::cx(&["grid grid-cols-[2fr,2fr,1fr] gap-2", Theme::TEXT_SM, Theme::FONT_MONO]),
                        span { class: Theme::TEXT_SUCCESS, "94318.8" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "0.21551196" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "4.72" }
                    }
                    div { 
                        class: Theme::cx(&["grid grid-cols-[2fr,2fr,1fr] gap-2", Theme::TEXT_SM, Theme::FONT_MONO]),
                        span { class: Theme::TEXT_SUCCESS, "94317.7" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "0.21551196" }
                        span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "4.72" }
                    }
                }
            }
        }
    }
}
