//! Context-switching types: `EXTERNAL`, `EMBEDDED PDV`, and `CharacterString`
use crate::display::write_int;
use crate::TagClass;
use crate::{
    INSTANCE_OF, INTEGER, ASN1Value,
    write_hex, ObjectDescriptor, CHARACTER_STRING, EMBEDDED_PDV, EXTERNAL, OCTET_STRING, OPTIONAL,
};
use crate::oid::OBJECT_IDENTIFIER;
use crate::bitstring::BIT_STRING;
use crate::construction::{ComponentSpec, TagSelector};
use std::fmt::Display;
use std::sync::Arc;

/// A pair of syntaxes: an abstract and a transfer (concrete) syntax. Used for
/// ASN.1 context-switching types.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct IdentificationSyntaxes {
    /// The abstract syntax identifier
    pub r#abstract: OBJECT_IDENTIFIER,

    /// The transfer syntax identifier
    pub transfer: OBJECT_IDENTIFIER,
}

impl IdentificationSyntaxes {

    /// Create a new syntaxes pair
    #[inline]
    pub const fn new(r#abstract: OBJECT_IDENTIFIER, transfer: OBJECT_IDENTIFIER) -> Self {
        IdentificationSyntaxes {
            r#abstract,
            transfer,
        }
    }
}

/// An association between a presentation context identifier and a transfer
/// syntax.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ContextNegotiation {
    /// The presentation context identifier. See ITU-T Recommendation X.226.
    pub presentation_context_id: INTEGER,
    /// The transfer syntax identifier
    pub transfer_syntax: OBJECT_IDENTIFIER,
}

impl ContextNegotiation {

    /// Construct a new [ContextNegotiation]
    #[inline]
    pub const fn new(presentation_context_id: INTEGER, transfer_syntax: OBJECT_IDENTIFIER) -> Self {
        ContextNegotiation {
            presentation_context_id,
            transfer_syntax,
        }
    }
}

/// Presentation syntax identification for an `EXTERNAL` value.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExternalIdentification {
    // syntaxes                (IdentificationSyntaxes),

    /// A single transfer syntax
    syntax(OBJECT_IDENTIFIER),

    /// A context ID negotiated at the presentation layer
    presentation_context_id(INTEGER),

    /// A binding between a context ID and a transfer syntax
    context_negotiation(ContextNegotiation),
    // transfer_syntax         (OBJECT_IDENTIFIER),
    // fixed,
}

/// Presentation value within an `EXTERNAL` value.
/// See ITU Recommendation X.690, Section 8.18.
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalEncoding<Asn1Type: Sized = Arc<ASN1Value>> {

    /// Any ASN.1 value
    single_ASN1_type(Asn1Type),

    /// A byte-aligned value
    octet_aligned(OCTET_STRING),

    /// A non-byte-aligned value
    arbitrary(BIT_STRING),
}

/// ASN.1 `EXTERNAL` value
#[derive(Debug, Clone, PartialEq)]
pub struct External<Asn1Type: Sized = Arc<ASN1Value>> {

    /// Identifies the type of encoding used in `data_value`
    pub identification: ExternalIdentification,

    /// A human-readable description of the encoding used in `data_value`
    pub data_value_descriptor: OPTIONAL<ObjectDescriptor>,

    /// The presentation data value itself
    pub data_value: ExternalEncoding<Asn1Type>,
}

// TODO: This name is just too dang long.
/// Presentation syntax identification for an `EMBEDDED PDV` or
/// `CharacterString` value
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum PresentationContextSwitchingTypeIdentification {
    /// An abstract syntax identifier and a transfer syntax identifier
    syntaxes(IdentificationSyntaxes),
    /// A single transfer syntax identifier
    syntax(OBJECT_IDENTIFIER),
    /// A context ID negotiated at the presentation layer
    presentation_context_id(INTEGER),
    /// A binding between a context ID and a transfer syntax
    context_negotiation(ContextNegotiation),
    /// A single transfer syntax identifier
    transfer_syntax(OBJECT_IDENTIFIER),
    /// A contextually-fixed syntax
    fixed,
}

/// An ASN.1 `EMBEDDED PDV` value
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct EmbeddedPDV {

    /// Identifies the type of encoding used in `data_value`
    pub identification: PresentationContextSwitchingTypeIdentification,

    // pub data_value_descriptor: ObjectDescriptor: required to be ABSENT via constraints

    /// The presentation data value itself
    pub data_value: OCTET_STRING,
}

