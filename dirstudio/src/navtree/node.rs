use yew::prelude::Properties;

pub type Kind = u8;
pub const KIND_TEST: Kind = 0;
pub const KIND_ENTRY: Kind = 1;
pub const KIND_POQ: Kind = 2;
pub const KIND_CONTINUATION_REF: Kind = 3;
pub const KIND_ACCESS_POINT_INFO: Kind = 4;


#[derive(Clone, PartialEq, Properties, Hash, Eq)]
pub struct NavTreeNode {
    pub id: u128,
    pub kind: Kind,
    pub text: String,
    pub subordinates: Vec<Box<NavTreeNode>>,
    pub from_entry: bool,
    pub alias: bool,
}

impl NavTreeNode {

    pub fn new (text: &str, subs: Vec<Box<NavTreeNode>>, ) -> NavTreeNode {
        NavTreeNode {
            id: 2,
            kind: KIND_TEST,
            text: String::from(text),
            subordinates: subs,
            from_entry: true,
            alias: false,
        }
    }

}
