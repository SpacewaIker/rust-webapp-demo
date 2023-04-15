use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: u32,
}

#[function_component(ArtistPage)]
pub fn artist_page(props: &Props) -> Html {
    html! {
        <div>
        { "Artist page for artist " } { props.id }
        </div>
    }
}
