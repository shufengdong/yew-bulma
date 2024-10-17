use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Add borders to all the cells.
    #[prop_or_default]
    pub bordered: bool,
    /// Add stripes to the table.
    #[prop_or_default]
    pub striped: bool,
    /// Make the cells narrower.
    #[prop_or_default]
    pub narrow: bool,
    /// Add a hover effect on each row.
    #[prop_or_default]
    pub hoverable: bool,
    /// Make the table fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Make the table scrollable, wrapping the table in a `div.table-container`.
    #[prop_or_default]
    pub scrollable: bool,
}

/// An HTML table component.
///
/// [https://bulma.io/documentation/elements/table/](https://bulma.io/documentation/elements/table/)
pub struct Table {}

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("table");
        classes.push(&ctx.props().classes);
        if ctx.props().bordered {
            classes.push("is-bordered");
        }
        if ctx.props().striped {
            classes.push("is-striped");
        }
        if ctx.props().narrow {
            classes.push("is-narrow");
        }
        if ctx.props().hoverable {
            classes.push("is-hoverable");
        }
        if ctx.props().fullwidth {
            classes.push("is-fullwidth");
        }
        if ctx.props().scrollable {
            html! {
                <div class={"table-container"}>
                    <table class={classes}>
                        { for ctx.props().children.iter() }
                    </table>
                </div>
            }
        } else {
            html! {
                <table class={classes}>
                    { for ctx.props().children.iter() }
                </table>
            }
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TableTrProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Add a selected show to the row.
    #[prop_or_default]
    pub selected: bool,
}

/// An HTML table row component.
///
/// [https://bulma.io/documentation/elements/table/](https://bulma.io/documentation/elements/table/)
pub struct TableTr {}

impl Component for TableTr {
    type Message = ();
    type Properties = TableTrProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::new();
        classes.push(&ctx.props().classes);
        if ctx.props().selected {
            classes.push("is-selected");
        };
        html! {
            <tr class={classes}>
                { for ctx.props().children.iter() }
            </tr>
        }
    }
}
