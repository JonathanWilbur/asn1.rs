use std::collections::HashMap;
use x500_client::DAPClient;
use x500::types::{
    ParseX500AttributeType,
    ParseX500Value,
    DisplayX500AttributeType,
    DisplayX500Value,
};
use x690::{X690Element, write_x690_node};
use asn1::{ObjectIdentifierIntoDescriptor, ObjectIdentifierFromDescriptor};

pub type SessionId = String;

#[derive(Clone)]
pub struct SessionState {
    pub client: DAPClient,
}

#[derive(Clone)]
pub struct ServerSideState {
    pub sessions: HashMap<SessionId, SessionState>,
}

impl ParseX500AttributeType for ServerSideState {  }

impl ParseX500Value<X690Element> for ServerSideState {

    // TODO:
    fn parse_value (&self, attr_type: &x500::InformationFramework::AttributeType, s: &str) -> Result<Option<X690Element>, std::fmt::Error> {
        Ok(None)
    }

}

impl ObjectIdentifierIntoDescriptor for ServerSideState {

    fn get_oid_descriptor (&self, oid: &asn1::OBJECT_IDENTIFIER) -> Option<String> {
        // TODO:
        None
    }

}

impl ObjectIdentifierFromDescriptor for ServerSideState {

    fn get_oid (&self, desc: &str) -> Option<asn1::OBJECT_IDENTIFIER> {
        // TODO:
        None
    }

}

impl DisplayX500AttributeType for ServerSideState {  }

impl DisplayX500Value<X690Element> for ServerSideState {

    fn value_to_string (self: &Self, attr_type: &x500::InformationFramework::AttributeType, value: &X690Element) -> Result<Option<String>, asn1::ASN1Error> {
        // TODO:
        Ok(None)
    }

    fn unrecognized_value_to_string (self: &Self, value: &X690Element) -> String {
        let mut bytes = Vec::new();
        write_x690_node(&mut bytes, &value).unwrap_or_default();
        format!("#{}", hex::encode(&bytes))
    }

}
