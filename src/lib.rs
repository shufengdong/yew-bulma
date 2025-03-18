extern crate core;

use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Debug;
use std::io;

use bytes::BytesMut;
use derive_more::Display;
use gloo_utils::{document, window};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CloseEvent, ErrorEvent, FormData, Headers, HtmlInputElement, HtmlSelectElement,
              HtmlTextAreaElement, MessageEvent, Request, RequestInit, Response, WebSocket};
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew_agent::{HandlerId, Public, WorkerLink};

pub use columns::*;
pub use components::breadcrumb::*;
pub use components::card::*;
pub use components::dropdown::*;
pub use components::menu::*;
pub use components::message::*;
pub use components::modal::*;
pub use components::navbar::*;
pub use components::pagination::*;
pub use components::panel::*;
pub use components::simplemodal::SimpleModalCard;
pub use components::tabs::*;
pub use elements::block::*;
pub use elements::button::*;
pub use elements::content::*;
pub use elements::delete::*;
pub use elements::icon::*;
pub use elements::image::*;
pub use elements::notification::*;
pub use elements::progress::*;
pub use elements::r#box::*;
pub use elements::table::*;
pub use elements::tag::*;
pub use elements::title::*;
pub use form::checkbox::*;
pub use form::control::*;
pub use form::field::*;
pub use form::file::*;
pub use form::input::*;
pub use form::radio::*;
pub use form::select::*;
pub use form::textarea::*;
pub use layout::container::*;
pub use layout::footer::*;
pub use layout::hero::*;
pub use layout::level::*;
pub use layout::media::*;
pub use layout::section::*;
pub use layout::tile::*;
use publish::{Publish, QoS};

#[cfg(feature = "calendar")]
pub mod calendar;
#[cfg(feature = "chart")]
pub mod chart;
pub mod columns;
pub mod components;
pub mod elements;
pub mod form;
pub mod layout;
mod publish;

pub const HEADER_TOKEN_INVALID: &str = "token-invalid";
pub const HEADER_PERMISSION_DENIED: &str = "permission-denied";

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

/// Common alignment classes.
#[derive(Clone, Debug, Display, PartialEq)]
#[display("is-{_variant}")]
pub enum Alignment {
    #[display("left")]
    Left,
    #[display("centered")]
    Centered,
    #[display("right")]
    Right,
}

/// Common size classes.
#[derive(Clone, Debug, Display, PartialEq)]
#[display("is-{_variant}")]
pub enum Size {
    #[display("small")]
    Small,
    #[display("normal")]
    Normal,
    #[display("medium")]
    Medium,
    #[display("large")]
    Large,
}

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq, Display)]
#[display("{}", err.as_string().unwrap_or_default())]
pub struct FetchError {
    err: JsValue,
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

impl IntoPropValue<Option<Cow<'static, str>>> for Size {
    fn into_prop_value(self) -> Option<Cow<'static, str>> {
        Some(Cow::from(self.to_string()))
    }
}

pub async fn async_ws_get_no_header(url: &str) -> Result<Vec<u8>, FetchError> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    // opts.mode(RequestMode::NoCors);
    // opts.headers(header);

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    check_resp(&resp).await?;
    let bytes = JsFuture::from(resp.array_buffer()?).await?;
    let abuf = bytes.dyn_into::<js_sys::ArrayBuffer>()?;
    let array = js_sys::Uint8Array::new(&abuf);
    let vec: Vec<u8> = array.to_vec();
    Ok(vec)
}

pub async fn async_ws_get(url: &str, header: &Headers) -> Result<Vec<u8>, FetchError> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    // opts.mode(RequestMode::NoCors);
    opts.set_headers(header);

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    check_resp(&resp).await?;
    let bytes = JsFuture::from(resp.array_buffer()?).await?;
    let abuf = bytes.dyn_into::<js_sys::ArrayBuffer>()?;
    let array = js_sys::Uint8Array::new(&abuf);
    let vec: Vec<u8> = array.to_vec();
    Ok(vec)
}

pub async fn async_ws_get_with_param(
    url: &str,
    header: &Headers,
    param: Option<HashMap<String, String>>,
) -> Result<Vec<u8>, FetchError> {
    let new_url = if let Some(p) = param {
        let param_vec = p
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>();
        let param_str = param_vec.join("&");
        format!("{}?{}", url, param_str)
    } else {
        url.to_string()
    };
    async_ws_get(&new_url, header).await
}

