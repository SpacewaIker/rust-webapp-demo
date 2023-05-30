use crate::components::SongView;
use entities::song::Model as Song;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(SongsPage)]
pub fn songs() -> Html {
    let songs = use_state(|| Vec::new());

    {
        let songs = songs.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get("/api/song/all")
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

    html! {
        <div>
            {for songs.iter().map(|song| html! {
                <SongView id={song.id} />
            })}
        </div>
    }
}
