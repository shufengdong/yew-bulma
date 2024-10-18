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
            let mut div = html! {
                <div class={self.class_str[i].clone()} style={self.style_str[i].clone()} />
            };
            if let Some(node) = nodes.remove(&i) {
                if self.with_box {
                    let mut box_node = html! { <div class={"box"} /> };
                    if let VNode::VTag(father) = &mut box_node {
                        father.add_child(node);
                    }
                    if let VNode::VTag(father) = &mut div {
                        father.add_child(box_node);
                    }
                } else {
                    if let VNode::VTag(father) = &mut div {
                        father.add_child(node);
                    }
                }
            }
            div
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