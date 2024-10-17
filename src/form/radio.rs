use yew::events::InputEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RadioProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub checked_value: Option<String>,
    #[prop_or_else(Callback::noop)]
    pub update: Callback<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// The mutually exclusive radio buttons in their native format.
///
/// [https://bulma.io/documentation/form/radio/](https://bulma.io/documentation/form/radio/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct Radio {}

impl Component for Radio {
    type Message = String;
    type Properties = RadioProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().update.emit(msg);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut classes = Classes::from("radio");
        classes.push(&ctx.props().classes);
        html! {
            <label class={classes} disabled={ctx.props().disabled}>
                <input
                    type={"radio"}
                    name={ctx.props().name.clone()}
                    value={ctx.props().value.clone()}
                    checked={ctx.props().checked_value.as_ref().map(|val| val == &ctx.props().value).unwrap_or(false)}
                    oninput={link.callback(|e: InputEvent| e.data().unwrap_or("none".to_string()))}
                    disabled={ctx.props().disabled}
                    />
                { for ctx.props().children.iter() }
            </label>
        }
    }
}
