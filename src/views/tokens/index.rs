use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct ViewArgs {}

impl ViewArgs {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[function_component]
pub(crate) fn View(args: &ViewArgs) -> Html {
    html! {
        <>
            <span>{"view: views/tokens/index"}</span>
        </>
    }
}
