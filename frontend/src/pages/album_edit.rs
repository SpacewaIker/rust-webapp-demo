use std::ops::Deref;

use entities::album::Model as Album;
use gloo_net::http::Request;
use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::theme::Theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(AlbumEdit)]
pub fn album_edit(props: &Props) -> Html {
    let album = use_state(|| Album {
        id: props.id,
        name: String::new(),
        date_published: String::new(),
    });

    {
        let id = props.id;
        let album = album.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get(&format!("/api/album/{}", id))
                        .send()
                        .await
                        .expect("Failed to send request to fetch album")
                        .json::<Album>()
                        .await
                        .expect("Failed to parse response into album");

                    album.set(resp);
                })
            },
            (),
        );
    }

    let save = {
        let id = props.id;
        let album = album.clone();
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            let album = album.clone();
            wasm_bindgen_futures::spawn_local(async move {
                Request::put(&format!("/api/album/{}", id))
                    .json(&*album)
                    .expect("Failed to serialize album")
                    .send()
                    .await
                    .expect("Failed to send request to save album");
            });

            let navigator = navigator.clone();
            navigator.back();
        })
    };

    let name_onchange = {
        let album = album.clone();
        Callback::from(move |event: Event| {
            let name = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            album.set(Album {
                name,
                ..album.deref().clone()
            })
        })
    };

    let date_onchange = {
        let album = album.clone();
        Callback::from(move |event: Event| {
            let date_published = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            album.set(Album {
                date_published,
                ..album.deref().clone()
            })
        })
    };

    let theme = use_context::<UseStateHandle<Theme>>().expect("No context found");
    let theme_style = theme.get_theme();

    let style = use_style!(
        r#"
            display: flex;
            flex-direction: column;
            align-items: center;
            height: 80vh;
            justify-content: center;

            div {
                margin: 5px 0;
            }

            label {
                display: inline-block;
                width: 15vw;
                margin-right: 10px;
            }

            input, select {
                width: 15vw;
                border-radius: 15px;
                border: 2px solid ${outline};
                background-color: ${surface};
                color: ${on_surface};
                height: 2em;
                padding: 0 10px;
            }

            button {
                margin-top: 10px;
                background-color: ${surface};
                color: ${on_surface};
                border: 2px solid ${outline};
                border-radius: 15px;
                padding: 10px;
            }

            button:hover {
                background-color: ${primary};
                color: ${on_primary};
                cursor: pointer;
            }
        "#,
        surface = theme_style.surface,
        on_surface = theme_style.on_surface,
        outline = theme_style.outline,
        primary = theme_style.primary,
        on_primary = theme_style.on_primary
    );

    html! {
        <div class={ style }>
            <div>
                <label>{"Name"}</label>
                <input type="text" value={album.name.clone()} onchange={name_onchange} />
            </div>
            <div>
                <label>{"Date published"}</label>
                <input type="date" value={album.date_published.clone()} onchange={date_onchange} />
            </div>
            <button onclick={save}>{"Save"}</button>
        </div>
    }
}
