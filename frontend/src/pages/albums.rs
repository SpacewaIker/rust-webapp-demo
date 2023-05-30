use crate::components::AlbumView;
use entities::album::Model as Album;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(AlbumsPage)]
pub fn albums_page() -> Html {
    let albums = use_state(|| Vec::new());

    {
        let albums = albums.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get("/api/album/all")
                        .send()
                        .await
                        .expect("Failed to fetch albums")
                        .json::<Vec<Album>>()
                        .await
                        .expect("Failed to parse albums");

                    albums.set(resp);
                })
            },
            (),
        );
    }

    html! {
        <div>
            {for albums.iter().map(|album| html! {
                <AlbumView id={album.id} />
            })}
        </div>
    }
}
