# Styled Components for Dioxus

```rs
use dioxus::prelude::*;
use styled_components::styled;

styled!(let Red: p = "color: red");
styled!(let Blue: p = "color: blue");

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Red { "I am red text" },
            Blue { "I am blue text"}
        }
    })
}
```

## Limitations

#### custom properties are not possible yet
Dioxus doesnt have a way to pass through properies in elements yet.
