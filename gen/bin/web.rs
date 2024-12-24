use dioxus::prelude::*;
use t5_rs::components::{
    nav::Nav,
    order_book::OrderBook,
    trading_chart::TradingChart,
    order_form::OrderForm,
};

fn main() {
    // Initialize the logger for web target
    wasm_logger::init(wasm_logger::Config::default());
    
    // Launch the web application
    dioxus_web::launch(app);
}

fn app() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Nav {}
            main { class: "flex-1 container mx-auto p-4",
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    TradingChart {}
                    OrderBook {}
                    OrderForm {}
                }
            }
        }
    }
}
