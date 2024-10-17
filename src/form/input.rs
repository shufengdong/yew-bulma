#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use yew::events::InputEvent;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_else(Callback::noop)]
    pub update: Callback<String>,
    /// Difference from update, triggered only when the focus is lost, callback with whole value
    #[prop_or_else(Callback::noop)]
    pub onenterdown: Callback<()>,
   #[prop_or_else(Callback::noop)]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The input type of this component.
    #[prop_or_else(|| InputType::Text)]
    pub r#type: InputType,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Use rounded appearance.
    #[prop_or_default]
    pub rounded: bool,
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
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub width: String,
}

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct Input {}

pub enum Msg {
    Update(String),
    Change(String),
    Keydown(KeyboardEvent),
}

impl Component for Input {
    type Message = Msg;
    type Properties = InputProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(value) => {
                ctx.props().update.emit(value);
                false
            }
            Msg::Change(value) => {
                ctx.props().onchange.emit(value);
                false
            }
            Msg::Keydown(e) => {
                // check if enter key is pressed
                if e.key_code() == 13 {
                    ctx.props().onenterdown.emit(());
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut classes = Classes::from("input");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if ctx.props().rounded {
            classes.push("is-rounded");
        }
        if ctx.props().loading {
            classes.push("is-loading");
        }
        if ctx.props().r#static {
            classes.push("is-static");
        }
        if ctx.props().checked {
            classes.push("checked");
        }
        html! {
            <input
                name={ctx.props().name.clone()}
                value={ctx.props().value.clone()}
                oninput={link.callback(|e: InputEvent| Msg::Update(e.data().unwrap_or_default()))}
                onchange={link.callback(move |e: Event| Msg::Change(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                onkeydown={link.callback(move |e: KeyboardEvent| Msg::Keydown(e))}
                class={classes}
                ref={ctx.props().r#ref.clone()}
                type={ctx.props().r#type.to_string()}
                placeholder={ctx.props().placeholder.clone()}
                disabled={ctx.props().disabled}
                readonly={ctx.props().readonly}
                size={ctx.props().width.clone()}
                />
        }
    }
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq)]
pub enum InputType {
    #[display("text")]
    Text,
    #[display("password")]
    Password,
    #[display("email")]
    Email,
    #[display("tel")]
    Tel,
     #[display("color")]
    Color,
    #[display("range")]
    Range,
}
