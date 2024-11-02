use std::collections::HashMap;
use yew::prelude::*;
use crate::components::mypagination::MyPagination;
use crate::components::myrownumdp::MyRowNumDP;
use crate::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub list_id: String,
    #[prop_or_default]
    pub index: Vec<usize>,
    #[prop_or_default]
    pub paths: Vec<String>,
    #[prop_or_else(Callback::noop)]
    pub on_select: Callback<usize>,
    #[prop_or_default]
    pub text_map: HashMap<String, String>,
}

pub enum Msg {
    // 列表分页相关
    RowNumPerPage(usize),
    JumpToPage(usize),
    ItemSelected(usize),
}
pub struct MyList {
    /// 每页行数
    row_num_per_page: usize,
    /// 当前的页码
    current_pagination: usize,
    selected: Option<usize>,
}

impl Component for MyList {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            row_num_per_page: 10,
            current_pagination: 1,
            selected: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RowNumPerPage(n) => {
                self.row_num_per_page = n;
                self.current_pagination = 1;
                return true;
            }
            Msg::JumpToPage(n) => {
                if self.current_pagination == n {
                    return false;
                }
                self.current_pagination = n;
                return true;
            }
            Msg::ItemSelected(i) => {
                let new_v = Some(i);
                if self.selected != new_v {
                    ctx.props().on_select.emit(i);
                    self.selected = new_v;
                    return true;
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let start = (self.current_pagination - 1) * self.row_num_per_page;
        let len = if self.row_num_per_page != 0 {
            usize::min(self.row_num_per_page, ctx.props().paths.len() - start)
        } else {
            ctx.props().paths.len()
        };
        let list_content = (0..len).map(|i| {
            let index = i + start;
            let name = ctx.props().paths[index].clone();
            let pos = ctx.props().index[index];
            let is_active = if let Some(selected) = &self.selected {
                if pos == *selected {
                    "is-active"
                } else {
                    ""
                }
            } else {
                ""
            };
            html! {
                <li>
                    <a class={format!("filetree-node {is_active}")}
                        onclick={link.callback(move |_|Msg::ItemSelected(pos))}>
                        <span>{name}</span>
                    </a>
                </li>
            }
        }).collect::<Html>();
        let row_dp_id = format!("mylist_rows_dp_{}", ctx.props().list_id);
        html! {
            <>
            <aside class={"menu filetree"}>
                <ul class="menu-list">{list_content}</ul>
            </aside>
            <Level>
                <LevelLeft>
                    <LevelItem>
                        <MyRowNumDP dp_id={row_dp_id} row_num_per_page={self.row_num_per_page}
                            on_select={link.callback(Msg::RowNumPerPage)}
                            text_map={ctx.props().text_map.clone()} is_simple={true}/>
                    </LevelItem>
                </LevelLeft>
                if ctx.props().paths.len() > self.row_num_per_page && self.row_num_per_page != 0 {
                    <LevelRight>
                        <LevelItem>
                            <MyPagination on_jump={link.callback(Msg::JumpToPage)}
                                ini_pagination={self.current_pagination}
                                total_item_num={ctx.props().paths.len()} is_simple={true}
                                row_num_per_page={self.row_num_per_page}
                                text_map={ctx.props().text_map.clone()} />
                        </LevelItem>
                    </LevelRight>
                }
            </Level>
            </>
        }
    }
}