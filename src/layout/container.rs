use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Add a `32px` margin to the left and right sides of the container.
    #[prop_or_default]
    pub fluid: bool,
}

/// A simple container to center your content horizontally.
///
/// [https://bulma.io/documentation/layout/container/](https://bulma.io/documentation/layout/container/)
pub struct Container {}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("container");
        classes.push(&ctx.props().classes);
        if ctx.props().fluid {
            classes.push("is-fluid");
        }
        html! {
            <div class={classes}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
