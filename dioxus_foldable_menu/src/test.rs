use dioxus::prelude::*;
use dioxus_primitives::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
#[component]
pub(super) fn Navigation(current_page: Signal<Page>, on_navigate: EventHandler<Page>) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/drop_menu.css"),
        }
        DropdownMenu { class: "dropdown-menu", default_open: false,
            DropdownMenuTrigger { class: "dropdown-menu-trigger", "Open Menu" }
            DropdownMenuContent { class: "dropdown-menu-content",

                for page in [Page::Home, Page::About, Page::Services, Page::Contact, Page::Menu, Page::Calendar] {
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
