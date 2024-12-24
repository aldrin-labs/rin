use dioxus::prelude::*;
use crate::theme::{Theme, ButtonVariant};
use common::{Market, Candle, mock};

#[derive(PartialEq)]
enum TimeInterval {
    OneMin,
    FiveMin,
    FifteenMin,
    ThirtyMin,
    OneHour,
    FourHour,
    OneDay,
}

#[component]
pub fn TradingChart() -> Element {
    let market = use_state(|| mock::generate_mock_market());
    let candles = use_state(|| mock::generate_mock_candles());
    let selected_interval = use_state(|| TimeInterval::OneDay);

    let refresh_data = move |_| {
        candles.set(mock::generate_mock_candles());
    };

    let format_price = |price: f64| format!("{:,.1}", price);
    
    // Get latest price from candles
    let latest_price = candles.first()
        .map(|candle| candle.close)
        .unwrap_or(0.0);

    let interval_button = move |interval: TimeInterval, label: &'static str| {
        let is_selected = *selected_interval == interval;
        rsx! {
            button { 
                class: Theme::cx(&[
                    "px-2 py-1", Theme::TEXT_XS,
                    if is_selected { 
                        Theme::cx(&[Theme::TEXT_PRIMARY, Theme::BG_BUTTON_ACTIVE])
                    } else {
                        Theme::cx(&[Theme::TEXT_MUTED, "hover:bg-[var(--button)] hover:text-[var(--text-secondary)]"])
                    },
                    Theme::ROUNDED
                ]),
                onclick: move |_| selected_interval.set(interval),
                "{label}"
            }
        }
    };

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
                            "{market.symbol}" 
                        }
                        button { 
                            class: Theme::button(ButtonVariant::Primary, Theme::TEXT_XS),
                            "Add a market"
                        }
                    }

                    // Time intervals
                    div { class: Theme::FLEX,
                        interval_button(TimeInterval::OneMin, "1m")
                        interval_button(TimeInterval::FiveMin, "5m")
                        interval_button(TimeInterval::FifteenMin, "15m")
                        interval_button(TimeInterval::ThirtyMin, "30m")
                        interval_button(TimeInterval::OneHour, "1h")
                        interval_button(TimeInterval::FourHour, "4h")
                        interval_button(TimeInterval::OneDay, "1D")
                    }

                    // Indicators and refresh buttons
                    div { class: "flex items-center ml-auto",
                        button { 
                            class: Theme::cx(&[Theme::TEXT_MUTED, "hover:text-[var(--text-primary)] mr-2"]),
                            onclick: refresh_data,
                            "⟲"
                        }
                        button { 
                            class: Theme::button(ButtonVariant::Primary, Theme::TEXT_XS),
                            "Indicators"
                        }
                    }
                }
            }

            // Chart area with price info
            div { class: "flex-1 relative",
                div { 
                    class: Theme::cx(&[
                        "absolute inset-0",
                        Theme::BG_PRIMARY
                    ]),
                    // Price info overlay
                    div { 
                        class: "absolute top-4 left-4 p-4 bg-[var(--card)] rounded-lg shadow-lg",
                        div { 
                            class: Theme::cx(&[Theme::TEXT_PRIMARY, Theme::FONT_MEDIUM]),
                            "Current Price"
                        }
                        div { 
                            class: Theme::cx(&[Theme::TEXT_SUCCESS, "text-2xl", Theme::FONT_MEDIUM]),
                            "{format_price(latest_price)}"
                        }
                        div {
                            class: Theme::cx(&[Theme::TEXT_MUTED, Theme::TEXT_SM]),
                            "24h Change: +2.5%"
                        }
                    }
                    
                    // Placeholder for actual chart
                    div {
                        class: "absolute inset-0 flex items-center justify-center",
                        span { class: Theme::TEXT_MUTED, "Chart visualization coming soon" }
                    }
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
                            span { class: Theme::TEXT_XS, "Market Info" }
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
                    // Market stats
                    div { 
                        class: Theme::cx(&["flex items-center ml-4", Theme::TEXT_XS, Theme::TEXT_MUTED]),
                        "24h Vol: {format_price(1234567.89)} {market.quote_currency}"
                    }
                }
            }
        }
    }
}
