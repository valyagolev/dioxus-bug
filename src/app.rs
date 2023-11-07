use dioxus::prelude::*;
pub fn app(cx: Scope) -> Element {
    let m = use_state(cx, || Some("a"));

    cx.render(rsx! {
        "hello"
        m.get()
        button {
            onclick: move |_| {
                m.with_mut(|m| {
                    match m {
                        None => { *m = Some("b") }
                        Some(_) => { *m = None }
                    }
                });
            },
            "hi"
        }
    })
}
