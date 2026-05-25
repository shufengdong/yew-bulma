use std::collections::HashMap;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::*;

pub const UI_TEXT_IDS: [&str; 3] = ["pp", "np", "jump_to"];

pub struct MyPagination {
    current_pagination: usize,
    jump_ref: NodeRef,
}

pub enum Msg {
    ChangePaginationPre,
    ChangePaginationNext,
    ChangePagination(usize),
    JumpToPage,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_else(Callback::noop)]
    pub on_jump: Callback<usize>,
    pub row_num_per_page: usize,
    pub total_item_num: usize,
    pub ini_pagination: usize,
    #[prop_or_default]
    pub is_simple: bool,
    #[prop_or_default]
    pub text_map: HashMap<String, String>,
}

impl Component for MyPagination {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current_pagination: ctx.props().ini_pagination,
            jump_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.current_pagination = ctx.props().ini_pagination;
        let row_num_per_page = ctx.props().row_num_per_page;
        let total_item_num = ctx.props().total_item_num;
        let pagination_sum = if (total_item_num % row_num_per_page != 0)
            || (total_item_num == 0)
        {
            total_item_num / row_num_per_page + 1
        } else {
            total_item_num / row_num_per_page
        };
        match msg {
            Msg::ChangePagination(n) => {
                if self.current_pagination == n {
                    return false;
                }
                self.current_pagination = n;
                ctx.props().on_jump.emit(self.current_pagination);
                return true;
            }
            Msg::ChangePaginationPre => {
                let p = self.current_pagination - 1;
                let mut page_tobe = 1;
                if p > 1 {
                    page_tobe = p;
                };
                self.current_pagination = page_tobe;
                ctx.props().on_jump.emit(self.current_pagination);
                return true;
            }
            Msg::ChangePaginationNext => {
                let page_tobe = if self.current_pagination + 1 > pagination_sum {
                    self.current_pagination
                } else {
                    self.current_pagination + 1
                };
                self.current_pagination = page_tobe;
                ctx.props().on_jump.emit(self.current_pagination);
                return true;
            }
            Msg::JumpToPage => {
                if let Some(input) = self.jump_ref.cast::<HtmlInputElement>() {
                    if let Ok(n) = input.value().parse::<usize>() {
                        if n > 0 && n <= pagination_sum {
                            self.current_pagination = n;
                            ctx.props().on_jump.emit(self.current_pagination);
                        }
                    }
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_page = ctx.props().ini_pagination;
        let link = ctx.link();
        let row_num_per_page = ctx.props().row_num_per_page;
        let total_item_num = ctx.props().total_item_num;
        // 测点展示表格 - 页码
        let previous = html! {
            <PaginationItem item_type={PaginationItemType::Previous}
                onclick={link.callback(|_| Msg::ChangePaginationPre)}>
                //{self.get_text(ctx, "pp")}
                <Icon size={Size::Small} awesome_icon={"fa fa-chevron-left"} />
            </PaginationItem>
        };
        let next = html! {
            <PaginationItem item_type={PaginationItemType::Next}
                onclick={link.callback(|_| Msg::ChangePaginationNext)}>
                //{self.get_text(ctx, "np")}
                <Icon size={Size::Small} awesome_icon={"fa fa-chevron-right"} />
            </PaginationItem>
        };
        let pagination_sum = if (total_item_num % row_num_per_page != 0)
            || (total_item_num == 0)
        {
            total_item_num / row_num_per_page + 1
        } else {
            total_item_num / row_num_per_page
        };
        let ellipsis = html! { <span class="pagination-ellipsis">{"\u{2026}"}</span> };
        // 页码自动省略
        let page_now = current_page;
        let last_page = pagination_sum;
        let pagination_body = if pagination_sum <= 8 {
            //如果总页码小于8，则全部展示
            html! {
                for n in 1..=pagination_sum {
                    <PaginationItem item_type={PaginationItemType::Link}
                        is_current={current_page == n}
                        onclick={link.callback(move |_| Msg::ChangePagination(n))}>
                        {n.to_string()}
                    </PaginationItem>
                }
            }
        } else if (1..=4).contains(&page_now) {
            //如果为前四页：1..=5 + ... + last
            html! {
                <>
                for n in 1..=5 {
                    <PaginationItem item_type={PaginationItemType::Link}
                        is_current={current_page == n}
                        onclick={link.callback(move |_| Msg::ChangePagination(n))}>
                        {n.to_string()}
                    </PaginationItem>
                }
                {ellipsis.clone()}
                <PaginationItem item_type={PaginationItemType::Link}
                    is_current={current_page == last_page}
                    onclick={link.callback(move |_| Msg::ChangePagination(last_page))}>
                    {last_page.to_string()}
                </PaginationItem>
                </>
            }
        } else if page_now >= last_page - 3 && page_now <= last_page {
            //后四页：1 + ... + (t-4)..=t
            html! {
                <>
                <PaginationItem item_type={PaginationItemType::Link}
                    is_current={current_page == 1}
                    onclick={link.callback(move |_| Msg::ChangePagination(1))}>
                    {"1"}
                </PaginationItem>
                {ellipsis.clone()}
                for n in (last_page - 4)..=last_page {
                    <PaginationItem item_type={PaginationItemType::Link}
                        is_current={current_page == n}
                        onclick={link.callback(move |_| Msg::ChangePagination(n))}>
                        {n.to_string()}
                    </PaginationItem>
                }
                </>
            }
        } else {
            //中间几页：1 + ... + (page_now-1)..=(page_now+1) + ... + last
            html! {
                <>
                <PaginationItem item_type={PaginationItemType::Link}
                    is_current={current_page == 1}
                    onclick={link.callback(move |_| Msg::ChangePagination(1))}>
                    {"1"}
                </PaginationItem>
                {ellipsis.clone()}
                for n in (page_now - 1)..=(page_now + 1) {
                    <PaginationItem item_type={PaginationItemType::Link}
                        is_current={current_page == n}
                        onclick={link.callback(move |_| Msg::ChangePagination(n))}>
                        {n.to_string()}
                    </PaginationItem>
                }
                {ellipsis}
                <PaginationItem item_type={PaginationItemType::Link}
                    is_current={current_page == last_page}
                    onclick={link.callback(move |_| Msg::ChangePagination(last_page))}>
                    {last_page.to_string()}
                </PaginationItem>
                </>
            }
        };
        // 页码跳转查询框
        let pagination_jump = html! {
            <Field addons=true >
                <Control>
                    <Input r#ref={&self.jump_ref} placeholder={self.get_text(ctx, "jump_to")} width={"4"}
                        onenterdown={link.callback(|_| Msg::JumpToPage)}/>
                </Control>
                <Control>
                    <Button classes={classes!("is-link","submit","is-outlined")}
                        onclick={link.callback(|_| Msg::JumpToPage)}>
                        <Icon size={Size::Small} awesome_icon={"fa fa-hand-o-right"} />
                    </Button>
                </Control>
            </Field>
        };
        let classes = if ctx.props().is_simple {
            Some(classes!("is-simple"))
        } else {
            None
        };
        html! {
             <Pagination previous={previous} next={next} alignment={Alignment::Right} classes={classes}>
                {pagination_body}
                {pagination_jump}
            </Pagination>
        }
    }
}

impl MyPagination {
    fn get_text(&self, ctx: &Context<Self>, key: &str) -> String {
        if let Some(s) = ctx.props().text_map.get(key) {
            s.clone()
        } else {
            key.to_string()
        }
    }
}
