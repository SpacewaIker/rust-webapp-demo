use std::ops::Deref;

use entities::{artist::Model as Artist, sea_orm_active_enums::Genre};
use gloo_net::http::Request;
use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::theme::Theme;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: i32,
}

#[function_component(ArtistEdit)]
pub fn artist_edit(props: &Props) -> Html {
    let artist = use_state(|| Artist {
        id: props.id,
        name: "Artist name".to_string(),
        genre: None,
        date_formed: "0000-00-00".to_string(),
    });

    {
        let id = props.id;
        let artist = artist.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get(&format!("/api/artist/{}", id))
                        .send()
                        .await
                        .expect("Failed to send request")
                        .json::<Artist>()
                        .await
                        .expect("Failed to parse response");

                    artist.set(resp);
                })
            },
            (),
        );
    }

    let save = {
        let id = props.id;
        let artist = artist.clone();
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            let artist = artist.clone();
            wasm_bindgen_futures::spawn_local(async move {
                Request::put(&format!("/api/artist/{}", id))
                    .json(&*artist)
                    .expect("Failed to serialize artist")
                    .send()
                    .await
                    .expect("Failed to send request to save artist");
            });

            let navigator = navigator.clone();
            navigator.back();
        })
    };

    let name_onchange = {
        let artist = artist.clone();
        Callback::from(move |event: Event| {
            let name = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            artist.set(Artist {
                name,
                ..artist.deref().clone()
            })
        })
    };

    let genre_onchange = {
        let artist = artist.clone();
        Callback::from(move |event: Event| {
            let genre = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<Genre>()
                .ok();

            artist.set(Artist {
                genre,
                ..artist.deref().clone()
            })
        })
    };

    let date_onchange = {
        let artist = artist.clone();
        Callback::from(move |event: Event| {
            let date_formed = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            artist.set(Artist {
                date_formed,
                ..artist.deref().clone()
            })
        })
    };

    let genres = vec![
        None,
        Some(Genre::Metal),
        Some(Genre::Classical),
        Some(Genre::Rock),
        Some(Genre::Jazz),
        Some(Genre::Pop),
    ];

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
                <label>{ "Name" }</label>
                <input type="text" value={ artist.name.clone() } onchange={ name_onchange } />
            </div>
            <div>
                <label>{ "Genre" }</label>
                <select onchange={ genre_onchange }>
                    {for genres.iter().map(|genre| html! {
                        <option value={ genre_to_string(genre) } selected={ genre == &artist.genre }>
                            { &*genre_to_string(genre) }
                        </option>
                    })}
                </select>
            </div>
            <div>
                <label>{ "Date formed" }</label>
                <input type="date" value={ artist.date_formed.clone() } onchange={ date_onchange } />
            </div>
            <button onclick={save}>{ "Save" }</button>
        </div>
    }
}

fn genre_to_string(genre: &Option<Genre>) -> String {
    genre.as_ref().map_or("None".to_string(), |g| g.to_string())
}
