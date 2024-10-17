#![allow(clippy::redundant_closure_call)]

use web_sys::{File as SysFile, HtmlInputElement};
use yew::events::Event;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FileProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub files: Vec<SysFile>,
    #[prop_or_else(Callback::noop)]
    pub update: Callback<Vec<SysFile>>,
    /// The display text for the file selector.
    //#[prop_or_else(|| "Choose a file...".into())]
    #[prop_or(true)]
    pub has_label: bool,
    #[prop_or_default]
    pub selector_label: String,
    /// The HTML contents to use for the file selector icon.
    #[prop_or_default]
    pub selector_icon: Html,

    #[prop_or_default]
    pub classes: Option<Classes>,
    /// An option to control if file names will be displayed; if a value is provided, then the
    /// `has-name` class will be added to this form element and the given value will be used as a
    /// placeholder until files are selected.
    #[prop_or_default]
    pub has_name: Option<String>,
    /// Move the CTA element to the right side of the component.
    #[prop_or_default]
    pub right: bool,
    /// Expand the file display name to the full width of the parent.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Display as a boxed block.
    #[prop_or_default]
    pub boxed: bool,
    /// Allow multiple files to be selected.
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub disabled: bool,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this component within its parent.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    #[prop_or_default]
    pub hidden: bool,
    /// 文件类型限制参数
    #[prop_or_default]
    pub accept: Option<String>,
}

/// A custom file upload input.
///
/// [https://bulma.io/documentation/form/file/](https://bulma.io/documentation/form/file/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct File {
    input_value: String,
}

impl Component for File {
    type Message = Vec<SysFile>;
    type Properties = FileProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            input_value: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().update.emit(msg);
        self.input_value = "".to_string();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut classes = Classes::from("file");
        classes.push(&ctx.props().classes);
        let mut style = "".to_string();
        if ctx.props().has_name.is_some() {
            classes.push("has-name");
        }
        if ctx.props().right {
            classes.push("is-right");
        }
        if ctx.props().fullwidth {
            classes.push("is-fullwidth");
        }
        if ctx.props().boxed {
            classes.push("is-boxed");
        }
        if ctx.props().hidden {
            style = "display: none;".to_string();
        }
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &ctx.props().alignment {
            classes.push(&alignment.to_string());
        }
        let mut icon_classes = Classes::from("file-icon");
        if !ctx.props().has_label {
            icon_classes.push("no-label");
        }
        let filenames = ctx
            .props()
            .files
            .iter()
            .map(|file| html! {<span class="file-name">{file.name()}</span>})
            .collect::<Vec<_>>();
        let onchange = link.callback(|e: Event| {
            let mut result = Vec::new();
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()));
                result.extend(files);
            }
            result
        });
        html! {
            <div class={classes} style={style}>
                <label class={"file-label"}>
                    <input disabled={ctx.props().disabled}
                        value={self.input_value.clone()}
                        type={"file"}
                        class={"file-input"}
                        name={ctx.props().name.clone()}
                        multiple={ctx.props().multiple}
                        accept={ctx.props().accept.clone()}
                        onchange={onchange}
                        />
                    <span class={"file-cta"} disabled={ctx.props().disabled}>
                        <span class={icon_classes}>
                            {ctx.props().selector_icon.clone()}
                        </span>
                        if ctx.props().has_label {
                        <span class={"file-label"}>
                            {ctx.props().selector_label.clone()}
                        </span>
                        }
                    </span>
                    {filenames}
                </label>
            </div>
        }
    }
}
