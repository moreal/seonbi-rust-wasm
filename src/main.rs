mod services;

use yew::{function_component, html, start_app, use_effect_with_deps, use_state};

#[function_component(App)]
fn app() -> Html {
    let source = use_state(|| "선비 紹介".to_string());
    let result = use_state(|| "".to_string());
    {
        let source = source.clone();
        let source_original = source.clone();
        let result = result.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let ret = services::seonbi::translate(&source).await.unwrap();
                result.set(ret);
            });
            || ()
        }, source_original.clone());
    }
    html! {
        <h1>{ result.as_str() }</h1>
    }
}

fn main() {
    start_app::<App>();
}
