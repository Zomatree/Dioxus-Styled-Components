use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, LitStr, parse::{Parse, ParseStream}, Token};
use quote::{quote, format_ident};
use rand::{thread_rng, distributions::{Alphanumeric, DistString}};

struct StyledElement {
    name: Ident,
    element: Ident,
    style: String
}

// styled!(let Foo: div = "")

impl Parse for StyledElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![let]>()?;
        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let element = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let style = input.parse::<LitStr>()?;

        Ok(Self { name, element, style: style.value() })
    }
}

#[proc_macro]
pub fn styled(stream: TokenStream) -> TokenStream {
    let StyledElement { name, element, style } = parse_macro_input!(stream as StyledElement);

    let props_name = format_ident!("{name}Props");

    let mut rng = thread_rng();
    let rand_chars = Alphanumeric.sample_string(&mut rng, 5);

    let class_name = format!("{name}-{rand_chars}");

    let expanded = quote! {
        #[derive(Props)]
        pub struct #props_name<'a> {
            pub children: Element<'a>
        }

        #[allow(none_snake_case)]
        pub fn #name<'a>(cx: Scope<'a, #props_name<'a>>) -> Element<'a> {
            let class = #class_name;

            static REGISTERED: ::std::sync::atomic::AtomicBool = ::std::sync::atomic::AtomicBool::new(false);
            let was_registered = REGISTERED.swap(true, ::std::sync::atomic::Ordering::Relaxed);

            if !was_registered {
                let document = styled_components::web_sys::window().unwrap().document().unwrap();
                let head = document.head().unwrap();

                let style_element = match document.get_element_by_id("styled-components-style") {
                    Some(element) => element,
                    None => {
                        let element = document.create_element("style").unwrap();
                        element.set_id("styled-components-style");
                        head.append_with_node_1(&element).unwrap();
                        element
                    }
                };

                let inner_css = style_element.inner_html();
                style_element.set_inner_html(&format!("{}.{}{{{}}}", inner_css, class, #style));
            };

            cx.render(rsx! {
                #element {
                    class: "{class}",
                    &cx.props.children
                }
            })
        }    };

    expanded.into()
}
