use crate::InformationFramework::{RDNSequence, RelativeDistinguishedName, AttributeType};

pub mod AVL_management;
pub mod AlgorithmObjectIdentifiers;
pub mod AttributeCertificateDefinitions;
pub mod AuthenticationFramework;
pub mod BasicAccessControl;
pub mod CaSubscription;
pub mod CertificateExtensions;
pub mod CommonProtocolSpecification;
pub mod CryptoTools;
pub mod DSAOperationalAttributeTypes;
pub mod DirectoryAbstractService;
pub mod DirectoryIDMProtocols;
pub mod DirectoryManagement;
pub mod DirectoryOSIProtocols;
pub mod DirectoryOperationalBindingTypes;
pub mod DirectoryShadowAbstractService;
pub mod DistributedOperations;
pub mod EnhancedSecurity;
pub mod ExtensionAttributes;
pub mod GenAlgo;
pub mod HierarchicalOperationalBindings;
pub mod IDMProtocolSpecification;
pub mod InformationFramework;
pub mod LdapSystemSchema;
// pub mod Lightweight-Directory-Access-Protocol-V3;
pub mod OSIProtocolSpecification;
pub mod OperationalBindingManagement;
pub mod PKIX1Implicit93;
pub mod PasswordPolicy;
pub mod PkiPMIProtocolSpecifications;
pub mod PkiPmiExternalDataTypes;
pub mod PkiPmiWrapper;
pub mod ProtProtocols;
pub mod ProtocolObjectIdentifiers;
pub mod SchemaAdministration;
pub mod SelectedAttributeTypes;
pub mod SelectedObjectClasses;
pub mod ServiceAdministration;
pub mod SpkmGssTokens;
pub mod TrustBroker;
pub mod UpperBounds;
pub mod UsefulDefinitions;
pub mod Wrapper;

pub mod bac;
pub mod dap;

use std::collections::HashSet;

pub fn check_name_form(
    rdn: &RelativeDistinguishedName,
    mandatories: &[AttributeType],
    optionals: &[AttributeType],
) -> bool {
    let checks_count = mandatories.len() + optionals.len();
    if rdn.len() < mandatories.len() || checks_count < rdn.len() {
        return false;
    }
    // For small names, it's faster to iterate over the RDNs than to allocate a set.
    if mandatories.len() < 4 && rdn.len() < 4 {
        for m in mandatories {
            let matched = rdn.iter().any(|atav| atav.type_ == *m);
            if !matched {
                return false;
            }
        }
    } else { // If the name is fairly large, accept the overhead of allocating a set.
        let mut unsatisfied_mandatories = HashSet::new();
        for mandatory in mandatories {
            unsatisfied_mandatories.insert(mandatory);
        }
        for atav in rdn {
            unsatisfied_mandatories.remove(&atav.type_);
        }
        if !unsatisfied_mandatories.is_empty() {
            return false;
        }
    }

    // Again, for small names, it's faster to iterate over the RDNs than to allocate a set.
    if mandatories.len() < 4 && optionals.len() < 4 && rdn.len() < 4 {
        for atav in rdn {
            let mut all_checks = mandatories
                .iter()
                .chain(optionals.iter());
            if !all_checks.any(|c| c == &atav.type_) {
                return false;
            }
        }
        return true;
    }
    let mut permitted_types = HashSet::new();
    for mandatory in mandatories {
        permitted_types.insert(mandatory);
    }
    for optional in optionals {
        permitted_types.insert(optional);
    }
    for atav in rdn {
        if !permitted_types.contains(&atav.type_) {
            return false;
        }
    }
    true
}

/// Get the RDN from a distinguished name
///
/// This function mostly exists for clarification: in X.500, the DN is ordered
/// from the root to the leaf: the opposite of LDAP.
#[inline(always)]
pub fn get_rdn<'a>(dn: &'a RDNSequence) -> Option<&'a RelativeDistinguishedName> {
    dn.last()
}

pub(crate) mod internal_macros {

