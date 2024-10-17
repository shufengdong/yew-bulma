use js_sys::Reflect;
use yew::prelude::*;

use crate::chart::select_and_render_json;
use crate::*;

const CHART_HEIGHT: u32 = 180;

pub enum Msg {
    Select(usize),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub templates: Vec<(String, String)>,
    #[prop_or(0)]
    pub init_selected: usize,
    #[prop_or_else(Callback::noop)]
    pub on_select: Callback<usize>,
}

pub struct ChartChooser {
    current_selected: usize,
}

impl Component for ChartChooser {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current_selected: ctx.props().init_selected,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(index) => {
                if self.current_selected != index {
                    self.current_selected = index;
                    ctx.props().on_select.emit(index);
                    return true;
                }
            }
        }
        false
    }

    fn rendered(&mut self, ctx: &Context<Self>, _: bool) {
        for (key, value) in &ctx.props().templates {
            if let Ok(options) = js_sys::JSON::parse(value) {
                if let Ok(chart) = Reflect::get(&options, &"chart".into()) {
                    Reflect::set(&chart, &"height".into(), &CHART_HEIGHT.into()).unwrap();
                } else {
                    let chart = js_sys::Object::new();
                    Reflect::set(&chart, &"height".into(), &CHART_HEIGHT.into()).unwrap();
                    Reflect::set(&options, &"chart".into(), &chart.into()).unwrap();
                }
                select_and_render_json(&format!("#{}", key), options);
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let cl: Vec<String> = (0..ctx.props().templates.len())
            .map(|i| {
                if i == self.current_selected {
                    "is-info".to_string()
                } else {
                    "is-dark".to_string()
                }
            })
            .collect();
        // 收集
        let content = html! {
            (0..ctx.props().templates.len()).map(|i| {
                let key = &ctx.props().templates[i].0;
                html! {
                    <Column classes={Classes::from("is-6")} >
                        <Message classes={Classes::from(&cl[i])}>
                            <MessageHeader>
                                <Level>
                                    <p>{format!("{}", key)}</p>
                                    <Radio value={key.clone()} update={link.callback(move |_|Msg::Select(i))}
                                        checked_value={if i == self.current_selected {key.clone()} else {"".to_string()}} />
                                </Level>
                            </MessageHeader>
                            <MessageBody><div id={key.to_string()} style={"width: 100%"}/></MessageBody>
                        </Message>
                    </Column>
                }
            }).collect::<Html>()
        };
        html! {
            <Columns multiline={true}>
                {content}
            </Columns>
        }
    }
}
