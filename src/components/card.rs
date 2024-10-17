use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or("".to_string())]
    pub width: String,
}

/// An all-around flexible and composable component; this is the card container.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct Card {}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("card");
        classes.push(&ctx.props().classes);
        let width = format!("width:{}", ctx.props().width);
        html! {
            <div class={classes} style={width}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for card header content; rendered as a horizontal bar with a shadow.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardHeader {}

impl Component for CardHeader {
    type Message = ();
    type Properties = CardHeaderProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("card-header");
        classes.push(&ctx.props().classes);
        html! {
            <header class={classes}>
                { for ctx.props().children.iter() }
            </header>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardImageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A fullwidth container for a responsive image.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardImage {}

impl Component for CardImage {
    type Message = ();
    type Properties = CardImageProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("card-image");
        classes.push(&ctx.props().classes);
        html! {
            <div class={classes}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or_default]
    pub id: Option<String>,
}

/// A container for any other content as the body of the card.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardContent {}

impl Component for CardContent {
    type Message = ();
    type Properties = CardContentProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("card-content");
        classes.push(&ctx.props().classes);
        html! {
            if let Some(id) = &ctx.props().id {
                <div class={classes} id={id.clone()}>
                    { for ctx.props().children.iter() }
                </div>
            } else {
                <div class={classes}>
                    { for ctx.props().children.iter() }
                </div>
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for card footer content; rendered as a horizontal list of controls.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardFooter {}

impl Component for CardFooter {
    type Message = ();
    type Properties = CardFooterProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("card-footer");
        classes.push(&ctx.props().classes);
        html! {
            <footer class={classes}>
                { for ctx.props().children.iter() }
            </footer>
        }
    }
}
