use dioxus::prelude::*;
use crate::theme::{Theme, ButtonVariant};
use common::{Order, OrderSide, Market, mock};

#[derive(PartialEq, Clone)]
enum OrderType {
    Limit,
    Market,
}

#[derive(PartialEq, Clone)]
enum TradeMode {
    Spot,
    Margin,
}

#[component]
pub fn OrderForm() -> Element {
    let market = use_state(|| mock::generate_mock_market());
    let order_type = use_state(|| OrderType::Limit);
    let trade_mode = use_state(|| TradeMode::Spot);
    let side = use_state(|| OrderSide::Buy);
    let price = use_state(|| "94319.4".to_string());
    let amount = use_state(|| "0.00005".to_string());
    
    // Calculate total based on price and amount
    let total = price.parse::<f64>().unwrap_or(0.0) * amount.parse::<f64>().unwrap_or(0.0);
    
    // Format numbers according to market decimals
    let format_price = |p: f64| format!("{:.*}", market.price_decimals as usize, p);
    let format_size = |s: f64| format!("{:.*}", market.size_decimals as usize, s);
    let format_total = |t: f64| format!("{:.2}", t);

    // Handle order submission
    let submit_order = move |_| {
        let order = Order {
            price: price.parse().unwrap_or(0.0),
            size: amount.parse().unwrap_or(0.0),
            side: side.clone(),
        };
        // TODO: Implement order submission logic
        log::info!("Submitting order: {:?}", order);
    };

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
                    button { 
                        class: Theme::cx(&[
                            "flex-1 py-1.5",
                            if *trade_mode == TradeMode::Spot { Theme::button(ButtonVariant::Primary, Theme::ROUNDED_L) } 
                            else { Theme::cx(&[Theme::BG_PRIMARY, Theme::ROUNDED_L, Theme::TEXT_SM, Theme::TEXT_MUTED]) }
                        ]),
                        onclick: move |_| trade_mode.set(TradeMode::Spot),
                        "Spot"
                    }
                    button { 
                        class: Theme::cx(&[
                            "flex-1 py-1.5",
                            if *trade_mode == TradeMode::Margin { Theme::button(ButtonVariant::Primary, Theme::ROUNDED_R) }
                            else { Theme::cx(&[Theme::BG_PRIMARY, Theme::ROUNDED_R, Theme::TEXT_SM, Theme::TEXT_MUTED]) }
                        ]),
                        onclick: move |_| trade_mode.set(TradeMode::Margin),
                        "Margin"
                    }
                }

                // Buy/Sell tabs
                div { class: "flex mb-3",
                    button { 
                        class: Theme::cx(&[
                            "flex-1 py-1.5", Theme::TEXT_SM,
                            if *side == OrderSide::Buy { 
                                Theme::cx(&[Theme::TEXT_SUCCESS, "border-b-2 border-[var(--text-success)]"])
                            } else {
                                Theme::TEXT_MUTED
                            }
                        ]),
                        onclick: move |_| side.set(OrderSide::Buy),
                        "Buy"
                    }
                    button { 
                        class: Theme::cx(&[
                            "flex-1 py-1.5", Theme::TEXT_SM,
                            if *side == OrderSide::Sell { 
                                Theme::cx(&[Theme::TEXT_DANGER, "border-b-2 border-[var(--text-danger)]"])
                            } else {
                                Theme::TEXT_MUTED
                            }
                        ]),
                        onclick: move |_| side.set(OrderSide::Sell),
                        "Sell"
                    }
                }

                // Order type dropdown
                div { class: "mb-3",
                    button { 
                        class: Theme::button(ButtonVariant::Primary, "w-full text-left flex justify-between items-center"),
                        onclick: move |_| order_type.set(if *order_type == OrderType::Limit { OrderType::Market } else { OrderType::Limit }),
                        span { "{if *order_type == OrderType::Limit { \"Limit\" } else { \"Market\" }}" }
                        span { "▼" }
                    }
                }

                // Price input (hidden for market orders)
                {
                    if *order_type == OrderType::Limit {
                        rsx! {
                            div { class: "mb-3",
                                label { 
                                    class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "mb-1 block"]), 
                                    "Limit price" 
                                }
                                div { class: "relative",
                                    input { 
                                        class: Theme::input(""),
                                        "type": "text",
                                        value: "{price}",
                                        oninput: move |evt| price.set(evt.value.clone())
                                    }
                                }
                            }
                        }
                    }
                }

                // Amount input
                div { class: "mb-3",
                    label { 
                        class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "mb-1 block"]), 
                        "Amount ({market.base_currency})" 
                    }
                    div { class: "relative",
                        input { 
                            class: Theme::input(""),
                            "type": "text",
                            value: "{amount}",
                            oninput: move |evt| amount.set(evt.value.clone())
                        }
                    }
                }

                // Total
                div { class: "mb-3",
                    label { 
                        class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "mb-1 block"]), 
                        "Total" 
                    }
                    div { class: Theme::FLEX,
                        input { 
                            class: Theme::input(""),
                            "type": "text",
                            value: "{format_total(total)}",
                            disabled: true
                        }
                        span { 
                            class: Theme::cx(&[Theme::TEXT_XS, Theme::TEXT_MUTED, "ml-2"]), 
                            "{market.quote_currency}" 
                        }
                    }
                }

                // Place order button
                button { 
                    class: Theme::cx(&[
                        "w-full py-2", Theme::ROUNDED,
                        Theme::TEXT_SM, Theme::TEXT_SECONDARY, Theme::FONT_MEDIUM,
                        if *side == OrderSide::Buy { "bg-[var(--text-success)]" } else { "bg-[var(--text-danger)]" }
                    ]),
                    onclick: submit_order,
                    "Place {if *side == OrderSide::Buy { \"buy\" } else { \"sell\" }} order"
                }
            }
        }
    }
}
