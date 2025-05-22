use crate::display::write_int;
use crate::{
    types::{
        ContextNegotiation, ExternalEncoding, ExternalIdentification, IdentificationSyntaxes,
        PresentationContextSwitchingTypeIdentification, INSTANCE_OF,
    },
    write_hex, ObjectDescriptor, CHARACTER_STRING, EMBEDDED_PDV, EXTERNAL, OCTET_STRING, OPTIONAL,
};
use std::fmt::Display;

impl EXTERNAL {
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

// TODO: Define component specs for this.
