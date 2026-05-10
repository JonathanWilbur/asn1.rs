use std::sync::{Arc, RwLock};
use std::net::SocketAddr;
use std::collections::HashMap;

use wildboar_asn1::{ASN1Error, OBJECT_IDENTIFIER};
use x690::X690Element;

#[derive(Clone, Debug)]
pub struct TLSParameters {
    // TODO:
}

#[derive(Clone, Debug)]
pub struct DTLSParameters {
    // TODO:
}

#[derive(Clone, Debug)]
pub struct TCPParameters {
    pub socket_addr: SocketAddr,
    pub keep_alive: bool,
    pub keep_alive_init_delay: u32,
    pub delay: bool,
    pub ttl: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct UDPParameters {
    pub socket_addr: SocketAddr,
}

#[derive(Clone, Debug)]
pub struct IDMListenerConfig {
    pub tcp: TCPParameters,
    pub tls: Option<TLSParameters>,
}

#[derive(Clone, Debug)]
pub struct LDAPListenerConfig {
    pub tcp: TCPParameters,
    pub tls: Option<TLSParameters>,
}

#[derive(Clone, Debug)]
pub struct ITOTListenerConfig {
    pub tcp: TCPParameters,
    pub tls: Option<TLSParameters>,
    // TODO:
}

#[derive(Clone, Debug)]
pub struct TP4ListenerConfig {
    // TODO:
}

/// X.25-over-TCP (XOT) Configuration
#[derive(Clone, Debug)]
pub struct XOTListenerConfig {
    // TODO:
}

/// DIXIE Protocol Listener Configuration
#[derive(Clone, Debug)]
pub struct DIXIEListenerConfig {
    // TODO:
}

#[derive(Clone, Debug)]
pub struct LPPListenerConfig {
    // TODO:
}

#[derive(Clone, Debug)]
pub enum ListenerConfig {
    Idm(IDMListenerConfig),
    Ldap(LDAPListenerConfig),
    Itot(ITOTListenerConfig),
    Tp4(TP4ListenerConfig),
    Xot(XOTListenerConfig),
    Dixie(DIXIEListenerConfig),
    Lpp(LPPListenerConfig),
}

#[derive(Clone, Debug)]
pub struct Config {
    pub listeners: Vec<ListenerConfig>,
}

#[derive(Debug)]
pub enum MatchOutcome {
    Defined(bool),
    Undefined,
    AssertionSyntaxError(ASN1Error),
    StoredSyntaxError(ASN1Error),
    IoError(std::io::Error),
}

// TODO: This is going to require a lot more thought.
pub type EqualityMatcher = fn() -> MatchOutcome;

pub type DefaultValueGetter = fn() -> X690Element;

pub type ValueNormalizer = fn(&X690Element) -> X690Element;

pub enum Matcher {
    Equality(EqualityMatcher),
}



pub struct AttributeTypeInfo {
    pub derivation: Option<OBJECT_IDENTIFIER>,
    pub equality_match: Option<OBJECT_IDENTIFIER>,
    pub ordering_match: Option<OBJECT_IDENTIFIER>,
    pub substrings_match: Option<OBJECT_IDENTIFIER>,
    pub single_valued: bool,
    pub collective: bool,
    pub dummy: bool,
    pub no_user_modification: bool,
    // pub usage: AttributeUsage, // TODO:
    pub ldap_syntax: Option<OBJECT_IDENTIFIER>,
    pub ldap_name: Vec<String>,
    pub ldap_desc: Option<String>,
    pub obsolete: bool,
    pub id: OBJECT_IDENTIFIER,

    // A function that takes a value of this type and returns Ok(()) if it is syntatically valid.
    // pub validator: Option<ValueValidator<Type>>,
}

pub struct MatchingRuleInfo {
    pub parent_matching_rules: Vec<OBJECT_IDENTIFIER>,
    pub unique_match_indicator: Option<OBJECT_IDENTIFIER>,
    pub ldap_syntax: Option<OBJECT_IDENTIFIER>,
    pub ldap_name: Vec<String>,
    pub ldap_desc: Option<String>,
    pub id: OBJECT_IDENTIFIER,
    pub matcher: Matcher,
    // TODO: suitable for matching
}

pub struct NameFormInfo {
    pub named_object_class: OBJECT_IDENTIFIER,
    pub mandatory_attributes: Vec<OBJECT_IDENTIFIER>,
    pub optional_attributes: Vec<OBJECT_IDENTIFIER>,
    pub ldap_name: Vec<String>,
    pub ldap_desc: Option<String>,
    pub id: OBJECT_IDENTIFIER,
}

pub struct ContextTypeInfo {
    pub absent_match: bool,
    pub id: OBJECT_IDENTIFIER,
    pub matcher: EqualityMatcher,
    pub default_value: Option<DefaultValueGetter>,
    pub normalize: Option<ValueNormalizer>,
}

pub struct LdapSyntaxInfo {
    pub ldap_desc: String,
    pub id: OBJECT_IDENTIFIER,

    // TODO: Are these really the best error types for this purpose?
    // pub convert_x500_value_to_ldap_value: fn(S, X690Element) -> Result<LDAPAttributeValue, ASN1Error>,
    // pub convert_ldap_value_to_x500_value: fn(S, LDAPAttributeValue) -> Result<X690Element, ASN1Error>,
}

#[derive(Debug, Clone)]
pub struct ObjectClassInfo {
    pub superclasses: Vec<ObjectClassInfo>,
    // pub kind: ObjectClassKind, // TODO:
    pub mandatory_attributes:Vec<OBJECT_IDENTIFIER>,
    pub optional_attributes: Vec<OBJECT_IDENTIFIER>,
    pub ldap_name: Vec<String>,
    pub ldap_desc: Option<String>,
    pub id: OBJECT_IDENTIFIER,
}

pub type AttributesKnowledge = HashMap<OBJECT_IDENTIFIER, AttributeTypeInfo>;
pub type MatchingRulesKnowledge = HashMap<OBJECT_IDENTIFIER, MatchingRuleInfo>;

pub struct X500DirServiceAppEntity {
    pub config: Config,
    pub attributes: AttributesKnowledge,
    pub matching_rules: MatchingRulesKnowledge,
    // TODO: Schema knowledge
    // TODO: language / locale?
    // TODO: DIT?
    // TODO: Associations
    // TODO: DSA relationships
    // TODO: events channel (used by OB)
    // TODO: nextOutboundInvokeId
}

// TODO: get_X_by_name as well
impl X500DirServiceAppEntity {

    pub fn get_attr_type_by_id(oid: &OBJECT_IDENTIFIER) -> Option<AttributeTypeInfo> {
        todo!()
    }

    pub fn get_matching_rule_by_id(oid: &OBJECT_IDENTIFIER) -> Option<MatchingRuleInfo> {
        todo!()
    }

    pub fn get_object_class_by_id(oid: &OBJECT_IDENTIFIER) -> Option<ObjectClassInfo> {
        todo!()
    }

    pub fn get_ldap_syntax_by_id(oid: &OBJECT_IDENTIFIER) -> Option<LdapSyntaxInfo> {
        todo!()
    }

    pub fn get_name_form_by_id(oid: &OBJECT_IDENTIFIER) -> Option<NameFormInfo> {
        todo!()
    }

    pub fn get_context_type_by_id(oid: &OBJECT_IDENTIFIER) -> Option<NameFormInfo> {
        todo!()
    }

}


pub struct Association {
    pub id: u64,
}

/// Just testing
pub async fn add_entry(
    dsae: X500DirServiceAppEntity,
    assn: Association,

) -> Result<(), ()> {

}
