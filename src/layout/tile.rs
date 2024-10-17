#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TileProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// The context modifier to use for this tile element, else none.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub ctx: Option<TileCtx>,
    /// Stack tiles vertically.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub vertical: bool,
    /// The size to assign to this tile element.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub size: Option<TileSize>,
}

/// A single tile element to build 2-dimensional whatever-you-like grids.
///
/// [https://bulma.io/documentation/layout/tiles/](https://bulma.io/documentation/layout/tiles/)
pub struct Tile {}

impl Component for Tile {
    type Message = ();
    type Properties = TileProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::new();
        classes.push(&ctx.props().classes);
        if let Some(ctx) = &ctx.props().ctx {
            match ctx {
                TileCtx::Ancestor => classes.push("grid"),
                TileCtx::Parent => classes.push("cell"),
                TileCtx::Child => classes.push("is-child")
            }
        }
        if ctx.props().vertical {
            classes.push("is-vertical");
        }
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        html! {
            <@{ctx.props().tag.clone()} class={classes}>
                { for ctx.props().children.iter() }
            </@>
        }
    }
}

/// Tile context modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq)]
#[display("is-{_variant}")]
pub enum TileCtx {
    #[display("ancestor")]
    Ancestor,
    #[display("parent")]
    Parent,
    #[display("child")]
    Child,
}

/// Tile size modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq, Serialize, Deserialize)]
#[display("is-{_variant}")]
pub enum TileSize {
    #[display("1")]
    One,
    #[display("2")]
    Two,
    #[display("3")]
    Three,
    #[display("4")]
    Four,
    #[display("5")]
    Five,
    #[display("6")]
    Six,
    #[display("7")]
    Seven,
    #[display("8")]
    Eight,
    #[display("9")]
    Nine,
    #[display("10")]
    Ten,
    #[display("11")]
    Eleven,
    #[display("12")]
    Twelve,
}

impl From<&str> for TileSize {
    fn from(value: &str) -> Self {
        match value {
            "is-1" | "1" => TileSize::One,
            "is-2" | "2" => TileSize::Two,
            "is-3" | "3" => TileSize::Three,
            "is-4" | "4" => TileSize::Four,
            "is-5" | "5" => TileSize::Five,
            "is-6" | "6" => TileSize::Six,
            "is-7" | "7" => TileSize::Seven,
            "is-8" | "8" => TileSize::Eight,
            "is-9" | "9" => TileSize::Nine,
            "is-10" | "10" => TileSize::Ten,
            "is-11" | "11" => TileSize::Eleven,
            "is-12" | "12" => TileSize::Twelve,
            _ => TileSize::One,
        }
    }
}
