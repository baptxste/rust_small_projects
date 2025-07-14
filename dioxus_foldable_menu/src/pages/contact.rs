//! Contact page component for the Dioxus foldable menu application

use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut form_submitted = use_signal(|| false);

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("assets/calendar.css"),
        }
        div {
            class: "page-container-narrow",

            div {
                class: "card",

                h2 {
                    class: "page-title-center",
                    "📞 Contactez-nous"
                }

                if form_submitted() {
                    div {
                        class: "alert alert-success",
                        "✅ Merci ! Votre message a été envoyé."
                    }
                }

                form {
                    onsubmit: move |_| {
                        form_submitted.set(true);
                        // Ici vous pourriez envoyer les données à un serveur
                    },

                    div {
                        class: "form-group",
                        label {
                            class: "form-label",
                            "Nom :"
                        }
                        input {
                            r#type: "text",
                            class: "form-input",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                            required: true,
                        }
                    }

                    div {
                        class: "form-group",
                        label {
                            class: "form-label",
                            "Email :"
                        }
                        input {
                            r#type: "email",
                            class: "form-input",
                            value: "{email}",
                            oninput: move |evt| email.set(evt.value()),
                            required: true,
                        }
                    }

                    div {
                        class: "form-group",
                        label {
                            class: "form-label",
                            "Message :"
                        }
                        textarea {
                            class: "form-textarea",
                            value: "{message}",
                            oninput: move |evt| message.set(evt.value()),
                            required: true,
                        }
                    }

                    button {
                        r#type: "submit",
                        class: "form-submit",
                        "📤 Envoyer"
                    }
                }
            }
        }
    }
}
