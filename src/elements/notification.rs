use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NotificationProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or_default]
    pub r#ref: NodeRef,
}

/// Bold notification blocks, to alert your users of something.
///
/// [https://bulma.io/documentation/elements/notification/](https://bulma.io/documentation/elements/notification/)
pub struct Notification {}

impl Component for Notification {
    type Message = ();
    type Properties = NotificationProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("notification");
        classes.push(&ctx.props().classes);
        html! {
            <div class={classes} ref={ctx.props().r#ref.clone()}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
