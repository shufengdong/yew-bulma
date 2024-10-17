use yew::events::InputEvent;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_else(Callback::noop)]
    pub update: Callback<String>,
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The number of rows to which this component will be locked.
    #[prop_or_default]
    pub rows: u32,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Fix the size of this component.
    #[prop_or_default]
    pub fixed_size: bool,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
}

/// A multiline textarea component.
///
/// [https://bulma.io/documentation/form/textarea/](https://bulma.io/documentation/form/textarea/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct TextArea {}

impl Component for TextArea {
    type Message = String;
    type Properties = TextAreaProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().update.emit(msg);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut classes = Classes::from("textarea");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if ctx.props().loading {
            classes.push("is-loading");
        }
        if ctx.props().r#static {
            classes.push("is-static");
        }
        if ctx.props().fixed_size {
            classes.push("has-fixed-size");
        }

        let rows = ctx.props().rows.to_string();
        html! {
            <textarea ref={ctx.props().r#ref.clone()}
                name={ctx.props().name.clone()}
                value={ctx.props().value.clone()}
                oninput={link.callback(|e: InputEvent| e.data().unwrap_or("none".to_string()))}
                class={classes}
                rows={rows}
                placeholder={ctx.props().placeholder.clone()}
                disabled={ctx.props().disabled}
                readonly={ctx.props().readonly}
                />
        }
    }
}
