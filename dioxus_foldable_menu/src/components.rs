//! Layout components for the Dioxus foldable menu application

use crate::Page;
extern crate core;
use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};

#[component]
pub fn Header() -> Element {
    rsx! {
        header {
            class: "header",

            h1 {
                "🌟 Mon Application Dioxus"
            }
            p {
                "Une application avec layout stable"
            }
        }
    }
}

#[component]
pub fn Navigation(current_page: Signal<Page>, on_navigate: EventHandler<Page>) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/drop_menu.css"),
        }
        DropdownMenu { class: "dropdown-menu", default_open: false,
            DropdownMenuTrigger { class: "dropdown-menu-trigger", "Open Menu" }
            DropdownMenuContent { class: "dropdown-menu-content",

                for page in [Page::Home, Page::About, Page::Services, Page::Contact, Page::Menu, Page::Calendar, Page::Drag] {
                    DropdownMenuItem {
                        class: "dropdown-menu-item",
                        value: "{page}".to_string(),
                        index: 3usize,
                        on_select: move |_| on_navigate.call(page.clone()),
                        "{page}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer",

            div {
                class: "footer-content",

                p {
                    class: "footer-text",
                    "© 2024 Mon Application Dioxus - Tous droits réservés"
                }

                div {
                    class: "footer-links",

                    a {
                        href: "#",
                        class: "footer-link",
                        "🏠 Accueil"
                    }
                    a {
                        href: "#",
                        class: "footer-link",
                        "📞 Contact"
                    }
                    a {
                        href: "#",
                        class: "footer-link",
                        "📋 Mentions légales"
                    }
                }
            }
        }
    }
}
