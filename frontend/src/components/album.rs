use entities::album::Model as Album;
use entities::artist::Model as Artist;
use entities::song::Model as Song;
use gloo_net::http::Request;
use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Link};

use crate::components::SongView;
use crate::router::Route;
use crate::theme::Theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
    #[prop_or(false)]
    pub full: bool,
}

#[function_component(AlbumView)]
pub fn album_view(props: &Props) -> Html {
    let album = use_state(|| Album {
        id: props.id,
        name: String::new(),
        date_published: String::new(),
    });

    let songs = use_state(|| Vec::new());

    let artists = use_state(|| Vec::new());

    let full = use_state(|| props.full);

    let navigator = use_navigator().unwrap();
    let edit = {
        let navigator = navigator.clone();
        let id = album.id;
        Callback::from(move |_| navigator.push(&Route::AlbumEdit { id }))
    };
    let delete = {
        let id = album.id;
        Callback::from(move |event: MouseEvent| {
            let window = web_sys::window().unwrap();
            let confirm = window
                .confirm_with_message("Press OK to delete this album")
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
                .parent_element() // div containing info
                .unwrap()
                .parent_element() // div containing album
                .unwrap()
                .remove();

            wasm_bindgen_futures::spawn_local(async move {
                Request::delete(&format!("/api/album/{}", id))
                    .send()
                    .await
                    .expect("Failed to send request to delete album");
            });
        })
    };

    {
        let id = props.id;
        let album = album.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get(&format!("/api/album/{}", id))
                        .send()
                        .await
                        .expect("Failed to fetch album")
                        .json::<Album>()
                        .await
                        .expect("Failed to parse album");

                    album.set(resp);
                })
            },
            (),
        );
    }

    {
        let album_id = props.id;
        let songs = songs.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get(&format!("/api/album/songs/{}", album_id))
                        .send()
                        .await
                        .expect("Failed to fetch songs")
                        .json::<Vec<Song>>()
                        .await
                        .expect("Failed to parse songs");

                    songs.set(resp);
                })
            },
            (),
        );
    }

    {
        let album_id = props.id;
        let artists = artists.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get(&format!("/api/album/artist/{}", album_id))
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

    let toggle_full = {
        let full = full.clone();
        Callback::from(move |_| full.set(!*full))
    };

    let theme = use_context::<UseStateHandle<Theme>>().expect("No context found");
    let theme_style = theme.get_theme();

    let style = use_style!(
        r#"
            background: ${surface};
            color: ${on_surface}
            width: 100%;
            border-radius: 50px;
            border: 1px solid ${outline};
            display: flex;
            flex-direction: column;
            margin-bottom: 15px;
            padding-bottom: 10px;

            div {
                display: flex;
                justify-content: space-around;
                align-items: baseline;
                width: 100%;
            }

            ul:first-of-type {
                list-style: none;
                display: flex;
            }

            a {
                padding: 5px 20px;
                text-decoration: none;
                color: ${on_surface};
            }

            ul {
                width: 90%;
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
        surface = theme_style.surface,
        on_surface = theme_style.on_surface,
        outline = theme_style.outline,
    );

    let title_style = use_style!(
        r#"
            margin: 0.67em 0;
            font-size: 2em;
            font-weight: bold;
        "#,
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

    let show_style = use_style!(
        r#"
            :hover {
                background-color: ${primary};
                color: ${on_primary};
            }
        "#,
        primary = theme_style.primary,
        on_primary = theme_style.on_primary,
    );

    let create_song_style = use_style!(
        r#"
            background-color: ${surface};
            border-radius: 10px;
            border: 2px solid ${outline};
            padding: 5px 10px;
            color: ${on_surface};
            font-size: 1.5rem;
            font-weight: bold;

            :hover {
                background-color: ${on_secondary};
                color: ${secondary};
            }
        "#,
        outline = theme_style.outline,
        surface = theme_style.surface,
        on_surface = theme_style.on_surface,
        secondary = theme_style.secondary,
        on_secondary = theme_style.on_secondary,
    );

    let add_artist_style = use_style!(
        r#"
            font-size: 1.75em;
        "#
    );

    html! {
        <div class={style}>
            <div>
                <Link<Route> classes={ title_style } to={ Route::Album { id: props.id } }>{ &album.name }</Link<Route>>
                <p>{ &album.date_published }</p>
                <span>
                    <button onclick={ edit } class={ edit_style }>{ "Edit" }</button>
                    <button onclick={ delete } class={ delete_style }>{ "Delete" }</button>
                    <button onclick={ toggle_full } class={ show_style }>
                        { if *full { "Hide songs" } else { "Show songs" } }
                    </button>
                </span>
            </div>
            <ul>
                {for artists.iter().map(|artist| {
                    let id: i32 = artist.id.try_into().unwrap();

                    html! {
                        <ArtistTag artist_id={ id } album_id={ props.id } artist_name={ artist.name.clone() } />
                    }
                })}
                <Link<Route> to={Route::AlbumArtist { album_id: props.id } } classes={ add_artist_style }>{ "+" }</Link<Route>>
            </ul>
            if *full {
                <ul>
                    {for songs.iter().map(|song| html! {
                        <SongView id={ song.id } />
                    })}
                    <Link<Route> classes={ create_song_style } to={ Route::SongCreate { album_id: props.id } }>{ "+" }</Link<Route>>
                </ul>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ArtistTagProps {
    artist_id: i32,
    artist_name: String,
    album_id: i32,
}

#[function_component(ArtistTag)]
fn artist_tag(props: &ArtistTagProps) -> Html {
    let remove_artist = {
        let artist_id = props.artist_id;
        let album_id = props.album_id;
        Callback::from(move |event: MouseEvent| {
            event
                .target()
                .unwrap()
                .unchecked_into::<HtmlElement>()
                .parent_element() // span
                .unwrap()
                .remove();

            wasm_bindgen_futures::spawn_local(async move {
                let mut id_vec = Vec::new();
                id_vec.push(artist_id);

                Request::delete(&format!("/api/album/artist/{}", album_id))
                    .json(&id_vec)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
            })
        })
    };

    let theme = use_context::<UseStateHandle<Theme>>().expect("No context found");
    let theme_style = theme.get_theme();

    let style = use_style!(
        r#"
            background-color: ${surface};
            display: inline-block;
            border: 2px solid ${outline};
            border-radius: 50px;
            margin: 0.5em;

        "#,
        surface = theme_style.surface,
        outline = theme_style.outline,
    );

    let x_style = use_style!(
        r#"
            border: none;
            color: ${error};
        "#,
        error = theme_style.error,
    );

    html! {
        <span class={ style }>
            <Link<Route> to={Route::Artist { id: props.artist_id }} >{ &*props.artist_name }</Link<Route>>
            <button onclick={ remove_artist } class={ x_style }>{ "X" }</button>
        </span>
    }
}