pub async fn async_ws_post(url: &str, header: &Headers, value: Option<JsValue>) -> Result<Vec<u8>, FetchError> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    // opts.mode(RequestMode::NoCors);
    if let Some(v) = value {
        opts.set_body(&v);
    }
    opts.set_headers(header);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    check_resp(&resp).await?;
    let bytes: JsValue = JsFuture::from(resp.array_buffer()?).await?;
    let abuf = bytes.dyn_into::<js_sys::ArrayBuffer>()?;
    let array = js_sys::Uint8Array::new(&abuf);
    let vec: Vec<u8> = array.to_vec();
    Ok(vec)
}

pub async fn async_ws_post_no_resp(url: &str, header: &Headers, value: Option<JsValue>) -> Result<bool, FetchError> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    // opts.mode(RequestMode::NoCors);
    if let Some(v) = value {
        opts.set_body(&v);
    }
    opts.set_headers(header);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    check_resp(&resp).await?;
    Ok(resp.ok())
}

pub async fn async_ws_post_file_no_resp(url: &str, header: &Headers, file: &web_sys::File) -> Result<bool, FetchError> {
    let opts = RequestInit::new();
    opts.set_method("POST");

    let form_data = FormData::new()?;
    form_data.append_with_blob("file", file)?;
    opts.set_body(&JsValue::from(form_data));
    opts.set_headers(header);

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    check_resp(&resp).await?;
    Ok(resp.ok())
}

pub async fn async_ws_put(url: &str, header: &Headers, value: Option<JsValue>) -> Result<Vec<u8>, FetchError> {
    let opts = RequestInit::new();
    opts.set_method("PUT");
    // opts.mode(RequestMode::NoCors);
    if let Some(v) = value {
        opts.set_body(&v);
    }
    opts.set_headers(header);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    check_resp(&resp).await?;
    let bytes = JsFuture::from(resp.array_buffer()?).await?;
    let abuf = bytes.dyn_into::<js_sys::ArrayBuffer>()?;
    let array = js_sys::Uint8Array::new(&abuf);
    let vec: Vec<u8> = array.to_vec();
    Ok(vec)
}

pub async fn async_ws_put_no_resp(url: &str, header: &Headers, value: Option<JsValue>) -> Result<bool, FetchError> {
    let opts = RequestInit::new();
    opts.set_method("PUT");
    // opts.mode(RequestMode::NoCors);
    if let Some(v) = value {
        opts.set_body(&v);
    }
    opts.set_headers(header);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    check_resp(&resp).await?;
    Ok(resp.ok())
}

pub async fn async_ws_delete(url: &str, header: &Headers, value: Option<JsValue>) -> Result<bool, FetchError> {
    let opts = RequestInit::new();
    opts.set_method("DELETE");
    // opts.mode(RequestMode::NoCors);
    if let Some(v) = value {
        opts.set_body(&v);
    }
    opts.set_headers(header);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    check_resp(&resp).await?;
    Ok(resp.ok())
}

async fn check_resp(resp: &Response) -> Result<bool, FetchError> {
    let resp_headers = resp.headers();
    let status = resp.status();
    let token_invalid = if let Ok(Some(s)) = resp_headers.get(HEADER_TOKEN_INVALID) {
        s == "true"
    } else {
        false
    };
    if token_invalid {
        return Err(FetchError::from(JsValue::from(HEADER_TOKEN_INVALID)));
    }
    let permission_denied = if let Ok(Some(s)) = resp_headers.get(HEADER_PERMISSION_DENIED) {
        s == "true"
    } else {
        false
    };
    if permission_denied {
        return Err(FetchError::from(JsValue::from(HEADER_PERMISSION_DENIED)));
    }
    if status != 200 {
        if let Ok(promise) = resp.text() {
            if let Ok(val) = JsFuture::from(promise).await {
                let msg = js_sys::JsString::from(val);
                if msg.length() > 0 {
                    return Err(FetchError::from(JsValue::from(msg)));
                }
            }
        }
        return Err(FetchError::from(JsValue::from(status)));
    }
    Ok(true)
}

