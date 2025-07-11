use yew::{prelude::*, virtual_dom::VNode};

#[function_component]
pub fn App() -> Html {
    let mut children: Vec<VNode> = vec![];

    #[cfg(feature = "aspect-ratio")]
    {
        use crate::aspect_ratio::AspectRatioDemo;
        children.push(html! {
            <AspectRatioDemo />
        });
    }
    #[cfg(feature = "avatar")]
    {
        use crate::avatar::AvatarDemo;
        children.push(html! {
            <AvatarDemo />
        });
    }
    #[cfg(feature = "checkbox")]
    {
        use crate::checkbox::CheckboxDemo;
        children.push(html! {
            <CheckboxDemo />
        });
    }
    #[cfg(feature = "label")]
    {
        use crate::label::LabelDemo;
        children.push(html! {
            <LabelDemo />
        });
    }
    #[cfg(feature = "progress")]
    {
        use crate::progress::ProgressDemo;
        children.push(html! {
            <ProgressDemo />
        });
    }
    #[cfg(feature = "select")]
    {
        use crate::select::SelectDemo;
        children.push(html! {
            <SelectDemo />
        });
    }
    #[cfg(feature = "separator")]
    {
        use crate::separator::SeparatorDemo;
        children.push(html! {
            <SeparatorDemo />
        });
    }
    #[cfg(feature = "switch")]
    {
        use crate::switch::SwitchDemo;
        children.push(html! {
            <SwitchDemo />
        });
    }
    #[cfg(feature = "tooltip")]
    {
        use crate::tooltip::TooltipDemo;
        children.push(html! {
            <TooltipDemo />
        });
    }

    html! {
        <div class="w-full h-full flex justify-center items-start">
            {children}
        </div>
    }
}
