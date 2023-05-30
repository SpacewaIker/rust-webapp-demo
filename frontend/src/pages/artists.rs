use entities::artist::Model as Artist;
use gloo_net::http::Request;
use stylist::yew::use_style;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{components::ArtistView, router::Route, theme::Theme};

#[function_component(ArtistsPage)]
pub fn artists_page() -> Html {
    let artists = use_state(|| Vec::new());

    {
        let aritsts = artists.clone();
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

                    aritsts.set(resp);
                })
            },
            (),
        );
    }

    let theme = use_context::<UseStateHandle<Theme>>().expect("No context found");
    let theme_style = theme.get_theme();

    let create_artist_style = use_style!(
        r#"
            background-color: ${surface};
            border-radius: 10px;
            border: 2px solid ${outline};
            padding: 5px 10px;
            color: ${on_surface};
            font-size: 1.5rem;
            font-weight: bold;
            text-decoration: none;

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
        <div>
            {for artists.iter().map(|artist| html! {
                <ArtistView id={ artist.id } />
            })}
            <Link<Route> classes={ create_artist_style } to={ Route::ArtistCreate }>{ "+" }</Link<Route>>
        </div>
    }
}
