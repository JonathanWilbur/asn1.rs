

pub struct Node {
    parent: u128,
    loading_subordinates: bool,
    subordinates: Vec<Rc<Node>>,
    // info, such as RDN
}

pub struct PerSessionState {
    // root_node: Rc<Node>, // id is always 0
    // selected_id: Option<u128>,
    // dit_nodes_by_id: Map<u128, Rc<Node>>
    // next_id: Arc<Mutex<u128>>,
}

/*

[Option<session>; 32] // index is the ID

list_subordinates (session_id: u8, node_id: u32) {
    sess = sessions.get(session_id)
    if !sess return?
    node = nodes.get(node_id)
    if !node return?
    if node.loading return
    dn = get_dn(node_id)
    if !dn return?
    outcome = x500_client::list(sess.client, list_args(dn))
    serialize list results
    return
}

client should await list_subordinates(), then update subordinates with response

Component on client-side:
State: {
    state Loading | Loaded
    subordinates vec![]
    subordinates Option<vec![]> (None means not yet loaded)
}

*/
