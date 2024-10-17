use std::collections::HashMap;

use petgraph::graph::EdgeReference;
use petgraph::prelude::*;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::components::filetree::FileTree;
use crate::TileSize;
use crate::*;

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum NodeType {
    Ancestor,
    Parent,
    Child,
    #[default]
    Normal,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TileNode {
    pub node_type: NodeType,
    pub is_vertical: bool,
    pub size: Option<TileSize>,
}

impl TileNode {
    pub fn new(node_type: NodeType, is_vertical: bool, size: TileSize) -> Self {
        TileNode {
            node_type,
            is_vertical,
            size: Some(size),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileTree {
    pub tree: DiGraph<TileNode, usize>,
}

impl TileNode {
    pub fn create_class(&self, is_selected: bool) -> String {
        let mut class_str = "tile".to_string();
        match self.node_type {
            NodeType::Ancestor => class_str += " is-ancestor",
            NodeType::Parent => class_str += " is-parent",
            NodeType::Child => {
                if is_selected {
                    class_str += " is-child notification is-info"
                } else {
                    class_str += " is-child box"
                }
            }
            _ => {}
        }
        if self.is_vertical {
            class_str += " is-vertical";
        }
        if let Some(size) = &self.size {
            class_str += " ";
            class_str += &size.to_string();
        }
        class_str
    }
}

impl TileTree {
    fn new() -> Self {
        let mut tree = DiGraph::new();
        tree.add_node(TileNode {
            node_type: NodeType::Ancestor,
            is_vertical: false,
            size: None,
        });
        TileTree { tree }
    }

    fn add_node(&mut self, father: usize, node: TileNode) -> usize {
        let new_node = self.tree.add_node(node);
        for i in self.tree.node_indices() {
            if father == i.index() {
                let mut max = 0;
                for edge in self.tree.edges(i) {
                    if *edge.weight() > max {
                        max = *edge.weight();
                    }
                }
                self.tree.add_edge(i, new_node, max + 1);
                break;
            }
        }
        new_node.index()
    }

    fn remove_node(&mut self, node: usize) {
        let i = NodeIndex::new(node);
        let mut chidren = Vec::new();
        for edge in self.tree.edges(i) {
            chidren.push(edge.target().index())
        }
        for child in chidren {
            self.remove_node(child);
        }
        let _ = self.tree.remove_node(i);
    }

    fn set_node_type(&mut self) {
        let root_index = NodeIndex::new(0);
        // 开始深度优先遍历
        let mut stack = Vec::new();
        stack.push(root_index);
        let mut parents: HashMap<NodeIndex, NodeIndex> =
            HashMap::with_capacity(self.tree.node_count());
        while let Some(node_index) = stack.pop() {
            let edges: Vec<EdgeReference<usize>> = self.tree.edges(node_index).collect();
            // this is a leaf
            if edges.is_empty() && node_index != root_index {
                let father_index = parents.get(&node_index).unwrap();
                if let Some(father_node) = self.tree.node_weight_mut(*father_index) {
                    if *father_index != root_index {
                        father_node.node_type = NodeType::Parent;
                    }
                }
                if let Some(child) = self.tree.node_weight_mut(node_index) {
                    child.node_type = NodeType::Child;
                }
                continue;
            }
            // 按照权重排序
            for edge in edges {
                stack.push(edge.target());
                parents.insert(edge.target(), edge.source());
            }
        }
    }

    fn create_paths(&self) -> Vec<String> {
        let n = self.tree.node_count();
        let mut ids: HashMap<NodeIndex, String> = HashMap::with_capacity(n);
        let mut result = Vec::with_capacity(n / 2);
        // 讲根节点加入
        let root_node = NodeIndex::new(0);
        ids.insert(root_node, "Tile(0)".to_string());
        // 开始深度优先遍历
        let mut stack = Vec::new();
        stack.push(root_node);
        while let Some(node_index) = stack.pop() {
            let mut edges: Vec<EdgeReference<usize>> = self.tree.edges(node_index).collect();
            // this is a leaf
            if edges.is_empty() {
                if let Some(id) = ids.get(&node_index) {
                    result.push(id.clone());
                }
            }
            // 进行排序
            edges.sort_by(|a, b| b.weight().cmp(a.weight()));
            for edge in edges {
                let child_index = edge.target();
                let child_name = format!("Tile({})", child_index.index());
                if let Some(father_id) = ids.get(&node_index) {
                    ids.insert(child_index, format!("{}/{}", father_id, child_name));
                } else {
                    //根节点下面的节点
                    ids.insert(child_index, child_name.to_string());
                }
                stack.push(child_index);
            }
        }
        result.shrink_to_fit();
        result
    }

    pub fn create_html(&self, prefix: Option<String>, selected: usize, mut nodes: HashMap<String, VNode>) -> VNode {
        let root_index = NodeIndex::new(0);
        let class_str = self.tree.node_weight(root_index).unwrap().create_class(false);
        let top_node = html! { <div class={class_str}/> };
        // 开始深度优先遍历
        let mut stack = Vec::new();
        let mut stack2 = Vec::with_capacity(self.tree.node_count());
        let mut node_pos = HashMap::with_capacity(self.tree.node_count());
        stack.push((root_index, top_node));
        while let Some((node_index, current_node)) = stack.pop() {
            let mut edges: Vec<EdgeReference<usize>> = self.tree.edges(node_index).collect();
            // 按照权重排序
            edges.sort_by(|a, b| a.weight().cmp(b.weight()));
            for edge in edges {
                let child_index = edge.target();
                let child = self.tree.node_weight(child_index).unwrap();
                let class_str = child.create_class(child_index.index() == selected);
                let child = if child.node_type != NodeType::Child {
                    html! { <div class={class_str}/> }
                } else if let Some(s) = &prefix {
                    let id = format!("{}-tile({})", s, child_index.index());
                    let mut div = html! { <div id={id.clone()} class={class_str}/> };
                    if let Some(node) = nodes.remove(&id) {
                        if let VNode::VTag(father) = &mut div {
                            father.add_child(node);
                        }
                    }
                    div
                } else {
                    let id = format!("Tile({})", child_index.index());
                    let mut div = html! { <div id={id.clone()} class={class_str}/> };
                    if let Some(node) = nodes.remove(&id) {
                        if let VNode::VTag(father) = &mut div {
                            father.add_child(node);
                        }
                    }
                    div
                };
                stack.push((child_index, child));
            }
            node_pos.insert(node_index, stack2.len());
            stack2.push((node_index, current_node));
        }
        while stack2.len() > 1 {
            let (node_index, current_node) = stack2.pop().unwrap();
            let edges = self.tree.edges_directed(node_index, Incoming);
            for edge in edges {
                let father_pos = node_pos.get(&edge.source()).unwrap();
                if let VNode::VTag(father) = &mut stack2[*father_pos].1 {
                    father.add_child(current_node);
                    break;
                }
            }
        }
        stack2.pop().unwrap().1
    }
}

pub enum Msg {
    Add,
    Remove,
    ChangeDirection,
    ChangeSize(String),
    PathSelected(String),
    Apply,
    Close,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub is_modal: bool,
    #[prop_or_default]
    pub modal_show: bool,
    #[prop_or_default]
    pub modal_title: String,
    #[prop_or_default]
    pub modal_commit_title: Option<String>,
    #[prop_or_default]
    pub modal_close_title: Option<String>,
    #[prop_or_else(Callback::noop)]
    pub on_apply: Callback<TileTree>,
    #[prop_or_else(Callback::noop)]
    pub on_close: Callback<()>,
}

pub struct TilesBuilder {
    tree: TileTree,
    current_selected: usize,
}

impl Component for TilesBuilder {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            tree: TileTree::new(),
            current_selected: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                let father = self.current_selected;
                self.tree.add_node(father, TileNode::default());
                self.tree.set_node_type();
                return true;
            }
            Msg::Remove => {
                // 根节点不能删除
                if self.current_selected > 0 {
                    self.tree.remove_node(self.current_selected);
                    self.tree.set_node_type();
                    self.current_selected = 0;
                    return true;
                }
            }
            Msg::ChangeDirection => {
                let node = self.tree.tree.node_weight_mut(NodeIndex::new(self.current_selected));
                if let Some(tile) = node {
                    tile.is_vertical = !tile.is_vertical;
                    self.tree.set_node_type();
                    return true;
                }
            }
            Msg::ChangeSize(size) => {
                let node = self.tree.tree.node_weight_mut(NodeIndex::new(self.current_selected));
                if let Some(tile) = node {
                    tile.size = Some(TileSize::from(size.as_str()));
                    self.tree.set_node_type();
                    return true;
                }
            }
            Msg::PathSelected(s) => {
                let start = s.rfind('(').unwrap() + 1;
                let end = s.rfind(')').unwrap();
                let node_index: usize = s[start..end].parse().unwrap();
                self.current_selected = node_index;
                return true;
            }
            Msg::Apply => {
                ctx.props().on_apply.emit(self.tree.clone());
            }
            Msg::Close => {
                ctx.props().on_close.emit(());
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let paths = self.tree.create_paths();
        let left = html! {
            <>
            <Level>
            <LevelLeft>
                <LevelItem>
                    <Button onclick={link.callback(|_|Msg::Add)}>{"+"}</Button>
                </LevelItem>
                <LevelItem>
                    <Button onclick={link.callback(|_|Msg::Remove)}>{"-"}</Button>
                </LevelItem>
                <LevelItem>
                    <Button onclick={link.callback(|_|Msg::ChangeDirection)}>{"vertical"}</Button>
                </LevelItem>
                <LevelItem>
                    <Control>
                        <Select update={link.callback(Msg::ChangeSize)}>
                            <option value={"is-1"}>{"is-1"}</option>
                            <option value={"is-2"}>{"is-2"}</option>
                            <option value={"is-3"}>{"is-3"}</option>
                            <option value={"is-4"}>{"is-4"}</option>
                            <option value={"is-5"}>{"is-5"}</option>
                            <option value={"is-6"}>{"is-6"}</option>
                            <option value={"is-7"}>{"is-7"}</option>
                            <option value={"is-8"}>{"is-8"}</option>
                            <option value={"is-9"}>{"is-9"}</option>
                            <option value={"is-10"}>{"is-10"}</option>
                            <option value={"is-11"}>{"is-11"}</option>
                            <option value={"is-12"}>{"is-12"}</option>
                        </Select>
                    </Control>
                </LevelItem>
            </LevelLeft>
            </Level>
            <FileTree paths={paths} on_selected={link.callback(Msg::PathSelected)}/>
            </>
        };
        let tiles = self.tree.create_html(None, self.current_selected, HashMap::with_capacity(0));
        let builder_body = html! {
            <Columns multiline={true}>
                 <Column classes={classes!("is-narrow")}>
                    {left}
                </Column>
                <Column>
                    {tiles}
                </Column>
            </Columns>
        };
        if ctx.props().is_modal {
            html! {
                <SimpleModalCard hidden={!ctx.props().modal_show}
                    title={ctx.props().modal_title.clone()}
                    commit_title={ctx.props().modal_commit_title.clone()}
                    close_title={ctx.props().modal_close_title.clone()}
                    on_commit={link.callback(|_| Msg::Apply)}
                    on_close={link.callback(|_| Msg::Close)}
                >
                    {builder_body}
                </SimpleModalCard>
            }
        } else {
            builder_body
        }
    }
}

pub enum ChooserMsg {
    Select(usize),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChooserProps {
    pub templates: Vec<u8>,
    #[prop_or_default]
    pub init_selected: usize,
    #[prop_or_else(Callback::noop)]
    pub on_select: Callback<usize>,
}

pub struct TilesChooser {
    trees: Vec<TileTree>,
    current_selected: usize,
}

impl Component for TilesChooser {
    type Message = ChooserMsg;
    type Properties = ChooserProps;

    fn create(ctx: &Context<Self>) -> Self {
        let trees = serde_json::from_slice(&ctx.props().templates).unwrap();
        Self {
            trees,
            current_selected: ctx.props().init_selected,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChooserMsg::Select(index) => {
                if self.current_selected != index {
                    self.current_selected = index;
                    ctx.props().on_select.emit(index);
                    return true;
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let cl: Vec<String> = (0..self.trees.len())
            .map(|i| {
                if i == self.current_selected {
                    "is-info".to_string()
                } else {
                    "is-dark".to_string()
                }
            })
            .collect();
        // 收集
        let content = (0..self.trees.len()).map(|i| {
            let key = format!("Tiles({})", i);
            let tile_tree = &self.trees[i];
            let tiles = tile_tree.create_html(None, self.current_selected, HashMap::with_capacity(0));
            html! {
                <Column classes={Classes::from("is-6")} >
                    <Message classes={Classes::from(&cl[i])}>
                        <MessageHeader>
                            <Level>
                                <p>{format!("{}", key)}</p>
                                <Radio value={key.clone()} update={link.callback(move |_|ChooserMsg::Select(i))}
                                    checked_value={if i == self.current_selected {key} else {"".to_string()}} />
                            </Level>
                        </MessageHeader>
                        <MessageBody>{tiles}</MessageBody>
                    </Message>
                </Column>
            }
        }).collect::<Html>();
        html! {
            <Columns multiline={true}>
                {content}
            </Columns>
        }
    }
}

#[test]
fn test_class_str() {
    let node = TileNode {
        node_type: NodeType::Ancestor,
        is_vertical: false,
        size: None,
    };
    println!("{}", node.create_class(false));
}

#[test]
fn test_tree_json() {
    let mut tree = TileTree::new();
    let node1 = tree.add_node(0, TileNode::new(NodeType::Normal, true, TileSize::Eight));
    let node2 = tree.add_node(0, TileNode::default());
    let node3 = tree.add_node(node1, TileNode::default());
    let node4 = tree.add_node(node1, TileNode::default());
    let node5 = tree.add_node(node3, TileNode::new(NodeType::Normal, true, TileSize::Four));
    let node6 = tree.add_node(node3, TileNode::default());
    let node7 = tree.add_node(node5, TileNode::default());
    let node8 = tree.add_node(node5, TileNode::default());
    tree.add_node(node6, TileNode::default());
    tree.add_node(node7, TileNode::default());
    tree.add_node(node8, TileNode::default());
    tree.add_node(node4, TileNode::default());
    tree.add_node(node2, TileNode::default());
    tree.set_node_type();
    let mut trees = Vec::with_capacity(6);
    for _i in 0..6 {
        trees.push(tree.clone());
    }
    println!("{}", serde_json::to_string(&trees).unwrap());
}

#[test]
fn test_vnode() {
    let node = html! {<div id={"id1"} />};
    let is_tag_node = if let VNode::VTag(_) = node { true } else { false };
    assert!(is_tag_node);
}
