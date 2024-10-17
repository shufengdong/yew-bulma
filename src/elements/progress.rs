#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The maximum amount of progress; the 100% value.
    #[prop_or_else(|| 1.0)]
    pub max: f32,
    /// The amount of progress which has been made.
    #[prop_or_default]
    pub value: Option<f32>,
}

/// A native HTML progress bar.
///
/// [https://bulma.io/documentation/elements/progress/](https://bulma.io/documentation/elements/progress/)
pub struct Progress {}

impl Component for Progress {
    type Message = ();
    type Properties = ProgressProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    //noinspection RsTypeCheck
    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("progress");
        classes.push(&ctx.props().classes);
        let max = ctx.props().max.to_string();
        if let Some(value) = &ctx.props().value {
            let v = value.to_string();
            let value_txt = html! {{format!("{}%", value)}};
            html! {
                <progress class={classes} max={max} value={v}>
                    {value_txt}
                </progress>
            }
        } else {
            html! {
                <progress class={classes} max={max} />
            }
        }
    }
}
