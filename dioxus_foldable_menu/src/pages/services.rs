//! Services page component for the Dioxus foldable menu application

use dioxus::prelude::*;

#[component]
pub fn ServicesPage() -> Element {
    let services = [
        ("🎨", "Design UI/UX", "Création d'interfaces modernes"),
        ("⚡", "Performance", "Optimisation et rapidité"),
        ("🔒", "Sécurité", "Protection des données"),
        ("📱", "Responsive", "Compatible tous appareils"),
    ];

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("assets/style.css"),
        }
        div {
            class: "page-container-wide",

            h2 {
                class: "page-title-center",
                "🛠️ Nos Services"
            }

            div {
                class: "grid-services",

                for (icon, title, description) in services {
                    div {
                        key: "{title}",
                        class: "card-center",

                        div {
                            class: "service-icon",
                            "{icon}"
                        }
                        h3 {
                            class: "service-title",
                            "{title}"
                        }
                        p {
                            class: "service-description",
                            "{description}"
                        }
                    }
                }
            }
        }
    }
}
