use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Element;

use crate::chart::ApexCharts;

pub fn select_and_render_ts(
    selectors: &str,
    template: &str,
    title: &str,
    show_legend: bool,
    has_zoom: bool,
    has_animation: bool,
    series_names: &[&str],
    series_data: &[&[i64]],
    x_axis: &[&[u64]],
) -> Option<ApexCharts> {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    if let Ok(Some(element)) = doc.query_selector(selectors) {
        render_ts_ints_auto(
            element,
            template,
            title,
            show_legend,
            has_zoom,
            has_animation,
            series_names,
            series_data,
            x_axis,
        )
    } else {
        log::warn!("!!Element not found, selectors: {}", selectors);
        None
    }
}

pub fn create_series_ints(
    series_names: &[&str],
    series_data: &[&[i64]],
    x_axis: &[&[u64]],
) -> js_sys::Array {
    let series = js_sys::Array::new_with_length(series_names.len() as u32);
    for i in 0..series_names.len() {
        let item = js_sys::Object::new();
        let data = js_sys::Array::new_with_length(series_data[i].len() as u32);
        for j in 0..series_data[i].len() {
            let t_v = js_sys::Array::new_with_length(2);
            t_v.set(0, (x_axis[i][j] as f64).into());
            t_v.set(1, series_data[i][j].into());
            data.set(j as u32, t_v.into());
        }
        let series_name = series_names[i].to_string();
        Reflect::set(&item, &"name".into(), &series_name.into()).unwrap();
        Reflect::set(&item, &"data".into(), &data.into()).unwrap();
        series.set(i as u32, item.into());
    }
    series
}

