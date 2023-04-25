use stylist::yew::use_style;
use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let style = use_style!(
        r#"
            display: flex;
            align-items: center;
            background: linear-gradient(to top, #fff, #445);

            h1 {
                display: inline-block;
                font-style: italic;
                font-size: 24px;
                width: 10%;
                margin: 0;
            }

            nav {
                display: inline-block;
                width: 90%;
            }

            ul {
                margin: 0;
                display: flex;
                justify-content: space-evenly;
                list-style: none;
            }

            li {
                margin: 0;
            }

            a {
                text-decoration: none;
                color: black;
                background: lightgrey;
                padding: 10px;
                border-radius: 15%;
            }

            a:active {
                color: black;
            }

            a:hover {
                background: grey;
            }
        "#
    );

    html! {
        <header class={style}>
            <h1>{"Rust Webapp Demo"}</h1>
            <nav>
            <ul>
                <li><a href="/">{"Home"}</a></li>
                <li><a href="/songs">{"Songs"}</a></li>
                <li><a href="/albums">{"Albums"}</a></li>
                <li><a href="/artists">{"Artists"}</a></li>
            </ul>
            </nav>
        </header>
    }
}
