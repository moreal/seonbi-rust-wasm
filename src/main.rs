mod services;

use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    let source = use_state(&cx, || "".to_string());
    let result_fut = use_future(&cx, (source,), |(source,)| async move {
        services::seonbi::translate(&source).await
    });
    let result = match result_fut.value() {
        Some(Ok(result)) => result,
        Some(Err(_)) => "Error",
        None => "Maybe loading"
    };

    cx.render(rsx! {
        style { [include_str!("../main.css")] }
        textarea {
            id: "translation-source",
            oninput: move |e| source.set(e.data.value.to_string())
        },
        div {            
            id: "translation-result",
            "{result}"
        }
    })
}

fn main() {
    dioxus::web::launch(app);
}