pub fn render_ts_ints_auto(
    element: Element,
    template: &str,
    title: &str,
    show_legend: bool,
    has_zoom: bool,
    has_animation: bool,
    series_names: &[&str],
    series_data: &[&[i64]],
    x_axis: &[&[u64]],
) -> Option<ApexCharts> {
    render_ts_ints(
        element,
        template,
        title,
        show_legend,
        has_zoom,
        has_animation,
        series_names,
        series_data,
        x_axis,
        (None, None, None),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn render_ts_ints(
    element: Element,
    template: &str,
    title: &str,
    show_legend: bool,
    has_zoom: bool,
    has_animation: bool,
    series_names: &[&str],
    series_data: &[&[i64]],
    x_axis: &[&[u64]],
    y_axis: (Option<&str>, Option<f64>, Option<f64>),
) -> Option<ApexCharts> {
    let series = create_series_ints(series_names, series_data, x_axis);
    render_time_series(
        element,
        template,
        title,
        show_legend,
        has_zoom,
        has_animation,
        series,
        false,
        y_axis,
    )
}

pub fn select_and_render_ts_floats(
    selectors: &str,
    template: &str,
    title: &str,
    show_legend: bool,
    has_zoom: bool,
    has_animation: bool,
    series_names: &[&str],
    series_data: &[&[f64]],
    x_axis: &[&[u64]],
) -> Option<ApexCharts> {
    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    if let Ok(Some(element)) = doc.query_selector(selectors) {
        render_ts_floats_auto(
            element,
            template,
            title,
            show_legend,
            has_zoom,
            has_animation,
            series_names,
            series_data,
            x_axis,
        )
    } else {
        log::warn!("!!Element not found, selectors: {}", selectors);
        None
    }
}

pub fn create_series_floats(
    series_names: &[&str],
    series_data: &[&[f64]],
    x_axis: &[&[u64]],
    is_data_only: bool,
) -> js_sys::Array {
    let series = js_sys::Array::new_with_length(series_names.len() as u32);
    for i in 0..series_names.len() {
        let data = js_sys::Array::new_with_length(series_data[i].len() as u32);
        for j in 0..series_data[i].len() {
            let t_v = js_sys::Array::new_with_length(2);
            t_v.set(0, (x_axis[i][j] as f64).into());
            t_v.set(1, series_data[i][j].into());
            // let t_v = js_sys::Object::new();
            // Reflect::set(&t_v, &"x".into(), &(x_axis[i][j] as f64).into()).unwrap();
            // Reflect::set(&t_v, &"y".into(), &(series_data[i][j]).into()).unwrap();
            data.set(j as u32, t_v.into());
        }
        let series_name = series_names[i].to_string();
        let item = js_sys::Object::new();
        if !is_data_only {
            Reflect::set(&item, &"name".into(), &series_name.into()).unwrap();
        }
        Reflect::set(&item, &"data".into(), &data.into()).unwrap();
        series.set(i as u32, item.into());
    }
    // log::warn!("{:?}", series.);
    series
}

pub fn create_options(
    template: &str,
    title: &str,
    show_legend: bool,
    has_zoom: bool,
    has_animation: bool,
    series: js_sys::Array,
    has_float: bool,
    y_axis: (Option<&str>, Option<f64>, Option<f64>),
) -> Option<Object> {
    if let Ok(v) = js_sys::JSON::parse(template) {
        let options = v.dyn_into::<Object>().unwrap();
        // x axis
        let xaxis = if let Ok(xaxis) = Reflect::get(&options, &"xaxis".into()) {
            if !xaxis.is_object() {
                let xaxis = js_sys::Object::new();
                Reflect::set(&options, &"xaxis".into(), &xaxis.into()).unwrap();
                Reflect::get(&options, &"xaxis".into()).unwrap()
            } else {
                xaxis
            }
        } else {
            let xaxis = js_sys::Object::new();
            Reflect::set(&options, &"xaxis".into(), &xaxis.into()).unwrap();
            Reflect::get(&options, &"xaxis".into()).unwrap()
        };
        Reflect::set(&xaxis, &"type".into(), &"datetime".into()).unwrap();
        // y axis formatter
        let yaxis = if let Ok(yaxis) = Reflect::get(&options, &"yaxis".into()) {
            if !yaxis.is_object() {
                let yaxis = js_sys::Object::new();
                Reflect::set(&options, &"yaxis".into(), &yaxis.into()).unwrap();
                Reflect::get(&options, &"yaxis".into()).unwrap()
            } else {
                yaxis
            }
        } else {
            let yaxis = js_sys::Object::new();
            Reflect::set(&options, &"yaxis".into(), &yaxis.into()).unwrap();
            Reflect::get(&options, &"yaxis".into()).unwrap()
        };
        let formatter = Closure::<dyn Fn(js_sys::Number) -> JsValue>::wrap(Box::new(
            move |val: js_sys::Number| val.to_fixed(3).unwrap().into(),
        ));
        let should_set = if let Ok(labels) = Reflect::get(&yaxis, &"labels".into()) {
            if !labels.is_object() {
                let labels = js_sys::Object::new();
                Reflect::set(&yaxis, &"labels".into(), &labels.into()).unwrap();
                true
            } else if let Ok(old_formatter) = Reflect::get(&labels, &"formatter".into()) {
                old_formatter.is_null() || old_formatter.is_undefined()
            } else {
                false
            }
        } else {
            let labels = js_sys::Object::new();
            Reflect::set(&yaxis, &"labels".into(), &labels.into()).unwrap();
            true
        };
        if !show_legend {
            if let Err(e) = Reflect::delete_property(&options, &"legend".into()) {
                log::warn!("!!Failed to delete legend property, err: {:?}", e);
            }
        }
        if should_set && has_float {
            let labels = Reflect::get(&yaxis, &"labels".into()).unwrap();
            Reflect::set(
                &labels,
                &"formatter".into(),
                formatter.as_ref().unchecked_ref(),
            )
            .unwrap();
            formatter.forget();
        }
        // y axis min and max
        if y_axis.0.is_some() || y_axis.1.is_some() || y_axis.2.is_some() {
            if let Some(name) = y_axis.0 {
                if let Ok(title) = Reflect::get(&yaxis, &"title".into()) {
                    if !title.is_object() {
                        let title = js_sys::Object::new();
                        Reflect::set(&title, &"text".into(), &name.to_string().into()).unwrap();
                        Reflect::set(&yaxis, &"title".into(), &title.into()).unwrap();
                    } else {
                        Reflect::set(&title, &"text".into(), &name.to_string().into()).unwrap();
                    }
                } else {
                    let title = js_sys::Object::new();
                    Reflect::set(&title, &"text".into(), &name.to_string().into()).unwrap();
                    Reflect::set(&yaxis, &"title".into(), &title.into()).unwrap();
                }
            }
            if let Some(min) = y_axis.1 {
                Reflect::set(&yaxis, &"min".into(), &min.into()).unwrap();
            }
            if let Some(max) = y_axis.2 {
                Reflect::set(&yaxis, &"max".into(), &max.into()).unwrap();
            }
        }
        // title
        if let Ok(title_obj) = Reflect::get(&options, &"title".into()) {
            if !title_obj.is_object() {
                let title_obj = js_sys::Object::new();
                Reflect::set(&title_obj, &"text".into(), &title.to_string().into()).unwrap();
                Reflect::set(&options, &"title".into(), &title_obj.into()).unwrap();
            } else {
                Reflect::set(&title_obj, &"text".into(), &title.to_string().into()).unwrap();
            }
        } else {
            let title_obj = js_sys::Object::new();
            Reflect::set(&title_obj, &"text".into(), &title.to_string().into()).unwrap();
            Reflect::set(&options, &"title".into(), &title_obj.into()).unwrap();
        }
        // zoom && animations
        if let Ok(chart) = Reflect::get(&options, &"chart".into()) {
            if !chart.is_object() {
                let chart = js_sys::Object::new();
                let zoom = js_sys::Object::new();
                let animation = js_sys::Object::new();
                Reflect::set(&zoom, &"enabled".into(), &has_zoom.into()).unwrap();
                Reflect::set(&animation, &"enabled".into(), &has_animation.into()).unwrap();
                Reflect::set(&chart, &"zoom".into(), &zoom.into()).unwrap();
                Reflect::set(&chart, &"animations".into(), &animation.into()).unwrap();
                Reflect::set(&options, &"chart".into(), &chart.into()).unwrap();
            } else {
                if let Ok(zoom) = Reflect::get(&chart, &"zoom".into()) {
                    if !zoom.is_object() {
                        let zoom = js_sys::Object::new();
                        Reflect::set(&zoom, &"enabled".into(), &has_zoom.into()).unwrap();
                        Reflect::set(&chart, &"zoom".into(), &zoom.into()).unwrap();
                    } else {
                        Reflect::set(&zoom, &"enabled".into(), &has_zoom.into()).unwrap();
                    }
                } else {
                    let zoom = js_sys::Object::new();
                    Reflect::set(&zoom, &"enabled".into(), &has_zoom.into()).unwrap();
                    Reflect::set(&chart, &"zoom".into(), &zoom.into()).unwrap();
                }
                if let Ok(animation) = Reflect::get(&chart, &"animations".into()) {
                    if !animation.is_object() {
                        let animation = js_sys::Object::new();
                        Reflect::set(&animation, &"enabled".into(), &has_animation.into()).unwrap();
                        Reflect::set(&chart, &"animations".into(), &animation.into()).unwrap();
                    } else {
                        Reflect::set(&animation, &"enabled".into(), &has_animation.into()).unwrap();
                    }
                } else {
                    let animation = js_sys::Object::new();
                    Reflect::set(&animation, &"enabled".into(), &has_animation.into()).unwrap();
                    Reflect::set(&chart, &"animations".into(), &animation.into()).unwrap();
                }
            }
        } else {
            let chart = js_sys::Object::new();
            let zoom = js_sys::Object::new();
            Reflect::set(&zoom, &"enabled".into(), &has_zoom.into()).unwrap();
            Reflect::set(&chart, &"zoom".into(), &zoom.into()).unwrap();
            Reflect::set(&options, &"chart".into(), &chart.into()).unwrap();
        };
        Reflect::set(&options, &"series".into(), &series.into()).unwrap();
        Some(options)
    } else {
        None
    }
}

pub fn render_ts_floats_auto(
    element: Element,
    template: &str,
    title: &str,
    show_legend: bool,
    has_zoom: bool,
    has_animation: bool,
    series_names: &[&str],
    series_data: &[&[f64]],
    x_axis: &[&[u64]],
) -> Option<ApexCharts> {
    render_ts_floats(
        element,
        template,
        title,
        show_legend,
        has_zoom,
        has_animation,
        series_names,
        series_data,
        x_axis,
        (None, None, None),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn render_ts_floats(
    element: Element,
    template: &str,
    title: &str,
    show_legend: bool,
    has_zoom: bool,
    has_animation: bool,
    series_names: &[&str],
    series_data: &[&[f64]],
    x_axis: &[&[u64]],
    y_axis: (Option<&str>, Option<f64>, Option<f64>),
) -> Option<ApexCharts> {
    let series = create_series_floats(series_names, series_data, x_axis, false);
    render_time_series(
        element,
        template,
        title,
        show_legend,
        has_zoom,
        has_animation,
        series,
        true,
        y_axis,
    )
}

pub fn create_series_mix(
    floats_names: &[&str],
    floats: &[&[f64]],
    floats_t: &[&[u64]],
    ints_names: &[&str],
    ints: &[&[i64]],
    ints_t: &[&[u64]],
) -> js_sys::Array {
    let series = js_sys::Array::new_with_length((floats.len() + ints.len()) as u32);
    for i in 0..floats_names.len() {
        let item = js_sys::Object::new();
        let data = js_sys::Array::new_with_length(floats[i].len() as u32);
        for j in 0..floats[i].len() {
            let t_v = js_sys::Array::new_with_length(2);
            t_v.set(0, (floats_t[i][j] as f64).into());
            t_v.set(1, floats[i][j].into());
            data.set(j as u32, t_v.into());
        }
        let series_name = floats_names[i].to_string();
        Reflect::set(&item, &"name".into(), &series_name.into()).unwrap();
        Reflect::set(&item, &"data".into(), &data.into()).unwrap();
        series.set(i as u32, item.into());
    }
    for i in 0..ints.len() {
        let item = js_sys::Object::new();
        let data = js_sys::Array::new_with_length(ints[i].len() as u32);
        for j in 0..ints[i].len() {
            let t_v = js_sys::Array::new_with_length(2);
            t_v.set(0, (ints_t[i][j] as f64).into());
            t_v.set(1, ints[i][j].into());
            data.set(j as u32, t_v.into());
        }
        let series_name = ints_names[i].to_string();
        Reflect::set(&item, &"name".into(), &series_name.into()).unwrap();
        Reflect::set(&item, &"data".into(), &data.into()).unwrap();
        series.set((i + floats.len()) as u32, item.into());
    }
    series
}

#[allow(clippy::too_many_arguments)]
pub fn render_ts_mix(
    element: Element,
    template: &str,
    title: &str,
    show_legend: bool,
    has_zoom: bool,
    has_animation: bool,
    floats_names: &[&str],
    floats: &[&[f64]],
    floats_t: &[&[u64]],
    ints_names: &[&str],
    ints: &[&[i64]],
    ints_t: &[&[u64]],
    y_axis: (Option<&str>, Option<f64>, Option<f64>),
) -> Option<ApexCharts> {
    let series = create_series_mix(floats_names, floats, floats_t, ints_names, ints, ints_t);
    render_time_series(
        element,
        template,
        title,
        show_legend,
        has_zoom,
        has_animation,
        series,
        !floats.is_empty(),
        y_axis,
    )
}

pub fn render_time_series(
    element: Element,
    template: &str,
    title: &str,
    show_legend: bool,
    has_zoom: bool,
    has_animation: bool,
    series: js_sys::Array,
    has_float: bool,
    y_axis: (Option<&str>, Option<f64>, Option<f64>),
) -> Option<ApexCharts> {
    let options = create_options(
        template,
        title,
        show_legend,
        has_zoom,
        has_animation,
        series,
        has_float,
        y_axis,
    )?;
    let chart = ApexCharts::new(element, options);
    chart.render();
    Some(chart)
}
