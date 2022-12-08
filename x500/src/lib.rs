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
pub mod MTSAbstractService;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
