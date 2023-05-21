use std::borrow::Borrow;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use crate::tree::TreeNode;
// use web_sys::ev
use dirstudio_api_types::{
    BindArgument,
    BindResult,
    SessionId,
    NavTreeNode,
    SessionInfo,
    SubordinateInfo,
    ReadResult, ReadArgument,
};
use serde_wasm_bindgen::{to_value, from_value};
use crate::{ipc::invoke, tree};
use wasm_bindgen_futures::spawn_local;
use log::{info, warn, error};
use crate::tree::TreeNode;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// fn bind (welcome: UseStateHandle<String>, name: String) {

// }

#[function_component(App)]
pub fn app() -> Html {
    let sessions = use_state(|| Vec::<SessionId>::from([String::from("asdf")]));
    let tree_roots = use_state(|| Vec::<NavTreeNode>::new());
    // let onclick = {
    //     let counter = counter.clone();
    //     Callback::from(move |_| counter.set(*counter + 1))
    // };
    // let tree = NavTreeNode::new(
    //     "C=US",
    //     vec![
    //         Box::new(NavTreeNode::new(
    //             "ST=FL",
    //             vec![
    //                 Box::new(NavTreeNode::new(
    //                     "L=Tampa",
    //                     vec![],
    //                 )),
    //                 Box::new(NavTreeNode::new(
    //                     "L=Ocala",
    //                     vec![],
    //                 )),
    //                 Box::new(NavTreeNode::new(
    //                     "L=Jacksonville",
    //                     vec![],
    //                 )),
    //             ],
    //         )),
    //         Box::new(NavTreeNode::new(
    //             "ST=GA",
    //             vec![],
    //         )),
    //         Box::new(NavTreeNode::new(
    //             "ST=TX",
    //             vec![],
    //         )),
    //         Box::new(NavTreeNode::new(
    //             "ST=CA",
    //             vec![],
    //         )),
    //         Box::new(NavTreeNode::new(
    //             "ST=NY",
    //             vec![],
    //         )),
    //         Box::new(NavTreeNode::new(
    //             "ST=NH",
    //             vec![],
    //         )),
    //     ],
    // );

    // let select_session = Callback::from(|e: | {
    //     invoke("bind_via_url", to_value(&String::from("idm://dsa01.root.mkdemo.wildboar.software:4632")).unwrap());
    // });

    let children = tree_roots
        .iter()
        .enumerate()
        .map(|(i, sub)| {
            html!{
                <TreeNode
                    node={sub.clone()}
                    depth={0}
                    alternation={(i % 2) == 0}
                    />
            }
        })
        .collect::<Html>();

    let session_options = sessions
        .iter()
        .map(|sess| {
            html!{
                <option value={sess.clone()}>{sess}</option>
            }
        })
        .collect::<Html>();

    let create_session = Callback::from(move |e| {
        let tree_roots = tree_roots.clone();
        let sessions = sessions.clone();
        spawn_local(async move {
            let outcome = invoke("bind", to_value(&BindArgument{
                url: String::from("idm://localhost:4632"),
                dn: None,
                password: None,
                signing_cert_path: None,
                signing_key_path: None,
                attr_cert_path: None,
            }).unwrap()).await;
            match outcome {
                Ok(js_result) => {
                    let r: BindResult = from_value(js_result).unwrap(); // FIXME: Handle errors.
                    // sessions.set([
                    //     sessions.as_slice(),
                    //     &[ r.session_id ],
                    // ].concat());
                    let session_id = r.session_id.clone();
                    let new_node = NavTreeNode { // TODO: Do not clone so much.
                        text: session_id.clone(),
                        content: dirstudio_api_types::NavTreeNodeContent::Session(SessionInfo {
                            id: session_id,
                        }),
                    };
                    tree_roots.set([
                        tree_roots.as_slice(),
                        &[ new_node ],
                    ].concat());
                },
                Err(e) => {
                    error!("Bind error: {:?}", e);
                },
            }
        });
    });

    html! {
        <main class="container">
            <div id="nav-column">
                <div>
                    <label for="session-select">
                        <h2>{"Sessions"}</h2>
                    </label>
                    <select
                        id="session-select"
                        onchange={create_session}
                        >
                        {session_options}
                        <option value="CREATE">{"Create New Session..."}</option>
                    </select>
                </div>
                <hr />
                {children}
            </div>
        </main>
    }
}
