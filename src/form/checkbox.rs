use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_else(Callback::noop)]
    pub update: Callback<bool>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// The 2-state checkbox in its native format.
///
/// [https://bulma.io/documentation/form/checkbox/](https://bulma.io/documentation/form/checkbox/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct Checkbox {}

impl Component for Checkbox {
    type Message = bool;
    type Properties = CheckboxProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().update.emit(msg);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut classes = Classes::from("checkbox");
        classes.push(&ctx.props().classes);
        let checked = ctx.props().checked;
        let onclick = if let Some(callback) = &ctx.props().onclick {
            callback.clone()
        } else {
            link.callback(move |_| !checked)
        };
        html! {
            <label>
                <a class={classes} disabled={ctx.props().disabled}>
                    <input type={"checkbox"}
                        ref={ctx.props().r#ref.clone()}
                        checked={ctx.props().checked}
                        name={ctx.props().name.clone()}
                        value={ctx.props().value.clone()}
                        onclick={onclick}
                        disabled={ctx.props().disabled}
                    />
                    { for ctx.props().children.iter() }
                </a>
            </label>
        }
    }
}
