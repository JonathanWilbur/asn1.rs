use tauri::{CustomMenuItem, Menu, Submenu};

pub fn get_app_submenu () -> Submenu {
    let manage_contexts = CustomMenuItem::new("man_ctx".to_string(), "Manage Contexts");
    let unified_log = CustomMenuItem::new("unif_log".to_string(), "Unified Log");
    let unified_ops = CustomMenuItem::new("unif_ops".to_string(), "Unified Operations");
    let about = CustomMenuItem::new("about".to_string(), "About");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    Submenu::new("App", Menu::new()
        .add_item(manage_contexts)
        .add_item(unified_log)
        .add_item(unified_ops)
        .add_item(about)
        .add_item(quit)
    )
}

pub fn get_session_submenu () -> Submenu {
    let create_from_ctx = CustomMenuItem::new("from_ctx".to_string(), "Create from Context");
    let create_from_bookmark = CustomMenuItem::new("from_bookmark".to_string(), "Create from Bookmark");
    let create_one_off = CustomMenuItem::new("one_off".to_string(), "Create One-Off Session");
    Submenu::new("Contexts", Menu::new()
        .add_item(create_from_ctx)
        .add_item(create_from_bookmark)
        .add_item(create_one_off)
    )
}

pub fn get_dit_submenu () -> Submenu {
    let bound_entry = CustomMenuItem::new("goto_bound_entry".to_string(), "Go to Bound Entry");
    let dsa_entry = CustomMenuItem::new("goto_dsa_entry".to_string(), "Go to DSA");
    let bookmark = CustomMenuItem::new("goto_bookmark".to_string(), "Go to Bookmark");
    let forward = CustomMenuItem::new("forward".to_string(), "Go Forward");
    let backward = CustomMenuItem::new("backward".to_string(), "Go Back");
    let export = CustomMenuItem::new("export_dit".to_string(), "Export DIT");
    Submenu::new("DIT", Menu::new()
        .add_item(bound_entry)
        .add_item(dsa_entry)
        .add_item(bookmark)
        .add_item(forward)
        .add_item(backward)
        .add_item(export)
    )
}

pub fn get_entry_submenu () -> Submenu {
    let copy_dn = CustomMenuItem::new("copy_dn".to_string(), "Copy DN to Clipboard");
    let copy_ldap_dn = CustomMenuItem::new("copy_ldap_dn".to_string(), "Copy LDAP DN to Clipboard");
    let export = CustomMenuItem::new("export_entry".to_string(), "Export Entry");
    Submenu::new("Entry", Menu::new()
        .add_item(copy_dn)
        .add_item(copy_ldap_dn)
        .add_item(export)
    )
}

pub fn get_attr_value_submenu () -> Submenu {
    let copy_str = CustomMenuItem::new("copy_as_str".to_string(), "Copy as String");
    let copy_hex = CustomMenuItem::new("copy_as_hex".to_string(), "Copy as Hex");
    let copy_base64 = CustomMenuItem::new("copy_as_base64".to_string(), "Copy as Base64");
    let export = CustomMenuItem::new("export_value".to_string(), "Export Value");
    Submenu::new("Attribute Values", Menu::new()
        .add_item(copy_str)
        .add_item(copy_hex)
        .add_item(copy_base64)
        .add_item(export)
    )
}

pub fn get_info_submenu () -> Submenu {
    let object_classes = CustomMenuItem::new("oc".to_string(), "Object Classes");
    let attribute_types = CustomMenuItem::new("at".to_string(), "Attribute Types");
    let context_types = CustomMenuItem::new("ct".to_string(), "Context Types");
    let matching_rules = CustomMenuItem::new("mr".to_string(), "Matching Rules");
    let name_forms = CustomMenuItem::new("nf".to_string(), "Name Forms");
    let cert_exts = CustomMenuItem::new("nf".to_string(), "Certificate Extensions");
    let algo = CustomMenuItem::new("algo".to_string(), "Algorithms");
    let other_names = CustomMenuItem::new("other_names".to_string(), "Other Names");
    let search_rules = CustomMenuItem::new("sr".to_string(), "Search Rules");
    let zonal_matches = CustomMenuItem::new("zm".to_string(), "Zonal Matches");
    let ldap_syntaxes = CustomMenuItem::new("lsx".to_string(), "LDAP Syntaxes");
    let oids = CustomMenuItem::new("oids".to_string(), "Object Identifiers");

    Submenu::new("Information", Menu::new()
        .add_item(object_classes)
        .add_item(attribute_types)
        .add_item(context_types)
        .add_item(matching_rules)
        .add_item(name_forms)
        .add_item(cert_exts)
        .add_item(algo)
        .add_item(other_names)
        .add_item(search_rules)
        .add_item(zonal_matches)
        .add_item(ldap_syntaxes)
        .add_item(oids)
    )
}

pub fn get_pki_pmi_submenu () -> Submenu {
    let trust_anchors = CustomMenuItem::new("trust_anchors".to_string(), "Manage Trust Anchors");
    let private_keys = CustomMenuItem::new("private_keys".to_string(), "Manage Private Keys");
    Submenu::new("PKI / PMI", Menu::new()
        .add_item(trust_anchors)
        .add_item(private_keys)
    )
}

pub fn get_view_submenu () -> Submenu {
    let session_info = CustomMenuItem::new("session_info".to_string(), "Session Info");
    Submenu::new("View", Menu::new()
        .add_item(session_info)
    )
}

pub fn get_help_submenu () -> Submenu {
    let docs = CustomMenuItem::new("docs".to_string(), "Documentation");
    let run_your_own = CustomMenuItem::new("run_your_own".to_string(), "Run your own DSA");
    let wildboar = CustomMenuItem::new("wildboar".to_string(), "Wildboar Software");
    let bug = CustomMenuItem::new("bug".to_string(), "Report a Bug");
    let support = CustomMenuItem::new("support".to_string(), "Professional Support");
    Submenu::new("Help", Menu::new()
        .add_item(docs)
        .add_item(run_your_own)
        .add_item(wildboar)
        .add_item(bug)
        .add_item(support)
    )
}

pub fn get_menu () -> Menu {
    let menu = Menu::new()
        .add_submenu(get_app_submenu())
        .add_submenu(get_session_submenu())
        .add_submenu(get_dit_submenu())
        .add_submenu(get_entry_submenu())
        .add_submenu(get_attr_value_submenu())
        .add_submenu(get_info_submenu())
        .add_submenu(get_pki_pmi_submenu())
        .add_submenu(get_view_submenu())
        .add_submenu(get_help_submenu())
        ;
    menu
}
