use dirstudio_api_types::{
    AttributeType,
    AttributeValue as ValueWithContexts,
};
use std::ops::Deref;
use yew::prelude::*;
use crate::components::IconButton::IconButton;

#[derive(Clone, PartialEq, Properties)]
pub struct AttributeValueProps {
    pub alternation: bool,
    pub depth: u32,
    pub attr_type: AttributeType,
    pub attr_value: ValueWithContexts,
    pub binary: bool,
    pub quick_delete: bool,
    pub quick_edit: bool,
    pub parent_selected: bool,
}

// TODO: Return param indicating whether attribute is "alterable"
#[function_component(AttributeValueNode)]
pub fn attribute_value(props: &AttributeValueProps) -> Html {
    let selected: UseStateHandle<bool> = use_state(|| props.parent_selected);
    let enabled: UseStateHandle<bool> = use_state(|| true);
    let expanded: UseStateHandle<bool> = use_state(|| true);
    let alt_class = if props.alternation { "odd" } else { "even" };
    let sel_class = if *selected.deref() || props.parent_selected { "selected" } else { "unselected" };
    let ena_class = if *enabled.deref() { "enabled" } else { "disabled" };
    let exp_class = if *expanded.deref() { "expanded" } else { "collapsed" };
    let chevron_img_src = if *expanded.deref() {
        "/public/ExpandMoreIcon.svg"
    } else {
        "/public/ChevronRightIcon.svg"
    };
    let checkbox_img_src = if *selected.deref() || props.parent_selected {
        "/public/CheckboxChecked.svg"
    } else {
        "/public/CheckboxUnchecked.svg"
    };
    let indent = String::from(" ").repeat(((props.depth as usize) * 4) + 1);

    // TODO: Right-click options
    // https://itnext.io/how-to-create-a-custom-right-click-menu-with-javascript-9c368bb58724
    // https://yew.rs/docs/next/concepts/html/events#available-events
    // TODO: Contexts
    // TODO: ARIA attributes
    // TODO: Fix scrolling weirdness.
    // let subordinates = props.attr_value.contexts
    //     .iter()
    //     .map(|v| {

    //     });

    /*
    Buttons: copy, delete, edit?, select, add (could be a separate row), info?
    There seem to be few enough buttons to make it worth just putting them in
    leading columns in the table.
    Columns: select, copy, edit (configurable), delete (configurable)
    */

    html!(
        <tr class={classes!([
            "detail-row",
            "detail-attr-value",
            alt_class,
            sel_class,
            ena_class,
            exp_class,
        ].as_ref())}>
            <td class={classes!(["detail-row-checkbox"].as_ref())}>
                <img
                    src={checkbox_img_src}
                    class={classes!(["mui-icon", "row-icon", "row-icon-button"].as_ref())}
                    />
            </td>
            <td class={classes!(["detail-row-copy"].as_ref())}>
                <IconButton
                    icon_path={AttrValue::from("/public/ContentCopy.svg")}
                    tooltip_text={AttrValue::from("Copy")}
                    onclick={Callback::from(|_| ())}
                    />
            </td>
            <td class={classes!(["detail-row-edit"].as_ref())}>
                <IconButton
                    icon_path={AttrValue::from("/public/Edit.svg")}
                    tooltip_text={AttrValue::from("Edit")}
                    onclick={Callback::from(|_| ())}
                    disabled={true}
                    />
            </td>
            <td class={classes!(["detail-row-delete"].as_ref())}>
                <IconButton
                    icon_path={AttrValue::from("/public/Delete.svg")}
                    tooltip_text={AttrValue::from("Delete")}
                    onclick={Callback::from(|_| ())}
                    disabled={true}
                    />
            </td>
            <td class={classes!(["detail-cell"].as_ref())}>
                <span class={classes!(["detail-cell-text"].as_ref())}>
                    <pre>{indent}</pre>
                    <img
                        src={chevron_img_src}
                        class={classes!(["mui-icon", "detail-node-chevron"].as_ref())}
                        />
                    {&props.attr_value.value}
                </span>
            </td>
        </tr>
    )
}