    macro_rules! filter_impl {
        ($me:ident, $depth:ident, $rl:ident, $fnname:ident, $x:ident, $eval:expr) => {
            {
                if $depth > $rl {
                    return None;
                }
                match ($me) {
                    Self::and(subs) => {
                        for sub in subs {
                            let is_match = sub
                                .$fnname($x, $depth + 1, $rl)?;
                            if !is_match {
                                return Some(false);
                            }
                        }
                        Some(true)
                    },
                    Self::or(subs) => {
                        let mut unknown = false;
                        for sub in subs {
                            let is_match = sub
                                .$fnname($x, $depth + 1, $rl);
                            if is_match.is_none() {
                                unknown = true;
                                continue;
                            }
                            if is_match.is_some_and(|x| x) {
                                return Some(true);
                            }
                        }
                        if unknown {
                            // None of the understood subfilters matched, but
                            return None;
                        }
                        Some(false)
                    },
                    Self::not(sub) => sub.$fnname($x, $depth + 1, $rl).map(|s| !s),
                    // $t::item(r) => Some($x.iter().any(|o| o == r)),
                    Self::item(b) => Some($eval(b)),
                    Self::_unrecognized(_) => None,
                }
            }
        };
    }

    pub(crate) use filter_impl;

}

#[cfg(test)]
mod tests {
    use super::DirectoryAbstractService::{DirectoryBindArgument, _encode_DirectoryBindArgument};
    use super::DirectoryIDMProtocols::id_idm_dap;
    use super::IDMProtocolSpecification::{IdmBind, _encode_IDM_PDU, IDM_PDU};
    use wildboar_asn1::{UNIV_TAG_OBJECT_IDENTIFIER, UNIV_TAG_SEQUENCE, Tag, TagClass, UNIV_TAG_UTF8_STRING};
    use x690::{x690_write_tlv, X690_TAG_CLASS_CONTEXT, X690Element, X690Value};
    use super::InformationFramework::{AttributeTypeAndValue, RelativeDistinguishedName};
    use super::SelectedAttributeTypes::{id_at_organizationIdentifier, id_at_organizationName};
    use super::check_name_form;
    use bytes::Bytes;

    #[test]
    fn encodes_an_idm_bind_correctly() {
        let bind_arg =
            _encode_DirectoryBindArgument(&DirectoryBindArgument::new(None, None, vec![])).unwrap();
        let idm_pdu = IDM_PDU::bind(IdmBind::new(id_idm_dap(), None, None, bind_arg, vec![]));
        let element = _encode_IDM_PDU(&idm_pdu).unwrap();
        let mut output: Vec<u8> = Vec::new();
        let bytes_written = x690_write_tlv(&mut output, &element).unwrap();

        assert_eq!(bytes_written, 13);
        assert_eq!(output.len(), 13);
        assert!(output.starts_with(&[
            X690_TAG_CLASS_CONTEXT
            | 0b0010_0000 // Constructed
            | 0,
            0x0B, // Length = 1
            0b0010_0000 // Constructed
            | UNIV_TAG_SEQUENCE as u8,
            0x09, // Length = 9
            UNIV_TAG_OBJECT_IDENTIFIER as u8,
            0x03, // Length = 3
            0x55,
            0x21,
            0x00, // 2.5.33.0 (id-dap-ip)
            X690_TAG_CLASS_CONTEXT
            | 0b0010_0000 // Constructed
            | 2,
            0x02, // Length 2
            0x31,
            0x00, // Empty DirectoryBindArgument.
        ]));
    }


    #[test]
    fn checks_name_form_correctly() {
        let rdn: RelativeDistinguishedName = vec![
            AttributeTypeAndValue::new(
                id_at_organizationIdentifier(),
                X690Element::new(
                    Tag::new(TagClass::UNIVERSAL, UNIV_TAG_UTF8_STRING),
                    X690Value::Primitive(Bytes::from_static(b"Wildboar Software")),
                ),
                vec![],
            )
        ];
        assert!(check_name_form(&rdn, &[
            id_at_organizationIdentifier(),
        ], &[id_at_organizationName()]));
    }
}

// bind         [0]  IdmBind{{protocol}},
