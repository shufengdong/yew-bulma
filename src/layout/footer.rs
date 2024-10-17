use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A simple responsive footer which can include anything.
///
/// [https://bulma.io/documentation/layout/footer/](https://bulma.io/documentation/layout/footer/)
pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = FooterProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("footer");
        classes.push(&ctx.props().classes);
        html! {
            <footer class={classes}>
                { for ctx.props().children.iter() }
            </footer>
        }
    }
}

#[test]
fn test_class() {
    let mut classes = Classes::from("footer");
    let c = classes!("copyright", "sidebar--closed");
    classes.push(Some(c));
    println!("{:?}", classes);
    assert!(classes.contains("sidebar--closed"));
}
