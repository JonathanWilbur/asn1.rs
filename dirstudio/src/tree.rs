use std::{ops::Deref, rc::Rc};
use yew::prelude::*;
use web_sys::MouseEvent;
use serde_wasm_bindgen::{to_value, from_value};
use crate::ipc::invoke;
use dirstudio_api_types::{
    NavTreeNode,
    NavTreeNodeContent,
    ListArgument,
    ReadArgument,
    ReadResult,
    SubordinateInfo,
    DirectoryName,
    ListResult,
    SessionId,
    DistinguishedName, RdnSequence,
};
use wasm_bindgen_futures::spawn_local;
use log::{info, error};
// Code basically copied from this example: https://github.com/yewstack/yew/blob/yew-v0.20.0/examples/futures/src/main.rs

// TODO: Props should only be created when the model is rendered.
#[derive(Clone, PartialEq, Properties)]
pub struct TreeNodeProps {
    // pub imm
    pub superior: Option<Rc<TreeNodeProps>>,
    pub session_id: SessionId,
    pub node: NavTreeNode,
    pub depth: u16,
    pub alternation: bool,
}

fn get_dn (props: &TreeNodeProps) -> DistinguishedName {
    let mut current = props;
    let mut rdns: RdnSequence = Vec::new();
    loop {
        match &current.node.content {
            NavTreeNodeContent::Subordinate(sub) => {
                if let Some(ref superior) = current.superior {
                    current = superior;
                    if let NavTreeNodeContent::Subordinate(_) = current.node.content {
                        rdns.push(sub.rdn.clone());
                    } else {
                        rdns.reverse();
                        return rdns;
                    }
                    continue;
                } else {
                    rdns.reverse();
                    return rdns;
                }
            },
            _ => {
                rdns.reverse();
                return rdns;
            },
        };
    }
}

// pub enum Msg {
//     FetchSubordinates,
//     // GetMarkdown,
//     // GetError,
// }
// pub struct NavTreeNodeComponent {
//     subordinates: Vec<NavTreeNode>,
// }

// impl Component for NavTreeNodeComponent {
//     type Message = Msg;
//     type Properties = TreeNodeProps;

//     fn create(ctx: &Context<Self>) -> Self {
//         NavTreeNodeComponent {
//             subordinates: vec![],
//         }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         // match msg {
//         //     Msg::FetchSubordinates => {
//         //         ctx.link().send_future(async {
//         //             match &ctx.props().node.content {
//         //                 NavTreeNodeContent::Subordinate(s) => {
//         //                     invoke("list", to_value(&ListArgument{
//         //                         object: s.dn.clone(),
//         //                         family: false,
//         //                         session_id: String::from("REPLACE_ME")
//         //                     }).unwrap()).await;
//         //                 }
//         //                 _ => false // TODO: Handle this more thoroughly.
//         //             };
//         //             false
//         //         });
//         //         false
//         //     },
//         //     _ => false,
//         // }
//         false
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let children = self.subordinates
//             .iter()
//             .enumerate()
//             .map(|(i, sub)| {
//                 html!{
//                     <NavTreeNodeComponent
//                         node={sub.deref().clone()}
//                         depth={ctx.props().depth + 1}
//                         alternation={if (i % 2) == 0 { !ctx.props().alternation } else { ctx.props().alternation }}
//                         />
//                 }
//             })
//             .collect::<Html>();
//         html! {
//             <li
//                 class="tree-li"
//                 // onclick={ctx.link()}
//                 >
//                 <div
//                     class={if ctx.props().alternation { "odd tree-item" } else { "even tree-item" }}
//                     style={format!("padding-left: {}px;", (ctx.props().depth * 40) + 10 )}
//                     >
//                     <span>{&ctx.props().node.text}</span>
//                 </div>
//                 <ul>{children}</ul>
//             </li>
//         }
//     }

// }

#[function_component(TreeNode)]
pub fn tree_node(props: &TreeNodeProps) -> Html {
    let depth = props.depth + 1;
    let children_fetched: UseStateHandle<bool> = use_state(|| false);
    let children: UseStateHandle<Vec<NavTreeNode>> = use_state(|| vec![]);
    let session_id = props.session_id.clone();

    match &props.node.content {
        NavTreeNodeContent::Session(s) => {
            if children.len() == 0 {
                let s = s.clone();
                let children = children.clone();
                spawn_local(async move {
                    let outcome = invoke("read", to_value(&ReadArgument{
                        sessionId: s.id,
                        object: DirectoryName::RdnSequence(vec![]),
                        // url: String::from("idm://localhost:4632"),
                        // dn: None,
                        // password: None,
                        // signing_cert_path: None,
                        // signing_key_path: None,
                        // attr_cert_path: None,
                    }).unwrap()).await;
                    match outcome {
                        Ok(js_result) => {
                            let r: ReadResult = from_value(js_result).unwrap(); // FIXME: Handle errors.
                            let new_node = NavTreeNode {
                                text: String::from("Root DSE"),
                                content: NavTreeNodeContent::Subordinate(SubordinateInfo {
                                    rdn: vec![],
                                    alias: false,
                                    entry: true,
                                }),
                            };
                            children.set([
                                children.as_slice(),
                                &[ new_node ],
                            ].concat());
                        },
                        Err(e) => {
                            error!("Read error: {:?}", e);
                        },
                    }
                });
            }
        },
        _ => {},
    };

    let click_handler = {
        let props = props.clone();
        let children = children.clone();
        Callback::from(move |e: MouseEvent| {
            match &props.node.content {
                NavTreeNodeContent::Subordinate(_) => {
                    let children = children.clone();
                    if !*children_fetched.deref() {
                        let dn = get_dn(&props);
                        let children = children.clone();
                        let session_id = props.session_id.clone();
                        children_fetched.set(true);
                        spawn_local(async move {
                            let outcome = invoke("list", to_value(&ListArgument{
                                sessionId: session_id,
                                object: DirectoryName::RdnSequence(dn),
                                family: false,
                            }).unwrap()).await;
                            info!("Finished list operation.");
                            match outcome {
                                Ok(js_result) => {
                                    let r: ListResult = from_value(js_result).unwrap(); // FIXME: Handle errors.
                                    info!("Returned {} subordinates.", r.subordinates.len());
                                    children.set(r.subordinates);
                                    // children.set(vec![]);
                                },
                                Err(e) => {
                                    error!("List error: {:?}", e);
                                },
                            }
                        });
                    }
                },
                _ => {},
            }
        })
    };

    let rc_props = Rc::new(props.clone());
    let subordinates = children
        .iter()
        .enumerate()
        .map(|(i, sub)| {
            html!{
                <TreeNode
                    superior={rc_props.clone()}
                    session_id={session_id.clone()}
                    node={sub.clone()}
                    depth={depth}
                    alternation={if (i % 2) == 0 { !props.alternation } else { props.alternation }}
                    />
            }
        })
        .collect::<Html>();

    html! {
        <li
            class="tree-li"
            onclick={click_handler}
            >
            <div
                class={if props.alternation { "odd tree-item" } else { "even tree-item" }}
                style={format!("padding-left: {}px;", (props.depth * 20) + 10 )}
                class="nav-node-text"
                >
                // <span class="nav-node-text">{&props.node.text}</span>
                {&props.node.text}
            </div>
            <ul>{subordinates}</ul>
        </li>
    }
}
