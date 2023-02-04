use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::tree::TreeNode;
use crate::navtree::node::NavTreeNode;
// use web_sys::ev

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {
    let tree = NavTreeNode::new(
        "C=US",
        vec![
            Box::new(NavTreeNode::new(
                "ST=FL",
                vec![
                    Box::new(NavTreeNode::new(
                        "L=Tampa",
                        vec![],
                    )),
                    Box::new(NavTreeNode::new(
                        "L=Ocala",
                        vec![],
                    )),
                    Box::new(NavTreeNode::new(
                        "L=Jacksonville",
                        vec![],
                    )),
                ],
            )),
            Box::new(NavTreeNode::new(
                "ST=GA",
                vec![],
            )),
            Box::new(NavTreeNode::new(
                "ST=TX",
                vec![],
            )),
            Box::new(NavTreeNode::new(
                "ST=CA",
                vec![],
            )),
            Box::new(NavTreeNode::new(
                "ST=NY",
                vec![],
            )),
            Box::new(NavTreeNode::new(
                "ST=NH",
                vec![],
            )),
        ],
    );

    // let select_session = Callback::from(|e: | {
    //     invoke("bind_via_url", to_value(&String::from("idm://dsa01.root.mkdemo.wildboar.software:4632")).unwrap());
    // });

    html! {
        <main class="container">
            <div id="nav-column">
                <div>
                    <label for="session-select">
                        <h2>{"Sessions"}</h2>
                    </label>
                    <select
                        id="session-select"
                        >
                        <option value="" selected={true}>{"--"}</option>
                        <option value="bigboi">{"Create New Session..."}</option>
                    </select>
                </div>
                <hr />
                <TreeNode
                    node={tree}
                    depth={0}
                    alternation={true}
                    />
            </div>
        </main>
    }
}
