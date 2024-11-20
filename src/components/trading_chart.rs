use dioxus::prelude::*;
use crate::theme::{Theme, ButtonVariant};

#[component]
pub fn TradingChart() -> Element {
    rsx! {
        div { 
            class: Theme::card("flex-1 flex flex-col min-h-[600px]"),
            // Chart controls
            div { 
                class: Theme::cx(&[Theme::BORDER_B, "p-2"]),
                div { class: Theme::FLEX,
                    // Market selector and add button
                    div { 
                        class: Theme::cx(&[Theme::FLEX, Theme::ITEMS_CENTER, "mr-4"]),
                        span { 
                            class: Theme::cx(&[Theme::TEXT_PRIMARY, Theme::FONT_MEDIUM]), 
                            "BTC/USD" 
                        }
                        button { 
                            class: Theme::button(ButtonVariant::Primary, Theme::TEXT_XS),
                            "Add a market"
                        }
                    }

                    // Time intervals
                    div { class: Theme::FLEX,
                        button { 
                            class: Theme::cx(&[
                                "px-2 py-1", Theme::TEXT_XS, Theme::TEXT_MUTED, Theme::ROUNDED,
                                "hover:bg-[var(--button)] hover:text-[var(--text-secondary)]"
                            ]), 
                            "1m" 
                        }
                        button { 
                            class: Theme::cx(&[
                                "px-2 py-1", Theme::TEXT_XS, Theme::TEXT_MUTED, Theme::ROUNDED,
                                "hover:bg-[var(--button)] hover:text-[var(--text-secondary)]"
                            ]), 
                            "5m" 
                        }
                        button { 
                            class: Theme::cx(&[
                                "px-2 py-1", Theme::TEXT_XS, Theme::TEXT_MUTED, Theme::ROUNDED,
                                "hover:bg-[var(--button)] hover:text-[var(--text-secondary)]"
                            ]), 
                            "15m" 
                        }
                        button { 
                            class: Theme::cx(&[
                                "px-2 py-1", Theme::TEXT_XS, Theme::TEXT_MUTED, Theme::ROUNDED,
                                "hover:bg-[var(--button)] hover:text-[var(--text-secondary)]"
                            ]), 
                            "30m" 
                        }
                        button { 
                            class: Theme::cx(&[
                                "px-2 py-1", Theme::TEXT_XS, Theme::TEXT_MUTED, Theme::ROUNDED,
                                "hover:bg-[var(--button)] hover:text-[var(--text-secondary)]"
                            ]), 
                            "1h" 
                        }
                        button { 
                            class: Theme::cx(&[
                                "px-2 py-1", Theme::TEXT_XS, Theme::TEXT_MUTED, Theme::ROUNDED,
                                "hover:bg-[var(--button)] hover:text-[var(--text-secondary)]"
                            ]), 
                            "4h" 
                        }
                        button { 
                            class: Theme::cx(&[
                                "px-2 py-1", Theme::TEXT_XS, Theme::TEXT_PRIMARY,
                                Theme::BG_BUTTON_ACTIVE, Theme::ROUNDED
                            ]), 
                            "1D" 
                        }
                    }

                    // Indicators button
                    button { 
                        class: Theme::button(ButtonVariant::Primary, &Theme::cx(&[Theme::TEXT_XS, "ml-auto"])),
                        "Indicators"
                    }
                }
            }

            // Chart area
            div { class: "flex-1 relative",
                div { 
                    class: Theme::cx(&[
                        "absolute inset-0 flex items-center justify-center",
                        Theme::BG_PRIMARY
                    ]),
                    span { class: Theme::TEXT_MUTED, "Trading Chart Coming Soon" }
                }
            }

            // Bottom tabs
            div { class: Theme::BORDER_T,
                div { class: "flex items-center px-2 h-9",
                    // Active tab
                    div { class: "h-full",
                        div { 
                            class: Theme::cx(&[
                                "flex items-center h-full px-3",
                                Theme::BG_BUTTON, Theme::TEXT_SECONDARY, Theme::ROUNDED_T,
                                "space-x-2"
                            ]),
                            span { class: Theme::TEXT_XS, "Balances" }
                            button { 
                                class: Theme::cx(&[
                                    Theme::TEXT_SECONDARY,
                                    "hover:text-[var(--text-primary)]",
                                    Theme::TEXT_XS
                                ]), 
                                "×" 
                            }
                        }
                    }
                    // Inactive tabs
                    button { 
                        class: Theme::cx(&[
                            "h-full px-3", Theme::TEXT_XS, Theme::TEXT_MUTED,
                            "hover:text-[var(--text-primary)]"
                        ]), 
                        "Positions" 
                    }
                    button { 
                        class: Theme::cx(&[
                            "h-full px-3", Theme::TEXT_XS, Theme::TEXT_MUTED,
                            "hover:text-[var(--text-primary)]"
                        ]), 
                        "Open orders" 
                    }
                    button { 
                        class: Theme::cx(&[
                            "h-full px-3", Theme::TEXT_XS, Theme::TEXT_MUTED,
                            "hover:text-[var(--text-primary)]"
                        ]), 
                        "Trades" 
                    }
                }
            }
        }
    }
}
