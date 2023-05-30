mod components;
mod pages;
mod router;
mod theme;

use router::{switch, Route};
use stylist::yew::use_style;
use theme::Theme;
use yew::prelude::*;
use yew_router::prelude::*;

use components::Navbar;

#[function_component(App)]
pub fn app() -> Html {
    let theme = use_state(|| Theme::Dark);

    let style = {
        let theme = theme.clone();
        let theme_style = theme.get_theme();
        use_style!(
            r#"
            height: 100%;
            width: 100%;
            font-family: 'Roboto', sans-serif;
            font-size: 16px;
            background-color: ${background};
            color: ${on_background};
        "#,
            background = theme_style.background,
            on_background = theme_style.on_background,
        )
    };

    let window_style = use_style!(
        r#"
            margin: 10px;
        "#
    );

    html! {
        <div class={style}>
            <BrowserRouter>
                <ContextProvider<UseStateHandle<Theme>> context={ theme }>
                    <Navbar />
                    <div class={ window_style }>
                        <Switch<Route> render={ switch } />
                    </div>
                </ContextProvider<UseStateHandle<Theme>>>
            </BrowserRouter>
        </div>
    }
}
