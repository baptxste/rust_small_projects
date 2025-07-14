//! About page component for the Dioxus foldable menu application

use dioxus::prelude::*;

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("assets/style.css"),
        }
        div {
            class: "page-container-narrow",

            div {
                class: "card",

                h2 {
                    class: "page-title-center",
                    "📖 À propos de nous"
                }

                p {
                    class: "about-text",
                    "Cette application démontre comment créer une structure stable avec Dioxus.
                    Le header, la navigation et le footer restent fixes, tandis que le contenu central
                    change dynamiquement selon la page sélectionnée."
                }

                div {
                    class: "tech-info",
                    h4 { "🔧 Technologies utilisées :" }
                    ul {
                        li { "Dioxus 0.7 - Framework UI en Rust" }
                        li { "CSS intégré pour le styling" }
                        li { "Gestion d'état avec use_signal" }
                        li { "Composants modulaires" }
                        li { "Architecture séparée en modules" }
                    }
                }
            }
        }
    }
}
