use entities::album::Model as Album;
use entities::song::Model as Song;
use gloo_net::http::Request;
use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Link};

use crate::{router::Route, theme::Theme};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(SongView)]
pub fn song_view(props: &Props) -> Html {
    let song = use_state(|| Song {
        id: props.id,
        name: String::new(),
        length_secs: 0,
        album_id: 0,
    });
    let album_name = use_state(|| String::new());

    let navigator = use_navigator().unwrap();
    let edit = {
        let navigator = navigator.clone();
        let id = song.id;
        Callback::from(move |_| navigator.push(&Route::SongEdit { id }))
    };
    let delete = {
        let id = song.id;
        Callback::from(move |event: MouseEvent| {
            let window = web_sys::window().unwrap();
            let confirm = window
                .confirm_with_message("Press OK to delete this song")
                .unwrap_or(false);

            if !confirm {
                return;
            }

            event
                .target()
                .unwrap()
                .unchecked_into::<HtmlElement>()
                .parent_element() // span
                .unwrap()
                .parent_element() // div containing song
                .unwrap()
                .remove();

            wasm_bindgen_futures::spawn_local(async move {
                Request::delete(&format!("/api/song/{}", id))
                    .send()
                    .await
                    .expect("Failed to send request to delete song");
            });
        })
    };

    {
        let id = props.id;
        let song = song.clone();
        let album_name = album_name.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get(&format!("/api/song/{}", id))
                        .send()
                        .await
                        .expect("Failed to send request to fetch song")
                        .json::<Song>()
                        .await
                        .expect("Failed to parse response into song");

                    let album_id = resp.album_id;
                    song.set(resp);

                    let resp = Request::get(&format!("/api/album/{}", album_id))
                        .send()
                        .await
                        .expect("Failed to send request to fetch album")
                        .json::<Album>()
                        .await
                        .expect("Failed to parse response into album");

                    album_name.set(resp.name);
                })
            },
            (),
        );
    }

    let theme = use_context::<UseStateHandle<Theme>>().expect("No context found");
    let theme_style = theme.get_theme();

    let style = use_style!(
        r#"
            display: flex;
            background-color: ${surface};
            color: ${on_surface};
            justify-content: space-around;
            align-items: baseline;
            width: 100%;
            border-radius: 50px;
            margin-bottom: 10px;

            a {
                color: ${on_surface};
            }

            h1 {
                font-size: 1.25rem;
            }

            p {
                font-size: 1rem;
            }

            button {
                background-color: ${surface};
                color: ${on_surface};
                border: 2px solid ${outline};
                border-radius: 10px;
                padding: 5px 10px;
                margin: 0 5px;
            }

            button:hover {
                cursor: pointer;
            }
        "#,
        outline = theme_style.outline,
        surface = theme_style.surface,
        on_surface = theme_style.on_surface,
    );

    let edit_style = use_style!(
        r#"
            :hover {
                background-color: ${tertiary};
                color: ${on_tertiary};
            }
        "#,
        tertiary = theme_style.tertiary,
        on_tertiary = theme_style.on_tertiary,
    );

    let delete_style = use_style!(
        r#"
            :hover {
                background-color: ${error};
                color: ${on_error};
            }
        "#,
        error = theme_style.error,
        on_error = theme_style.on_error,
    );

    html! {
        <div class={ style }>
            <h1>{ &song.name }</h1>
            <p>{ &*sec_to_minsec(song.length_secs) }</p>
            <Link<Route> to={ Route::Artist { id: song.album_id } }>{ &*album_name }</Link<Route>>
            <span>
                <button onclick={ edit } class={ edit_style }>{ "Edit" }</button>
                <button onclick={ delete } class={ delete_style }>{ "Delete" }</button>
            </span>
        </div>
    }
}

fn sec_to_minsec(sec: i32) -> String {
    let min = sec / 60;
    let sec = sec % 60;

    format!("{}:{:02}", min, sec)
}
