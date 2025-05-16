use std::collections::HashMap;
use std::collections::HashSet;

use log::debug;
use petgraph::prelude::*;
use petgraph::stable_graph::EdgeReference;
use serde::{Deserialize, Serialize};

use web_sys::{Event, HtmlElement};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_agent::{Bridge, Bridged};

use crate::components::mypagination::MyPagination;
use crate::components::myrownumdp::MyRowNumDP;
use crate::*;
use crate::calendar::get_timestamp;

pub const UI_TEXT_IDS: [&str; 2] = ["same_path_note", "search_tree_node"];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FoldUnexpanded {
    value: HashMap<String, bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Msg {
    LayoutClicked(String, bool),
    NodeChecked(bool, String),
    LeafSelected(String),
    Reload(Option<u32>),
    New(String, bool),
    Rename(String),
    Move(String),
    Delete(Vec<String>),
    MoveUp,
    MoveDown,
    // 查找树节点
    Find,
    // 列表分页相关
    RowNumPerPage(usize),
    JumpToPage(usize),
    // 无动作
    None,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_else(Callback::noop)]
    pub on_selected: Callback<String>,
    /// 复选框勾选事件
    /// (选中的路径列表, 本次操作的路径, 勾选true取消勾选false)
    #[prop_or_else(Callback::noop)]
    pub on_checked: Callback<(Vec<String>, String, bool)>,
    #[prop_or_else(Callback::noop)]
    pub on_reload: Callback<Option<u32>>,
    #[prop_or_else(Callback::noop)]
    pub on_add: Callback<String>,
    #[prop_or_else(Callback::noop)]
    pub on_delete: Callback<Vec<String>>,
    #[prop_or_else(Callback::noop)]
    pub on_change: Callback<(String, String)>,
    #[prop_or_default]
    pub paths: Vec<String>,
    #[prop_or_default]
    pub tree_id: String,
    #[prop_or_default]
    pub has_search: bool,
    #[prop_or_default]
    pub has_pagination: bool,
    /// 是否多选，默认单选
    #[prop_or_default]
    pub is_multiple: Option<bool>,
    /// 选择路径可选约束，默认没有约束
    /// 表示允许该列表内的路径可选，其他不可选
    #[prop_or_default]
    pub path_constraint: Option<Vec<String>>,
    #[prop_or_default]
    pub search_placeholder: String,
    #[prop_or_default]
    pub type_map: HashMap<String, u8>,
    #[prop_or_default]
    pub icon_map: HashMap<u8, String>,
    #[prop_or_default]
    pub text_map: HashMap<String, String>,
    /// 默认选中的节点，不触发on_selected事件
    #[prop_or_default]
    pub selected: Option<String>,
}

pub struct FileTree {
    /// 文件目录打开的情况
    folder_unexpanded: FoldUnexpanded,
    /// 当前选中的节点
    selected: Option<String>,
    find_start: Option<usize>,
    /// 当前勾选的节点
    checked: HashSet<String>,
    /// 对应的树数据结构
    graph: StableDiGraph<String, usize>,
    /// 根节点索引
    root_index: NodeIndex,
    /// 下一个edge的id
    next_edge_id: usize,
    /// 消息总线
    _producer: Box<dyn Bridge<MyEventBus>>,
    /// 不启用路由时的本地路径存储
    local_paths: Vec<String>,
    /// 搜索框
    find_input_ref: NodeRef,
    /// 每页行数
    row_num_per_page: usize,
    /// 当前的页码
    current_pagination: usize,
}