pub fn socket_send(socket: &WebSocket, topic: String, payload: Vec<u8>) -> Result<(), io::Error> {
    let msg = Publish::new(topic, QoS::AtMostOnce, payload);
    let mut write = BytesMut::new();
    if let Err(e) = msg.write(&mut write) {
        log::warn!("!!Failed to write msg: {:?}", msg);
        Err(io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))
    } else {
        match socket.send_with_u8_array(write.as_ref()) {
            Ok(_) => Ok(()),
            Err(e) => {
                log::warn!("!!Error when mqtt sending: {:?}", e);
                Err(io::Error::new(io::ErrorKind::BrokenPipe, format!("{:?}", e)))
            }
        }
    }
}

// 将程序生成的blob用<a>元素下载到本地
pub fn download_blob(url: &str, file_name: &str) -> Result<(), JsValue> {
    let a_ele = document().create_element("a")?;
    a_ele.set_attribute("href", url)?;
    a_ele.set_attribute("download", file_name)?;
    let event = document().create_event("MouseEvent")?;
    event.init_event_with_bubbles_and_cancelable("click", true, false);
    a_ele.dispatch_event(&event)?;
    web_sys::Url::revoke_object_url(url)?;

    Ok(())
}

fn create_ws_socket(
    url: &str,
    backend: &str,
    on_open: &JsValue,
    on_message: &JsValue,
    on_error: &JsValue,
    on_close: &JsValue,
) -> Option<WebSocket> {
    let backend = backend.trim().to_string();
    if let Ok(protocol) = window().location().protocol() {
        let proto = if protocol.starts_with("https") {
            String::from("wss")
        } else {
            String::from("ws")
        };
        let backend_now = if !backend.is_empty() {
            backend.replace("https://", "").replace("http://", "")
        } else if let Ok(host) = window().location().host() {
            host
        } else {
            return None;
        };

        let ws_uri = proto + "://" + &backend_now + url;
        log::debug!("Connect to web socket server: {}", ws_uri);
        if let Ok(socket) = WebSocket::new(&ws_uri) {
            socket.set_binary_type(web_sys::BinaryType::Arraybuffer);
            socket.set_onopen(Some(on_open.unchecked_ref()));
            socket.set_onmessage(Some(on_message.unchecked_ref()));
            socket.set_onerror(Some(on_error.unchecked_ref()));
            socket.set_onclose(Some(on_close.unchecked_ref()));
            return Some(socket);
        }
    }
    None
}

/// 页面带version参数查询时拼装参数Map的方法
pub fn get_version_param(ver: Option<u32>) -> Option<HashMap<String, String>> {
    if let Some(ver_now) = ver {
        let mut param = HashMap::with_capacity(1);
        param.insert("version".to_string(), ver_now.to_string());
        Some(param)
    } else {
        None
    }
}

// 获取表单的指定内容
pub fn get_input_value_by_name(element_name: &str) -> String {
    let mut query_info = "".to_string();
    if let Some(node) = document().get_elements_by_name(element_name).get(0) {
        if let Ok(input) = node.clone().dyn_into::<HtmlInputElement>() {
            query_info.push_str(&input.value());
        } else if let Ok(input) = node.dyn_into::<HtmlTextAreaElement>() {
            query_info.push_str(&input.value());
        }
    }
    query_info.trim().to_string()
}

// 获取指定名称checkbox的状态
pub fn get_check_by_name(element_name: &str) -> bool {
    if let Some(node) = document().get_elements_by_name(element_name).get(0) {
        let input = node.dyn_into::<HtmlInputElement>().unwrap();
        return input.checked();
    }
    false
}

// 获取指定名称checkbox的状态
pub fn get_check_value(input_ref: &NodeRef) -> bool {
    return if let Some(input) = input_ref.cast::<HtmlInputElement>() {
        input.checked()
    } else {
        false
    };
}

// 获取下拉菜单的指定内容
pub fn get_select_by_name(element_name: &str) -> String {
    let mut query_info = "".to_string();
    if let Some(node) = document().get_elements_by_name(element_name).get(0) {
        if let Ok(input) = node.dyn_into::<HtmlSelectElement>() {
            query_info.push_str(&input.value());
        }
    }
    query_info.trim().to_string()
}

