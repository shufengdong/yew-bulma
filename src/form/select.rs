#![allow(clippy::redundant_closure_call)]

use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SelectProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_else(Callback::noop)]
    pub update: Callback<String>,

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A wrapper around an HTML `select` tag.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute.
pub struct Select {
    ele_ref: NodeRef,
}

impl Component for Select {
    type Message = ();
    type Properties = SelectProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            ele_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _: Self::Message) -> bool {
        if let Some(ele) = self.ele_ref.cast::<HtmlSelectElement>() {
            ctx.props().update.emit(ele.value());
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut classes = Classes::from("select");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if ctx.props().loading {
            classes.push("is-loading");
        }
        html! {
            <div class={classes}>
                <select ref={self.ele_ref.clone()}
                    name={ctx.props().name.clone()}
                    value={ctx.props().value.clone()}
                    disabled={ctx.props().disabled}
                    onchange={link.callback(|_| ())}
                >
                    { for ctx.props().children.iter() }
                </select>
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Properties, Clone, PartialEq)]
pub struct MultiSelectProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: Vec<String>,
    #[prop_or_else(Callback::noop)]
    pub update: Callback<Vec<String>>,

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Size of the list to display.
    #[prop_or_else(|| 4)]
    pub list_size: u32,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A wrapper around an HTML `select` tag with the `multiple=true` attribute.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute.
pub struct MultiSelect {
    ele_ref: NodeRef,
}

impl Component for MultiSelect {
    type Message = ();
    type Properties = MultiSelectProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            ele_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _: Self::Message) -> bool {
        if let Some(ele) = self.ele_ref.cast::<HtmlSelectElement>() {
            let v: String = ele.value();
            let msg: Vec<String> = v.split(',').map(|s| s.to_string()).collect();
            ctx.props().update.emit(msg);
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut classes = Classes::from("select is-multiple");
        classes.push(&ctx.props().classes);
        if let Some(size) = &ctx.props().size {
            classes.push(&size.to_string());
        }
        if ctx.props().loading {
            classes.push("is-loading");
        }
        let size: String = ctx.props().list_size.to_string();
        html! {
            <div class={classes}>
                <select ref={self.ele_ref.clone()}
                    multiple={true}
                    size={size}
                    name={ctx.props().name.clone()}
                    value={ctx.props().value.join(",")}
                    disabled={ctx.props().disabled}
                    onchange={link.callback(|_| ())}
                >
                    { for ctx.props().children.iter() }
                </select>
            </div>
        }
    }
}
