use crate::{
    types::{
        ExternalEncoding, ExternalIdentification, PresentationContextSwitchingTypeIdentification,
    },
    ObjectDescriptor, CHARACTER_STRING, EMBEDDED_PDV, EXTERNAL, OCTET_STRING, OPTIONAL,
};

impl EXTERNAL {
    pub fn new(
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
    pub fn new(
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
    pub fn new(
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