impl Component for FileTree {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let debug_start = js_sys::Date::now();
        debug!("文件树组件create开始……");
        let tree_id = ctx.props().tree_id.clone();
        let local_storage = window().local_storage().unwrap().unwrap();
        let folder_unexpanded = if !tree_id.is_empty() {
            if let Ok(Some(s)) = local_storage.get_item(&tree_id) {
                if let Ok(v) = serde_json::from_str::<Vec<String>>(&s) {
                    FoldUnexpanded { value: v.into_iter().map(|x| (x, true)).collect() }
                } else {
                    FoldUnexpanded { value: HashMap::new() }
                }
            } else {
                FoldUnexpanded { value: HashMap::new() }
            }
        } else {
            FoldUnexpanded { value: HashMap::new() }
        };
        let row_dp_id = format!("filetree_rows_dp_{tree_id}");
        let page_now = if let Ok(Some(s)) = local_storage.get_item(&row_dp_id) {
            s.parse::<usize>().unwrap()
        } else {
            10
        };
        let empty_path = vec!["".to_string()];
        let paths = if ctx.props().paths.is_empty() {
            empty_path.as_slice()
        } else {
            ctx.props().paths.as_slice()
        };
        let (graph, root_index, next_edge_id) = create_graph(paths);
        let cb = {
            let link = ctx.link().clone();
            move |msg| {
                link.send_message(match msg {
                    MyMsg::FileTree(message) => message,
                    MyMsg::FileTreeWithId(id, message) => {
                        if id == tree_id {
                            message
                        } else {
                            Msg::None
                        }
                    }
                    _ => Msg::None,
                })
            }
        };
        let mut file_tree = FileTree {
            folder_unexpanded,
            graph,
            root_index,
            next_edge_id,
            selected: ctx.props().selected.clone(),
            find_start: None,
            checked: HashSet::new(),
            local_paths: vec![],
            find_input_ref: NodeRef::default(),
            row_num_per_page: page_now,
            _producer: MyEventBus::bridge(std::rc::Rc::new(cb)),
            current_pagination: 1,
        };
        file_tree.update_path_in_tree_view();
        let elapsed = js_sys::Date::now() - debug_start;
        debug!("文件树组件create耗时: {}ms", elapsed);
        file_tree
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LayoutClicked(path, is_icon) => {
                // debug!("Path {} is selected.", path);
                if let Some(b) = self.folder_unexpanded.value.get_mut(&path) {
                    *b = !*b;
                }
                // 保存
                if !ctx.props().tree_id.is_empty() {
                    let mut need_save = Vec::new();
                    for (k, v) in &self.folder_unexpanded.value {
                        if *v {
                            need_save.push(k.clone());
                        }
                    }
                    if let Ok(v) = serde_json::to_string(&need_save) {
                        // debug!("saved:\n{}", v);
                        let local_storage = window().local_storage().unwrap().unwrap();
                        if let Err(e) = local_storage.set_item(&ctx.props().tree_id, &v) {
                            debug!("!!Failed to save tree, err: {:?}", e);
                        }
                    }
                }
                // 如果点击的是icon图标，则不进行选中操作
                if !is_icon {
                    self.selected = Some(path.clone());
                    ctx.props().on_selected.emit(path);
                }
                self.update_path_in_tree_view();
                return true;
            }
            Msg::LeafSelected(path) => {
                self.selected = Some(path.clone());
                ctx.props().on_selected.emit(path);
                return true;
            }
            Msg::NodeChecked(checked, path) => {
                if checked {
                    self.checked.insert(path.clone());
                } else {
                    self.checked.remove(&path);
                }
                ctx.props().on_checked.emit((
                    self.checked.clone().into_iter().collect(),
                    path,
                    checked,
                ));
                return true;
            }
            Msg::Reload(version) => {
                ctx.props().on_reload.emit(version);
                self.selected = None;
            }
            Msg::Delete(paths) => {
                if let Some(selected) = &self.selected {
                    if paths.contains(selected) {
                        self.selected = None;
                    }
                }
                for path in &paths {
                    self.remove_path(path.clone());
                }
                self.selected = None;
                ctx.props().on_delete.emit(paths.clone());
                return true;
            }
            Msg::Rename(new_name) => {
                debug!("Rename path from tree: {:?}", self.selected);
                if let Some(selected) = &self.selected {
                    let old_path = selected.clone();
                    let new_path = if let Some(pos) = selected.rfind('/') {
                        let mut s = selected.clone();
                        s.truncate(pos);
                        format!("{s}/{new_name}")
                    } else {
                        new_name.clone()
                    };
                    if self.find_node(new_path.clone()).is_some() {
                        alert(&self.get_text(ctx, "same_path_note"));
                        return false;
                    }
                    let node_index = self.find_node(selected.clone());
                    if let Some(index) = node_index {
                        let name = self.graph.node_weight_mut(index).unwrap();
                        name.clear();
                        name.push_str(&new_name);
                        self.selected = Some(new_path.clone());
                        self.update_path_in_tree_view();
                        let value = (old_path.clone(), new_path.clone());
                        ctx.props().on_change.emit(value);
                        ctx.props().on_selected.emit(new_path);
                        return true;
                    }
                }
            }
            Msg::New(new_path, is_select) => {
                self.add_path(new_path.clone());
                if is_select {
                    self.selected = Some(new_path.clone());
                    ctx.props().on_selected.emit(new_path.clone());
                }
                self.update_path_in_tree_view();
                ctx.props().on_add.emit(new_path);
                return true;
            }
            Msg::Move(new_path) => {
                if self.selected.is_none() {
                    return false;
                }
                if new_path.is_empty() {
                    return false;
                }
                if self.find_node(new_path.clone()).is_some() {
                    alert(&self.get_text(ctx, "same_path_note"));
                    return false;
                }
                let old_path = self.selected.as_ref().unwrap().clone();
                self.replace(old_path.clone(), new_path.clone());
                self.selected = Some(new_path.clone());
                self.update_path_in_tree_view();
                let value = (old_path.clone(), new_path.clone());
                ctx.props().on_change.emit(value);
                ctx.props().on_selected.emit(new_path);
                return true;
            }
            Msg::MoveUp => {
                return self.move_node(true);
            }
            Msg::MoveDown => {
                return self.move_node(false);
            }
            Msg::Find => {
                let start = if let Some(pos) = self.find_start {
                    pos + 1
                } else {
                    0
                };
                if let Some(input) = self.find_input_ref.cast::<HtmlInputElement>() {
                    let to_find: String = input.value().trim().to_lowercase();
                    if !to_find.is_empty() {
                        for (i, path) in self.local_paths.iter().enumerate().skip(start) {
                            if path.to_lowercase().contains(&to_find) {
                                self.find_start = Some(i);
                                self.selected = Some(path.clone());
                                self.current_pagination = if self.row_num_per_page != 0 {
                                    i / self.row_num_per_page + 1
                                } else {
                                    1
                                };
                                return true;
                            }
                        }
                    }
                }
                self.selected = None;
                self.find_start = None;
                return true;
            }
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
            Msg::None => {}
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if old_props.paths != ctx.props().paths {
            self.update_graph(ctx, &ctx.props().paths);
        }
        if old_props.selected.is_some() && old_props.selected == self.selected {
            self.selected = None;
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let debug_start = js_sys::Date::now();
        debug!("文件树组件build_tree_html开始……");
        let tree = self.build_tree_html(ctx);
        let elapsed = js_sys::Date::now() - debug_start;
        debug!("文件树组件build_tree_html耗时: {}ms", elapsed);
        let to_find = if let Some(input) = self.find_input_ref.cast::<HtmlInputElement>() {
            input.value()
        } else {
            "".to_string()
        };
        let search_placeholder = if !ctx.props().search_placeholder.is_empty() {
            ctx.props().search_placeholder.clone()
        } else {
            self.get_text(ctx, "search_tree_node")
        };
        let tree_id = &ctx.props().tree_id;
        let row_dp_id = format!("filetree_rows_dp_{tree_id}");
        html! {
            <>
            // 搜索栏
            if ctx.props().has_search {
                <Field classes={classes!("has-addons")}>
                    <Control classes={classes!("is-expanded", "has-icons-left")}>
                        <Input placeholder={search_placeholder}
                            r#ref={self.find_input_ref.clone()} value={to_find}
                            onenterdown={link.callback(|_|Msg::Find)}/>
                        <Icon classes ={classes!("is-left")}>
                            <i class={"fa fa-filter"}></i>
                        </Icon>
                    </Control>
                    <Control>
                        <Button classes={classes!("is-outlined")}
                            onclick={link.callback(|_| Msg::Find)}>
                            <Icon awesome_icon={"fa fa-search"} />
                        </Button>
                    </Control>
                </Field>
            }
            // 文件树
            {tree}
            // 分页
            if ctx.props().has_pagination {
                <Level>
                    <LevelLeft>
                        <LevelItem>
                            <MyRowNumDP dp_id={row_dp_id} row_num_per_page={self.row_num_per_page}
                                on_select={link.callback(Msg::RowNumPerPage)}
                                text_map={ctx.props().text_map.clone()} is_simple={true}/>
                        </LevelItem>
                    </LevelLeft>
                    if self.local_paths.len() > self.row_num_per_page && self.row_num_per_page != 0 {
                        <LevelRight>
                            <LevelItem>
                                <MyPagination on_jump={link.callback(Msg::JumpToPage)}
                                    ini_pagination={self.current_pagination}
                                    total_item_num={self.local_paths.len()} is_simple={true}
                                    row_num_per_page={self.row_num_per_page}
                                    text_map={ctx.props().text_map.clone()} />
                            </LevelItem>
                        </LevelRight>
                    }
                </Level>
            }
            </>
        }
    }
}

