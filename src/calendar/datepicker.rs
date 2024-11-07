use std::collections::HashMap;

use chrono::{Local, LocalResult::Single, NaiveDateTime, Offset, TimeZone};
use js_sys::{Date, Function, JsString, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Element;
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

use crate::*;
use crate::{MyEventBus, MyMsg};

pub const UI_TEXT_IDS: [&str; 7] = [
    "ok",
    "cancel",
    "choose_date",
    "now",
    "today",
    "clear",
    "validate",
];

const DATE_TIME_FORMAT: &str = "%m/%d/%Y %H:%M";
const DIALOG_ID: &str = "datepicker_dialog";
const TIMESTAMP_DATE_START: &str = "01/01/1970";

#[wasm_bindgen]
extern "C" {
    #[warn(non_camel_case_types)]
    type bulmaCalendar;
    #[wasm_bindgen(constructor)]
    fn new(element: &Element, option: Object) -> bulmaCalendar;
    #[wasm_bindgen(method)]
    fn value(this: &bulmaCalendar) -> Option<JsString>;
    #[wasm_bindgen(method)]
    fn on(this: &bulmaCalendar, name: &str, callback: &Function);
}

#[derive(Clone, Debug, Display, PartialEq)]
#[display("{_variant}")]
pub enum PickerType {
    #[display("date")]
    Date,
    #[display("time")]
    Time,
    #[display("datetime")]
    Datetime,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DateProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub is_range: bool,
    #[prop_or_else(|| PickerType::Date)]
    pub picker_type: PickerType,
    #[prop_or_default]
    pub is_modal: bool,
    #[prop_or_else(|| true)]
    pub need_init: bool,
    #[prop_or_default]
    pub start: Option<u64>,
    #[prop_or_default]
    pub end: Option<u64>,
    #[prop_or_default]
    pub is_button: bool,
    #[prop_or_default]
    pub is_link: bool,
    #[prop_or_else(Callback::noop)]
    pub on_date_picked: Callback<(u64, u64)>,
    #[prop_or_default]
    pub text_map: HashMap<String, String>,
    #[prop_or_else(|| "zh-CN".into())]
    pub lang: String,
    #[prop_or_default]
    pub minute_steps: Option<u8>,
}

pub enum Msg {
    DatePicked,
    Close,
}

pub struct DatePicker {
    event_bus: Dispatcher<MyEventBus>,
    ele_ref: NodeRef,
    listener: Option<Closure<dyn Fn(JsValue)>>,
}

impl Component for DatePicker {
    type Message = Msg;
    type Properties = DateProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            event_bus: MyEventBus::dispatcher(),
            ele_ref: NodeRef::default(),
            listener: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DatePicked => {
                if ctx.props().is_button || ctx.props().is_link {
                    let close_msg = MyMsg::Modal(ModalMsg::CloseFromAgent(DIALOG_ID.to_string()));
                    self.event_bus.send(close_msg);
                }
                if let Some(ele) = self.ele_ref.cast::<Element>() {
                    if let Ok(value) = Reflect::get(&ele, &"bulmaCalendar".into()) {
                        let calendar = value.unchecked_ref::<bulmaCalendar>();
                        let s = calendar.value();
                        if s.is_none() {
                            return false;
                        }
                        let s = s.unwrap();
                        let format_str = format!("{}", s);
                        match ctx.props().picker_type {
                            PickerType::Datetime | PickerType::Date => {
                                if ctx.props().is_range {
                                    let tmp_s: Vec<&str> =
                                        format_str.as_str().split('-').collect();
                                    if tmp_s.len() != 2 {
                                        return false;
                                    }
                                    let (start_str, end_str) = match ctx.props().picker_type {
                                        PickerType::Date => (
                                            format!("{} 00:00", tmp_s[0].trim()),
                                            format!("{} 00:00", tmp_s[1].trim()),
                                        ),
                                        PickerType::Datetime => (
                                            tmp_s[0].trim().to_string(),
                                            tmp_s[1].trim().to_string(),
                                        ),
                                        _ => ("".to_string(), "".to_string()),
                                    };
                                    if let Ok(naivedate) = NaiveDateTime::parse_from_str(start_str.as_str(), DATE_TIME_FORMAT) {
                                        if let Single(start_t) = naivedate.and_local_timezone(Local) {
                                            if let Ok(naivedate2) = NaiveDateTime::parse_from_str(end_str.as_str(), DATE_TIME_FORMAT) {
                                                if let Single(end_t) = naivedate2.and_local_timezone(Local) {
                                                    let start = start_t.timestamp_millis() as u64;
                                                    let end = end_t.timestamp_millis() as u64;
                                                    ctx.props().on_date_picked.emit((start, end + 59999));
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let start_str = match ctx.props().picker_type {
                                        PickerType::Date => format!("{} 00:00", s),
                                        PickerType::Datetime => format!("{}", s),
                                        _ => "".to_string(),
                                    };
                                    if let Ok(naivedate) = NaiveDateTime::parse_from_str(start_str.as_str(), DATE_TIME_FORMAT) {
                                        if let Single(start_t) = naivedate.and_local_timezone(Local) {
                                            let start = start_t.timestamp_millis() as u64;
                                            ctx.props().on_date_picked.emit((start, start + 24 * 60 * 60 * 1000 - 1));
                                        }
                                    }
                                }
                            }
                            PickerType::Time => {
                                if ctx.props().is_range {
                                    let tmp_s: Vec<&str> =
                                        format_str.as_str().split('-').collect();
                                    if tmp_s.len() != 2 {
                                        return false;
                                    }
                                    let start_str = format!("{} {}", TIMESTAMP_DATE_START, tmp_s[0].trim());
                                    let end_str = format!("{} {}", TIMESTAMP_DATE_START, tmp_s[1].trim());
                                    if let (Ok(start_t), Ok(end_t)) = 
                                        (NaiveDateTime::parse_from_str(start_str.as_str(), DATE_TIME_FORMAT), NaiveDateTime::parse_from_str(end_str.as_str(), DATE_TIME_FORMAT)) {
                                        let start = start_t.and_utc().timestamp_millis() as u64;
                                        let end = end_t.and_utc().timestamp_millis() as u64;
                                        ctx.props().on_date_picked.emit((start, end));
                                    }
                                } else {
                                    let start_str = format!("{} {}", TIMESTAMP_DATE_START, s);
                                    if let Ok(start_t) = NaiveDateTime::parse_from_str(start_str.as_str(), DATE_TIME_FORMAT) {
                                        let start = start_t.and_utc().timestamp_millis() as u64;
                                        ctx.props()
                                            .on_date_picked
                                            .emit((start, start + 24 * 60 * 60 * 1000 - 1));
                                    }
                                }
                            }
                        }
                        return false;
                    }
                }
            }
            Msg::Close => {
                let close_msg = MyMsg::Modal(ModalMsg::CloseFromAgent(DIALOG_ID.to_string()));
                self.event_bus.send(close_msg);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let display_mode = if ctx.props().is_modal {
            "dialog"
        } else {
            "default"
        };
        let lang = ctx.props().lang.clone();

        if ctx.props().is_button || ctx.props().is_link {
            let chooser_trigger = if ctx.props().is_button {
                html! { <Button><Icon awesome_icon={"fa fa-clock-o"} /></Button> }
            } else {
                html! { <a><Icon awesome_icon={"fa fa-clock-o"} /></a> }
            };
            let chooser_body = html! {
                if ctx.props().is_range {
                    <input ref={self.ele_ref.clone()} id={ctx.props().id.clone()} class={"input"}
                        type={ctx.props().picker_type.to_string()}
                        lang={lang} data-is-range={"true"} data-color={"info"} data-display-mode={"inline"} />
                } else {
                     <input ref={self.ele_ref.clone()} id={ctx.props().id.clone()} class={"input"}
                        lang={lang} data-color={"info"}
                        type={ctx.props().picker_type.to_string()} data-display-mode={"inline"} />
                }
            };
            let chooser_footer = html! {
                <>
                <Buttons>
                    <Button onclick={link.callback(|_| Msg::DatePicked)}>
                        {self.get_text(ctx, "ok")}
                    </Button>
                    <Button onclick={link.callback(|_| Msg::Close)}>
                        {self.get_text(ctx, "cancel")}
                    </Button>
                </Buttons>
                </>
            };
            html! {
                <ModalCard id={DIALOG_ID} title={self.get_text(ctx, "choose_date")} trigger={chooser_trigger}
                    body={chooser_body} footer={chooser_footer} width={"340px"}/>
            }
        } else {
            html! {
                if ctx.props().is_range {
                    <input ref={self.ele_ref.clone()} id={ctx.props().id.clone()} class={"input"}
                        type={ctx.props().picker_type.to_string()}
                        lang={lang} data-is-range={"true"} data-color={"info"} data-display-mode={display_mode} />
                } else {
                     <input ref={self.ele_ref.clone()} id={ctx.props().id.clone()} class={"input"}
                        type={ctx.props().picker_type.to_string()}
                        lang={lang} data-color={"info"} data-display-mode={display_mode} />
                }
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(ele) = self.ele_ref.cast::<Element>() {
                let cancel_label = self.get_text(ctx, "cancel");
                let clear_label = self.get_text(ctx, "clear");
                let today_label = self.get_text(ctx, "today");
                let now_label = self.get_text(ctx, "now");
                let validate_label = self.get_text(ctx, "validate");
                let options = Object::new();
                Reflect::set(&options, &"cancelLabel".into(), &cancel_label.into()).unwrap();
                Reflect::set(&options, &"clearLabel".into(), &clear_label.into()).unwrap();
                Reflect::set(&options, &"nowLabel".into(), &now_label.into()).unwrap();
                Reflect::set(&options, &"todayLabel".into(), &today_label.into()).unwrap();
                Reflect::set(&options, &"validateLabel".into(), &validate_label.into()).unwrap();
                if let Some(minute_steps) = ctx.props().minute_steps {
                    Reflect::set(&options, &"minuteSteps".into(), &minute_steps.into()).unwrap();
                }
                // 初始化开始和结束时间
                if ctx.props().need_init {
                    let (start, end) = self.get_init_date(ctx);
                    Reflect::set(&options, &"startDate".into(), &start.clone().into()).unwrap();
                    Reflect::set(&options, &"startTime".into(), &start.into()).unwrap();
                    if ctx.props().is_range {
                        Reflect::set(&options, &"endDate".into(), &end.clone().into()).unwrap();
                        Reflect::set(&options, &"endTime".into(), &end.into()).unwrap();
                    }
                }
                if ctx.props().is_range {
                    Reflect::set(&options, &"isRange".into(), &true.into()).unwrap();
                }
                let instance = bulmaCalendar::new(&ele, options);
                Reflect::set(&ele.into(), &"bulmaCalendar".into(), &instance.into()).unwrap();
            }
        }
        if first_render && !ctx.props().is_button && !ctx.props().is_link {
            if let Some(ele) = self.ele_ref.cast::<Element>() {
                if let Ok(value) = Reflect::get(&ele, &JsValue::from_str("bulmaCalendar")) {
                    let on_click = ctx.link().callback(|_| Msg::DatePicked);
                    let listener =
                        Closure::<dyn Fn(JsValue)>::wrap(Box::new(move |_| on_click.emit(())));
                    value
                        .unchecked_ref::<bulmaCalendar>()
                        .on("select", listener.as_ref().unchecked_ref());
                    self.listener = Some(listener);
                }
            }
        }
    }
}

impl DatePicker {
    fn get_text(&self, ctx: &Context<Self>, key: &str) -> String {
        if let Some(s) = ctx.props().text_map.get(key) {
            s.clone()
        } else {
            key.to_string()
        }
    }
    /// 获得初始化的开始和结束时间
    fn get_init_date(&self, ctx: &Context<Self>) -> (Option<Date>, Option<Date>) {
        // 本地时间与utc的偏移量，如果是Time类型的，需要减去这部分偏移量才能显示正确
        let local_offset = match ctx.props().picker_type {
            PickerType::Time => {
                Local
                    .timestamp_opt(0, 0)
                    .unwrap()
                    .offset()
                    .fix()
                    .local_minus_utc() as f64
                    * 1000.
            }
            _ => 0.,
        };
        // 组装开始和结束时间
        if ctx.props().is_range {
            let start = if let Some(start) = ctx.props().start {
                Date::new(&(start as f64 - local_offset).into())
            } else {
                Date::new(&(Date::now() - 24. * 60. * 60. * 1000.).into())
            };
            let end = if let Some(end) = ctx.props().end {
                Date::new(&(end as f64 - local_offset).into())
            } else {
                Date::new(&(Date::now() + 24. * 60. * 60. * 1000.).into())
            };
            (Some(start), Some(end))
        } else {
            let start = if let Some(start) = ctx.props().start {
                Date::new(&(start as f64 - local_offset).into())
            } else {
                Date::new_0()
            };
            (Some(start), None)
        }
    }
}