/// An ASN.1 `CharacterString` value
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct CharacterString {

    /// Identifies the type of encoding used in `string_value`
    pub identification: PresentationContextSwitchingTypeIdentification,

    /// The presentation data value itself
    pub string_value: OCTET_STRING,
}

/// ASN.1 `INSTANCE OF` value
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceOf<Asn1Type: Sized = Arc<ASN1Value>> {
    /// The `type-id` field
    pub type_id: OBJECT_IDENTIFIER,
    /// The `value` field
    pub value: Asn1Type,
}

impl <Asn1Type> InstanceOf<Asn1Type> where Asn1Type: Sized {

    /// Construct a new `INSTANCE OF` value
    #[inline]
    pub const fn new(type_id: OBJECT_IDENTIFIER, value: Asn1Type) -> Self {
        InstanceOf { type_id, value }
    }
}

impl EXTERNAL {

    /// Construct a new `EXTERNAL` value
    #[inline]
    pub const fn new(
        identification: ExternalIdentification,
        data_value_descriptor: OPTIONAL<ObjectDescriptor>,
        data_value: ExternalEncoding,
    ) -> EXTERNAL {
        EXTERNAL {
            identification,
            data_value_descriptor,
            data_value,
        }
    }
}

impl EMBEDDED_PDV {

    /// Construct a new `EMBEDDED PDV` value
    #[inline]
    pub const fn new(
        identification: PresentationContextSwitchingTypeIdentification,
        data_value: OCTET_STRING,
    ) -> EMBEDDED_PDV {
        EMBEDDED_PDV {
            identification,
            data_value,
        }
    }
}

impl CHARACTER_STRING {

    /// Construct a new `CharacterString` value
    #[inline]
    pub const fn new(
        identification: PresentationContextSwitchingTypeIdentification,
        string_value: OCTET_STRING,
    ) -> CHARACTER_STRING {
        CHARACTER_STRING {
            identification,
            string_value,
        }
    }
}

impl TryFrom<PresentationContextSwitchingTypeIdentification> for ExternalIdentification {
    type Error = ();

    fn try_from(
        value: PresentationContextSwitchingTypeIdentification,
    ) -> Result<Self, Self::Error> {
        match value {
            PresentationContextSwitchingTypeIdentification::fixed => Err(()),
            PresentationContextSwitchingTypeIdentification::transfer_syntax(_) => Err(()),
            PresentationContextSwitchingTypeIdentification::syntaxes(_) => Err(()),
            PresentationContextSwitchingTypeIdentification::context_negotiation(cn) => {
                Ok(ExternalIdentification::context_negotiation(cn))
            }
            PresentationContextSwitchingTypeIdentification::presentation_context_id(pcid) => {
                Ok(ExternalIdentification::presentation_context_id(pcid))
            }
            PresentationContextSwitchingTypeIdentification::syntax(syn) => {
                Ok(ExternalIdentification::syntax(syn))
            }
        }
    }
}

impl From<ExternalIdentification> for PresentationContextSwitchingTypeIdentification {
    fn from(value: ExternalIdentification) -> Self {
        match value {
            ExternalIdentification::syntax(syn) => {
                PresentationContextSwitchingTypeIdentification::syntax(syn)
            }
            ExternalIdentification::presentation_context_id(pcid) => {
                PresentationContextSwitchingTypeIdentification::presentation_context_id(pcid)
            }
            ExternalIdentification::context_negotiation(cn) => {
                PresentationContextSwitchingTypeIdentification::context_negotiation(cn)
            }
        }
    }
}

impl From<INSTANCE_OF> for EXTERNAL {
    #[inline]
    fn from(value: INSTANCE_OF) -> Self {
        EXTERNAL {
            identification: ExternalIdentification::syntax(value.type_id),
            data_value_descriptor: None,
            data_value: ExternalEncoding::single_ASN1_type(value.value.clone()),
        }
    }
}

impl Display for ExternalEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalEncoding::single_ASN1_type(v) => write!(f, "single-ASN1-type:{}", v),
            ExternalEncoding::octet_aligned(v) => {
                f.write_str("octet-aligned:")?;
                write_hex(v, f)
            }
            ExternalEncoding::arbitrary(v) => write!(f, "arbitrary:{}", v),
        }
    }
}

