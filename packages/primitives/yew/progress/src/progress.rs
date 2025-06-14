use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent, struct_component};
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct ProgressProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ProgressChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "progress")]
pub struct ProgressChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn Progress(_props: &ProgressProps) -> Html {
    html! { <></> }
}
