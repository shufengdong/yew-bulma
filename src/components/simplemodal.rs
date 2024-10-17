use yew::prelude::*;

pub enum Msg {
    Commit,
    ButtonClicked1,
    Close,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SimpleModalCardProps {
    /// The title of this modal.
    pub title: String,
    #[prop_or_default]
    pub commit_title: Option<String>,
    #[prop_or_default]
    pub close_title: Option<String>,
    #[prop_or_default]
    pub button_title1: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub hidden: Option<bool>,
    #[prop_or_default]
    pub classes: Option<Classes>,
    #[prop_or_default]
    pub width: Option<String>,
    #[prop_or_else(Callback::noop)]
    pub on_button1_click: Callback<()>,
    #[prop_or_else(Callback::noop)]
    pub on_commit: Callback<()>,
    #[prop_or_else(Callback::noop)]
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub content_ref: NodeRef,
}

/// 基于 ModalCard 组件做了精简
/// 因为 ModalCard 必须与按钮配套使用，且在页面初始化时必须初始化内容HTML
/// 如果页面有多个相同数据结构的数据（如表格数据），必须提前将每个弹出框内容初始化到页面
/// 这就导致了如果数据行较多，页面加载有明显的性能问题
/// 所以新增了简化版的 ModalCard ，对于以上场景，页面上只初始化一份内容HTML，然后根据页面逻辑变换内容
/// 1、隐藏：SimpleModalCard 的显示通过传递 hidden 属性进行控制
/// 所以在 on_commit 和 on_close 回调函数中，需要主动改变 hidden 属性
/// 对于 on_commit 的回调，应该先判断提交内容是否正确，再设置 hidden=true
/// 2、底部按钮：传递 commit_title 属性则显示底部按钮，同时需配合 on_commit 回调函数进行逻辑处理
pub struct SimpleModalCard {}

impl Component for SimpleModalCard {
    type Message = Msg;
    type Properties = SimpleModalCardProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Commit => {
                ctx.props().on_commit.emit(());
            }
            Msg::Close => {
                ctx.props().on_close.emit(());
            }
            Msg::ButtonClicked1 => {
                ctx.props().on_button1_click.emit(());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("modal");
        let need_hidden = if let Some(v) = ctx.props().hidden {
            v
        } else {
            false
        };
        let need_foot = ctx.props().commit_title.is_some();
        let link = ctx.link();
        classes.push("is-active");
        classes.push(&ctx.props().classes);
        let width = if let Some(w) = &ctx.props().width {
            format!("width:{}px", w)
        } else {
            "width:640px".to_string()
        };
        html! {
            <>
            if !need_hidden {
                <div class={classes}>
                    <div class={"modal-background"} onclick={link.callback(move |_|Msg::Close)}></div>
                    <div class={"modal-card"} style={width}>
                        <header class={"modal-card-head"}>
                            <p class={"modal-card-title"}>{ctx.props().title.clone()}</p>
                            <button aria-label="close" class="delete"
                                onclick={link.callback(move |_|Msg::Close)}></button>
                        </header>
                        <section class={"modal-card-body"} ref={ctx.props().content_ref.clone()}>
                            { for ctx.props().children.iter() }
                        </section>
                        if need_foot {
                            <footer class="modal-card-foot">
                                <div class="buttons">
                                    if ctx.props().button_title1.is_some() {
                                        <button onclick={link.callback(move |_|Msg::ButtonClicked1)} class="button is-primary">
                                            {ctx.props().button_title1.clone().unwrap_or_default()}
                                        </button>
                                    }
                                    <button onclick={link.callback(move |_|Msg::Commit)} class="button is-primary">
                                        {ctx.props().commit_title.clone().unwrap_or_default()}
                                    </button>
                                    if ctx.props().close_title.is_some() {
                                        <button onclick={link.callback(move |_|Msg::Close)} class="button is-primary">
                                            {ctx.props().close_title.clone().unwrap_or_default()}
                                        </button>
                                    }
                                </div>
                            </footer>
                        }
                    </div>
                    <button aria-label="close" class="modal-close is-large"
                        onclick={link.callback(move |_|Msg::Close)}></button>
                </div>
            }
            </>
        }
    }
}
