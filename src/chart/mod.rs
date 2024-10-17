#![allow(non_snake_case)]

use std::collections::HashMap;

use js_sys::{Array, Object};
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

pub mod chartcard;
pub mod chartchooser;
pub mod line;
pub mod timeseries;

#[wasm_bindgen]
extern "C" {
    pub type ApexCharts;
    #[wasm_bindgen(constructor)]
    pub fn new(element: Element, option: Object) -> ApexCharts;
    #[wasm_bindgen(method)]
    pub fn render(this: &ApexCharts);
    #[wasm_bindgen(method)]
    pub fn updateOptions(this: &ApexCharts, option: Object);
    #[wasm_bindgen(method)]
    pub fn updateSeries(this: &ApexCharts, newSeries: Array, animate: js_sys::Boolean);
    #[wasm_bindgen(method)]
    pub fn newData(this: &ApexCharts, newSeries: Array);
    #[wasm_bindgen(method)]
    pub fn destroy(this: &ApexCharts);
}

pub fn create_templates(json: &str) -> HashMap<String, String> {
    let mut templates = HashMap::new();
    if let Ok(Value::Array(values)) = serde_json::from_str::<Value>(json) {
        for v in values {
            if let Value::String(name) = &v["name"] {
                log::debug!("name: {}", name);
                templates.insert(name.clone(), v["content"].to_string());
            }
        }
    }
    templates
}

pub fn render_template(element: Element, template: &str) {
    // 先清空
    while element.child_element_count() > 0 {
        element.first_element_child().unwrap().remove();
    }
    if let Ok(v) = js_sys::JSON::parse(template) {
        let options = v.dyn_into::<Object>().unwrap();
        let chart = ApexCharts::new(element, options);
        chart.render();
    }
}

pub fn select_and_render(selectors: &str, template: &str) {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    if let Ok(Some(element)) = doc.query_selector(selectors) {
        render_template(element, template);
    }
}

pub fn select_and_render_json(selectors: &str, options: JsValue) {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    if let Ok(Some(element)) = doc.query_selector(selectors) {
        // 先清空
        while element.child_element_count() > 0 {
            element.first_element_child().unwrap().remove();
        }
        let options = options.dyn_into::<Object>().unwrap();
        let chart = ApexCharts::new(element, options);
        chart.render();
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    #[test]
    fn test_config() {
        let data = r#"{
        "series": [{
            "name": "Desktops",
            "data": [10, 41, 35, 51, 49, 62, 69, 91, 148]
        }],
        "chart": {
          "height": 350,
          "type": "line",
          "zoom": {
            "enabled": false
          }
        },
        "dataLabels": {
          "enabled": false
        },
        "stroke": {
          "curve": "straight"
        },
        "title": {
          "text": "Product Trends by Month",
          "align": "left"
        },
        "grid": {
          "row": {
            "colors": [
                "\u0023f3f3f3",
                "transparent"
            ],
            "opacity": 0.5
          }
        },
        "xaxis": {
          "categories": ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep"]
        }
       }"#;
        let r = serde_json::from_str::<Value>(data);
        println!("{:?}", r);
        assert!(r.is_ok());
        let v = r.unwrap();
        println!("{}", v);
        assert_eq!(
            v["grid"]["row"]["colors"][0],
            Value::String("#f3f3f3".to_string())
        );
    }
}
