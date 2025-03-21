#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use yew::prelude::*;

use crate::components::dropdown::DropdownMsg;

/// The message type used by the `Navbar` component.
pub enum NavbarMsg {
    ToggleMenu,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Make the navbar fixed to the top or bottom of the UI.
    #[prop_or_default]
    pub fixed: Option<NavbarFixed>,
    /// Seamlessly integrate the navbar in any visual context.
    ///
    /// [https://bulma.io/documentation/components/navbar/#transparent-navbar](https://bulma.io/documentation/components/navbar/#transparent-navbar)
    #[prop_or_default]
    pub transparent: bool,
    /// Sets **top** and **bottom** paddings with **1rem**, **left** and **right** paddings with **2rem**.
    ///
    /// [https://bulma.io/documentation/components/navbar/#navbar-helper-classes](https://bulma.io/documentation/components/navbar/#navbar-helper-classes)
    #[prop_or_default]
    pub spaced: bool,
    /// The contents of the navbar brand. The `navbar-burger` is automatically appended to the
    /// end of this content.
    ///
    /// [https://bulma.io/documentation/components/navbar/#navbar-brand](https://bulma.io/documentation/components/navbar/#navbar-brand)
    /// If true, the contents of the navbar will be wrapped in a container.
    #[prop_or_default]
    pub padded: bool,
    /// The contents of the `navbar-brand` section of the navbar.
    #[prop_or_default]
    pub navbrand: Option<Html>,
    /// The contents of the `navbar-start` section of the navbar.
    #[prop_or_default]
    pub navstart: Option<Html>,
    /// The contents of the `navbar-end` section of the navbar.
    #[prop_or_default]
    pub navend: Option<Html>,
    /// A bool controlling if the navbar should have a navbar burger for smaller viewports.
    #[prop_or_else(|| true)]
    pub navburger: bool,
    /// Extra classes for the navbar burger.
    #[prop_or_default]
    pub navburger_classes: Option<Classes>,
}

