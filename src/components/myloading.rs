use yew::prelude::*;
use crate::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

pub enum Msg {
}
pub struct MyLoading {
    is_loading: bool,
}

impl Component for MyLoading {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_loading: ctx.props().is_loading,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _: &Self::Properties) -> bool {
        self.is_loading = ctx.props().is_loading;
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let style = if self.is_loading {
            "display: block"
        } else {
            "display: none"
        };
        html! {
            <>
            <div class={"my-loader has-text-centered"} style={style}>
                <div class={"loader pic"}></div>
                <div class={"text"}>{"Loading..."}</div>
            </div>
            </>
        }
    }
}