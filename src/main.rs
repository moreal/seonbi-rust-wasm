use yew::{function_component, html, start_app};

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Seonbi Rust WASM" }</h1>
    }
}

fn main() {
    start_app::<App>();
}
