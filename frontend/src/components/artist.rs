use entities::album::Model as Album;
use entities::artist::Model as Artist;
use gloo_net::http::Request;
use stylist::yew::use_style;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, Link};

use crate::{components::AlbumView, router::Route, theme::Theme};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: i32,
    #[prop_or(false)]
    pub full: bool,
}

#[function_component(ArtistView)]
pub fn artist_view(props: &Props) -> Html {
    let artist = use_state(|| Artist {
        id: 0,
        name: "Artist name".to_string(),
        genre: None,
        date_formed: "0000-00-00".to_string(),
    });

    let albums = use_state(|| Vec::new());

    let full = use_state(|| props.full);

    let navigator = use_navigator().unwrap();
    let edit = {
        let navigator = navigator.clone();
        let id = artist.id;
        Callback::from(move |_| navigator.push(&Route::ArtistEdit { id }))
    };
    let delete = {
        let id = artist.id;
        Callback::from(move |event: MouseEvent| {
            let window = web_sys::window().unwrap();
            let confirm = window
                .confirm_with_message("Press OK to delete this artist")
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
                .parent_element() // div containing artist
                .unwrap()
                .remove();

            wasm_bindgen_futures::spawn_local(async move {
                Request::delete(&format!("/api/artist/{}", id))
                    .send()
                    .await
                    .expect("Failed to send request to delete artist");
            });
        })
    };

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

    {
        let id = props.id;
        let albums = albums.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get(&format!("/api/artist/albums/{}", id))
                        .send()
                        .await
                        .expect("Failed to send request")
                        .json::<Vec<Album>>()
                        .await
                        .expect("Failed to parse response");

                    albums.set(resp);
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
            background-color: ${surface};
            color: ${on_surface}
            width: 100%;
            border-radius: 50px;
            border: 1px solid ${outline};
            display: flex;
            flex-direction: column;
            margin-bottom: 10px;

            div {
                display: flex;
                justify-content: space-around;
                align-items: baseline;
                width: 100%;
            }

            a {
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
        "#
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

    let create_album_style = use_style!(
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

    html! {
        <div class={ style }>
            <div>
                <Link<Route> classes={ title_style } to={ Route::Artist { id: props.id } }>{ &*artist.name }</Link<Route>>
                <p>{ "Genre: " } { &*artist.genre.as_ref().map_or("Unknown".to_string(), |g| g.to_string()) }</p>
                <p>{ "Date formed: " } { &*artist.date_formed }</p>
                <span>
                    <button onclick={ edit } class={ edit_style }>{ "Edit" }</button>
                    <button onclick={ delete } class={ delete_style }>{ "Delete" }</button>
                    <button onclick={ toggle_full } class={ show_style }>
                        { if *full { "Hide albums" } else { "Show albums" } }
                    </button>
                </span>
            </div>
            if *full {
                <ul>
                    {for albums.iter().map(|album| html! {
                        <AlbumView id={ album.id } full={ props.full } />
                    })}
                    <Link<Route> classes={ create_album_style } to={ Route::AlbumCreate { artist_id: props.id } }>{ "+" }</Link<Route>>
                </ul>
            }
        </div>
    }
}
