use crate::api;
use crate::ui::*;
use crate::Route;
use dioxus::prelude::*;
use crate::theme::ThemeVariant;
use dotenvy_macro::dotenv;

#[component]
pub fn AboutPage(theme: Signal<ThemeVariant>, route: Signal<Route>) -> Element {
    let authed_image_url: Resource<Result<String, reqwest::Error>> = use_resource(move || {
        api::get_authed_image(
            "".to_string(), // TODO: Fix auth token
            "images/t5.png".to_string(),
        )
    });

    let public_image_url = format!(
        "{}/storage/v1/object/public/public-images/t5.png",
        dotenv!("SUPABASE_URL")
    );

    rsx! {
        div { class: "flex flex-col items-center h-screen bg-gray-900 text-white py-20",
            h1 { class: "text-4xl mb-8 font-bold", "About T5 ⚙️" }
            div { class: "flex flex-row justify-center items-center gap-5 mb-8",
                div { class: "flex flex-col items-center gap-3",
                    i { "Public Image" }
                    img { class: "w-32 h-32", src: public_image_url }
                }
                match &*authed_image_url.read_unchecked() {
                    Some(Ok(url)) => {
                        rsx! {
                            div { class: "flex flex-col items-center gap-3",
                                i { "Authed Image" }
                                img {
                                    class: "w-32 h-32",
                                    src: {
                                        format!(
                                            "{}/storage/v1{}",
                                            dotenv!("SUPABASE_URL"),
                                            &url
                                        )
                                    }
                                }
                            }
                        }
                    }
                    _ => rsx!{},
                }
            }
            p { class: "text-center px-10",
                "An opinionated cross-platform full-stack application template developed with Rust, Cargo Mobile 2, Dioxus, Warp, Diesel, PostgreSQL, Supabase Auth, Bun and TailwindCSS."
            }
            div { class: "mt-6",
                TextButton {
                    text: String::from("Home Page"),
                    class: String::from(""),
                    variant: Variant::Neutral,
                    onclick: move |_| route.set(Route::Home)
                }
            }
        }
    }
}
