use crate::pages::artist::ArtistPage;
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
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { "Home" },
        Route::Artist { id } => html! { <ArtistPage id={id} /> },
        Route::NotFound => html! { "Not found" },
    }
}
