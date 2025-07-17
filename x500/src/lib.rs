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
pub mod PKI_Stub;
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

pub mod ToString;
pub mod types;

#[cfg(test)]
mod tests {
    use super::DirectoryAbstractService::{DirectoryBindArgument, _encode_DirectoryBindArgument};
    use super::DirectoryIDMProtocols::id_idm_dap;
    use super::IDMProtocolSpecification::{IdmBind, _encode_IDM_PDU, IDM_PDU};
    use wildboar_asn1::{UNIV_TAG_OBJECT_IDENTIFIER, UNIV_TAG_SEQUENCE};
    use x690::{x690_write_tlv, X690_TAG_CLASS_CONTEXT};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

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
}

// bind         [0]  IdmBind{{protocol}},
