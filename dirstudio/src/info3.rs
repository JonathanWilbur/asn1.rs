use dirstudio_api_types::{
    AttributeType,
    AttributeValue,
    Context,
    ContextType,
    OidInfo,
    EntryInfoNodeContent, EntryInfoItem,
};
use std::{ops::Deref, rc::Rc};
use yew::prelude::*;
use web_sys::MouseEvent;
use serde_wasm_bindgen::{to_value, from_value};
use crate::ipc::invoke;
use wasm_bindgen_futures::spawn_local;
use log::{info, error};
use wildboar_asn1::OBJECT_IDENTIFIER;
use std::cell::RefCell;
use crate::components::AttributeTypeNode::AttributeTypeNode;
use crate::components::AttributeNode::AttributeNode;

#[derive(Clone, PartialEq, Properties)]
pub struct EntryInfoProps {
    pub info: Vec<EntryInfoItem>,
}

/*

I think it only makes sense for subordinates to be included in props, not the
parent. I originally thought it would be best to traverse up the tree to get
attribute type data.

I think I am going to go with the single-window approach

I have also decided that I am not going to display family members a compound entry.
The separateFamilyMembers service control will always be active. The reasons are:
1. Avoiding complexity, and therefore bugginess.
2. The UI getting insanely cramped as you attempt to have a potentially infinitely deep hierarchy of entries.

I will also not break down values into "value details" nodes, because there is already a layer beneath them: context types.

AttributeTypeComponent
AttributeComponent (collapsible)
- AttributeValueComponent (collapsible)
  - ContextTypeComponent (collapsible)
    - ContextValueComponent (not collapsible)

These all need to be separate components because they require different props
to have complete information and they will have different actions.

 */

#[function_component(EntryInfoComponent)]
pub fn entry_info(props: &EntryInfoProps) -> Html {
    let constituents = props.info
        .iter()
        .enumerate()
        .map(|(i, info)| {
            match info {
                EntryInfoItem::AttributeType(attr_type) => {
                    html!(
                        <AttributeTypeNode
                            alternation={false} // FIXME: This will involve some trickery.
                            depth={0}
                            attr_type={attr_type.to_owned()}
                            />
                    )
                },
                EntryInfoItem::Attribute(attr) => {
                    html!(
                        <AttributeNode
                            alternation={false} // FIXME: This will involve some trickery.
                            depth={0}
                            attr={attr.to_owned()}
                            />
                    )
                },
            }
        })
        .collect::<Vec<Html>>();
    html!(
        <table style={"width: 100%; max-width: 100%; table-layout: fixed;"}>
            <thead>
                <th style={"width: 26px"}></th>
                <th style={"width: 26px"}></th>
                <th style={"width: 26px"}></th>
                <th style={"width: 26px"}></th>
                <th style={"width: auto"}></th>
            </thead>
            <tbody style={"width: 100%; max-width: 100%"}>
                {constituents}
            </tbody>
        </table>
    )
}
