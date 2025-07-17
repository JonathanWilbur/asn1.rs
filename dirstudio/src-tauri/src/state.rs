use std::collections::HashMap;
use x500_client::DAPClient;
use x500::types::{
    ParseX500AttributeType,
    ParseX500Value,
    DisplayX500AttributeType,
    DisplayX500Value,
    parse_value,
    value_to_string,
};
use x690::{X690Element, x690_write_tlv};
use wildboar_asn1::{ObjectIdentifierIntoDescriptor, ObjectIdentifierFromDescriptor};
use std::sync::Arc;
use tauri::async_runtime::Mutex;

pub type SessionId = String;

#[derive(Clone)]
pub struct SessionState {
    pub client: Arc<Mutex<DAPClient>>,
}

#[derive(Clone)]
pub struct ServerSideState {
    pub sessions: Arc<Mutex<HashMap<SessionId, SessionState>>>,
}

impl ParseX500AttributeType for ServerSideState {  }

impl ParseX500Value<X690Element> for ServerSideState {

    fn parse_value (&self, attr_type: &x500::InformationFramework::AttributeType, s: &str) -> Result<Option<X690Element>, std::fmt::Error> {
        parse_value(self, attr_type, s)
    }

}

impl ObjectIdentifierIntoDescriptor for ServerSideState {

    fn get_oid_descriptor (&self, oid: &wildboar_asn1::OBJECT_IDENTIFIER) -> Option<String> {
        // TODO:
        None
    }

}

impl ObjectIdentifierFromDescriptor for ServerSideState {

    fn get_oid (&self, desc: &str) -> Option<wildboar_asn1::OBJECT_IDENTIFIER> {
        // TODO:
        None
    }

}

impl DisplayX500AttributeType for ServerSideState {  }

impl DisplayX500Value<X690Element> for ServerSideState {

    fn value_to_string (self: &Self, attr_type: &x500::InformationFramework::AttributeType, value: &X690Element) -> Result<Option<String>, wildboar_asn1::ASN1Error> {
        value_to_string(self, attr_type, value)
    }

    fn unrecognized_value_to_string (self: &Self, value: &X690Element) -> String {
        let mut bytes = Vec::new();
        x690_write_tlv(&mut bytes, &value).unwrap_or_default();
        format!("#{}", hex::encode(&bytes))
    }

}
