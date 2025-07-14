//! Dioxus application with stable layout (header, menu, content, footer)
//! The content area changes based on navigation while other parts remain stable

// use crate::pages::calendar::CalendarPage;
use dioxus::prelude::*;
// pub mod calendar;
mod components;
mod pages;

use dioxus_mobile::use_window;
use pages::about::AboutPage;
use pages::calendar::CalendarPage;
use pages::contact::ContactPage;
use pages::drag_drop::FileDropArea;
use pages::home::HomePage;
use pages::menu::MenuPage;
use pages::services::ServicesPage;
fn main() {
    dioxus::launch(App);
}
#[derive(Clone, PartialEq, Copy)]
enum Page {
    Home,
    About,
    Services,
    Contact,
    Menu,
    Calendar,
    Drag,
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Home => write!(f, "Accueil"),
            Page::About => write!(f, "À propos"),
            Page::Services => write!(f, "Services"),
            Page::Contact => write!(f, "Contact"),
            Page::Menu => write!(f, "Menu"),
            Page::Calendar => write!(f, "Calendrier"),
            Page::Drag => write!(f, "Glisser déposer"),
        }
    }
}

#[component]
fn App() -> Element {
    let mut current_page = use_signal(|| Page::Home);

    rsx! {
        head {
            meta {
                name:"viewport",
                content:"width=device-width, initial-scale=1.0, viewport-fit=cover"
            }
        }
        div {
            class: "app-container",
            // HEADER - Partie stable
            components::Header {}
            // NAVIGATION - Partie stable
            components::Navigation { current_page, on_navigate: move |page| current_page.set(page) }
            // CONTENU PRINCIPAL - Partie qui change
            main {
                class: "main-content",
                match current_page() {
                    Page::Home => rsx! { HomePage {} },
                    Page::About => rsx! { AboutPage {} },
                    Page::Services => rsx! { ServicesPage {} },
                    Page::Contact => rsx! { ContactPage {} },
                    Page::Menu => rsx! { MenuPage {} },
                    Page::Calendar => rsx! { CalendarPage {} },
                    Page::Drag => rsx! { FileDropArea {} },
                }
            }
            // FOOTER - Partie stable
            components::Footer {}
        }
    }
}