impl FileTree {
    #[inline]
    fn get_text(&self, ctx: &Context<Self>, key: &str) -> String {
        if let Some(s) = ctx.props().text_map.get(key) {
            s.clone()
        } else {
            key.to_string()
        }
    }

    fn build_tree_html(&self, ctx: &Context<Self>) -> VNode {
        let link = ctx.link();
        // 是否多选
        let is_multiple = if let Some(v) = ctx.props().is_multiple {
            v
        } else {
            false
        };
        // 存储element的map
        let mut top_ele = html! {<aside class={"menu filetree"} />};
        let root_ul = html! {<ul class={"menu-list"} />};
        // 生成树，不包含收起的节点
        let (tree, root_index) = self.create_show_tree(ctx);
        // id前缀，用于一个页面同时存在多棵树时，区分不同树
        let id_prefix = &ctx.props().tree_id;
        // 存储节点对应的path
        let mut paths = HashMap::with_capacity(tree.node_count());
        let mut stack2 = Vec::with_capacity(tree.node_count());
        let mut node_pos = HashMap::with_capacity(tree.node_count());
        let mut stack = Vec::new();
        // 先先把根节点放进去
        stack.push((root_index, root_ul));
        // 开始深度优先遍历
        while let Some((node_index, current_node)) = stack.pop() {
            let mut edges: Vec<petgraph::graph::EdgeReference<usize>> = tree.edges(node_index).collect();
            if edges.len() > 0 {
                node_pos.insert(node_index, stack2.len());
            }
            stack2.push((node_index, current_node));
            // 进行排序
            edges.sort_by(|a, b| a.weight().cmp(b.weight()));
            for edge in edges {
                let child_index = edge.target();
                let child_name = tree.node_weight(child_index).unwrap();
                let path = if let Some(father_id) = paths.get(&node_index) {
                    format!("{father_id}/{child_name}")
                } else {
                    // 根节点下面的节点
                    child_name.to_string()
                };
                paths.insert(child_index, path.clone());
                let id = format!("{id_prefix}_{path}");
                // 生成对应的element
                let i_node = if let Some(t) = ctx.props().type_map.get(&path) {
                    if let Some(s) = ctx.props().icon_map.get(t) {
                        html! {<i class={s}></i>}
                    } else if let Some(b) = self.folder_unexpanded.value.get(&path) {
                        if *b {
                            html! {<i class={"fa fa-folder"}></i>}
                        } else {
                            html! {<i class={"fa fa-folder-open"}></i>}
                        }
                    } else {
                        html! {}
                    }
                } else if let Some(b) = self.folder_unexpanded.value.get(&path) {
                    if *b {
                        html! {<i class={"fa fa-folder"}></i>}
                    } else {
                        html! {<i class={"fa fa-folder-open"}></i>}
                    }
                } else {
                    html! {}
                };

                // 是否需要复选框
                let need_checkbox = if is_multiple {
                    // 是否满足路径约束
                    if let Some(paths) = &ctx.props().path_constraint {
                        paths.contains(&path)
                    } else {
                        true
                    }
                } else {
                    false
                };
                let is_active = if let Some(selected) = &self.selected {
                    if path == *selected {
                        "is-active"
                    } else {
                        ""
                    }
                } else {
                    ""
                };
                let li = if need_checkbox {
                    // 复选框情况下的html构成
                    let is_checked = self.checked.contains(&path);
                    html! {
                        <li>
                            <div>
                                <a class={"filetree-check"}>
                                    <input id={format!("checkbox_{id}")} r#type={"checkbox"} checked={is_checked}
                                        onchange={link.callback(move |e: Event| Msg::NodeChecked(!is_checked,
                                            e.target_unchecked_into::<HtmlInputElement>().value()))} />
                                </a>
                                <a id={id} class={format!("filetree-node with-check {is_active}")}>
                                    {i_node}
                                    <span>{child_name}</span>
                                </a>
                            </div>
                        </li>
                    }
                } else {
                    // 是否满足路径约束
                    let cross_constraint = if let Some(paths) = ctx.props().path_constraint.clone() {
                        paths.contains(&path)
                    } else {
                        true
                    };
                    // 非复选框的html构成
                    html! {
                        <li>
                            if cross_constraint && !self.folder_unexpanded.value.contains_key(&path) {
                                <a id={id} class={format!("filetree-node {is_active}")}
                                     onclick={link.callback(move |_| Msg::LeafSelected(path.clone()))} >
                                    {i_node}
                                    <span>{child_name}</span>
                                </a>
                            } else if self.folder_unexpanded.value.contains_key(&path) {
                                <a id={id} class={format!("filetree-node {is_active}")}
                                     onclick={link.callback(move |e: MouseEvent| Msg::LayoutClicked(path.clone(),
                                            e.target_unchecked_into::<HtmlElement>().class_name().starts_with("fa fa-")))} >
                                    {i_node}
                                    <span>{child_name}</span>
                                </a>
                            } else {
                                <a id={id} class={format!("filetree-node {is_active}")}>
                                    {i_node}
                                    <span>{child_name}</span>
                                </a>
                            }
                        </li>
                    }
                };
                stack.push((child_index, li));
            }
        }
        let mut node_ul: HashMap<NodeIndex, VNode> = HashMap::new();
        while stack2.len() > 1 {
            let (node_index, mut current_node) = stack2.pop().unwrap();
            let edges = tree.edges_directed(node_index, Incoming);
            if let Some(ul) = node_ul.remove(&node_index) {
                if let VNode::VTag(father) = &mut current_node {
                    father.add_child(ul);
                }
            }
            for edge in edges {
                let father_pos = node_pos.get(&edge.source()).unwrap();
                if let VNode::VTag(father) = &mut stack2[*father_pos].1 {
                    if let Some(ul) = node_ul.get_mut(&edge.source()) {
                        if let VNode::VTag(ul) = ul {
                            ul.add_child(current_node);
                        }
                    } else if father.tag() == "li" {
                        let mut ul = html! {<ul />};
                        if let VNode::VTag(v) = &mut ul {
                            v.add_child(current_node);
                        }
                        node_ul.insert(edge.source(), ul);
                    } else {
                        father.add_child(current_node);
                    }
                    break;
                }
            }
        }
        let (_, root_ul) = stack2.pop().unwrap();
        if let VNode::VTag(father) = &mut top_ele {
            father.add_child(root_ul);
        }
        top_ele
    }

