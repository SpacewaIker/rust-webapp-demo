mod components;
mod pages;
mod router;

use router::{switch, Route};
use stylist::yew::use_style;
use yew::prelude::*;
use yew_router::prelude::*;

use components::Navbar;

#[function_component(App)]
pub fn app() -> Html {
    let style = use_style!(
        r#"
            font-family: 'Roboto', sans-serif;
            font-size: 16px;
            
        "#
    );

    html! {
        <div class={style}>
            <BrowserRouter>
                <Navbar />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}
