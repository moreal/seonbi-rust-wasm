mod services;

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::{function_component, html, start_app, use_effect_with_deps, use_state, Callback};

#[function_component(App)]
fn app() -> Html {
    let source = use_state(|| "선비 紹介 國漢文混用".to_string());
    let result = use_state(|| "".to_string());
    {
        let source = source.clone();
        let source_original = source.clone();
        let result = result.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let ret = services::seonbi::translate(&source).await.unwrap();
                    result.set(ret);
                });
                || ()
            },
            source_original,
        );
    }

    let oninput = {
        let source = source.clone();
        Callback::from(move |e: yew::html::oninput::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
                source.set(input.value());
            }
        })
    };

    html! {
        <div>
            <textarea value={source.to_string()} {oninput}/>
            <h1>{ result.as_str() }</h1>
        </div>
    }
}

fn main() {
    start_app::<App>();
}
