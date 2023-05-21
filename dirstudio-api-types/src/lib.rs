// use x500::PKI_Stub::AttributeTypeAndValue;
use x500::types::{
    DisplayX500AttributeType,
    DisplayX500Value,
    ParseX500AttributeType,
    ParseX500Value,
};
use yew::prelude::Properties;
use x500::DistributedOperations::{
    ReferenceType,
    OperationProgress_nameResolutionPhase,
    MasterOrShadowAccessPoint_category,
};
use x500::InformationFramework::{
    AttributeTypeAndValue,
    RelativeDistinguishedName,
    RDNSequence as X500RDNSequence,
    Name,
};
use x500::CertificateExtensions::{
    GeneralName,
    EDIPartyName,
};
use x500::SelectedAttributeTypes::UnboundedDirectoryString;
use asn1::{OID_ARC, ObjectIdentifierIntoDescriptor, OBJECT_IDENTIFIER};
use std::net::{ToSocketAddrs, SocketAddr};
use rose::{InvokeIdInt};
use x690::X690Element;
pub type Base64String = String;
pub type SessionId = String;


#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, serde::Serialize, serde::Deserialize)]
pub enum AbortReason {
    // From IDM
    MistypedPDU,
    UnboundRequest,
    InvalidPDU,
    ResourceLimitation,
    ConnectionFailed,
    InvalidProtocol,
    ReasonNotSpecified,
    // From ACSE
    ProtocolError,
    AuthenticationMechanismNameNotRecognized,
    AuthenticationMechanismNameRequired,
    AuthenticationFailure,
    AuthenticationRequired,
    // From Me
    Other,
}

impl Default for AbortReason {
    fn default() -> Self {
        AbortReason::ReasonNotSpecified
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, serde::Serialize, serde::Deserialize)]
pub enum RejectReason {
    MistypedPDU,
    DuplicateInvokeIdRequest,
    UnsupportedOperationRequest,
    UnknownOperationRequest,
    MistypedArgumentRequest,
    ResourceLimitationRequest,
    UnknownInvokeIdResult,
    MistypedResultRequest,
    UnknownInvokeIdError,
    UnknownError,
    MistypedParameterError,
    UnsupportedIDMVersion,
    UnsuitableIDMVersion,
    InvalidIDMVersion,
    UnrecognizedPDU,
    BadlyStructuredPDU,
    ReleaseInProgress,
    Other,
}