/// A responsive horizontal navbar that can support images, links, buttons, and dropdowns.
///
/// [https://bulma.io/documentation/components/navbar/](https://bulma.io/documentation/components/navbar/)
pub struct Navbar {
    is_menu_open: bool,
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = NavbarProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            is_menu_open: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavbarMsg::ToggleMenu => {
                self.is_menu_open = !self.is_menu_open;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        // navbar classes
        let mut classes = Classes::from("navbar");
        classes.push(&ctx.props().classes);
        if let Some(fixed) = &ctx.props().fixed {
            classes.push(&fixed.to_string());
        }

        // navbar-menu classes
        let mut navclasses = Classes::from("navbar-menu");
        let mut burgerclasses = Classes::from("navbar-burger");
        burgerclasses.push(&ctx.props().navburger_classes);
        if self.is_menu_open {
            navclasses.push("is-active");
            burgerclasses.push("is-active");
        }
        let togglecb = link.callback(|_| NavbarMsg::ToggleMenu);
        let navbrand = if let Some(navbrand) = &ctx.props().navbrand {
            html! {
                <div class="navbar-brand">
                    {navbrand.clone()}
                    {if ctx.props().navburger {
                        html! {
                            <a class={burgerclasses} onclick={togglecb}
                                role={"button"} aria-label={"menu"}
                                aria-expanded={if self.is_menu_open { "true" } else { "false" }}
                            >
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                            </a>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            }
        } else {
            html! {}
        };
        let navstart = if let Some(navstart) = &ctx.props().navstart {
            html! {<div class="navbar-start">{navstart.clone()}</div>}
        } else {
            html! {}
        };
        let navend = if let Some(navend) = &ctx.props().navend {
            html! {<div class="navbar-end">{navend.clone()}</div>}
        } else {
            html! {}
        };
        let contents = html! {
            <>
            {navbrand}
            <div class={navclasses}>
                {navstart}
                {navend}
            </div>
            </>
        };

        if ctx.props().padded {
            html! {
                <nav class={classes} role={"navigation"} aria-label={"main navigation"}>
                    <div class={"container"}>{contents}</div>
                </nav>
            }
        } else {
            html! {
                <nav class={classes} role={"navigation"} aria-label={"main navigation"}>{contents}</nav>
            }
        }
    }
}

/// The 2 possible fixed positions available for a navbar.
///
/// [https://bulma.io/documentation/components/navbar/#fixed-navbar](https://bulma.io/documentation/components/navbar/#fixed-navbar)
///
/// NOTE WELL: in order to work properly, the root `html` or `body` element must be configured with
/// the corresponding `has-navbar-fixed-top` or `has-navbar-fixed-bottom` class.
#[derive(Clone, Debug, Display, PartialEq)]
#[display("is-{_variant}")]
pub enum NavbarFixed {
    #[display("fixed-top")]
    Top,
    #[display("fixed-bottom")]
    Bottom,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

/// The two HTML tags allowed for a navbar-item.
///
/// [https://bulma.io/documentation/components/navbar/#navbar-item](https://bulma.io/documentation/components/navbar/#navbar-item)
#[derive(Clone, Debug, Display, PartialEq)]
pub enum NavbarItemTag {
    #[display("a")]
    A,
    #[display("div")]
    Div,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| NavbarItemTag::Div)]
    pub tag: NavbarItemTag,
    /// Add the `has-dropdown` class to this element, indicating that it is the parent
    /// of a dropdown menu.
    #[prop_or_default]
    pub has_dropdown: bool,
    /// Turn this into a full-width element.
    #[prop_or_default]
    pub expanded: bool,
    /// Add a bottom border on hover, and show the bottom border using `is_active=true`.
    #[prop_or_default]
    pub tab: bool,
    /// Show the bottom border when `is_tab=true`.
    #[prop_or_default]
    pub active: bool,
    /// An optional `href` for when this element is using the `a` tag.
    #[prop_or_default]
    pub href: Option<String>,
    /// An optional `rel` for when this element is using the `a` tag.
    #[prop_or_default]
    pub rel: Option<String>,
    /// An optional `target` for when this element is using the `a` tag.
    #[prop_or_default]
    pub target: Option<String>,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// A single element of the navbar.
///
/// [https://bulma.io/documentation/components/navbar/](https://bulma.io/documentation/components/navbar/)
pub struct NavbarItem {}

impl Component for NavbarItem {
    type Message = ();
    type Properties = NavbarItemProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // navbar classes
        let mut classes = Classes::from("navbar-item");
        classes.push(&ctx.props().classes);
        if ctx.props().has_dropdown {
            classes.push("has-dropdown");
        }
        if ctx.props().expanded {
            classes.push("is-expanded");
        }
        if ctx.props().tab {
            classes.push("is-tab");
        }
        if ctx.props().active {
            classes.push("is-active");
        }
        match ctx.props().tag {
            NavbarItemTag::A => {
                html! {
                    <a
                        class={classes}
                        href={ctx.props().href.clone().unwrap_or(String::from("javascript:"))}
                        rel={ctx.props().rel.clone().unwrap_or_default()}
                        target={ctx.props().target.clone().unwrap_or_default()}
                        onclick={ctx.props().onclick.clone()}
                    >
                        { for ctx.props().children.iter() }
                    </a>
                }
            }
            NavbarItemTag::Div => {
                html! {
                    <div class={classes} onclick={ctx.props().onclick.clone()}>
                        { for ctx.props().children.iter() }
                    </div>
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarDividerProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// An element to display a horizontal rule in a navbar-dropdown.
///
/// [https://bulma.io/documentation/components/navbar/#dropdown-menu](https://bulma.io/documentation/components/navbar/#dropdown-menu)
pub struct NavbarDivider {}

impl Component for NavbarDivider {
    type Message = ();
    type Properties = NavbarDividerProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("navbar-divider");
        classes.push(&ctx.props().classes);
        html! {
            <hr class={classes}/>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarDropdownProps {
    /// The content of the dropdown; these should all be `NavbarItems` & `NavbarDividers`.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The contents of the navbar-link used for triggering the dropdown menu.
    #[prop_or_else(|| html ! {})]
    pub navlink: Html,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// Configure this manu to be a dropup.
    #[prop_or_default]
    pub dropup: bool,
    /// Render the contents of this dropdown to the right.
    #[prop_or_default]
    pub right: bool,
    /// Remove the arrow from the dropdown menu trigger.
    #[prop_or_default]
    pub arrowless: bool,
    /// Use the boxed style for the dropdown, typically coupled with a transparent navbar.
    #[prop_or_default]
    pub boxed: bool,
}

/// A navbar dropdown menu, which can include navbar items and dividers.
///
/// This component is a composite of all of the elements needed in order to properly generate
/// a navbar dropdown component.
///
/// [https://bulma.io/documentation/components/navbar/#dropdown-menu](https://bulma.io/documentation/components/navbar/#dropdown-menu)
pub struct NavbarDropdown {
    is_menu_active: bool,
}

impl Component for NavbarDropdown {
    type Message = DropdownMsg;
    type Properties = NavbarDropdownProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            is_menu_active: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if ctx.props().hoverable {
            return false;
        }
        match msg {
            DropdownMsg::Open => self.is_menu_active = true,
            DropdownMsg::Close => self.is_menu_active = false,
            _ => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        // navbar-item classes
        let mut classes = Classes::from("navbar-item has-dropdown");
        classes.push(&ctx.props().classes);
        if ctx.props().dropup {
            classes.push("has-dropdown-up");
        }

        // navbar-dropdown classes
        let mut dropclasses = Classes::from("navbar-dropdown");
        if ctx.props().right {
            dropclasses.push("is-right");
        }
        if ctx.props().boxed {
            dropclasses.push("is-boxed");
        }

        // navbar-link classes
        let mut linkclasses = Classes::from("navbar-link");
        if ctx.props().arrowless {
            linkclasses.push("is-arrowless");
        }

        let opencb = if ctx.props().hoverable {
            classes.push("is-hoverable");
            Callback::noop()
        } else {
            link.callback(|_| DropdownMsg::Open)
        };
        let overlay = if self.is_menu_active {
            classes.push("is-active");
            html! {<div onclick={link.callback(|_| DropdownMsg::Close)} style={"z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"}></div>}
        } else {
            html! {}
        };
        html! {
            <div class={classes}>
                {overlay}
                <a class={linkclasses} onclick={opencb}>{ctx.props().navlink.clone()}</a>
                <div class={dropclasses}>
                    { for ctx.props().children.iter() }
                </div>
            </div>
        }
    }
}
