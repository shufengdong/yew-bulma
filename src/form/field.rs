use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FieldProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// A text label for the field.
    #[prop_or_default]
    pub label: Option<String>,
    /// Extra classes for the label container.
    #[prop_or_default]
    pub label_classes: Option<Classes>,
    /// A help message for the field.
    #[prop_or_default]
    pub help: Option<String>,
    /// Extra classes for the help message container.
    #[prop_or_default]
    pub help_classes: Option<Classes>,
    /// A convenience bool to add the `is-danger` class to the help classes when `true`.
    #[prop_or_default]
    pub help_has_error: bool,
    /// Has icons on the left of the field's controls.
    #[prop_or_default]
    pub icons_left: bool,
    /// Has icons on the right of the field's controls.
    #[prop_or_default]
    pub icons_right: bool,
    /// Allow addons to the field's controls.
    #[prop_or_default]
    pub addons: bool,
    /// Alignment for the field's addons.
    #[prop_or_default]
    pub addons_align: Option<AddonsAlign>,
    /// All controls in this field should be grouped.
    #[prop_or_default]
    pub grouped: bool,
    /// Alignment for grouped controls.
    #[prop_or_default]
    pub grouped_align: Option<GroupedAlign>,
    /// Allow the grouped controls to span multiple lines.
    #[prop_or_default]
    pub multiline: bool,
    /// Make this a horizontal field.
    #[prop_or_default]
    pub horizontal: bool,
}

/// A container for form controls
pub struct Field {}

impl Component for Field {
    type Message = ();
    type Properties = FieldProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("field");
        classes.push(&ctx.props().classes);
        if ctx.props().icons_left {
            classes.push("has-icons-left");
        }
        if ctx.props().icons_right {
            classes.push("has-icons-right");
        }
        if ctx.props().addons {
            classes.push("has-addons");
        }
        if let Some(align) = &ctx.props().addons_align {
            classes.push(&align.to_string());
        }
        if ctx.props().grouped {
            classes.push("is-grouped");
        }
        if let Some(align) = &ctx.props().grouped_align {
            classes.push(&align.to_string());
        }
        if ctx.props().multiline {
            classes.push("is-grouped-multiline");
        }
        if ctx.props().horizontal {
            classes.push("is-horizontal");
        }

        // Build the label if label content is provided.
        let label = match &ctx.props().label {
            Some(label_content) => match &ctx.props().label_classes {
                Some(label_classes_str) => {
                    let mut label_classes = label_classes_str.clone();
                    if ctx.props().horizontal {
                        label_classes.push("field-label");
                        html! {
                            <div class={label_classes}>
                                <label class={"label"}>{label_content.clone()}</label>
                            </div>
                        }
                    } else {
                        label_classes.push("label");
                        html! {<label class={label_classes}>{label_content.clone()}</label>}
                    }
                }
                None => {
                    if ctx.props().horizontal {
                        html! {<div class={"field-label"}><label class={"label"}>{label_content.clone()}</label></div>}
                    } else {
                        html! {<label class={"label"}>{label_content.clone()}</label>}
                    }
                }
            },
            None => html! {},
        };

        // Build the help label if present.
        let help = match &ctx.props().help {
            Some(help_content) => match &ctx.props().help_classes {
                Some(help_classes_str) => {
                    let mut help_classes = help_classes_str.clone();
                    help_classes.push("help");
                    if ctx.props().help_has_error {
                        help_classes.push("is-danger");
                    }
                    html! {<label class={help_classes}>{help_content.clone()}</label>}
                }
                None => {
                    let mut help_classes = Classes::from("help");
                    if ctx.props().help_has_error {
                        help_classes.push("is-danger");
                    }
                    html! {<label class={help_classes}>{help_content.clone()}</label>}
                }
            },
            None => html! {},
        };

        // Build the body section.
        let mut body = html! {<>{ for ctx.props().children.iter() }</>};
        if ctx.props().horizontal {
            body = html! {<div class={"field-body"}>{body}</div>}
        }

        html! {
            <div class={classes}>
                {label}
                {body}
                {help}
            </div>
        }
    }
}

/// The two alignment options available for field addons.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display("has-addons-{_variant}")]
pub enum AddonsAlign {
    #[display("centered")]
    Centered,
    #[display("right")]
    Right,
}

/// The two alignment options available for grouped field controls.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display("is-grouped-{_variant}")]
pub enum GroupedAlign {
    #[display("centered")]
    Centered,
    #[display("right")]
    Right,
}

/// The three sizes available for horizontal field labels.
///
/// https://bulma.io/documentation/form/general/#horizontal-form
#[derive(Clone, Debug, Display, PartialEq)]
#[display("is-{_variant}")]
pub enum LabelSize {
    #[display("small")]
    Small,
    #[display("medium")]
    Medium,
    #[display("large")]
    Large,
}
