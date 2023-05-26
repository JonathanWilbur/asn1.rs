use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct IconButtonProps {
    pub icon_path: AttrValue,
    pub tooltip_text: AttrValue,
    pub onclick: Callback<MouseEvent, ()>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let ena_class = if props.disabled { "disabled" } else { "enabled" };
    html!(
        <span
            class={classes!(["tooltip"].as_ref())}
            onclick={props.onclick.clone()}>
            <img
                src={&props.icon_path}
                class={classes!(["mui-icon", "row-icon", "row-icon-button", ena_class].as_ref())}
                />
            <span class="tooltip-text">{&props.tooltip_text}</span>
        </span>
    )
}
