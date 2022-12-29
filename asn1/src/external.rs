use crate::types::{
    ExternalIdentification,
    PresentationContextSwitchingTypeIdentification,
};

impl TryFrom<PresentationContextSwitchingTypeIdentification> for ExternalIdentification {
    type Error = ();

    fn try_from(value: PresentationContextSwitchingTypeIdentification) -> Result<Self, Self::Error> {
        match value {
            PresentationContextSwitchingTypeIdentification::fixed => Err(()),
            PresentationContextSwitchingTypeIdentification::transfer_syntax(_) => Err(()),
            PresentationContextSwitchingTypeIdentification::syntaxes(_) => Err(()),
            PresentationContextSwitchingTypeIdentification::context_negotiation(cn) => Ok(ExternalIdentification::context_negotiation(cn)),
            PresentationContextSwitchingTypeIdentification::presentation_context_id(pcid) => Ok(ExternalIdentification::presentation_context_id(pcid)),
            PresentationContextSwitchingTypeIdentification::syntax(syn) => Ok(ExternalIdentification::syntax(syn)),
        }
    }

}

impl From<ExternalIdentification> for PresentationContextSwitchingTypeIdentification {

    fn from(value: ExternalIdentification) -> Self {
        match value {
            ExternalIdentification::syntax(syn) => PresentationContextSwitchingTypeIdentification::syntax(syn),
            ExternalIdentification::presentation_context_id(pcid) => PresentationContextSwitchingTypeIdentification::presentation_context_id(pcid),
            ExternalIdentification::context_negotiation(cn) => PresentationContextSwitchingTypeIdentification::context_negotiation(cn),
        }
    }

}
