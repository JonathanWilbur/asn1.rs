use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ContextMenuProps {
    pub r#ref: NodeRef,
}

#[function_component(ContextMenu)]
pub fn context_menu(props: &ContextMenuProps) -> Html {
    html!(
        <div id="context-menu" ref={props.r#ref.clone()}>
            <div class="context-menu-item">{"Option 1"}</div>
        </div>
    )
}
