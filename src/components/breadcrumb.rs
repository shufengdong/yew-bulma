use derive_more::Display;
use yew::prelude::*;

use crate::Alignment;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BreadcrumbProps {
    /// The `li` child elements of this breadcrumb.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<BreadcrumbSize>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The separator type to use between breadcrumb segments.
    #[prop_or_default]
    pub separator: Option<BreadcrumbSeparator>,
}

/// A simple breadcrumb component to improve your navigation experience.
///
/// [https://bulma.io/documentation/components/breadcrumb/](https://bulma.io/documentation/components/breadcrumb/)
pub struct Breadcrumb {}

impl Component for Breadcrumb {
    type Message = ();
    type Properties = BreadcrumbProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("breadcrumb");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &ctx.props().alignment {
            classes.push(&alignment.to_string());
        }
        if let Some(separator) = &ctx.props().separator {
            classes.push(separator.to_string());
        }
        html! {
            <nav class={classes} aria-label={"breadcrumbs"}>
                <ul>
                    { for ctx.props().children.iter() }
                </ul>
            </nav>
        }
    }
}

/// The 3 sizes available for a breadcrumb.
///
/// https://bulma.io/documentation/components/breadcrumb/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display("are-{_variant}")]
pub enum BreadcrumbSize {
    #[display("small")]
    Small,
    #[display("medium")]
    Medium,
    #[display("large")]
    Large,
}

/// The 4 additional separators for a breadcrump.
///
/// https://bulma.io/documentation/components/breadcrumb/#alternative-separators
#[derive(Clone, Debug, Display, PartialEq)]
#[display("has-{_variant}-separator")]
pub enum BreadcrumbSeparator {
    #[display("arrow")]
    Arrow,
    #[display("bullet")]
    Bullet,
    #[display("dot")]
    Dot,
    #[display("succeeds")]
    Succeeds,
}
