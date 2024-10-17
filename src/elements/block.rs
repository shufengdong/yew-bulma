use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Bulma’s most basic spacer block
///
/// [https://bulma.io/documentation/elements/block/](https://bulma.io/documentation/elements/block/)
pub struct Block {}

impl Component for Block {
    type Message = ();
    type Properties = BlockProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("block");
        classes.push(&ctx.props().classes);
        html! {
            <div class={classes}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
