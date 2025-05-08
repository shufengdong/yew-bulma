use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Msg {
    Show,
    Hide,
    None,
}
pub struct MyLoading {
    is_loading: bool,
    /// 消息总线
    _subscription: Box<dyn Bridge<MyEventBus>>,
}

impl Component for MyLoading {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |msg| {
                link.send_message(match msg {
                    MyMsg::Loading(message) => message,
                    _ => Msg::None,
                })
            }
        };
        let _subscription = MyEventBus::bridge(std::rc::Rc::new(cb));
        Self {
            _subscription,
            is_loading: ctx.props().is_loading,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Show => {
                self.is_loading = true;
                return true;
            }
            Msg::Hide => {
                self.is_loading = false;
                return true;
            }
            Msg::None => {
                return false;
            }
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