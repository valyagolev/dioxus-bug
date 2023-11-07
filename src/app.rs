use dioxus::prelude::*;

#[component]
pub fn Comp2(cx: Scope) -> Element {
    let m = use_state(cx, || true);

    cx.render(rsx! {
        br {}
        "repro2"

        if *m.get() {
            " why"
        }

        button {
            onclick: move |_| {
                m.with_mut(|m| {
                    *m = !*m;
                });
            },
            "repro2"
        }
    })
}

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

        Comp2 {}
    })
}
