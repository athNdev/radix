use web_sys::{js_sys::Number, wasm_bindgen::JsCast};
use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent, struct_component};
use yew_style::Style;

#[derive(Clone, Debug, PartialEq)]
struct ProgressContextValue {
    value: Number,
    max: Number
}

#[derive(PartialEq, Properties)]
pub struct ProgressProps {
    #[prop_or_default]
    pub get_value_label: Callback<bool>,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from 'button'
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub max: String,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub as_child: Option<Callback<ProgressChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "progress")]
pub struct ProgressChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,

    // Attributes from `progress`

}

#[function_component]
pub fn Progress(props: &ProgressProps) -> Html {
    let div_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), div_ref.clone()]);

    html! { <></> }
}

#[derive(PartialEq, Properties)]
pub struct ProgressIndicatorProps {
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

#[derive(Clone, Default, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct ProgressIndicatorChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn ProgressIndicator(props: &ProgressIndicatorProps) -> Html {
    html! { <></> }
}

fn default_get_value_label(value: u8, max: u8) -> String {
    return "".to_string();
}
fn get_progress_state(value: u8, max_value: u8) -> String {
    return "".to_string();
}
fn get_invalid_max_error(prop_value: String, component_name: String) -> String {
    return "".to_string();
}
fn get_invalid_value_error(prop_value: String, component_name: String) -> String {
    return "".to_string();
}