use std::ops::Deref;
use yew::prelude::*;
use crate::navtree::node::NavTreeNode;
use web_sys::MouseEvent;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use crate::ipc::invoke;

// TODO: Props should only be created when the model is rendered.
#[derive(Clone, PartialEq, Properties)]
pub struct TreeNodeProps {
    pub node: NavTreeNode,
    pub depth: u16,
    pub alternation: bool,
}

#[function_component(TreeNode)]
pub fn tree_node(props: &TreeNodeProps) -> Html {
    let depth = props.depth + 1;
    let children = props.node.subordinates
        .iter()
        .enumerate()
        .map(|(i, sub)| {
            html!{
                <TreeNode
                    node={sub.deref().clone()}
                    depth={depth}
                    alternation={if (i % 2) == 0 { !props.alternation } else { props.alternation }}
                    />
            }
        })
        .collect::<Html>();
    let click_handler = Callback::from(|e: MouseEvent| {
        invoke("bind_via_url", to_value(&String::from("idm://dsa01.root.mkdemo.wildboar.software:4632")).unwrap());
    });
    html! {
        <li
            class="tree-li"
            >
            <div
                class={if props.alternation { "odd tree-item" } else { "even tree-item" }}
                style={format!("padding-left: {}px;", (props.depth * 40) + 10 )}
                onclick={click_handler}
                >
                <span>{&props.node.text}</span>
            </div>
            <ul>{children}</ul>
        </li>
    }
}
