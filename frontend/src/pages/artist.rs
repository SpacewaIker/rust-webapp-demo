use entities::artist::Model as Artist;
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: u32,
}

#[function_component(ArtistPage)]
pub fn artist_page(props: &Props) -> Html {
    let artist = use_state(|| Artist {
        id: 0,
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

    html! {
        <div>
            <h1>{ &*artist.name }</h1>
            <p>{ "Genre: " } { &*artist.genre.as_ref().map_or("Unknown".to_string(), |g| g.to_string()) }</p>
            <p>{ "Date formed: " } { &*artist.date_formed }</p>
        </div>
    }
}
