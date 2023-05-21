use std::ops::Deref;
use yew::prelude::*;
use web_sys::MouseEvent;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use crate::ipc::invoke;
use dirstudio_api_types::{
    NavTreeNode,
    NavTreeNodeContent,
    ListArgument,
};
use serde_json::json;
use log::info;
// Code basically copied from this example: https://github.com/yewstack/yew/blob/yew-v0.20.0/examples/futures/src/main.rs

// TODO: Props should only be created when the model is rendered.
#[derive(Clone, PartialEq, Properties)]
pub struct TreeNodeProps {
    pub node: NavTreeNode,
    pub depth: u16,
    pub alternation: bool,
}

enum Msg {
    FetchSubordinates,
    // GetMarkdown,
    // GetError,
}
struct NavTreeNodeComponent {
    subordinates: Vec<NavTreeNode>,
}

impl Component for NavTreeNodeComponent {
    type Message = Msg;
    type Properties = TreeNodeProps;

    fn create(ctx: &Context<Self>) -> Self {
        NavTreeNodeComponent {
            subordinates: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // match msg {
        //     Msg::FetchSubordinates => {
        //         ctx.link().send_future(async {
        //             match &ctx.props().node.content {
        //                 NavTreeNodeContent::Subordinate(s) => {
        //                     invoke("list", to_value(&ListArgument{
        //                         object: s.dn.clone(),
        //                         family: false,
        //                         session_id: String::from("REPLACE_ME")
        //                     }).unwrap()).await;
        //                 }
        //                 _ => false // TODO: Handle this more thoroughly.
        //             };
        //             false
        //         });
        //         false
        //     },
        //     _ => false,
        // }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let children = self.subordinates
            .iter()
            .enumerate()
            .map(|(i, sub)| {
                html!{
                    <NavTreeNodeComponent
                        node={sub.deref().clone()}
                        depth={ctx.props().depth + 1}
                        alternation={if (i % 2) == 0 { !ctx.props().alternation } else { ctx.props().alternation }}
                        />
                }
            })
            .collect::<Html>();
        html! {
            <li
                class="tree-li"
                // onclick={ctx.link()}
                >
                <div
                    class={if ctx.props().alternation { "odd tree-item" } else { "even tree-item" }}
                    style={format!("padding-left: {}px;", (ctx.props().depth * 40) + 10 )}
                    >
                    <span>{&ctx.props().node.text}</span>
                </div>
                <ul>{children}</ul>
            </li>
        }
    }

}

// #[function_component(TreeNode)]
// pub fn tree_node(props: &TreeNodeProps) -> Html {
//     let depth = props.depth + 1;
//     let children = use_state(|| vec![]);
//     let onclick = {
//         let children = children.clone();
//         // Callback::from(move |_| children.set(*counter + 1))
//     };

//     // let children = props.node.subordinates
//     //     .iter()
//     //     .enumerate()
//     //     .map(|(i, sub)| {
//     //         html!{
//     //             <TreeNode
//     //                 node={sub.deref().clone()}
//     //                 depth={depth}
//     //                 alternation={if (i % 2) == 0 { !props.alternation } else { props.alternation }}
//     //                 />
//     //         }
//     //     })
//     //     .collect::<Html>();
//     let click_handler = Callback::from(|e: MouseEvent| {
//         // NOTE: Using serde_json to create raw JSON was not working.
//         // Specifically, the error was something like "missing required key message".
//         // I didn't see what exactly was wrong, though.
//         invoke("js2rs", to_value(&Invocation {
//             message: Command::BindViaURL(String::from("idm://localhost:4632"))
//         }).unwrap());
//     });
//     html! {
//         <li
//             class="tree-li"
//             onclick={click_handler}
//             >
//             <div
//                 class={if props.alternation { "odd tree-item" } else { "even tree-item" }}
//                 style={format!("padding-left: {}px;", (props.depth * 40) + 10 )}
//                 >
//                 <span>{&props.node.text}</span>
//             </div>
//             <ul>{children}</ul>
//         </li>
//     }
// }
