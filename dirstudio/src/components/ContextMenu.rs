use yew::prelude::*;
use web_sys::{Element, Node, HtmlElement};
use gloo::events::{EventListener, EventListenerOptions};

#[derive(Clone, PartialEq, Properties)]
pub struct ContextMenuProps {
    pub r#ref: NodeRef,
    pub visible: bool,
}

#[function_component(ContextMenu)]
pub fn context_menu(props: &ContextMenuProps) -> Html {
    let vis_class = if props.visible { "visible" } else { "hidden" };
    let node_ref = props.r#ref.clone();
    // This exists just to ignore right-clicks on the context menu itself.
    use_effect_with_deps(
        {
            move |_| {
                let mut context_menu_listener = None;
                if let Some(element) = node_ref.cast::<HtmlElement>() {
                    let ignore_default_right_click_menu = Callback::from(move |e: Event| {
                        log::warn!("asdfqwer");
                        e.prevent_default();
                        e.stop_propagation();
                    });
                    let listener = EventListener::new_with_options(
                        &element,
                        "contextmenu",
                        EventListenerOptions{
                            passive: false, // Seems to be required to stop the default context menu from popping up.
                            ..Default::default()
                        },
                        move |e| ignore_default_right_click_menu.emit(e.clone())
                    );
                    context_menu_listener = Some(listener);
                }
                move || drop(context_menu_listener)
            }
        },
        [props.r#ref.clone()],
    );

    html!(
        <div id="context-menu" ref={props.r#ref.clone()} class={classes!([vis_class].as_ref())}>
            <div class="context-menu-item">{"Option 1"}</div>
        </div>
    )
}