impl Display for ExternalIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalIdentification::context_negotiation(cn) => write!(f, "context-negotiation:{}", cn),
            ExternalIdentification::presentation_context_id(pcid) => {
                f.write_str("presentation-context-id:")?;
                write_int(&pcid, f)
            }
            ExternalIdentification::syntax(syn) => write!(f, "syntax:{}", syn.to_asn1_string()),
        }
    }
}

impl Display for PresentationContextSwitchingTypeIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PresentationContextSwitchingTypeIdentification::context_negotiation(cn) => write!(f, "context-negotiation:{}", cn),
            PresentationContextSwitchingTypeIdentification::presentation_context_id(pcid) => {
                f.write_str("presentation-context-id:")?;
                write_int(&pcid, f)
            }
            PresentationContextSwitchingTypeIdentification::syntax(syn) => write!(f, "syntax:{}", syn.to_asn1_string()),
            PresentationContextSwitchingTypeIdentification::syntaxes(syn) => write!(f, "syntaxes:{}", syn),
            PresentationContextSwitchingTypeIdentification::transfer_syntax(syn) => write!(f, "transfer-syntax:{}", syn.to_asn1_string()),
            PresentationContextSwitchingTypeIdentification::fixed => f.write_str("fixed:NULL"),
        }
    }
}

impl Display for ContextNegotiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{ presentation-context-id ")?;
        write_int(&self.presentation_context_id, f)?;
        write!(f, ", transfer-syntax {} }}", self.transfer_syntax.to_asn1_string())
    }
}

impl Display for IdentificationSyntaxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ abstract {}, transfer {} }}",
            self.r#abstract.to_asn1_string(),
            self.transfer.to_asn1_string(),
        )
    }
}

impl Display for EXTERNAL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(dvd) = &self.data_value_descriptor {
            write!(f,
                "{{ identification {}, data-value-descriptor \"{}\", data-value {} }}",
                self.identification, dvd, self.data_value,
            )
        } else {
            write!(f,
                "{{ identification {}, data-value {} }}",
                self.identification, self.data_value,
            )
        }
    }
}

impl Display for EMBEDDED_PDV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ identification {}, data-value '", self.identification)?;
        write_hex(&self.data_value, f)?;
        f.write_str("'H }")
    }
}

impl Display for CHARACTER_STRING {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ identification {}, string-value '", self.identification)?;
        write_hex(&self.string_value, f)?;
        f.write_str("'H }")
    }
}

/// The RootComponentTypeList #1 Components for `EXTERNAL`
pub const _rctl1_components_for_EXTERNAL: &[ComponentSpec; 3] = &[
    ComponentSpec::req("identification", TagSelector::tag((TagClass::CONTEXT, 0))),
    ComponentSpec::opt("data-value-descriptor", TagSelector::tag((TagClass::CONTEXT, 1))),
    ComponentSpec::req("data-value", TagSelector::tag((TagClass::CONTEXT, 2))),
];

/// The RootComponentTypeList #2 Components for `EXTERNAL`
pub const _rctl2_components_for_EXTERNAL: &[ComponentSpec; 0] = &[];

/// The ExtensionAdditionList Components for `EXTERNAL`
pub const _eal_components_for_EXTERNAL: &[ComponentSpec; 0] = &[];

/// The RootComponentTypeList #1 Components for `EMBEDDED PDV`
pub const _rctl1_components_for_EMBEDDED_PDV: &[ComponentSpec; 3] = _rctl1_components_for_EXTERNAL;

/// The RootComponentTypeList #2 Components for `EMBEDDED PDV`
pub const _rctl2_components_for_EMBEDDED_PDV: &[ComponentSpec; 0] = _rctl2_components_for_EXTERNAL;

/// The ExtensionAdditionList Components for `EMBEDDED PDV`
pub const _eal_components_for_EMBEDDED_PDV: &[ComponentSpec; 0] = _eal_components_for_EXTERNAL;

/// The RootComponentTypeList #1 Components for `CharacterString`
pub const _rctl1_components_for_CharacterString: &[ComponentSpec; 2] = &[
    ComponentSpec::req("identification", TagSelector::tag((TagClass::CONTEXT, 0))),
    ComponentSpec::req("string-value", TagSelector::tag((TagClass::CONTEXT, 2))),
];

/// The RootComponentTypeList #2 Components for `CharacterString`
pub const _rctl2_components_for_CharacterString: &[ComponentSpec; 0] = &[];

/// The ExtensionAdditionList Components for `CharacterString`
pub const _eal_components_for_CharacterString: &[ComponentSpec; 0] = &[];
