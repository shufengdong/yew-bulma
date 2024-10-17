use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::elements::button::Button;
use crate::{MyEventBus, MyMsg};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DropdownProps {
    /// The ID of this dropdown, used for triggering close events from other parts of the app.
    pub id: String,
    /// The content of the dropdown menu.
    ///
    /// This content will be placed directly within the `div.dropdown-content` container.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// Any additional classes to use for the trigger button.
    #[prop_or_default]
    pub button_classes: Option<Classes>,
    /// The content of the trigger button.
    #[prop_or_default]
    pub button_html: Html,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DropdownMsg {
    Open,
    Close,
    CloseFromAgent(String),
    None,
}

/// An interactive dropdown menu for discoverable content.
///
/// [https://bulma.io/documentation/components/dropdown/](https://bulma.io/documentation/components/dropdown/)
pub struct Dropdown {
    #[allow(dead_code)]
    subscription: Box<dyn Bridge<MyEventBus>>,
    is_menu_active: bool,
}

impl Component for Dropdown {
    type Message = DropdownMsg;
    type Properties = DropdownProps;

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |msg| {
                link.send_message(match msg {
                    MyMsg::Dropdown(message) => message,
                    _ => DropdownMsg::None,
                })
            }
        };
        let subscription = MyEventBus::bridge(std::rc::Rc::new(cb));
        Self {
            subscription,
            is_menu_active: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if ctx.props().hoverable {
            return false;
        }
        match msg {
            DropdownMsg::Open => self.is_menu_active = true,
            DropdownMsg::Close => self.is_menu_active = false,
            DropdownMsg::CloseFromAgent(id) => {
                if id == ctx.props().id {
                    self.is_menu_active = false;
                } else {
                    return false;
                }
            }
            DropdownMsg::None => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut classes = Classes::from("dropdown");
        classes.push(&ctx.props().classes);
        let opencb = if ctx.props().hoverable {
            classes.push("is-hoverable");
            Callback::noop()
        } else {
            link.callback(|_| DropdownMsg::Open)
        };
        let overlay = if self.is_menu_active {
            classes.push("is-active");
            html! {<div onclick={link.callback(|_| DropdownMsg::Close)} style={"z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"}></div>}
        } else {
            html! {}
        };
        html! {
            <div class={classes}>
                {overlay}
                <div class={"dropdown-trigger"}>
                    <Button classes={ctx.props().button_classes.clone()} onclick={opencb}>
                        {ctx.props().button_html.clone()}
                    </Button>
                </div>
                <div class={"dropdown-menu"} role={"menu"} style={"min-width: unset"}>
                    <div class={"dropdown-content"}>
                        { for ctx.props().children.iter() }
                    </div>
                </div>
            </div>
        }
    }
}
