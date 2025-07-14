//! Home page component for the Dioxus foldable menu application

use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("assets/style.css"),
        }
        div {
            class: "page-container",

            h2 {
                class: "page-title",
                "🏠 Bienvenue sur la page d'accueil"
            }

            div {
                class: "grid-2",

                div {
                    class: "card-small",
                    h3 { "✨ Fonctionnalités" }
                    ul {
                        li { "Layout stable et responsive" }
                        li { "Navigation fluide" }
                        li { "Composants réutilisables" }
                        li { "Gestion d'état avec Dioxus" }
                    }
                }

                div {
                    class: "card-small",
                    h3 { "🚀 Avantages" }
                    ul {
                        li { "Performance optimale" }
                        li { "Code Rust moderne" }
                        li { "Interface réactive" }
                        li { "Facilité de maintenance" }
                    }
                }
            }
        }
    }
}
