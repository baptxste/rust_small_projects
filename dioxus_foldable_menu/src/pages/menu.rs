//! Menu page component for the Dioxus foldable menu application

use dioxus::prelude::*;

#[component]
pub fn MenuPage() -> Element {
    let mut is_folded = use_signal(|| false);
    let mut selected_item = use_signal(|| String::new());

    let menu_items = vec![
        "🍕 Pizza Margherita".to_string(),
        "🍔 Burger Classique".to_string(),
        "🍝 Pâtes Carbonara".to_string(),
        "🥗 Salade César".to_string(),
        "🍰 Tarte aux Pommes".to_string(),
    ];

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("assets/style.css"),
        }
        div {
            class: "menu-container",

            h2 {
                class: "page-title-center",
                "🍽️ Menu Pliable"
            }

            button {
                class: "btn btn-primary",
                onclick: move |_| is_folded.set(!is_folded()),
                if is_folded() { "📂 Déplier le menu" } else { "📁 Plier le menu" }
            }

            if !is_folded() {
                div {
                    class: "menu-list",

                    for (index, item) in menu_items.iter().enumerate() {
                        div {
                            key: "{index}",
                            class: format!("menu-item {}",
                                if selected_item() == *item { "selected" } else { "" }
                            ),
                            onclick: {
                                let item_clone = item.clone();
                                move |_| selected_item.set(item_clone.clone())
                            },

                            "{item}"
                        }
                    }
                }

                if !selected_item().is_empty() {
                    div {
                        class: "menu-selection",
                        "✅ Sélectionné : {selected_item()}"
                    }
                }
            }
        }
    }
}
