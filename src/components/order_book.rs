use dioxus::prelude::*;
use crate::theme::Theme;
use common::{OrderBookData, Order, OrderSide, mock};

#[component]
pub fn OrderBook() -> Element {
    let orderbook = use_state(|| mock::generate_mock_orderbook());
    let refresh_orderbook = move |_| {
        orderbook.set(mock::generate_mock_orderbook());
    };

    // Calculate totals for each order
    let asks_with_total: Vec<(Order, f64)> = orderbook.asks.iter()
        .scan(0.0, |acc, order| {
            *acc += order.size;
            Some((order.clone(), *acc))
        })
        .collect();

    let bids_with_total: Vec<(Order, f64)> = orderbook.bids.iter()
        .scan(0.0, |acc, order| {
            *acc += order.size;
            Some((order.clone(), *acc))
        })
        .collect();

    // Format price with commas and fixed decimals
    let format_price = |price: f64| format!("{:,.1}", price);
    let format_size = |size: f64| format!("{:.8}", size);
    let format_total = |total: f64| format!("{:.2}", total);

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
                    button { 
                        class: Theme::cx(&[Theme::TEXT_MUTED, "hover:text-[var(--text-primary)]"]),
                        onclick: refresh_orderbook,
                        "⟲" 
                    }
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
                    asks_with_total.iter().map(|(order, total)| {
                        rsx! {
                            div { 
                                class: Theme::cx(&["grid grid-cols-[2fr,2fr,1fr] gap-2", Theme::TEXT_SM, Theme::FONT_MONO]),
                                span { class: Theme::TEXT_DANGER, "{format_price(order.price)}" }
                                span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "{format_size(order.size)}" }
                                span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "{format_total(total * order.price)}" }
                            }
                        }
                    })
                }

                // Current price (using first bid as current price)
                {
                    let current_price = orderbook.bids.first().map(|order| order.price).unwrap_or(0.0);
                    rsx! {
                        div { 
                            class: Theme::cx(&[
                                "grid grid-cols-[2fr,2fr,1fr] gap-2 py-2",
                                Theme::BORDER, Theme::FONT_MONO
                            ]),
                            div { 
                                class: Theme::cx(&[Theme::TEXT_SUCCESS, Theme::FONT_MEDIUM, Theme::FLEX, Theme::ITEMS_CENTER]),
                                span { "{format_price(current_price)}" }
                                span { class: Theme::cx(&["ml-1", Theme::TEXT_XS]), "↑" }
                            }
                            span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY, Theme::TEXT_SM]), "-" }
                            span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY, Theme::TEXT_SM]), "-" }
                        }
                    }
                }

                // Buy orders (green)
                div { class: "space-y-0.5 mt-2",
                    bids_with_total.iter().map(|(order, total)| {
                        rsx! {
                            div {
                        class: Theme::cx(&["grid grid-cols-[2fr,2fr,1fr] gap-2", Theme::TEXT_SM, Theme::FONT_MONO]),
                                span { class: Theme::TEXT_SUCCESS, "{format_price(order.price)}" }
                                span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "{format_size(order.size)}" }
                                span { class: Theme::cx(&["text-right", Theme::TEXT_PRIMARY]), "{format_total(total * order.price)}" }
                            }
                        }
                    })
                }
            }
        }
    }
}
