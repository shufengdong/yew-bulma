use derive_more::Display;
use yew::events::MouseEvent;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PaginationProps {
    /// The child `li`, `pagination-link` & `pagination-ellipsis` elements for pagination.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// Make the pagination elements rounded.
    #[prop_or_default]
    pub rounded: bool,

    /// The `pagination-previous` element to use.
    pub previous: Html,
    /// The `pagination-next` element to use.
    pub next: Html,
}

/// A responsive, usable, and flexible pagination component.
///
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
pub struct Pagination {}

impl Component for Pagination {
    type Message = ();
    type Properties = PaginationProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pagination");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &ctx.props().alignment {
            classes.push(&alignment.to_string());
        }
        if ctx.props().rounded {
            classes.push("is-rounded");
        }
        html! {
            <nav class={classes} role={"navigation"} aria-label={"pagination"}>
                {ctx.props().previous.clone()}
                {ctx.props().next.clone()}
                <ul class="pagination-list">
                    { for ctx.props().children.iter() }
                </ul>
            </nav>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PaginationItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or_default]
    pub is_current: bool,
    /// The pagination item type for this component.
    #[prop_or_else(|| PaginationItemType::Link)]
    pub item_type: PaginationItemType,
    /// The aria label to use for this element.
    #[prop_or_default]
    pub label: String,
    /// The click handler for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// A pagination element representing a link to a page number, the previous page or the next page.
///
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
pub struct PaginationItem {}

impl Component for PaginationItem {
    type Message = ();
    type Properties = PaginationItemProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::new();
        classes.push(&ctx.props().classes);
        classes.push(&ctx.props().item_type.to_string());
        if ctx.props().is_current {
            classes.push("is-current");
        };
        html! {
            <a class={classes}
                aria-label={ctx.props().label.clone()} onclick={ctx.props().onclick.clone()}>
                { for ctx.props().children.iter() }
            </a>
        }
    }
}

/// A pagination item type.
#[derive(Clone, Debug, Display, PartialEq)]
#[display("pagination-{_variant}")]
pub enum PaginationItemType {
    /// A pagination link for a specific page number.
    #[display("link")]
    Link,
    /// A pagination button for the next page.
    #[display("next")]
    Next,
    /// A pagination button for the previous page.
    #[display("previous")]
    Previous,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

/// A horizontal ellipsis for pagination range separators.
///
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
pub struct PaginationEllipsis;

impl Component for PaginationEllipsis {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _: &Context<Self>, _: &<Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {<span class="pagination-ellipsis">{"\u{2026}"}</span>}
    }
}
