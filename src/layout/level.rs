#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "nav".into())]
    pub tag: String,
}

/// A multi-purpose horizontal level, which can contain almost any other element.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct Level {}

impl Component for Level {
    type Message = ();
    type Properties = LevelProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("level");
        classes.push(&ctx.props().classes);
        html! {
            <@{ctx.props().tag.clone()} class={classes}>
                { for ctx.props().children.iter() }
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelLeftProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A container for level elements to be grouped to the left of the container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct LevelLeft {}

impl Component for LevelLeft {
    type Message = ();
    type Properties = LevelLeftProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("level-left");
        classes.push(&ctx.props().classes);
        html! {
            <@{ctx.props().tag.clone()} class={classes}>
                { for ctx.props().children.iter() }
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelRightProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A container for level elements to be grouped to the right of the container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct LevelRight {}

impl Component for LevelRight {
    type Message = ();
    type Properties = LevelRightProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("level-right");
        classes.push(&ctx.props().classes);
        html! {
            <@{ctx.props().tag.clone()} class={classes}>
                { for ctx.props().children.iter() }
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// An individual element of a level container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct LevelItem {}

impl Component for LevelItem {
    type Message = ();
    type Properties = LevelItemProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("level-item");
        classes.push(&ctx.props().classes);
        html! {
            <@{ctx.props().tag.clone()} class={classes}>
                { for ctx.props().children.iter() }
            </@>
        }
    }
}
