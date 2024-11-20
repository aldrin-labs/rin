use dioxus::prelude::*;

#[component]
pub fn TradingChart() -> Element {
    rsx! {
        div { 
            class: "bg-navy-900 flex-1 p-4",
            div { class: "flex justify-between items-center mb-4",
                div { class: "flex items-center space-x-4",
                    span { class: "text-white", "BTC/USD" }
                    button { class: "bg-gray-700 px-4 py-1 rounded", "Add a market" }
                }
                div { class: "flex space-x-2",
                    button { class: "bg-gray-700 px-4 py-1 rounded", "1m" }
                    button { class: "bg-gray-700 px-4 py-1 rounded", "5m" }
                    button { class: "bg-gray-700 px-4 py-1 rounded", "15m" }
                    button { class: "bg-gray-700 px-4 py-1 rounded", "30m" }
                    button { class: "bg-gray-700 px-4 py-1 rounded", "1h" }
                    button { class: "bg-gray-700 px-4 py-1 rounded", "4h" }
                    button { class: "bg-gray-700 px-4 py-1 rounded", "1D" }
                    button { class: "bg-gray-700 px-4 py-1 rounded", "Indicators" }
                }
            }
            div { 
                id: "chart",
                class: "w-full h-[500px] bg-navy-800 rounded"
            }
        }
    }
}