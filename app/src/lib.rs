use dioxus::prelude::*;

#[must_use]
pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        style { [include_str!("../styles/bootstrap.min.css")] },
        style { [include_str!("../styles/style.css")] },
        Router {
            crate::navbar {}
            div {
                class: "container",
                Route { to: "/", crate::home {} }
                Route { to: "/image", crate::rust_img {} }
                Route { to: "/examples", crate::examples {} }
                Route { to: "/events", crate::events {} }
                Route { to: "", "Err 404 Route Not Found" }
            }
        }
    })
}

fn home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            "Welcome!"
        }
        p {
            "Here is a simple Dioxus app!"
        }
    })
}

fn examples(cx: Scope) -> Element {
    let rows = vec![
        ("Cool Guy", 39, "some stuff"),
        ("Emoji Guy", 5, "😻 😼 😽 🙀"),
        ("Last Guy", 123, ""),
    ];
    cx.render(rsx! {
        h1 {
            "Bootstrap examples"
        }
        h3 {
            "Card"
        }
        p {
            "Standard Bootstrap Card for making sections."
        }
        section {
            class: "card",
            header {
                h3 { "This is a card!" }
            }
            p { "Some cool stuff as content!" }
        }
        h3 {
            "Table"
        }
        p {
            "Table with styling."
        }
        table {
            class: "table table-striped",
            thead {
                class: "table-dark",
                tr {
                    th { "Name"}
                    th { "Age" }
                    th { "Description" }
                }
            }
            tbody {
                rows.iter().map(|(name, age, desc)| rsx!(
                    tr {
                        td { "{name}" }
                        td { "{age}" }
                        td { "{desc}" }
                    }
                ))
            }
        }
    })
}

fn rust_img(cx: Scope) -> Element {
    let rustacean = base64::encode(include_bytes!("../img/rustacean-flat-gesture.png"));
    cx.render(rsx! {
        img {
            src: "data:image/png;base64, {rustacean}",
            width: "100%",
        }
    })
}

fn navbar(cx: Scope) -> Element {
    let links = vec![
        ("Home", "/"),
        ("Image", "/image"),
        ("Examples", "/examples"),
        ("Events", "/events"),
    ];
    cx.render(rsx! {
        nav {
            class: "navbar navbar-expand bg-light",
            div {
                class: "container-fluid",
                ul {
                    class: "navbar-nav",
                    links.iter().map(|(name, link)| rsx!(
                        Link {
                            to: "{link}",
                            class: "nav-link",
                            li {
                                "{name}"
                            }
                        }
                    ))
                }
            }
        }
    })
}

fn events(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);
    let username = use_state(&cx, || "".to_string());
    let password = use_state(&cx, || "".to_string());
    let show_welcome_message = use_state(&cx, || false);
    let inputs = vec![
        ("text", "username", "", "Input your user"),
        ("password", "password", "", "Give us your password!"),
    ];
    rsx!(cx,
        h3 { "Simple local state" }
        p { "Your count: {count}" }
        button {
            class: "btn btn-light m-1",
            onclick: move |_| count += 1,
            "+"
        }
        button {
            class: "btn btn-light m-1",
            onclick: move |_| count -= 1,
            "-"
        }
        h3 { "Form Test" }
        div {
            h1 { "Form" }
            form {
                onsubmit: move |ev| {
                    username.set(ev.values.get("username").unwrap_or(&"default".to_string()).to_string());
                    password.set(ev.values.get("password").unwrap_or(&"default".to_string()).to_string());
                    show_welcome_message.set(true);
                },
                oninput: move |ev| println!("Input {:?}", ev.values),
                inputs.iter().map(|(rtype, name, value, desc)| rsx!(
                    div {
                        class: "mb-3",
                        label {
                            r#for: "{name}-id",
                            class: "form-label",
                            "{desc}"
                        }
                        input {
                            r#type: "{rtype}",
                            class: "form-control",
                            id: "{name}-id",
                            name: "{name}",
                            aria_describedby: "emailHelp",
                            value: "{value}"
                        }
                        div {
                            class: "form-text",
                            "{desc}"
                        }
                    }
                ))
                button {
                    class: "btn btn-primary", 
                    r#type: "submit", 
                    value: "Submit", 
                    "Submit the form" 
                }
                p {
                    show_welcome_message.then(|| rsx!{
                        "Welcome "
                        span {
                            color: "blue",
                            "{username}"
                        }
                        " your password "
                        span {
                            color: "red",
                            "{password}"
                        }
                        " is taken by anon123!"
                    })
                }
            }
        }
    )
}
