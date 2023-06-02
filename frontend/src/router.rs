use crate::components::{AlbumView, ArtistView};
use crate::pages::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/artist/:id")]
    Artist { id: i32 },
    #[at("/artist/edit/:id")]
    ArtistEdit { id: i32 },
    #[at("/artist/create")]
    ArtistCreate,
    #[at("/artists")]
    Artists,

    #[at("/albums")]
    Albums,
    #[at("/album/edit/:id")]
    AlbumEdit { id: i32 },
    #[at("/album/create/:artist_id")]
    AlbumCreate { artist_id: i32 },
    #[at("/album/:id")]
    Album { id: i32 },
    #[at("/album/artist/:album_id")]
    AlbumArtist { album_id: i32 },

    #[at("/song/create/:album_id")]
    SongCreate { album_id: i32 },
    #[at("/song/edit/:id")]
    SongEdit { id: i32 },
    #[at("/songs")]
    Songs,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },

        Route::Artist { id } => html! { <ArtistView id={id} full={true} /> },
        Route::ArtistEdit { id } => html! { <ArtistEdit id={id} /> },
        Route::ArtistCreate => html! { <ArtistCreate /> },
        Route::Artists => html! { <ArtistsPage /> },

        Route::Album { id } => html! { <AlbumView id={id} full={true} /> },
        Route::AlbumEdit { id } => html! { <AlbumEdit id={id} /> },
        Route::AlbumCreate { artist_id } => html! { <AlbumCreate artist_id={artist_id} /> },
        Route::Albums => html! { <AlbumsPage /> },
        Route::AlbumArtist { album_id } => html! { <AlbumArtistCreate album_id={album_id} />},

        Route::SongCreate { album_id } => html! { <SongCreate album_id={album_id} /> },
        Route::SongEdit { id } => html! { <SongEdit id={id} /> },
        Route::Songs => html! { <SongsPage /> },

        Route::NotFound => html! { "Not found" },
    }
}
