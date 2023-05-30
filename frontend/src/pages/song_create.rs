use std::ops::Deref;

use entities::song::Model as Song;
use gloo_net::http::Request;
use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::theme::Theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub album_id: i32,
}

#[function_component(SongCreate)]
pub fn song_create(props: &Props) -> Html {
    let song = use_state(|| Song {
        id: 0,
        name: String::new(),
        length_secs: 0,
        album_id: props.album_id,
    });

    let create = {
        let song = song.clone();
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            let song = song.clone();
            wasm_bindgen_futures::spawn_local(async move {
                Request::post("/api/song/")
                    .json(&*song)
                    .expect("Failed to serialize song")
                    .send()
                    .await
                    .expect("Failed to send request to save song");
            });

            let navigator = navigator.clone();
            navigator.back();
        })
    };

    let name_onchange = {
        let song = song.clone();
        Callback::from(move |event: Event| {
            let name = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            song.set(Song {
                name,
                ..song.deref().clone()
            })
        })
    };

    let length_onchange = {
        let song = song.clone();
        Callback::from(move |event: Event| {
            let length_secs = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>()
                .unwrap();

            song.set(Song {
                length_secs,
                ..song.deref().clone()
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
                <input type="text" onchange={name_onchange} />
            </div>
            <div>
                <label>{"Length (seconds)"}</label>
                <input type="text" onchange={length_onchange} />
            </div>
            <button onclick={create}>{"Create"}</button>
        </div>
    }
}