/// 获取input元件的值
/// 主要是将空值转为None，方便程序判断
pub fn get_input_value(input_ref: &NodeRef) -> Option<String> {
    return if let Some(input) = input_ref.cast::<HtmlInputElement>() {
        match input.value().as_str() {
            "" => None,
            _ => Some(input.value()),
        }
    } else {
        None
    };
}

pub fn get_textarea_value(input_ref: &NodeRef) -> Option<String> {
    return if let Some(area) = input_ref.cast::<HtmlTextAreaElement>() {
        match area.value().as_str() {
            "" => None,
            _ => Some(area.value()),
        }
    } else {
        None
    };
}

pub fn set_textarea_value(input_ref: &NodeRef, content: &str) {
    if let Some(area) = input_ref.cast::<HtmlTextAreaElement>() {
        area.set_value(content);
    }
}

pub fn set_textarea_value_by_name(element_name: &str, content: &str) {
    if let Some(input_ref) = document().get_elements_by_name(element_name).get(0) {
        if let Ok(area) = input_ref.dyn_into::<HtmlTextAreaElement>() {
            area.set_value(content);
        }
    }
}

pub struct MySocket {
    url: String,
    backend: String,
    socket: Option<WebSocket>,
    on_open: Closure<dyn Fn(JsValue)>,
    on_message: Closure<dyn Fn(MessageEvent)>,
    on_error: Closure<dyn Fn(ErrorEvent)>,
    on_close: Closure<dyn Fn(CloseEvent)>,
}

impl MySocket {
    pub fn new(
        url: String,
        backend: String,
        on_open: Closure<dyn Fn(JsValue)>,
        on_message: Closure<dyn Fn(MessageEvent)>,
        on_error: Closure<dyn Fn(ErrorEvent)>,
        on_close: Closure<dyn Fn(CloseEvent)>,
    ) -> Self {
        Self {
            url,
            backend,
            socket: None,
            on_open,
            on_message,
            on_error,
            on_close,
        }
    }

    pub fn close(&mut self) {
        if let Some(s) = self.socket.take() {
            s.close().unwrap();
        }
    }

    pub fn connect(&mut self) -> bool {
        let on_open = self.on_open.as_ref();
        let on_message = self.on_message.as_ref();
        let on_error = self.on_error.as_ref();
        let on_close = self.on_close.as_ref();
        let socket = create_ws_socket(&self.url, &self.backend, on_open,
                                      on_message, on_error, on_close);
        if let Some(s) = self.socket.take() {
            s.close().unwrap();
        }
        self.socket = socket;
        self.socket.is_some()
    }

    pub fn send<T>(&self, topic: &str, value: &T) -> bool
    where
        T: Serialize,
    {
        if self.socket.is_none() {
            return false;
        }
        if let Ok(payload) = serde_cbor::to_vec(value) {
            if let Err(e) = socket_send(self.socket.as_ref().unwrap(), topic.to_string(), payload) {
                log::warn!("!!Failed to send REGISTER_TOPIC to server, {:?}", e);
                return false;
            }
        }
        true
    }

    pub fn is_connected(&self) -> bool {
        self.socket.is_some() && self.socket.as_ref().unwrap().ready_state() == 1
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MyMsg {
    FileTree(components::filetree::Msg),
    FileTreeWithId(String, components::filetree::Msg),
    Modal(ModalMsg),
    Dropdown(DropdownMsg),
    #[cfg(feature = "chart")]
    ChartMsg(chart::chartcard::Msg),
}

#[derive(Clone, Debug)]
pub struct MyEventBus {
    link: WorkerLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl yew_agent::Worker for MyEventBus {
    type Reach = Public<Self>;
    type Message = ();
    type Input = MyMsg;
    type Output = MyMsg;

    fn create(link: WorkerLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, msg.clone());
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }

    fn name_of_resource() -> &'static str {
        "bulma-worker.js"
    }
}

pub fn create_table_div(table_head: Html, table_body: Html) -> Html {
    html! {
        <div style="line-height: 34px; text-align: center;">
            <Table striped=true narrow=true fullwidth=true hoverable=true >
                <thead>
                    { table_head }
                </thead>
                <tbody>
                    { table_body }
                </tbody>
            </Table>
        </div>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
