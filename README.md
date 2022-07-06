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

#### need to add web-sys as a dependancy

`web-sys = { version = "0.3.58", features = ["Document", "Window", "Element", "HtmlHeadElement"] }`

#### custom properties are not possible yet
Dioxus doesnt have a way to pass through properies in elements yet.
