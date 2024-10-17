use std::collections::HashMap;

use log::debug;
use serde::{Deserialize, Serialize};
use web_sys::Element;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::chart::timeseries::{
    create_series_floats, create_series_ints, create_series_mix, render_ts_floats, render_ts_ints,
    render_ts_mix,
};
use crate::chart::ApexCharts;
use crate::*;

pub const UI_TEXT_IDS: [&str; 8] = [
    "min",
    "max",
    "average",
    "sum",
    "show_statistics",
    "show_sum",
    "show_legend",
    "card_chart",
];

pub struct ChartView {
    chart: Option<ApexCharts>,
    data: Option<ChartSeries>,
    template: String,
    chart_root_ref: NodeRef,
    is_show_statistics: bool,
    is_show_sum: bool,
    is_show_legend: bool,
    min_point: Option<(usize, usize)>,
    max_point: Option<(usize, usize)>,
    average_value: Option<f64>,
    sum_value: Option<f64>,
    #[allow(dead_code)]
    subscription: Box<dyn Bridge<MyEventBus>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChartSeries {
    TimeSeries(Vec<TimeSeries>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TimeSeries {
    Ints(String, Vec<u64>, Vec<i64>),
    Floats(String, Vec<u64>, Vec<f64>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Msg {
    ShowStatistics,
    ShowSum,
    ShowLegend,
    ChangeTemplate(String, String),
    UpdateSeries(String, ChartSeries),
    None,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    #[prop_or_default]
    pub title: String,
    #[prop_or("".to_string())]
    pub init_template: String,
    #[prop_or_default]
    pub y_axis_title: String,
    #[prop_or_default]
    pub card_title: String,
    #[prop_or_default]
    pub text_map: HashMap<String, String>,
    // 是否带缩放工具栏，默认带缩放
    #[prop_or_else(|| true)]
    pub has_zoom: bool,
    // 是否带动画，注意，如果数据量较大，带动画会很卡，默认不带动画
    #[prop_or_default]
    pub has_animation: bool,
}

impl Component for ChartView {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let local_storage = window().local_storage().unwrap().unwrap();
        let template = if let Ok(Some(s)) = local_storage.get_item(&ctx.props().id) {
            s
        } else {
            ctx.props().init_template.clone()
        };
        let key = format!("{}-is_show_statistics", ctx.props().id);
        let is_show_statistics = if let Ok(Some(s)) = local_storage.get_item(&key) {
            s == "true"
        } else {
            false
        };
        let key = format!("{}-is_show_sum", ctx.props().id);
        let is_show_sum = if let Ok(Some(s)) = local_storage.get_item(&key) {
            s == "true"
        } else {
            false
        };
        let key = format!("{}-is_show_legend", ctx.props().id);
        let is_show_legend = if let Ok(Some(s)) = local_storage.get_item(&key) {
            s == "true"
        } else {
            false
        };
        let cb = {
            let link = ctx.link().clone();
            move |msg| {
                link.send_message(match msg {
                    MyMsg::ChartMsg(message) => message,
                    _ => Msg::None,
                })
            }
        };
        let subscription = MyEventBus::bridge(std::rc::Rc::new(cb));
        Self {
            chart: None,
            data: None,
            subscription,
            template,
            chart_root_ref: NodeRef::default(),
            is_show_statistics,
            is_show_sum,
            is_show_legend,
            min_point: None,
            max_point: None,
            average_value: None,
            sum_value: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowStatistics => {
                self.is_show_statistics = !self.is_show_statistics;
                let key = format!("{}-is_show_statistics", ctx.props().id);
                let value = if self.is_show_statistics {
                    "true"
                } else {
                    "false"
                };
                let local_storage = window().local_storage().unwrap().unwrap();
                local_storage.set_item(&key, value).unwrap();
                return true;
            }
            Msg::ShowSum => {
                self.is_show_sum = !self.is_show_sum;
                let key = format!("{}-is_show_sum", ctx.props().id);
                let value = if self.is_show_sum { "true" } else { "false" };
                let local_storage = window().local_storage().unwrap().unwrap();
                local_storage.set_item(&key, value).unwrap();
                return true;
            }
            Msg::ShowLegend => {
                self.is_show_legend = !self.is_show_legend;
                let key = format!("{}-is_show_legend", ctx.props().id);
                let value = if self.is_show_legend { "true" } else { "false" };
                let local_storage = window().local_storage().unwrap().unwrap();
                local_storage.set_item(&key, value).unwrap();
                return true;
            }
            Msg::ChangeTemplate(id, template) => {
                debug!(
                    "Change template msg to id {} received, my id: {}",
                    id,
                    ctx.props().id
                );
                if ctx.props().id == id && self.template != template {
                    self.template = template;
                    let local_storage = window().local_storage().unwrap().unwrap();
                    local_storage.set_item(&id, &self.template).unwrap();
                    if let Some(chart) = &self.chart {
                        chart.destroy();
                    };
                    if let Some(chart) = &self.chart {
                        chart.destroy();
                    }
                    self.chart = None;
                    return true;
                }
            }
            Msg::UpdateSeries(id, series) => {
                debug!(
                    "Update series msg to id {} received, my id: {}",
                    id,
                    ctx.props().id
                );
                if ctx.props().id != id {
                    return false;
                }
                let mut min_series = 0;
                let mut min_pos = 0;
                let mut max_series = 0;
                let mut max_pos = 0;
                let mut min = f64::MAX;
                let mut max = f64::MIN;
                let mut sum = 0.;
                let mut count = 0;
                let mut is_null = false;
                let mut names = Vec::new();
                match &series {
                    ChartSeries::TimeSeries(data) => {
                        if data.is_empty() {
                            is_null = true;
                        }
                        for (index, data_i) in data.iter().enumerate() {
                            match data_i {
                                TimeSeries::Ints(name, _, v) => {
                                    names.push(name.clone());
                                    count += v.len();
                                    for (i, v_i) in v.iter().enumerate() {
                                        sum += *v_i as f64;
                                        if (*v_i as f64) < min {
                                            min = *v_i as f64;
                                            min_series = index;
                                            min_pos = i;
                                        }
                                        if (*v_i as f64) > max {
                                            max = *v_i as f64;
                                            max_series = index;
                                            max_pos = i;
                                        }
                                    }
                                }
                                TimeSeries::Floats(name, _, v) => {
                                    names.push(name.clone());
                                    count += v.len();
                                    for (i, v_i) in v.iter().enumerate() {
                                        sum += *v_i;
                                        if *v_i < min {
                                            min = *v_i;
                                            min_series = index;
                                            min_pos = i;
                                        }
                                        if *v_i > max {
                                            max = *v_i;
                                            max_series = index;
                                            max_pos = i;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if !is_null {
                    self.min_point = Some((min_series, min_pos));
                    self.max_point = Some((max_series, max_pos));
                    self.average_value = Some(sum / (count as f64));
                    self.sum_value = Some(sum);
                    // judge whether chart should rebuild
                    if let Some(old_series) = &self.data {
                        let mut old_names = Vec::new();
                        match old_series {
                            ChartSeries::TimeSeries(old_data) => {
                                for v in old_data {
                                    match v {
                                        TimeSeries::Ints(name, _, _) => {
                                            old_names.push(name.clone())
                                        }
                                        TimeSeries::Floats(name, _, _) => {
                                            old_names.push(name.clone())
                                        }
                                    }
                                }
                            }
                        }
                        if old_names != names {
                            if let Some(chart) = &self.chart {
                                chart.destroy();
                            }
                            self.chart = None;
                        }
                    }
                    self.data = Some(series);
                    return true;
                } else {
                    if let Some(chart) = &self.chart {
                        chart.destroy();
                    }
                    self.chart = None;
                }
            }
            Msg::None => {}
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let dropdown_button = html! {
            <Icon awesome_icon={"fa fa-bars"} />
        };
        let dropdown_id = format!("{}-dropdown", ctx.props().id);
        let min = if let Some(data) = &self.data {
            if let Some((i, j)) = &self.min_point {
                match data {
                    ChartSeries::TimeSeries(series) => match &series[*i] {
                        TimeSeries::Ints(_, _, v) => v[*j].to_string(),
                        TimeSeries::Floats(_, _, v) => {
                            format!("{:.3}", v[*j])
                        }
                    },
                }
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };
        let max = if let Some(data) = &self.data {
            if let Some((i, j)) = &self.max_point {
                match data {
                    ChartSeries::TimeSeries(series) => match &series[*i] {
                        TimeSeries::Ints(_, _, v) => v[*j].to_string(),
                        TimeSeries::Floats(_, _, v) => {
                            format!("{:.3}", v[*j])
                        }
                    },
                }
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };
        let average = if let Some(a) = self.average_value {
            format!("{:.3}", a)
        } else {
            "".to_string()
        };
        let sum = if let Some(a) = self.sum_value {
            format!("{:.3}", a)
        } else {
            "".to_string()
        };
        html! {
            <Card>
                <CardHeader>
                     <Level classes={classes!("is-mobile", "card-header-title")}>
                        <LevelRight>
                            <p class="card-header-title">{ctx.props().card_title.clone()}</p>
                        </LevelRight>
                        <LevelLeft>
                            if self.is_show_statistics {
                                if !max.is_empty() {
                                    <LevelItem>
                                        <div>
                                            <p class={"heading"}>{self.get_text(ctx, "max")}</p>
                                            <Title size={HeaderSize::Is4}>{max}</Title>
                                        </div>
                                    </LevelItem>
                                }
                                if !min.is_empty() {
                                    <LevelItem>
                                        <div>
                                            <p class={"heading"}>{self.get_text(ctx, "min")}</p>
                                            <Title size={HeaderSize::Is4}>{min}</Title>
                                        </div>
                                    </LevelItem>
                                }
                                if !average.is_empty() {
                                    <LevelItem>
                                        <div>
                                            <p class={"heading"}>{self.get_text(ctx, "average")}</p>
                                            <Title size={HeaderSize::Is4}>{average}</Title>
                                        </div>
                                    </LevelItem>
                                }
                            }
                            if self.is_show_sum {
                                if !sum.is_empty() {
                                    <LevelItem>
                                        <div>
                                            <p class={"heading"}>{self.get_text(ctx, "sum")}</p>
                                            <Title size={HeaderSize::Is4}>{sum}</Title>
                                        </div>
                                    </LevelItem>
                                }
                            }
                        </LevelLeft>
                        <LevelRight>
                            <Dropdown id={dropdown_id}
                                button_classes={classes!("is-small", "card-header-icon")}
                                button_html={dropdown_button}>
                                <Checkbox classes={classes!("dropdown-item")} checked={self.is_show_statistics}
                                    update={link.callback(|_|Msg::ShowStatistics)}>
                                    {self.get_text(ctx, "show_statistics")}
                                </Checkbox>
                                <Checkbox classes={classes!("dropdown-item")} checked={self.is_show_sum}
                                    update={link.callback(|_|Msg::ShowSum)}>
                                    {self.get_text(ctx, "show_sum")}
                                </Checkbox>
                                <Checkbox classes={classes!("dropdown-item")} checked={self.is_show_legend}
                                    update={link.callback(|_|Msg::ShowLegend)}>
                                    {self.get_text(ctx, "show_legend")}
                                </Checkbox>
                            </Dropdown>
                        </LevelRight>
                    </Level>
                </CardHeader>
                <CardContent>
                    <div ref={self.chart_root_ref.clone()} />
                </CardContent>
            </Card>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(div) = self.chart_root_ref.cast::<Element>() {
            // 先清空
            if self.chart.is_none() {
                while div.child_element_count() > 0 {
                    div.first_element_child().unwrap().remove();
                }
            }
        } else {
            return;
        }
        if self.template.is_empty() {
            return;
        }
        let element = self.chart_root_ref.cast::<Element>().unwrap();
        let title = ctx.props().title.clone();
        if let Some(series) = &self.data {
            match series {
                ChartSeries::TimeSeries(vec) => {
                    let mut floats_names = Vec::new();
                    let mut ints_names = Vec::new();
                    let mut floats = Vec::new();
                    let mut floats_t = Vec::new();
                    let mut ints = Vec::new();
                    let mut ints_t = Vec::new();
                    for series in vec {
                        match series {
                            TimeSeries::Ints(name, t, v) => {
                                ints_names.push(name.as_str());
                                ints_t.push(t.as_slice());
                                ints.push(v.as_slice());
                            }
                            TimeSeries::Floats(name, t, v) => {
                                floats_names.push(name.as_str());
                                floats_t.push(t.as_slice());
                                floats.push(v.as_slice());
                            }
                        }
                    }
                    if floats.is_empty() && ints.is_empty() {
                        return;
                    }
                    if floats.is_empty() {
                        if self.chart.is_none() {
                            self.chart = render_ts_ints(
                                element,
                                &self.template,
                                &title,
                                self.is_show_legend,
                                ctx.props().has_zoom,
                                ctx.props().has_animation,
                                &ints_names,
                                &ints,
                                &ints_t,
                                (Some(&ctx.props().y_axis_title), None, None),
                            );
                        } else {
                            let new_series = create_series_ints(&ints_names, &ints, &ints_t);
                            self.chart
                                .as_ref()
                                .unwrap()
                                .updateSeries(new_series, false.into());
                        }
                    } else if ints.is_empty() {
                        if self.chart.is_none() {
                            self.chart = render_ts_floats(
                                element,
                                &self.template,
                                &title,
                                self.is_show_legend,
                                ctx.props().has_zoom,
                                ctx.props().has_animation,
                                &floats_names,
                                &floats,
                                &floats_t,
                                (Some(&ctx.props().y_axis_title), None, None),
                            );
                        } else {
                            let new_series =
                                create_series_floats(&floats_names, &floats, &floats_t, true);
                            self.chart
                                .as_ref()
                                .unwrap()
                                .updateSeries(new_series, false.into());
                        }
                    } else if self.chart.is_none() {
                        self.chart = render_ts_mix(
                            element,
                            &self.template,
                            &title,
                            self.is_show_legend,
                            ctx.props().has_zoom,
                            ctx.props().has_animation,
                            &floats_names,
                            &floats,
                            &floats_t,
                            &ints_names,
                            &ints,
                            &ints_t,
                            (None, None, None),
                        );
                    } else {
                        let new_series = create_series_mix(
                            &floats_names,
                            &floats,
                            &floats_t,
                            &ints_names,
                            &ints,
                            &ints_t,
                        );
                        self.chart
                            .as_ref()
                            .unwrap()
                            .updateSeries(new_series, false.into());
                    }
                }
            }
        }
    }
}

impl ChartView {
    fn get_text(&self, ctx: &Context<Self>, key: &str) -> String {
        if let Some(s) = ctx.props().text_map.get(key) {
            s.clone()
        } else {
            key.to_string()
        }
    }
}
