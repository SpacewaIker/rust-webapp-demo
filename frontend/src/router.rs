use crate::pages::{AlbumsPage, ArtistPage, ArtistsPage, HomePage, SongsPage};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/artist/:id")]
    Artist { id: u32 },
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/artists")]
    Artists,
    #[at("/albums")]
    Albums,
    #[at("/songs")]
    Songs,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Artist { id } => html! { <ArtistPage id={id} /> },
        Route::Artists => html! { <ArtistsPage /> },
        Route::Albums => html! { <AlbumsPage /> },
        Route::Songs => html! { <SongsPage /> },
        Route::NotFound => html! { "Not found" },
    }
}
