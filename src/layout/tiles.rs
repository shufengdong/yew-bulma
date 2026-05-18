use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use yew::prelude::*;
use yew::virtual_dom::VNode;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tiles {
    pub id: String,
    pub with_box: bool,
    pub class_str: Vec<String>,
    pub style_str: Vec<String>
}

impl Tiles {
    pub fn create_html(&self, mut nodes: HashMap<usize, VNode>) -> Html {
        let cells: Html = (0..self.class_str.len()).map(|i| {
            let child = match nodes.remove(&i) {
                Some(node) if self.with_box => html! { <div class={"box"}>{node}</div> },
                Some(node) => node,
                None => Html::default(),
            };
            html! {
                <div class={self.class_str[i].clone()} style={self.style_str[i].clone()}>
                    {child}
                </div>
            }
        }).collect();
        html! {
            <div class="fixed-grid has-12-cols">
                <div class="grid">
                    {cells}
                </div>
            </div>
        }
    }
}