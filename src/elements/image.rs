use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ImageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<ImageSize>,
}

/// A container for responsive images.
///
/// [https://bulma.io/documentation/elements/image/](https://bulma.io/documentation/elements/image/)
pub struct Image {}

impl Component for Image {
    type Message = ();
    type Properties = ImageProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("image");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        html! {
            <figure class={classes}>
                { for ctx.props().children.iter() }
            </figure>
        }
    }
}

/// Available placeholder sizes for figures.
///
/// https://bulma.io/documentation/elements/image/
#[derive(Clone, Debug, Display, PartialEq)]
#[display("is-{_variant}")]
pub enum ImageSize {
    #[display("16x16")]
    Is16x16,
    #[display("24x24")]
    Is24x24,
    #[display("32x32")]
    Is32x32,
    #[display("48x48")]
    Is48x48,
    #[display("64x64")]
    Is64x64,
    #[display("96x96")]
    Is96x96,
    #[display("128x128")]
    Is128x128,
    #[display("Square")]
    IsSquare,
    #[display("1by1")]
    Is1by1,
    #[display("5by4")]
    Is5by4,
    #[display("4by3")]
    Is4by3,
    #[display("3by2")]
    Is3by2,
    #[display("5by3")]
    Is5by3,
    #[display("16by9")]
    Is16by9,
    #[display("2by1")]
    Is2by1,
    #[display("3by1")]
    Is3by1,
    #[display("4by5")]
    Is4by5,
    #[display("3by4")]
    Is3by4,
    #[display("2by3")]
    Is2by3,
    #[display("3by5")]
    Is3by5,
    #[display("9by16")]
    Is9by16,
    #[display("1by2")]
    Is1by2,
    #[display("1by3")]
    Is1by3,
}
