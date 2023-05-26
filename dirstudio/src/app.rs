
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use dirstudio_api_types::{
    BindArgument,
    BindResult,
    SessionId,
    NavTreeNode,
    SessionInfo,
    EntryInformation,
    ModifyRight,
    DistinguishedName,
    EntryInfoItem,
    Attribute,
    AttributeValue, DirectoryName, OidInfo,
};
use serde_wasm_bindgen::{to_value, from_value};
use crate::ipc::invoke;
use wasm_bindgen_futures::spawn_local;
use log::error;
use crate::tree::TreeNode;
use std::rc::Rc;
use crate::info3::EntryInfoComponent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(PartialEq, Eq, Clone)]
pub struct DetailsColumnState {
    pub entry: Option<EntryInformation>,
    pub modify_rights: Vec<ModifyRight>,
    pub performer: Option<DistinguishedName>,
    pub alias_dereferenced: bool,
    pub ldap_message: Option<String>,
}

#[derive(PartialEq, Eq, Clone)]
pub enum DetailsColumnAction {
    SetEntry(DetailsColumnState)
}

impl Reducible for DetailsColumnState {
    type Action = DetailsColumnAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            DetailsColumnAction::SetEntry(s) => s,
        };
        next_state.into()
    }
}

impl Default for DetailsColumnState {

    fn default() -> Self {
        DetailsColumnState {
            entry: None,
            modify_rights: vec![],
            performer: None,
            alias_dereferenced: false,
            ldap_message: None,
        }
    }

}

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct DetailsProviderProps {
    #[prop_or_default]
    pub children: Children,
}

pub type DetailsPaneContext = UseReducerHandle<DetailsColumnState>;

#[function_component]
pub fn DetailsProvider(props: &DetailsProviderProps) -> Html {
    let details = use_reducer(DetailsColumnState::default);

    html! {
        <ContextProvider<DetailsPaneContext> context={details}>
            {props.children.clone()}
        </ContextProvider<DetailsPaneContext>>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let sessions = use_state(|| Vec::<SessionId>::from([String::from("asdf")]));
    let tree_roots = use_state(|| Vec::<NavTreeNode>::new());
    let children = tree_roots
        .iter()
        .enumerate()
        .map(|(i, sub)| {
            html!{
                <TreeNode
                    session_id={sub.text.clone()}
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

    let details: DetailsColumnState = DetailsColumnState {
        entry: Some(EntryInformation{
            name: DirectoryName::RdnSequence(vec![]),
            from_entry: true,
            info: vec![
                EntryInfoItem::Attribute(Attribute{
                    attr_type: OidInfo{
                        name: Some(String::from("commonName")),
                        numeric: vec![ 2, 5, 4, 3 ],
                    },
                    binary: false,
                    values: vec![
                        AttributeValue{
                            value: String::from("Jonathan M. Wilbur"),
                            contexts: vec![],
                        },
                        AttributeValue{
                            value: String::from("M.C. Peepants"),
                            contexts: vec![],
                        },
                    ],
                }),
                EntryInfoItem::Attribute(Attribute{
                    attr_type: OidInfo{
                        name: Some(String::from("surname")),
                        numeric: vec![ 2, 5, 4, 4 ],
                    },
                    binary: false,
                    values: vec![
                        AttributeValue{
                            value: String::from("Wilbur"),
                            contexts: vec![],
                        },
                        AttributeValue{
                            value: String::from("SchmittyWerbenJaegerManJensen McSchmittyWerbenJaegerManJensen O'SchmittyWerbenJaegerManJensen SchmittyWerbenJaegerManJensenSchmittyWerbenJaegerManJensen"),
                            contexts: vec![],
                        },
                    ],
                }),
            ],
            incomplete: false,
            partial_name: false,
            derived: false,
        }),
        ..Default::default()
    };

    let entry_info = details.entry.unwrap_or_default();
    html! {
        <main class="container">
            <div id="nav-column">
                <div>
                    // <label for="session-select">
                    //     <h2>{"Sessions"}</h2>
                    // </label>
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
            <div id="details-column">
                <div>
                    <pre id="details-dn">{"c=US,st=FL,l=STJ,l=St. Johns,l=Oxford Estates,cn=Jonathan M. Wilbur"}</pre>
                </div>
                <hr />
                <EntryInfoComponent
                    info={entry_info.info}
                    />
            </div>
        </main>
    }
}