    fn move_node(&mut self, is_up: bool) -> bool {
        if let Some(selected) = &self.selected {
            // 存储节点对应的path
            let father = if let Some(pos) = selected.rfind('/') {
                selected[0..pos].to_string()
            } else {
                selected.clone()
            };
            if let Some(father_node) = self.find_node(father) {
                if let Some(son_node) = self.find_node(selected.clone()) {
                    if let Some(to_move) = self.graph.find_edge(father_node, son_node) {
                        let mut edges: Vec<EdgeReference<usize>> = self.graph.edges(father_node).collect();
                        // 进行排序
                        edges.sort_by(|a, b| a.weight().cmp(b.weight()));
                        let mut edge_ids = Vec::with_capacity(edges.len());
                        for edge in edges {
                            edge_ids.push(edge.id());
                        }
                        for i in 0..edge_ids.len() {
                            if edge_ids[i] == to_move {
                                if i == 0 && is_up {
                                    return false;
                                }
                                if i == edge_ids.len() - 1 && !is_up {
                                    return false;
                                }
                                // 交换权重值
                                let j = if is_up { i - 1 } else { i + 1 };
                                let wi = *self.graph.edge_weight(edge_ids[i]).unwrap();
                                let wj = *self.graph.edge_weight(edge_ids[j]).unwrap();
                                if let Some(w1) = self.graph.edge_weight_mut(edge_ids[j]) {
                                    *w1 = wi;
                                }
                                if let Some(w2) = self.graph.edge_weight_mut(edge_ids[i]) {
                                    *w2 = wj;
                                }
                                self.update_path_in_tree_view();
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn remove_path(&mut self, path: String) {
        debug!("Node num before remove {}", self.graph.node_count());
        let to_remove = self.find_node(path);
        if to_remove.is_none() {
            return;
        }
        self.remove_node_deeply(to_remove.unwrap());
        debug!("Node num after remove {}", self.graph.node_count());
    }

    fn remove_node_deeply(&mut self, node: NodeIndex) {
        // 开始深度优先遍历
        let mut stack = Vec::new();
        stack.push(node);
        while let Some(node_index) = stack.pop() {
            let mut ids = Vec::new();
            for edge in self.graph.edges(node_index) {
                let child_index = edge.target();
                stack.push(child_index);
                ids.push(edge.id());
            }
            // 先删除边
            for id in ids {
                self.graph.remove_edge(id);
            }
            // 再删除节点
            let removed = self.graph.remove_node(node_index);
            debug!("Remove {:?}", removed)
        }
    }

    fn replace(&mut self, old_path: String, new_path: String) {
        if old_path == new_path {
            return;
        }
        // 先插入新的节点
        let new_index = self.add_path(new_path);
        let to_remove = self.find_node(old_path);
        if to_remove.is_none() {
            return;
        }
        let mut children = Vec::new();
        let mut ids = Vec::new();
        for edge in self.graph.edges(to_remove.unwrap()) {
            children.push(edge.target());
            ids.push(edge.id());
        }
        // 先删除边
        for id in ids {
            self.graph.remove_edge(id);
        }
        // 再删除原节点
        self.graph.remove_node(to_remove.unwrap());
        // 增加新的边
        let mut next_id = self.next_edge_id;
        for child in children {
            self.graph.add_edge(new_index, child, next_id);
            next_id += 1;
        }
        self.next_edge_id = next_id;
    }

    fn add_path(&mut self, new_path: String) -> NodeIndex {
        let nodes: Vec<&str> = new_path.split('/').collect();
        let mut father_index = self.root_index;
        for node in nodes {
            let mut is_exist = false;
            for edge in self.graph.edges(father_index) {
                let son_index = edge.target();
                if let Some(name) = self.graph.node_weight(son_index) {
                    if name == node {
                        is_exist = true;
                        father_index = son_index;
                        break;
                    }
                }
            }
            if is_exist {
                continue;
            }
            let current_index = self.graph.add_node(node.into());
            let id = self.next_edge_id;
            self.next_edge_id += 1;
            self.graph.add_edge(father_index, current_index, id);
            father_index = current_index;
        }
        father_index
    }

    fn find_node(&self, path: String) -> Option<NodeIndex> {
        let nodes: Vec<&str> = path.split('/').collect();
        let mut current_index = self.root_index;
        for node in nodes {
            let mut is_exist = false;
            for edge in self.graph.edges(current_index) {
                let son_index = edge.target();
                if let Some(name) = self.graph.node_weight(son_index) {
                    if name == node {
                        is_exist = true;
                        current_index = son_index;
                        break;
                    }
                }
            }
            if !is_exist {
                return None;
            }
        }
        Some(current_index)
    }

    fn update_path_in_tree_view(&mut self) {
        let n = self.graph.node_count();
        let mut paths: HashMap<NodeIndex, String> = HashMap::with_capacity(n);
        let mut result = Vec::with_capacity(n / 2);
        // 开始深度优先遍历
        let mut stack = Vec::new();
        stack.push(self.root_index);
        while let Some(node_index) = stack.pop() {
            if let Some(path) = paths.get(&node_index) {
                result.push(path.clone());
            }
            let mut edges: Vec<EdgeReference<usize>> = self.graph.edges(node_index).collect();
            if !edges.is_empty() {  // this is a container
                if let Some(path) = paths.get(&node_index) {
                    if let Some(b) = self.folder_unexpanded.value.get(path) {
                        if *b {
                            continue;
                        }
                    } else {
                        self.folder_unexpanded.value.insert(path.clone(), false);
                    }
                }
                // 进行排序
                edges.sort_by(|a, b| b.weight().cmp(a.weight()));
                for edge in edges {
                    let child_index = edge.target();
                    let child_name = self.graph.node_weight(child_index).unwrap();
                    if let Some(father_id) = paths.get(&node_index) {
                        paths.insert(child_index, format!("{father_id}/{child_name}"));
                    } else {
                        // 根节点下面的节点
                        paths.insert(child_index, child_name.to_string());
                    }
                    stack.push(child_index);
                }
            } else {  // this is a leaf
                if let Some(path) = paths.get(&node_index) {
                    self.folder_unexpanded.value.remove(path);
                }
            }
        }
        result.shrink_to_fit();
        self.local_paths = result;
    }

    fn update_graph(&mut self, ctx: &Context<Self>, all_paths: &[String]) {
        let empty_path = vec!["".to_string()];
        let paths = if all_paths.is_empty() {
            &empty_path
        } else {
            all_paths
        };
        let debug_start = js_sys::Date::now();
        debug!("文件树组件update_graph开始……");
        let (graph, root_index, next_edge_id) = create_graph(paths);
        let elapsed = js_sys::Date::now() - debug_start;
        debug!("文件树组件update_graph耗时: {}ms", elapsed);
        self.selected = ctx.props().selected.clone();
        self.graph = graph;
        self.root_index = root_index;
        self.next_edge_id = next_edge_id;
        self.current_pagination = 1;
        let debug_start = js_sys::Date::now();
        debug!("文件树组件update_path_in_tree_view开始……");
        self.update_path_in_tree_view();
        let elapsed = js_sys::Date::now() - debug_start;
        debug!("文件树组件update_path_in_tree_view耗时: {}ms", elapsed);
    }

    fn create_show_tree(&self, ctx: &Context<Self>) -> (DiGraph<String, usize>, NodeIndex) {
        // 构造树结构
        let paths = if ctx.props().has_pagination {
            let start = (self.current_pagination - 1) * self.row_num_per_page;
            let len = if self.row_num_per_page != 0 {
                usize::min(self.row_num_per_page, self.local_paths.len() - start)
            } else {
                self.local_paths.len()
            };
            &self.local_paths[start..start + len]
        } else {
            self.local_paths.as_slice()
        };
        let mut weight = 0;
        let mut graph = DiGraph::new();
        let root_index = graph.add_node("root".into());
        for p in paths {
            let nodes: Vec<&str> = p.split('/').collect();
            let mut father_index = root_index;
            for node_name in nodes {
                let mut is_exist = false;
                for edge in graph.edges(father_index) {
                    let son_index = edge.target();
                    if let Some(name) = graph.node_weight(son_index) {
                        if name == node_name {
                            is_exist = true;
                            father_index = son_index;
                            break;
                        }
                    }
                }
                if is_exist {
                    continue;
                }
                weight += 1;
                let current_index = graph.add_node(node_name.into());
                graph.add_edge(father_index, current_index, weight);
                father_index = current_index;
            }
        }
        (graph, root_index)
    }
}

fn create_graph(paths: &[String]) -> (StableDiGraph<String, usize>, NodeIndex, usize) {
    // 构造树结构
    let mut graph = StableDiGraph::new();
    let root_index = graph.add_node("root".into());
    let mut edge_id = 0;
    for p in paths {
        let nodes: Vec<&str> = p.split('/').collect();
        let mut father_index = root_index;
        for node in nodes {
            let mut is_exist = false;
            for edge in graph.edges(father_index) {
                let son_index = edge.target();
                if let Some(name) = graph.node_weight(son_index) {
                    if name == node {
                        is_exist = true;
                        father_index = son_index;
                        break;
                    }
                }
            }
            if is_exist {
                continue;
            }
            let current_index = graph.add_node(node.into());
            graph.add_edge(father_index, current_index, edge_id);
            edge_id += 1;
            father_index = current_index;
        }
    }
    (graph, root_index, edge_id)
}

#[test]
fn test_tree_string_gen() {
    let v = vec![
        "dd2".to_string(),
        "dd3".to_string(),
        "a/aa1".to_string(),
        "a/aa2".to_string(),
        "dd1".to_string(),
        "a/b/bb4".to_string(),
        "a/b/c/cc1".to_string(),
        "a/aa3".to_string(),
        "a/b/bb1".to_string(),
        "a/b/bb2".to_string(),
        "a/b/bb3".to_string(),
        "a/b/c/cc2".to_string(),
        "a/b/c/cc3".to_string(),
        "a/b/c/cc4".to_string(),
    ];
    let (graph, _, _) = create_graph(&v);
    assert_eq!(graph.node_count(), 18);
    assert_eq!(graph.edge_count(), 17);
}

#[test]
fn test_big_graph() {
    let cap = 20000;
    let mut v = Vec::with_capacity(cap);
    for i in 0..cap {
        v.push(format!("a/b/c/cc{i}"));
    }
    let time = get_timestamp();
    let (_, _, _) = create_graph(&v);
    let elapsed = get_timestamp() - time;
    println!("大图生成耗时: {}ms", elapsed);
}