use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

use crate::*;

pub const UI_TEXT_IDS: [&str; 2] = ["all", "rows_per_page"];

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub dp_id: String,
    #[prop_or_else(|| 10)]
    pub row_num_per_page: usize,
    #[prop_or_else(Callback::noop)]
    pub on_select: Callback<usize>,
    #[prop_or_default(|| false)]
    pub is_simple: bool,
    #[prop_or_default]
    pub text_map: HashMap<String, String>,
}

pub enum Msg {
    RowNumPerPage(usize),
}

pub struct MyRowNumDP {
    event_bus: Dispatcher<MyEventBus>,
}

impl Component for MyRowNumDP {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            event_bus: MyEventBus::dispatcher()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
       return match msg {
            Msg::RowNumPerPage(n) => {
                let local_storage = window().local_storage().unwrap().unwrap();
                local_storage
                    .set_item(&ctx.props().dp_id, &n.to_string())
                    .unwrap();
                let close_msg =
                    MyMsg::Dropdown(DropdownMsg::CloseFromAgent(ctx.props().dp_id.clone()));
                self.event_bus.send(close_msg);
                ctx.props().on_select.emit(n);
                true
            }
        };
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let row_num_choose_button = if ctx.props().is_simple {
            html! { <Icon awesome_icon={"fa fa-chevron-down"}/> }
        } else {
            let row_num_info = if ctx.props().row_num_per_page != 0 {
                format!(
                    "{}{}",
                    ctx.props().row_num_per_page,
                    self.get_text(ctx, "rows_per_page")
                )
            } else {
                format!(
                    "{}{}",
                    self.get_text(ctx, "all"),
                    self.get_text(ctx, "rows_per_page")
                )
            };
            html! {
                <>
                <span>{row_num_info}</span>
                <Icon awesome_icon={"fa fa-angle-down"}/>
                </>
            }
        };
        let mut classes = Classes::new();
        if ctx.props().is_simple {
            classes.push("is-ghost")
        };


        html! {
            <Dropdown id={ctx.props().dp_id.clone()} button_html={row_num_choose_button}
                button_classes={classes}>
                <a onclick={link.callback(|_|Msg::RowNumPerPage(0))}
                    class={"dropdown-item"}>{self.get_text(ctx, "all")}</a>
                <a onclick={link.callback(|_|Msg::RowNumPerPage(10))}
                    class={"dropdown-item"}>{"10"}</a>
                <a onclick={link.callback(|_|Msg::RowNumPerPage(20))}
                    class={"dropdown-item"}>{"20"}</a>
                <a onclick={link.callback(|_|Msg::RowNumPerPage(30))}
                    class={"dropdown-item"}>{"30"}</a>
                <a onclick={link.callback(|_|Msg::RowNumPerPage(50))}
                    class={"dropdown-item"}>{"50"}</a>
                <a onclick={link.callback(|_|Msg::RowNumPerPage(80))}
                    class={"dropdown-item"}>{"80"}</a>
            </Dropdown>
        }
    }
}

impl MyRowNumDP {
    fn get_text(&self, ctx: &Context<Self>, key: &str) -> String {
        if let Some(s) = ctx.props().text_map.get(key) {
            s.clone()
        } else {
            key.to_string()
        }
    }
}