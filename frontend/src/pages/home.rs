use yew::prelude::*;

#[function_component(HomePage)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
            <p>{"Welcome to the home page!"}</p>
        </div>
    }
}
