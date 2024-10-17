use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BoxProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A white box to contain other elements.
///
/// [https://bulma.io/documentation/elements/box/](https://bulma.io/documentation/elements/box/)
pub struct BulmaBox {}

impl Component for BulmaBox {
    type Message = ();
    type Properties = BoxProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("box");
        classes.push(&ctx.props().classes);
        html! {
            <div class={classes}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
