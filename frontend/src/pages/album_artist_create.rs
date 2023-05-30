use entities::artist::Model as Artist;
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

#[function_component(AlbumArtistCreate)]
pub fn album_artist_create(props: &Props) -> Html {
    let artists = use_state(|| Vec::new());
    let artist_id = use_state(|| 0);

    {
        let artists = artists.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get("/api/artist/all")
                        .send()
                        .await
                        .expect("Failed to fetch artists")
                        .json::<Vec<Artist>>()
                        .await
                        .expect("Failed to parse artists");

                    artists.set(resp);
                })
            },
            (),
        );
    }

    let artist_onchange = {
        let artist_id = artist_id.clone();
        Callback::from(move |event: Event| {
            let id = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<i32>()
                .unwrap();

            artist_id.set(id);
        })
    };

    let onclick = {
        let navigator = use_navigator().unwrap();
        let artist_id = artist_id.clone();
        let album_id = props.album_id;
        Callback::from(move |_| {
            let artist_id = artist_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut id_vec = Vec::new();
                id_vec.push(*artist_id);

                Request::post(&format!("/api/album/artist/{}", album_id))
                    .json(&id_vec)
                    .expect("Failed to serialize artist id vector")
                    .send()
                    .await
                    .expect("Failed to add album artist");
            });

            let navigator = navigator.clone();
            navigator.back();
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
            <label>{ "Select Artist to Add" }</label>
            <select onchange={ artist_onchange }>
                <option value={ "".to_string() } selected={ true } disabled={ true }>{ "Select an artist" }</option>
                {for artists.iter().map(|artist| html! {
                    <option value={ artist.id.to_string() }>{ &artist.name }</option>
                })}
            </select>
            <button onclick={ onclick }>{ "Add Artist" }</button>
        </div>
    }
}
