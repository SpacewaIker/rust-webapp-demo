use std::ops::Deref;

use gloo_net::http::Request;
use markdown::to_html;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home() -> Html {
    let content = use_state(|| Html::default());

    {
        let content = content.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let readme = Request::get("https://raw.githubusercontent.com/SpacewaIker/rust-webapp-demo/main/README.md")
                        .send()
                        .await
                        .expect("Failed to fetch README.md")
                        .text()
                        .await
                        .expect("Failed to read README.md");

                    let html_string = to_html(&readme);
                    let html = Html::from_html_unchecked(html_string.into());
                    content.set(html);
                })
            },
            (),
        )
    }

    content.deref().clone()
}
