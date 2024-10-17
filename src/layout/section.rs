use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// A size modifier to control spacing.
    #[prop_or_default]
    pub size: Option<SectionSize>,
}

/// A simple container to divide your page into sections.
///
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
pub struct Section {}

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("section");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        html! {
            <section class={classes}>
                { for ctx.props().children.iter() }
            </section>
        }
    }
}

/// The 2 sizes available for sections, which controls spacing.
///
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
#[derive(Clone, Debug, Display, PartialEq)]
#[display("is-{_variant}")]
pub enum SectionSize {
    #[display("medium")]
    Medium,
    #[display("large")]
    Large,
}