impl Default for RejectReason {
    fn default() -> Self {
        RejectReason::MistypedPDU
    }
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct OidInfo {
    pub numeric: Vec<u32>,
    pub name: Option<String>,
}

pub type AttributeType = OidInfo;
pub type ContextType = OidInfo;

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct Atav {
    pub attr_type: AttributeType,
    pub value: Base64String,
}
pub type RDN = Vec<Atav>;
pub type RdnSequence = Vec<Vec<Atav>>;
pub type DistinguishedName = RdnSequence;

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum DirectoryName {
    RdnSequence(DistinguishedName),
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApiEdiPartyName {
    pub name_assigner: Option<String>,
    pub party_name: String,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum ApiGeneralName {
    Other((OidInfo, Base64String)),
    Rfc822Name(String),
    DnsName(String),
    X400Address(Base64String),
    DirectoryName(DirectoryName),
    EdiPartyName(ApiEdiPartyName),
    Uri(String),
    IpAddress(String),
    RegisteredID(OidInfo),
    Unrecognized(Base64String),
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct SubordinateInfo {
    pub dn: DistinguishedName,
    pub entry: bool,
    pub alias: bool,
    // pub performer: Option<String>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct Context {
    pub context_type: ContextType,
    pub binary: bool, // If true, values are base64.
    pub values: Vec<String>,
    pub fallback: bool,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct AttributeValue {
    pub value: String,
    pub contexts: Vec<Context>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct Attribute {
    pub attr_type: String,
    pub binary: bool, // If true, values are base64.
    pub values: Vec<AttributeValue>,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum EntryInfoItem {
    AttributeType(AttributeType),
    Attribute(Attribute),
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct EntryInformation {
    pub name: DirectoryName,
    pub from_entry: bool,
    pub info: Vec<EntryInfoItem>,
    pub incomplete: bool,
    pub partial_name: bool,
    pub derived: bool,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct PresentationAddress {
    pub p_selector: Option<Base64String>,
    pub s_selector: Option<Base64String>,
    pub t_selector: Option<Base64String>,
    pub n_addresses: Vec<Base64String>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ProtocolInformation {
    pub n_address: Base64String,
    pub profiles: Vec<Vec<OID_ARC>>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct AccessPointInfo {
    pub ae_title: String,
    pub address: PresentationAddress,
    pub protocol_information: Vec<ProtocolInformation>,
    pub category: MasterOrShadowAccessPoint_category,
    pub chaining_required: bool,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApiOperationProgress {
    pub name_resolution_phase: OperationProgress_nameResolutionPhase,
    pub next_rdn_to_be_resolved: Option<u16>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ContinuationRefInfo {
    pub target_object: String,
    pub aliased_rdns: Option<u16>,
    pub operation_progress: ApiOperationProgress,
    pub rdns_resolved: Option<u16>,
    pub ref_type: ReferenceType,
    pub access_points: Vec<AccessPointInfo>,
    pub entry_only: bool,
    pub exclusions: Vec<String>,
    pub return_to_dua: bool,
    pub name_resolve_on_master: bool,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum EntryCount {
    Best(u64),
    Low(u64),
    Exact(u64),
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct PoqInfo {
    pub limit_problem: Option<u16>,
    // pub unexplored
    pub unavailable_critical_extensions: bool,
    pub query_ref: Option<String>, // Base64-encoded.
    pub entry_count: Option<EntryCount>,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum NavTreeNodeContent {
    Test,
    Subordinate(SubordinateInfo),
    POQ(PoqInfo),
    ContinuationRef(ContinuationRefInfo),
    AccessPoint(AccessPointInfo),
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct NavTreeNode {
    pub text: String,
    pub content: NavTreeNodeContent,
}

impl NavTreeNode {

    pub fn new_test (text: &str) -> NavTreeNode {
        NavTreeNode {
            text: String::from(text),
            content: NavTreeNodeContent::Test,
        }
    }

}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ListArgument {
    pub session_id: String,
    pub object: DistinguishedName,
    pub family: bool,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ListResult {
    pub name: DistinguishedName,
    pub subordinates: Vec<NavTreeNode>,
    // pub signature_valid: bool,
    // pub signature_error: Option<Error>
    pub alias_dereferenced: bool,
    pub ldap_message: Option<String>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ContextAssertion {
    pub context_type: ContextType,
    pub binary: bool, // If true, values are base64.
    pub values: Vec<String>,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum AssertedContexts {
    All,
    Selected(Vec<ContextAssertion>),
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct AttributeValueAssertion {
    pub attr_type: String,
    pub assertion: String,
    pub binary: bool, // If true, assertion is base64.
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum ModifyRightType {
    Entry,
    Attribute(String),
    Value(AttributeValueAssertion),
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ModifyRight {
    pub right_type: ModifyRightType,
    pub add: bool,
    pub remove: bool,
    pub rename: bool,
    pub relocate: bool, // "move" is a Rust keyword.
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ReadResult {
    pub name: DistinguishedName,
    pub entry: EntryInformation,
    pub modify_rights: Vec<ModifyRight>,
    pub performer: Option<DistinguishedName>,
    pub alias_dereferenced: bool,
    pub ldap_message: Option<String>,
}

// Used for RemoveEntry, ChangePassword, and AdministerPassword.
#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ModificationOperationResult {
    pub performer: Option<DistinguishedName>,
    pub alias_dereferenced: bool,
    pub ldap_message: Option<String>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct DirectoryAttributeOrValueError {
    pub problem: u16, // Ignored for updateError.
    pub attr_type: AttributeType,
    pub binary: bool, // If true, value is Base64.
    pub value: Option<String>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct HierarchySelections {
    pub self_: bool,
    pub children: bool,
    pub parent: bool,
    pub hierarchy: bool,
    pub top: bool,
    pub subtree: bool,
    pub siblings: bool,
    pub siblingChildren: bool,
    pub siblingSubtree: bool,
    pub all: bool,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct SearchControlOptions {
    pub searchAliases: bool,
    pub matchedValuesOnly: bool,
    pub checkOverspecified: bool,
    pub performExactly: bool,
    pub includeAllAreas: bool,
    pub noSystemRelaxation: bool,
    pub dnAttribute: bool,
    pub matchOnResidualName: bool,
    pub entryCount: bool,
    pub useSubset: bool,
    pub separateFamilyMembers: bool,
    pub searchFamily: bool,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct ServiceControlOptions {
    pub preferChaining: bool,
    pub chainingProhibited: bool,
    pub localScope: bool,
    pub dontUseCopy: bool,
    pub dontDereferenceAliases: bool,
    pub subentries: bool,
    pub copyShallDo: bool,
    pub partialNameResolution: bool,
    pub manageDSAIT: bool,
    pub noSubtypeMatch: bool,
    pub noSubtypeSelection: bool,
    pub countFamily: bool,
    pub dontSelectFriends: bool,
    pub dontMatchFriends: bool,
    pub allowWriteableCopy: bool,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum PwdResponseWarning {
    TimeLeft(u32),
    GraceRemaining(u32),
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct PwdResponse {
    pub warning: Option<PwdResponseWarning>,
    pub error: Option<u8>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct DirectoryOperationError {
    pub ber_encoding: Option<Base64String>,
    pub err_code: u16,
    pub problem: u16,
    pub object: Option<DirectoryName>,
    pub operation: Option<InvokeIdInt>, // Used in abandonFailed
    pub candidate: Option<ContinuationRefInfo>, // Used in referral
    pub specifics: Vec<DirectoryAttributeOrValueError>,
    pub alias_dereferenced: bool,
    pub performer: Option<DistinguishedName>,

    // These come from notification attributes
    pub ldap_message: Option<String>,
    pub dsa_problem: Option<OidInfo>,
    pub search_service_problem: Option<OidInfo>,
    pub service_type: Option<OidInfo>,
    pub attribute_type_list: Vec<OidInfo>,
    pub matching_rule_list: Vec<OidInfo>,
    pub context_type_list: Vec<OidInfo>,
    pub hierarchy_select_list: Option<HierarchySelections>,
    pub service_control_options_list: Option<ServiceControlOptions>,
    pub search_control_options_list: Option<SearchControlOptions>,
    pub applied_relaxation: Option<OidInfo>,
    pub pwd_response: Option<PwdResponse>,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum BindErrorType {
    ServiceError(u16),
    SecurityError(u16),
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct BindError {
    pub v1: bool,
    pub v2: bool,
    pub error: BindErrorType,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum DirectoryClientError {
    // TODO: Make a struct for TLS data
    MalformedUrl,
    MalformedConfiguration,
    HostNameNotResolved,
    HostNotReachable,
    TransportAssociationRejected,
    SessionAssociationRejected,
    PresentationAssociationRejected,
    ApplicationAssociationRejected,
    TlsCannotBeEstablished,
    TlsVerificationFailed,
    TlsCertificateExpired,
    TlsCertificateRevoked,
    TlsClientCertificateRejected,
    TlsAccessDenied,
    TlsVersionNegotiationFailure,
    TlsClientCertificateRequired,
    SigningKeyNotFound,
    SigningCertsNotFound,
    UnrecognizedErrorReceived,
    MalformedResponse,
    BindError(BindError),
    OperationError(DirectoryOperationError),
    Rejection(RejectReason),
    Abort(AbortReason),
    UnbindError,
    Unimplemented,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum TlsProtectionLevel {
    None,
    StartTls,
    Implicit,
}

#[derive(Clone, PartialEq, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub enum CredentialType {
    Anonymous,
    SimpleNoPassword,
    SimpleWithPassword,
    Strong,
    Spkm,
    Sasl(String),
    External,
    Unrecognized,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct BindArgument {
    pub url: String,
    pub dn: Option<DistinguishedName>,
    pub password: Option<String>,
    pub signing_cert_path: Option<String>,
    pub signing_key_path: Option<String>,
    pub attr_cert_path: Option<String>,
}

#[derive(Clone, PartialEq, Properties, Hash, Eq, serde::Deserialize, serde::Serialize)]
pub struct BindResult {
    pub version: u8,
    pub session_id: String,
    pub tls_protection: TlsProtectionLevel,
    pub mtls: bool,
    pub dsa_authenticated: bool,
    pub dsa_credential_type: CredentialType,
    pub pwd_response: Option<PwdResponse>,
    pub tls_cert_path: Option<Vec<Base64String>>,
    pub signing_cert_path: Option<Vec<Base64String>>,
    pub attr_cert_path: Option<Vec<Base64String>>,
    pub responding_ae_title: Option<ApiGeneralName>,
    pub responding_ap_invocation_identifier: Option<u32>,
    pub responding_ae_invocation_identifier: Option<u32>,
}

pub trait IntoApiAtav {

    fn serialize_x500_atav (&self, atav: &AttributeTypeAndValue) -> Result<Atav, std::fmt::Error>;

}

pub trait FromApiAtav {

    fn deserialize_x500_atav (&self, atav: &Atav) -> Result<AttributeTypeAndValue, std::fmt::Error>;

}

pub trait IntoApiRdn {

    fn serialize_x500_rdn (&self, rdn: &RelativeDistinguishedName) -> Result<RDN, std::fmt::Error>;

}

pub trait FromApiRdn {

    fn deserialize_x500_rdn (&self, rdn: &RDN) -> Result<RelativeDistinguishedName, std::fmt::Error>;

}

pub trait IntoApiRdnSequence {

    fn serialize_x500_rdns (&self, rdns: &X500RDNSequence) -> Result<RdnSequence, std::fmt::Error>;

}

pub trait FromApiRdnSequence {

    fn deserialize_x500_rdns (&self, rdns: &RdnSequence) -> Result<X500RDNSequence, std::fmt::Error>;

}

pub trait IntoApiDirectoryName {

    fn serialize_x500_name (&self, name: &Name) -> Result<DirectoryName, std::fmt::Error>;

}

pub trait FromApiDirectoryName {

    fn deserialize_x500_name (&self, name: &DirectoryName) -> Result<Name, std::fmt::Error>;

}

pub trait IntoApiGeneralName {

    fn serialize_x500_gen_name (&self, name: &GeneralName) -> Result<ApiGeneralName, std::fmt::Error>;

}

pub trait FromApiGeneralName {

    fn deserialize_x500_gen_name (&self, name: &ApiGeneralName) -> Result<GeneralName, std::fmt::Error>;

}

impl <T> IntoApiAtav for T
    where T: DisplayX500AttributeType + DisplayX500Value<X690Element> {

    fn serialize_x500_atav (&self, atav: &AttributeTypeAndValue) -> Result<Atav, std::fmt::Error> {
        let value: String = self
            .value_to_string(&atav.type_, &atav.value)
            .map_err(|_| std::fmt::Error)?
            .ok_or_else(|| self.unrecognized_value_to_string(&atav.value))
            .unwrap();
        Ok(Atav {
            attr_type: OidInfo {
                numeric: atav.type_.0.clone(),
                name: self.attr_type_to_name(&atav.type_),
            },
            value,
        })
    }

}

impl <T> FromApiAtav for T
    where T: ParseX500AttributeType + ParseX500Value<X690Element> {

    fn deserialize_x500_atav (&self, atav: &Atav) -> Result<AttributeTypeAndValue, std::fmt::Error> {
        let attr_type: OBJECT_IDENTIFIER = OBJECT_IDENTIFIER::new(&atav.attr_type.numeric);
        let value: X690Element = self
            .parse_value(&attr_type, &atav.value)?
            .ok_or::<std::fmt::Error>(std::fmt::Error)?;
        Ok(AttributeTypeAndValue {
            type_: attr_type,
            value,
            _unrecognized: vec![],
        })
    }

}

impl <T: FromApiAtav> FromApiRdn for T {

    fn deserialize_x500_rdn (&self, rdn: &RDN) -> Result<RelativeDistinguishedName, std::fmt::Error> {
        let mut ret = Vec::new();
        for atav in rdn {
            ret.push(self.deserialize_x500_atav(&atav)?);
        }
        Ok(ret)
    }

}

impl <T: IntoApiAtav> IntoApiRdn for T {

    fn serialize_x500_rdn (&self, rdn: &RelativeDistinguishedName) -> Result<RDN, std::fmt::Error> {
        let mut ret = Vec::new();
        for atav in rdn {
            ret.push(self.serialize_x500_atav(&atav)?);
        }
        Ok(ret)
    }

}


impl <T: FromApiRdn> FromApiRdnSequence for T {

    fn deserialize_x500_rdns (&self, rdns: &RdnSequence) -> Result<X500RDNSequence, std::fmt::Error> {
        let mut ret = Vec::new();
        for rdn in rdns {
            ret.push(self.deserialize_x500_rdn(&rdn)?);
        }
        Ok(ret)
    }

}

impl <T: IntoApiRdn> IntoApiRdnSequence for T {

    fn serialize_x500_rdns (&self, rdns: &X500RDNSequence) -> Result<RdnSequence, std::fmt::Error> {
        let mut ret = Vec::new();
        for rdn in rdns {
            ret.push(self.serialize_x500_rdn(&rdn)?);
        }
        Ok(ret)
    }

}

impl <T: FromApiRdnSequence> FromApiDirectoryName for T {

    fn deserialize_x500_name (&self, name: &DirectoryName) -> Result<Name, std::fmt::Error> {
        match name {
            DirectoryName::RdnSequence(rdns) => Ok(Name::rdnSequence(self.deserialize_x500_rdns(rdns)?)),
        }
    }

}

impl <T: IntoApiRdnSequence> IntoApiDirectoryName for T {

    fn serialize_x500_name (&self, name: &Name) -> Result<DirectoryName, std::fmt::Error> {
        match name {
            Name::rdnSequence(rdns) => Ok(DirectoryName::RdnSequence(self.serialize_x500_rdns(rdns)?)),
        }
    }

}

impl <T> FromApiGeneralName for T
    where T: FromApiDirectoryName {

    fn deserialize_x500_gen_name (&self, name: &ApiGeneralName) -> Result<GeneralName, std::fmt::Error> {
        match name {
            // ApiGeneralName::Other(g) => Err(std::fmt::Error),
            ApiGeneralName::Rfc822Name(g) => Ok(GeneralName::rfc822Name(g.clone())),
            ApiGeneralName::DnsName(g) => Ok(GeneralName::dNSName(g.clone())),
            // ApiGeneralName::X400Address(g) => {},
            ApiGeneralName::DirectoryName(g) => Ok(GeneralName::directoryName(self.deserialize_x500_name(&g)?)),
            ApiGeneralName::EdiPartyName(g) => Ok(GeneralName::ediPartyName(EDIPartyName{
                nameAssigner: g.name_assigner.as_ref().map(|s| UnboundedDirectoryString::uTF8String(s.clone())),
                partyName: UnboundedDirectoryString::uTF8String(g.party_name.clone()),
                _unrecognized: vec![],
            })),
            ApiGeneralName::Uri(g) => Ok(GeneralName::uniformResourceIdentifier(g.clone())),
            ApiGeneralName::IpAddress(g) => {
                let addr_or_not = g.to_socket_addrs().map_err(|_| std::fmt::Error)?.next();
                if addr_or_not.is_none() {
                    return Err(std::fmt::Error);
                }
                match &addr_or_not.unwrap() {
                    SocketAddr::V4(v4addr) => Ok(GeneralName::iPAddress(Vec::from(v4addr.ip().octets()))),
                    SocketAddr::V6(v6addr) => Ok(GeneralName::iPAddress(Vec::from(v6addr.ip().octets()))),
                }
            },
            ApiGeneralName::RegisteredID(g) => Ok(GeneralName::registeredID(OBJECT_IDENTIFIER::new(&g.numeric))),
            // ApiGeneralName::Unrecognized(g) => {},
            _ => Err(std::fmt::Error),
        }
    }

}

impl <T> IntoApiGeneralName for T
    where T: IntoApiDirectoryName + ObjectIdentifierIntoDescriptor {

    fn serialize_x500_gen_name (&self, name: &GeneralName) -> Result<ApiGeneralName, std::fmt::Error> {
        match name {
            // GeneralName::otherName(g) => {},
            GeneralName::rfc822Name(g) => Ok(ApiGeneralName::Rfc822Name(g.clone())),
            GeneralName::dNSName(g) => Ok(ApiGeneralName::DnsName(g.clone())),
            // GeneralName::x400Address(g) => {},
            GeneralName::directoryName(g) => Ok(ApiGeneralName::DirectoryName(self.serialize_x500_name(&g)?)),
            GeneralName::ediPartyName(g) => Ok(ApiGeneralName::EdiPartyName(ApiEdiPartyName {
                name_assigner: g.nameAssigner.as_ref().map(|n| n.to_string()),
                party_name: g.partyName.to_string(),
            })),
            GeneralName::uniformResourceIdentifier(g) => Ok(ApiGeneralName::Uri(g.clone())),
            GeneralName::iPAddress(g) => {
                if g.len() == 4 {
                    Ok(ApiGeneralName::IpAddress(format!("{}.{}.{}.{}", g[0], g[1], g[2], g[3]).to_string()))
                }
                else if g.len() == 16 {
                    Ok(ApiGeneralName::IpAddress(format!(
                        "{}:{}:{}:{}:{}:{}:{}:{}",
                        &hex::encode(&g[0..=1]),
                        &hex::encode(&g[2..=3]),
                        &hex::encode(&g[4..=5]),
                        &hex::encode(&g[6..=7]),
                        &hex::encode(&g[8..=9]),
                        &hex::encode(&g[10..=11]),
                        &hex::encode(&g[12..=13]),
                        &hex::encode(&g[14..=15]),
                    ).to_string()))
                }
                else {
                    Err(std::fmt::Error)
                }
            },
            GeneralName::registeredID(g) => Ok(ApiGeneralName::RegisteredID(OidInfo {
                numeric: g.0.clone(),
                name: self.get_oid_descriptor(&g),
            })),
            // GeneralName::_unrecognized(g) => {},
            _ => Err(std::fmt::Error),
        }
    }

}
