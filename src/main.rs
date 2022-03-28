mod services;

use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    let source = use_state(&cx, || "".to_string());
    let result = use_future(&cx, (source,), |(source,)| async move {
        services::seonbi::translate(&source).await
    });

    cx.render(rsx! {
        div {
            textarea {
                oninput: move |e| source.set(e.data.value.to_string())
            },
            match result.value() {
                Some(Ok(result)) => rsx! { h1 {
                    "{result}"
                }},
                Some(Err(_)) => rsx! { div { "Error" } },
                None => rsx! { div { "Maybe loading" } }
            }
        }
    })
}

fn main() {
    dioxus::web::launch(app);
}
