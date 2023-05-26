use dirstudio_api_types::{
    Attribute,
    AttributeType,
    Context,
    ContextType,
    OidInfo,
    EntryInfoNodeContent,
    EntryInfoItem,
};
use wasm_bindgen::{JsCast, __rt::IntoJsResult};
use std::{ops::Deref, rc::Rc};
use yew::prelude::*;
use web_sys::{MouseEvent, HtmlStyleElement};
use serde_wasm_bindgen::{to_value, from_value};
use crate::ipc::invoke;
use wasm_bindgen_futures::spawn_local;
use log::{info, error};
use asn1::OBJECT_IDENTIFIER;
use crate::components::AttributeValueNode::AttributeValueNode;
use crate::components::IconButton::IconButton;
use crate::components::ContextMenu::ContextMenu;
use crate::ipc::CopyToClipboardCommand;
use gloo::events::{EventListener, EventListenerOptions};
use yew::functional::use_effect_with_deps;
use web_sys::{Element, Node, HtmlElement};
use gloo::utils::document;

#[derive(Clone, PartialEq, Properties)]
pub struct AttributeNodeProps {
    pub alternation: bool,
    pub depth: u32,
    pub attr: Attribute,
}

// TODO: Return param indicating whether attribute is "alterable"
#[function_component(AttributeNode)]
pub fn attribute_node(props: &AttributeNodeProps) -> Html {
    let selected: UseStateHandle<bool> = use_state(|| false);
    let enabled: UseStateHandle<bool> = use_state(|| true);
    let expanded: UseStateHandle<bool> = use_state(|| true);
    let show_context_menu: UseStateHandle<bool> = use_state(|| false);
    let alt_class = if props.alternation { "odd" } else { "even" };
    let sel_class = if *selected.deref() { "selected" } else { "unselected" };
    let ena_class = if *enabled.deref() { "enabled" } else { "disabled" };
    let exp_class = if *expanded.deref() { "expanded" } else { "collapsed" };
    let chevron_img_src = if *expanded.deref() {
        "/public/ExpandMoreIcon.svg"
    } else {
        "/public/ChevronRightIcon.svg"
    };
    let checkbox_img_src = if *selected.deref() {
        "/public/CheckboxChecked.svg"
    } else {
        "/public/CheckboxUnchecked.svg"
    };
    let indent = String::from(" ").repeat(((props.depth as usize) * 4) + 1);
    let type_string = format!("{}", &props.attr.attr_type).to_string();

    let onclick_checkbox = {
        if *enabled.deref() {
            let selected = selected.clone();
            Callback::from(move |_e: MouseEvent| selected.set(!*selected.deref()))
        } else {
            Callback::from(|_e: MouseEvent| {})
        }
    };

    // let copy_enabled = cfg!(web_sys_unstable_apis);

    let onclick_copy = {
        let oid = OBJECT_IDENTIFIER::new(&props.attr.attr_type.numeric);
        Callback::from(move |_e: MouseEvent| {
            let oid_str = oid.to_string();
            spawn_local(async move {
                let outcome = invoke("copy_to_clipboard", to_value(&CopyToClipboardCommand{
                    text: oid_str,
                }).unwrap()).await;
                match outcome {
                    Ok(r) => {
                        info!("Copied to clipboard: {:?}", r);
                    },
                    Err(e) => {
                        error!("Failed to copy to clipboard: {:?}", e);
                    },
                }
            });
        })
    };

    let onclick_chevron = {
        if *enabled.deref() {
            let expanded = expanded.clone();
            Callback::from(move |_e: MouseEvent| expanded.set(!*expanded.deref()))
        } else {
            Callback::from(|_e: MouseEvent| {})
        }
    };

    let node_ref = use_node_ref();
    let context_menu_ref = use_node_ref();

    use_effect_with_deps(
        {
            let node_ref = node_ref.clone();
            let context_menu_ref = context_menu_ref.clone();
            let show_context_menu = show_context_menu.clone();
            let context_menu_ref_2 = context_menu_ref.clone();
            let show_context_menu_2 = show_context_menu.clone();
            let show_context_menu_3 = show_context_menu.clone();
            move |_| {
                let mut context_menu_listener = None;
                let mut context_menu_click_away_listener = None;
                let mut context_menu_move_listener = None;

                if let Some(element) = node_ref.cast::<HtmlElement>() {
                    let tr_element = element.clone();
                    let on_click_away = Callback::from(move |e: Event| -> Result<(), ()> {
                        let mouse_event = e.dyn_ref::<MouseEvent>().ok_or(())?;
                        // See: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
                        let is_right_click: bool = mouse_event.button() == 2;
                        let target = e.target().ok_or(())?;
                        let target_el = target.dyn_ref::<HtmlElement>().ok_or(())?;
                        let menu_node = context_menu_ref.get().ok_or(())?;
                        let menu_el = menu_node.dyn_into::<HtmlElement>().map_err(|_| ())?;
                        let offset_parent = target_el.offset_parent().ok_or(())?;
                        if offset_parent != *menu_el.as_ref() // If we did not click on the context menu itself...
                            && ( // ...and this click didn't originate from within this row, or...
                                !tr_element.contains(Some(&target_el.as_ref()))
                                // ...it wasn't a right click.
                                || !is_right_click
                            )
                            {
                            show_context_menu.set(false);
                        }
                        Ok(())
                    });
                    let on_context_menu = Callback::from(move |e: Event| {
                        if let Ok(mouse_event) = e.dyn_into::<MouseEvent>() {
                            mouse_event.prevent_default();
                            let x = mouse_event.client_x();
                            let y = mouse_event.client_y();
                            let context_menu_ref = context_menu_ref_2.clone();
                            let show_context_menu = show_context_menu_2.clone();
                            if let Some(cm_node) = context_menu_ref.get() {
                                if let Ok(cm_html_el) = cm_node.dyn_into::<HtmlElement>() {
                                    let style = cm_html_el.style();
                                    style.set_property("top",  format!("{}px", y).as_str());
                                    style.set_property("left", format!("{}px", x).as_str());
                                    show_context_menu.set(true);
                                }
                            }
                        } else {
                            error!("Could not convert Event to a MouseEvent");
                        }
                    });
                    if *show_context_menu_3.deref() {
                        if let Some(body) = document().body() {
                            let on_click_away_2 = on_click_away.clone();
                            let listener1 = EventListener::new(
                                &body,
                                "click",
                                move |e| { on_click_away.emit(e.clone()); }
                            );
                            context_menu_click_away_listener = Some(listener1);
                            let listener2 = EventListener::new(
                                &body,
                                "contextmenu",
                                move |e| { on_click_away_2.emit(e.clone()); }
                            );
                            context_menu_move_listener = Some(listener2);
                        }
                    }
                    let listener = EventListener::new_with_options(
                        &element,
                        "contextmenu",
                        EventListenerOptions{
                            passive: false, // Seems to be required to stop the default context menu from popping up.
                            ..Default::default()
                        },
                        move |e| on_context_menu.emit(e.clone())
                    );
                    context_menu_listener = Some(listener);
                }
                move || {
                    drop(context_menu_click_away_listener);
                    drop(context_menu_move_listener);
                    drop(context_menu_listener)
                }
            }
        },
        (node_ref.clone(), context_menu_ref.clone(), show_context_menu.clone()),
    );

    let subordinates = if *expanded.deref() {
        props.attr.values
            .iter()
            .enumerate()
            .map(move |(i, v)| {
                html!(
                    <AttributeValueNode
                        alternation={if (i % 2) == 0 { !props.alternation } else { props.alternation }}
                        depth={props.depth + 1}
                        attr_type={props.attr.attr_type.clone()} // TODO: Avoid cloning.
                        attr_value={v.to_owned()}
                        binary={props.attr.binary}
                        quick_delete={true}
                        quick_edit={true}
                        parent_selected={*selected.deref()}
                        />
                )
            })
            .collect::<Vec<Html>>()
    } else {
        vec![]
    };

    html!(
        <>
            <ContextMenu
                r#ref={context_menu_ref}
                visible={*show_context_menu.deref()}
                />
            <tr
                class={classes!([
                    "detail-row",
                    "detail-attr",
                    alt_class,
                    sel_class,
                    ena_class,
                    exp_class,
                ].as_ref())}
                ref={node_ref}>
                <td class={classes!(["detail-row-checkbox"].as_ref())}>
                    <img
                        src={checkbox_img_src}
                        class={classes!(["mui-icon", "row-icon", "row-icon-button"].as_ref())}
                        onclick={onclick_checkbox}
                        />
                </td>
                <td class={classes!(["detail-row-copy"].as_ref())}>
                    <IconButton
                        icon_path={AttrValue::from("/public/ContentCopy.svg")}
                        tooltip_text={AttrValue::from("Copy")}
                        onclick={onclick_copy}
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
                            onclick={onclick_chevron}
                            />
                        {&type_string}
                    </span>
                </td>
            </tr>
            {subordinates}
        </>
    )
}
