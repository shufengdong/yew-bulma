use js_sys::{Object, Reflect};
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::chart::ApexCharts;

pub fn select_and_render_ints(
    selectors: &str,
    template: &str,
    title: &str,
    series_names: &[&str],
    series_data: &[&[i64]],
    x_axis: &[&str],
) {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    if let Ok(Some(element)) = doc.query_selector(selectors) {
        render_ints_auto(element, template, title, series_names, series_data, x_axis);
    } else {
        log::warn!("!!Element not found, selectors: {}", selectors);
    }
}

pub fn render_ints_auto(
    element: Element,
    template: &str,
    title: &str,
    series_names: &[&str],
    series_data: &[&[i64]],
    x_axis: &[&str],
) {
    render_ints(
        element,
        template,
        title,
        series_names,
        series_data,
        x_axis,
        (None, None, None),
    );
}

pub fn render_ints(
    element: Element,
    template: &str,
    title: &str,
    series_names: &[&str],
    series_data: &[&[i64]],
    x_axis: &[&str],
    y_axis: (Option<&str>, Option<f64>, Option<f64>),
) {
    let series = js_sys::Array::new_with_length(series_names.len() as u32);
    for i in 0..series_names.len() {
        let item = js_sys::Object::new();
        let data = js_sys::Array::new_with_length(series_data[i].len() as u32);
        for j in 0..series_data[i].len() {
            data.set(j as u32, series_data[i][j].into());
        }
        let series_name = series_names[i].to_string();
        Reflect::set(&item, &"name".into(), &series_name.into()).unwrap();
        Reflect::set(&item, &"data".into(), &data.into()).unwrap();
        series.set(i as u32, item.into());
    }
    render_series(element, template, title, series, x_axis, y_axis);
}

pub fn select_and_render_floats(
    selectors: &str,
    template: &str,
    title: &str,
    series_names: &[&str],
    series_data: &[&[f64]],
    x_axis: &[&str],
) {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    if let Ok(Some(element)) = doc.query_selector(selectors) {
        render_floats_auto(element, template, title, series_names, series_data, x_axis);
    } else {
        log::warn!("!!Element not found, selectors: {}", selectors);
    }
}

pub fn render_floats_auto(
    element: Element,
    template: &str,
    title: &str,
    series_names: &[&str],
    series_data: &[&[f64]],
    x_axis: &[&str],
) {
    render_floats(
        element,
        template,
        title,
        series_names,
        series_data,
        x_axis,
        (None, None, None),
    );
}

pub fn render_floats(
    element: Element,
    template: &str,
    title: &str,
    series_names: &[&str],
    series_data: &[&[f64]],
    x_axis: &[&str],
    y_axis: (Option<&str>, Option<f64>, Option<f64>),
) {
    let series = js_sys::Array::new_with_length(series_names.len() as u32);
    for i in 0..series_names.len() {
        let item = js_sys::Object::new();
        let data = js_sys::Array::new_with_length(series_data[i].len() as u32);
        for j in 0..series_data[i].len() {
            data.set(j as u32, series_data[i][j].into());
        }
        let series_name = series_names[i].to_string();
        Reflect::set(&item, &"name".into(), &series_name.into()).unwrap();
        Reflect::set(&item, &"data".into(), &data.into()).unwrap();
        series.set(i as u32, item.into());
    }
    render_series(element, template, title, series, x_axis, y_axis);
}

pub fn render_series(
    element: Element,
    template: &str,
    title: &str,
    series: js_sys::Array,
    x_axis: &[&str],
    y_axis: (Option<&str>, Option<f64>, Option<f64>),
) {
    if let Ok(v) = js_sys::JSON::parse(template) {
        let options = v.dyn_into::<Object>().unwrap();
        // x axis
        let xaxis = Reflect::get(&options, &"xaxis".into()).unwrap();
        let categories = js_sys::Array::new_with_length(x_axis.len() as u32);
        for (i, x_axis_i) in x_axis.iter().enumerate() {
            categories.set(i as u32, x_axis_i.to_string().into());
        }
        Reflect::set(&xaxis, &"categories".into(), &categories.into()).unwrap();
        // y axis min and max
        if y_axis.0.is_some() || y_axis.1.is_some() || y_axis.2.is_some() {
            let yaxis = Reflect::get(&options, &"yaxis".into()).unwrap();
            if let Some(name) = y_axis.0 {
                let title = Reflect::get(&yaxis, &"title".into()).unwrap();
                Reflect::set(&title, &"text".into(), &name.to_string().into()).unwrap();
            }
            if let Some(min) = y_axis.1 {
                Reflect::set(&yaxis, &"min".into(), &min.into()).unwrap();
            }
            if let Some(max) = y_axis.2 {
                Reflect::set(&yaxis, &"max".into(), &max.into()).unwrap();
            }
        }
        // title
        let title_obj = Reflect::get(&options, &"title".into()).unwrap();
        Reflect::set(&title_obj, &"text".into(), &title.to_string().into()).unwrap();
        Reflect::set(&options, &"series".into(), &series.into()).unwrap();
        let chart = ApexCharts::new(element, options);
        chart.render();
    }
}
