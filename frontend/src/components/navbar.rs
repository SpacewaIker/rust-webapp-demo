use stylist::yew::use_style;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{router::Route, theme::Theme};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let theme = use_context::<UseStateHandle<Theme>>().expect("No context found");

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            theme.set(match *theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            })
        })
    };

    {
        let theme = theme.clone();
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let theme = theme.clone();
                let theme_style = theme.get_theme();

                let html = document.get_elements_by_tag_name("html").item(0);
                if let Some(html) = html {
                    html.set_attribute(
                        "style",
                        &format!("background-color: {}", theme_style.background),
                    )
                    .expect("Failed to set attribute");
                }
            },
            (),
        );
    }

    let style = {
        let theme = theme.clone();
        let theme_style = theme.get_theme();
        use_style!(
            r#"
                display: flex;
                align-items: center;
                background-color: ${background}; 
                outline: 1px solid ${outline};
                color: ${on_background};

                h1 {
                    display: inline-block;
                    font-style: italic;
                    font-size: 24px;
                    width: 10%;
                    margin: 0;
                }

                nav {
                    display: inline-block;
                    width: 90%;
                }

                ul {
                    margin: 0;
                    display: flex;
                    justify-content: space-evenly;
                    list-style: none;
                }

                li {
                    margin: 0;
                }

                a {
                    text-decoration: none;
                    background: ${primary_container};
                    color: ${on_primary_container};
                    padding: 10px;
                    border-radius: 15px;
                    border: none;
                }

                a:hover {
                    background-color: ${primary};
                    color: ${on_primary}
                }

                button {
                    padding: 10px;
                    border: 2px solid ${outline};
                    background-color: ${background};
                    color: ${on_background};
                    border-radius: 15px;
                }

                button:hover {
                    cursor: pointer;
                }
            "#,
            background = theme_style.background,
            outline = theme_style.outline,
            on_background = theme_style.on_background,
            primary = theme_style.primary,
            on_primary = theme_style.on_primary,
            primary_container = theme_style.primary_container,
            on_primary_container = theme_style.on_primary_container,
        )
    };

    html! {
        <header class={style}>
            <h1>{"Rust Webapp Demo"}</h1>
            <nav>
            <ul>
                <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                <li><Link<Route> to={Route::Songs}>{ "Songs" }</Link<Route>></li>
                <li><Link<Route> to={Route::Albums}>{ "Albums" }</Link<Route>></li>
                <li><Link<Route> to={Route::Artists}>{ "Artists" }</Link<Route>></li>
                <button onclick={ toggle_theme }>
                    {match *theme {
                        Theme::Light => "Light",
                        Theme::Dark => "Dark",
                    }}
                </button>
            </ul>
            </nav>
        </header>
    }
}
