#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// Maintain the normal spacing between titles and subtitles.
    #[prop_or_default]
    pub is_spaced: bool,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
    /// Show the title when mouse on it
    #[prop_or_default]
    pub title: String,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
pub struct Title {}

impl Component for Title {
    type Message = ();
    type Properties = TitleProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("title");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if ctx.props().is_spaced {
            classes.push("is-spaced");
        }
        let tag = ctx.props().tag.clone();
        html! {
            <@{tag} class={classes} title={ctx.props().title.clone()}>
                { for ctx.props().children.iter() }
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SubtitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
    /// Show the title when mouse on it
    #[prop_or_default]
    pub title: String,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
pub struct Subtitle {}

impl Component for Subtitle {
    type Message = ();
    type Properties = SubtitleProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("subtitle");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        let tag = ctx.props().tag.clone();
        html! {
            <@{tag} class={classes} title={ctx.props().title.clone()}>
                { for ctx.props().children.iter() }
            </@>
        }
    }
}

/// The six sizes available for titles & subtitles.
///
/// https://bulma.io/documentation/elements/title/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display("is-{_variant}")]
pub enum HeaderSize {
    #[display("1")]
    Is1,
    #[display("2")]
    Is2,
    #[display("3")]
    Is3,
    #[display("4")]
    Is4,
    #[display("5")]
    Is5,
    #[display("6")]
    Is6,
}
