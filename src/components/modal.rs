use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::{MyEventBus, MyMsg};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ModalMsg {
    Open,
    Close,
    CloseFromAgent(String),
    None,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The content of the `"modal-content"` element.
    #[prop_or_default]
    pub children: Children,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A classic modal overlay, in which you can include any content you want.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
pub struct Modal {
    #[allow(dead_code)]
    subscription: Box<dyn Bridge<MyEventBus>>,
    is_active: bool,
}

impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProps;

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |msg| {
                link.send_message(match msg {
                    MyMsg::Modal(message) => message,
                    _ => ModalMsg::None,
                })
            }
        };
        let subscription = MyEventBus::bridge(std::rc::Rc::new(cb));
        Self {
            subscription,
            is_active: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModalMsg::Close => {
                self.is_active = false;
            }
            ModalMsg::Open => {
                self.is_active = true;
            }
            ModalMsg::CloseFromAgent(id) => {
                if id == ctx.props().id {
                    self.is_active = false;
                } 
            }
            ModalMsg::None => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut classes = Classes::from("modal");
        classes.push(&ctx.props().classes);
        let (opencb, closecb) = if self.is_active {
            classes.push("is-active");
            (Callback::noop(), link.callback(|_| ModalMsg::Close))
        } else {
            (link.callback(|_| ModalMsg::Open), Callback::noop())
        };
        html! {
            <>
            <div onclick={opencb}>
                {ctx.props().trigger.clone()}
            </div>
            <div id={ctx.props().id.clone()} class={classes}>
                <div class={"modal-background"} onclick={closecb.clone()}></div>
                <div class={"modal-content"}>
                    { for ctx.props().children.iter() }
                </div>
                <button class={"modal-close is-large"} aria-label={"close"} onclick={closecb}></button>
            </div>
            </>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalCardProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The title of this modal.
    pub title: String,
    /// The content to be placed in the `modal-card-body` not including the modal-card-header /
    /// modal-card-title, which is handled by the `modal_title` prop.
    #[prop_or_default]
    pub body: Html,
    /// The content to be placed in the `modal-card-footer`.
    #[prop_or_default]
    pub footer: Html,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or_default]
    pub hidden: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub width: String,
}

/// A classic modal with a header, body, and footer section.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
pub struct ModalCard {
    #[allow(dead_code)]
    subscription: Box<dyn Bridge<MyEventBus>>,
    is_active: bool,
}

impl Component for ModalCard {
    type Message = ModalMsg;
    type Properties = ModalCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |msg| {
                link.send_message(match msg {
                    MyMsg::Modal(message) => message,
                    _ => ModalMsg::None,
                })
            }
        };
        let subscription = MyEventBus::bridge(std::rc::Rc::new(cb));
        Self {
            subscription,
            is_active: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModalMsg::Close => {
                self.is_active = false;
            }
            ModalMsg::Open => {
                self.is_active = true;
            }
            ModalMsg::CloseFromAgent(id) => {
                if id == ctx.props().id {
                    self.is_active = false;
                } 
            }
            ModalMsg::None => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("modal");
        let link = ctx.link();
        classes.push(&ctx.props().classes);
        let mut style = "".to_string();
        if ctx.props().hidden {
            style = "display: none;".to_string();
        } else if ctx.props().disabled {
            style = "pointer-events: none;".to_string();
        };
        let (opencb, closecb) = if self.is_active {
            classes.push("is-active");
            (Callback::noop(), link.callback(|_| ModalMsg::Close))
        } else {
            (link.callback(|_| ModalMsg::Open), Callback::noop())
        };
        let width = format!("width:{}", ctx.props().width);
        html! {
            <>
            <div onclick={opencb} style={style}>
                {ctx.props().trigger.clone()}
            </div>
            <div id={ctx.props().id.clone()} class={classes}>
                <div class={"modal-background"} onclick={closecb.clone()}></div>
                <div class={"modal-card"} style={width}>
                    <header class={"modal-card-head"}>
                        <p class={"modal-card-title"}>{ctx.props().title.clone()}</p>
                        <button class={"delete"} aria-label={"close"} onclick={closecb.clone()}></button>
                    </header>
                    <section class={"modal-card-body"}>
                        {ctx.props().body.clone()}
                    </section>
                    <footer class={"modal-card-foot"}>
                        {ctx.props().footer.clone()}
                    </footer>
                </div>
                <button class={"modal-close is-large"} aria-label={"close"} onclick={closecb.clone()}></button>
            </div>
            </>
        }
    }
}
