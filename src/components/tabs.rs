use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Add a more classic style with borders to this component.
    #[prop_or_default]
    pub boxed: bool,
    /// Add the "radio button" style to the elements of this component.
    #[prop_or_default]
    pub toggle: bool,
    /// Make the tab elements of this component rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Make this component fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub centered: bool,
}

/// Simple responsive horizontal navigation tabs, with different styles.
///
/// [https://bulma.io/documentation/components/tabs/](https://bulma.io/documentation/components/tabs/)
///
/// For integration with Yew Router, it is recommended that the `RouterButton` or `RouterAnchor`
/// components be used as the individual tab elements for this component.
pub struct Tabs {}

impl Component for Tabs {
    type Message = ();
    type Properties = TabsProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("tabs");
        classes.push(&ctx.props().classes);
        if let Some(alignment) = &ctx.props().alignment {
            classes.push(&alignment.to_string());
        }
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if ctx.props().boxed {
            classes.push("is-boxed");
        }
        if ctx.props().toggle {
            classes.push("is-toggle");
        }
        if ctx.props().rounded {
            classes.push("is-rounded");
        }
        if ctx.props().fullwidth {
            classes.push("is-fullwidth");
        }
        if ctx.props().centered {
            classes.push("is-centered");
        }
        html! {
            <div class={classes}>
                <ul>
                    { for ctx.props().children.iter() }
                </ul>
            </div>
        }
    }
}
