use dirstudio_api_types::OidInfo;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct AttributeTypeNodeProps {
    pub alternation: bool,
    pub depth: u32,
    pub attr_type: OidInfo,
}

#[function_component(AttributeTypeNode)]
pub fn attribute_type_node(props: &AttributeTypeNodeProps) -> Html {
    let indent = String::from(" ").repeat(((props.depth as usize) * 4) + 1);
    html!(
        <tr>
            <td>
                <input type="checkbox" />
            </td>
            <td>
                <input type="checkbox" />
            </td>
            <td>
                <input type="checkbox" />
            </td>
            <td class={classes!(["detail-cell"].as_ref())}>
                <pre>{indent}{format!("{}", &props.attr_type).to_string()}</pre>
            </td>
        </tr>
    )
}
