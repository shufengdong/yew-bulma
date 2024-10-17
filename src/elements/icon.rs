use yew::events::MouseEvent;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub children: Children,
     #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this icon, often used within form controls.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    #[prop_or_default]
    pub awesome_icon: Option<String>,
}

/// A container for any type of icon font.
///
/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
pub struct Icon {}

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("icon");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &ctx.props().alignment {
            classes.push(&alignment.to_string());
        }
        if !ctx.props().text.is_empty() {
            html! {
                <span class={classes!("icon-text")} onclick={ctx.props().onclick.clone()}>
                    <span class={classes}>
                        if let Some(icon) = &ctx.props().awesome_icon {
                            <i class={icon}></i>
                        }
                        { for ctx.props().children.iter() }
                    </span>
                    <span>{ctx.props().text.clone()}</span>
                </span>
            }
        } else {
            html! {
                <span class={classes} onclick={ctx.props().onclick.clone()}>
                    if let Some(icon) = &ctx.props().awesome_icon {
                        <i class={icon}></i>
                    }
                    { for ctx.props().children.iter() }
                </span>
            }
        }
    }
}
