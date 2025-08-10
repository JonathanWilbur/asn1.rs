use wildboar_asn1::OBJECT_IDENTIFIER;

pub fn common_attr_type_to_long_name(attr_type: &OBJECT_IDENTIFIER) -> Option<&'static str> {
    let x690_slice = attr_type.as_x690_slice();
    let x690_len = x690_slice.len();
    if x690_len == 3 && x690_slice.starts_with(&[0x55, 4]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            3 => Some("commonName"),
            4 => Some("surname"),
            5 => Some("serialNumber"),
            6 => Some("countryName"),
            7 => Some("localityName"),
            8 => Some("stateOrProvinceName"),
            9 => Some("streetAddress"),
            10 => Some("organizationName"),
            11 => Some("organizationalUnitName"),
            12 => Some("title"),
            13 => Some("description"),
            14 => Some("searchGuide"),
            15 => Some("businessCategory"),
            16 => Some("postalAddress"),
            17 => Some("postalCode"),
            18 => Some("postOfficeBox"),
            19 => Some("physicalDeliveryOfficeName"),
            20 => Some("telephoneNumber"),
            21 => Some("telexNumber"),
            23 => Some("facsimileTelephoneNumber"),
            24 => Some("x121Address"),
            25 => Some("internationalISDNNumber"),
            26 => Some("registeredAddress"),
            27 => Some("destinationIndicator"),
            28 => Some("preferredDeliveryMethod"),
            29 => Some("presentationAddress"),
            30 => Some("supportedApplicationContext"),
            31 => Some("member"),
            32 => Some("owner"),
            33 => Some("roleOccupant"),
            34 => Some("seeAlso"),
            41 => Some("name"),
            42 => Some("givenName"),
            43 => Some("initials"),
            44 => Some("generationQualifier"),
            45 => Some("uniqueIdentifier"),
            46 => Some("dnQualifier"),
            47 => Some("enhancedSearchGuide"),
            48 => Some("protocolInformation"),
            49 => Some("distinguishedName"),
            50 => Some("uniqueMember"),
            51 => Some("houseIdentifier"),
            54 => Some("dmdName"),
            65 => Some("pseudonym"),
            66 => Some("communicationsService"),
            67 => Some("communicationsNetwork"),
            77 => Some("uuidpair"),
            78 => Some("tagOid"),
            79 => Some("uiiFormat"),
            80 => Some("uiiInUrn"),
            81 => Some("contentUrl"),
            83 => Some("uri"),
            86 => Some("urn"),
            87 => Some("url"),
            88 => Some("utmCoordinates"),
            89 => Some("urnC"),
            90 => Some("uii"),
            91 => Some("epc"),
            92 => Some("tagAfi"),
            93 => Some("epcFormat"),
            94 => Some("epcInUrn"),
            95 => Some("ldapUrl"),
            96 => Some("tagLocation"),
            97 => Some("organizationIdentifier"),
            98 => Some("countryCode3c"),
            99 => Some("countryCode3n"),
            100 => Some("dnsName"),
            104 => Some("intEmail"),
            105 => Some("jid"),
            106 => Some("objectIdentifier"),
            _ => return None,
        };
        // TODO: More attributes
    }

    // Always enabled because it has some important attributes used in naming.
    // 0.9.2342.19200300.100.1. is 0x0992268993F22C6401
    if x690_len == 10
        && x690_slice.starts_with(&[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 1])
    {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("userid"),
            2 => Some("textEncodedORAddress"),
            3 => Some("rfc822mailbox"),
            4 => Some("info"),
            5 => Some("favouriteDrink"),
            6 => Some("roomNumber"),
            7 => Some("photo"),
            8 => Some("userClass"),
            9 => Some("host"),
            10 => Some("manager"),
            11 => Some("documentIdentifier"),
            12 => Some("documentTitle"),
            13 => Some("documentVersion"),
            14 => Some("documentAuthor"),
            15 => Some("documentLocation"),
            20 => Some("homePhone"),
            21 => Some("secretary"),
            22 => Some("otherMailbox"),
            23 => Some("lastModifiedTime"),
            24 => Some("lastModifiedBy"),
            25 => Some("domainComponent"),
            26 => Some("aRecord"),
            27 => Some("mDRecord"),
            28 => Some("mXRecord"),
            29 => Some("nSRecord"),
            30 => Some("sOARecord"),
            31 => Some("cNAMERecord"),
            37 => Some("associatedDomain"),
            38 => Some("associatedName"),
            39 => Some("homePostalAddress"),
            40 => Some("personalTitle"),
            41 => Some("mobileTelephoneNumber"),
            42 => Some("pagerTelephoneNumber"),
            43 => Some("friendlyCountryName"),
            44 => Some("uniqueIdentifier"),
            45 => Some("organizationalStatus"),
            46 => Some("janetMailbox"),
            47 => Some("mailPreferenceOption"),
            48 => Some("buildingName"),
            49 => Some("dSAQuality"),
            50 => Some("singleLevelQuality"),
            51 => Some("subtreeMinimumQuality"),
            52 => Some("subtreeMaximumQuality"),
            53 => Some("personalSignature"),
            54 => Some("dITRedirect"),
            55 => Some("audio"),
            56 => Some("documentPublisher"),
            60 => Some("jpegPhoto"),
            _ => return None,
        };
    }

    #[cfg(feature = "ldap_attrs")]
    if x690_len == 11 && x690_slice.starts_with(&[1, 2, 0x86, 0x48, 0x86, 0xF7, 0x14, 1, 4, 0x83]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            0x5E => Some("calCalURI"), // 478
            0x5F => Some("calFBURL"),  // 479
            0x60 => Some("calCAPURI"), // 480
            0x61 => Some("calCalAdrURI"),
            0x62 => Some("calOtherCalURIs"),
            0x63 => Some("calOtherFBURLs"),
            0x64 => Some("calOtherCAPURIs"),
            0x65 => Some("calOtherCalAdrURIs"),
            _ => None,
        };
    }

    #[cfg(feature = "ldap_attrs")]
    if x690_len == 9 && x690_slice.starts_with(&[1, 3, 18, 0, 2, 24, 46, 1]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            101 => Some("printer-device-id"),
            102 => Some("printer-device-service-count"),
            104 => Some("printer-uuid"),
            105 => Some("printer-charge-info"),
            106 => Some("printer-charge-info-uri"),
            107 => Some("printer-geo-location"),
            108 => Some("printer-ipp-features-supported"),
            _ => None,
        };
    }

    #[cfg(feature = "ldap_attrs")]
    if x690_len == 8 && x690_slice.starts_with(&[1, 3, 18, 0, 2, 4, 0x88]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            0x53 => Some("printer-xri-supported"), // 1.3.18.0.2.4.1107
            0x54 => Some("printer-aliases"),
            0x55 => Some("printer-charset-configured"),
            0x56 => Some("printer-job-priority-supported"),
            0x57 => Some("printer-job-k-octets-supported"),
            0x58 => Some("printer-current-operator"),
            0x59 => Some("printer-service-person"),
            0x5A => Some("printer-delivery-orientation-supported"),
            0x5B => Some("printer-stacking-order-supported"),
            0x5C => Some("printer-output-features-supported"),
            0x5D => Some("printer-media-local-supported"),
            0x5E => Some("printer-copies-supported"),
            0x5F => Some("printer-natural-language-configured"),
            0x60 => Some("printer-print-quality-supported"),
            0x61 => Some("printer-resolution-supported"),
            0x62 => Some("printer-media-supported"),
            0x63 => Some("printer-sides-supported"),
            0x64 => Some("printer-number-up-supported"),
            0x65 => Some("printer-finishings-supported"),
            0x66 => Some("printer-pages-per-minute-color"),
            0x67 => Some("printer-pages-per-minute"),
            0x68 => Some("printer-compression-supported"),
            0x69 => Some("printer-color-supported"),
            0x6A => Some("printer-document-format-supported"),
            0x6B => Some("printer-charset-supported"),
            0x6C => Some("printer-multiple-document-jobs-supported"),
            0x6D => Some("printer-ipp-versions-supported"),
            0x6E => Some("printer-more-info"),
            0x6F => Some("printer-name"),
            0x70 => Some("printer-location"),
            0x71 => Some("printer-generated-natural-language-supported"),
            0x72 => Some("printer-make-and-model"),
            0x73 => Some("printer-info"),
            0x74 => Some("printer-uri"),
            _ => None,
        };
    }

    #[cfg(feature = "ldap_attrs")]
    if x690_len == 8 && x690_slice.starts_with(&[1, 3, 6, 1, 1, 10, 4]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("uddiBusinessKey"),
            2 => Some("uddiAuthorizedName"),
            3 => Some("uddiOperator"),
            4 => Some("uddiName"),
            5 => Some("uddiDescription"),
            6 => Some("uddiDiscoveryURLs"),
            7 => Some("uddiUseType"),
            8 => Some("uddiPersonName"),
            9 => Some("uddiPhone"),
            10 => Some("uddiEMail"),
            11 => Some("uddiSortCode"),
            12 => Some("uddiTModelKey"),
            13 => Some("uddiAddressLine"),
            14 => Some("uddiIdentifierBag"),
            15 => Some("uddiCategoryBag"),
            16 => Some("uddiKeyedReference"),
            17 => Some("uddiServiceKey"),
            18 => Some("uddiBindingKey"),
            19 => Some("uddiAccessPoint"),
            20 => Some("uddiHostingRedirector"),
            21 => Some("uddiInstanceDescription"),
            22 => Some("uddiInstanceParms"),
            23 => Some("uddiOverviewDescription"),
            24 => Some("uddiOverviewURL"),
            25 => Some("uddiFromKey"),
            26 => Some("uddiToKey"),
            27 => Some("uddiUUID"),
            28 => Some("uddiIsHidden"),
            29 => Some("uddiIsProjection"),
            30 => Some("uddiLang"),
            31 => Some("uddiv3BusinessKey"),
            32 => Some("uddiv3ServiceKey"),
            33 => Some("uddiv3BindingKey"),
            34 => Some("uddiv3TmodelKey"),
            35 => Some("uddiv3DigitalSignature"),
            36 => Some("uddiv3NodeId"),
            37 => Some("uddiv3EntityModificationTime"),
            38 => Some("uddiv3SubscriptionKey"),
            39 => Some("uddiv3SubscriptionFilter"),
            40 => Some("uddiv3NotificationInterval"),
            41 => Some("uddiv3MaxEntities"),
            42 => Some("uddiv3ExpiresAfter"),
            43 => Some("uddiv3BriefResponse"),
            44 => Some("uddiv3EntityKey"),
            45 => Some("uddiv3EntityCreationTime"),
            46 => Some("uddiv3EntityDeletionTime"),
            _ => None,
        };
    }

    #[cfg(feature = "ldap_attrs")]
    if x690_len == 8 && x690_slice.starts_with(&[1, 3, 6, 1, 1, 11, 2]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("vPIMTelephoneNumber"),
            2 => Some("vPIMRfc822Mailbox"),
            3 => Some("vPIMSpokenName"),
            4 => Some("vPIMSupportedUABehaviors"),
            5 => Some("vPIMSupportedAudioMediaTypes"),
            6 => Some("vPIMSupportedMessageContext"),
            7 => Some("vPIMTextName"),
            8 => Some("vPIMExtendedAbsenceStatus"),
            9 => Some("vPIMMaxMessageSize"),
            10 => Some("vPIMSubMailboxes"),
            _ => None,
        };
    }

    #[cfg(feature = "ldap_attrs")]
    if x690_len == 8 && x690_slice.starts_with(&[1, 3, 6, 1, 1, 6, 2]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            3 => Some("pcimKeywords"),
            4 => Some("pcimGroupName"),
            5 => Some("pcimRuleName"),
            6 => Some("pcimRuleEnabled"),
            7 => Some("pcimRuleConditionListType"),
            8 => Some("pcimRuleConditionList"),
            9 => Some("pcimRuleActionList"),
            10 => Some("pcimRuleValidityPeriodList"),
            11 => Some("pcimRuleUsage"),
            12 => Some("pcimRulePriority"),
            13 => Some("pcimRuleMandatory"),
            14 => Some("pcimRuleSequencedActions"),
            15 => Some("pcimRoles"),
            16 => Some("pcimConditionGroupNumber"),
            17 => Some("pcimConditionNegated"),
            18 => Some("pcimConditionName"),
            19 => Some("pcimConditionDN"),
            20 => Some("pcimValidityConditionName"),
            21 => Some("pcimTimePeriodConditionDN"),
            22 => Some("pcimActionName"),
            23 => Some("pcimActionOrder"),
            24 => Some("pcimActionDN"),
            25 => Some("pcimTPCTime"),
            26 => Some("pcimTPCMonthOfYearMask"),
            27 => Some("pcimTPCDayOfMonthMask"),
            28 => Some("pcimTPCDayOfWeekMask"),
            29 => Some("pcimTPCTimeOfDayMask"),
            30 => Some("pcimTPCLocalOrUtcTime"),
            31 => Some("pcimVendorConstraintData"),
            32 => Some("pcimVendorConstraintEncoding"),
            33 => Some("pcimVendorActionData"),
            34 => Some("pcimVendorActionEncoding"),
            35 => Some("pcimPolicyInstanceName"),
            36 => Some("pcimRepositoryName"),
            37 => Some("pcimSubtreesAuxContainedSet"),
            38 => Some("pcimGroupsAuxContainedSet"),
            39 => Some("pcimRulesAuxContainedSet"),
            _ => None,
        };
    }

    #[cfg(feature = "ldap_attrs")]
    if x690_len == 8 && x690_slice.starts_with(&[1, 3, 6, 1, 1, 9, 2]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("pcelsPolicySetName"),
            2 => Some("pcelsDecisionStrategy"),
            3 => Some("pcelsPolicySetList"),
            4 => Some("pcelsPriority"),
            5 => Some("pcelsPolicySetDN"),
            6 => Some("pcelsConditionListType"),
            7 => Some("pcelsConditionList"),
            8 => Some("pcelsActionList"),
            9 => Some("pcelsSequencedActions"),
            10 => Some("pcelsExecutionStrategy"),
            11 => Some("pcelsVariableDN"),
            12 => Some("pcelsValueDN"),
            13 => Some("pcelsIsMirrored"),
            14 => Some("pcelsVariableName"),
            15 => Some("pcelsExpectedValueList"),
            16 => Some("pcelsVariableModelClass"),
            17 => Some("pcelsVariableModelProperty"),
            18 => Some("pcelsExpectedValueTypes"),
            19 => Some("pcelsValueName"),
            20 => Some("pcelsIPv4AddrList"),
            21 => Some("pcelsIPv6AddrList"),
            22 => Some("pcelsMACAddrList"),
            23 => Some("pcelsStringList"),
            24 => Some("pcelsBitStringList"),
            25 => Some("pcelsIntegerList"),
            26 => Some("pcelsBoolean"),
            27 => Some("pcelsReusableContainerName"),
            28 => Some("pcelsReusableContainerList"),
            29 => Some("pcelsRole"),
            30 => Some("pcelsRoleCollectionName"),
            31 => Some("pcelsElementList"),
            32 => Some("pcelsFilterName"),
            33 => Some("pcelsFilterIsNegated"),
            34 => Some("pcelsIPHdrVersion"),
            35 => Some("pcelsIPHdrSourceAddress"),
            36 => Some("pcelsIPHdrSourceAddressEndOfRange"),
            37 => Some("pcelsIPHdrSourceMask"),
            38 => Some("pcelsIPHdrDestAddress"),
            39 => Some("pcelsIPHdrDestAddressEndOfRange"),
            40 => Some("pcelsIPHdrDestMask"),
            41 => Some("pcelsIPHdrProtocolID"),
            42 => Some("pcelsIPHdrSourcePortStart"),
            43 => Some("pcelsIPHdrSourcePortEnd"),
            44 => Some("pcelsIPHdrDestPortStart"),
            45 => Some("pcelsIPHdrDestPortEnd"),
            46 => Some("pcelsIPHdrDSCPList"),
            47 => Some("pcelsIPHdrFlowLabel"),
            48 => Some("pcels8021HdrSourceMACAddress"),
            49 => Some("pcels8021HdrSourceMACMask"),
            50 => Some("pcels8021HdrDestMACAddress"),
            51 => Some("pcels8021HdrDestMACMask"),
            52 => Some("pcels8021HdrProtocolID"),
            53 => Some("pcels8021HdrPriority"),
            54 => Some("pcels8021HdrVLANID"),
            55 => Some("pcelsFilterListName"),
            56 => Some("pcelsFilterDirection"),
            57 => Some("pcelsFilterEntryList"),
            58 => Some("pcelsVendorVariableData"),
            59 => Some("pcelsVendorVariableEncoding"),
            60 => Some("pcelsVendorValueData"),
            61 => Some("pcelsVendorValueEncoding"),
            62 => Some("pcelsRuleValidityPeriodList"),
            _ => None,
        };
    }

    // 1.3.6.1.4.1.11.1.3.1.1.0
    #[cfg(feature = "ldap_attrs")]
    if x690_len == 12 && x690_slice.starts_with(&[1, 3, 6, 1, 4, 1, 11, 1, 3, 1, 1]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            0 => Some("defaultServerList"),
            1 => Some("defaultSearchBase"),
            2 => Some("preferredServerList"),
            3 => Some("searchTimeLimit"),
            4 => Some("bindTimeLimit"),
            5 => Some("followReferrals"),
            6 => Some("authenticationMethod"),
            7 => Some("profileTTL"),
            9 => Some("attributeMap"),
            10 => Some("credentialLevel"),
            11 => Some("objectclassMap"),
            12 => Some("defaultSearchScope"),
            13 => Some("serviceCredentialLevel"),
            14 => Some("serviceSearchDescriptor"),
            15 => Some("serviceAuthenticationMethod"),
            16 => Some("dereferenceAliases"),
            _ => None,
        };
    }

    // 1.3.6.1.4.1.1466.101.120.x
    #[cfg(feature = "ldap_attrs")]
    if x690_len == 11 && x690_slice.starts_with(&[1, 3, 6, 1, 4, 1, 0x8B, 0x3A, 101, 120]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("administratorsAddress"),
            5 => Some("namingContexts"),
            6 => Some("altServer"),
            7 => Some("supportedExtension"),
            13 => Some("supportedControl"),
            14 => Some("supportedSASLMechanisms"),
            15 => Some("supportedLDAPVersion"),
            16 => Some("ldapSyntaxes"),
            _ => None,
        };
    }

    // 1.3.6.1.4.1.16572.2.2.x
    #[cfg(feature = "ldap_attrs")]
    if x690_len == 12 && x690_slice.starts_with(&[1, 3, 6, 1, 4, 1, 0x81, 0x81, 0x3C, 2, 2]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("providerCertificateHash"),
            2 => Some("providerCertificate"),
            3 => Some("providerName"),
            4 => Some("mailReceipt"),
            5 => Some("managedDomains"),
            6 => Some("LDIFLocationURL"),
            7 => Some("providerUnit"),
            _ => None,
        };
    }

    // 1.3.6.1.4.1.31103.1.x
    #[cfg(feature = "ldap_attrs")]
    if x690_len == 11 && x690_slice.starts_with(&[1, 3, 6, 1, 4, 1, 0x81, 0xF2, 0x7F, 1]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("fedfsUuid"),
            4 => Some("fedfsFsnUuid"),
            8 => Some("fedfsFslUuid"),
            12 => Some("fedfsAnnotation"),
            13 => Some("fedfsDescr"),
            14 => Some("fedfsNceDN"),
            15 => Some("fedfsFsnTTL"),
            103 => Some("fedfsNfsCurrency"),
            104 => Some("fedfsNfsGenFlagWritable"),
            105 => Some("fedfsNfsGenFlagGoing"),
            106 => Some("fedfsNfsGenFlagSplit"),
            107 => Some("fedfsNfsTransFlagRdma"),
            108 => Some("fedfsNfsClassSimul"),
            109 => Some("fedfsNfsClassHandle"),
            110 => Some("fedfsNfsClassFileid"),
            111 => Some("fedfsNfsClassWritever"),
            112 => Some("fedfsNfsClassChange"),
            113 => Some("fedfsNfsClassReaddir"),
            114 => Some("fedfsNfsReadRank"),
            115 => Some("fedfsNfsReadOrder"),
            116 => Some("fedfsNfsWriteRank"),
            117 => Some("fedfsNfsWriteOrder"),
            118 => Some("fedfsNfsVarSub"),
            119 => Some("fedfsNfsValidFor"),
            120 => Some("fedfsNfsURI"),
            _ => None,
        };
    }

    // 1.3.6.1.4.1.453.7.2.x
    #[cfg(feature = "ldap_attrs")]
    if x690_len == 11 && x690_slice.starts_with(&[1, 3, 6, 1, 4, 1, 0x83, 0x45, 7, 2]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("textTableKey"),
            2 => Some("textTableValue"),
            3 => Some("associatedX400Gateway"),
            // 3 => Some("distinguishedNameTableKey"), (These are duplicates, it seems.)
            5 => Some("associatedORAddress"),
            7 => Some("oRAddressComponentType"),
            8 => Some("associatedInternetGateway"),
            9 => Some("mcgamTables"),
            _ => None,
        };
    }

    // 2.16.840.1.113730.3.1.x
    #[cfg(feature = "ldap_attrs")]
    if x690_len == 11 && x690_slice.starts_with(&[2, 16, 0x86, 0x48, 1, 0x86, 0xF8, 0x42, 3, 1]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("carLicense"),
            2 => Some("departmentNumber"),
            3 => Some("employeeNumber"),
            4 => Some("employeeType"),
            34 => Some("ref"),
            39 => Some("preferredLanguage"),
            40 => Some("userSMIMECertificate"),

            _ => None,
        };
    }

    // TODO: One offs:
    // 1.2.840.113549.1.9.1 == "emailAddress" / "email"
    // 1.3.6.1.1.16.4 "entryUUID"
    // 1.3.6.1.1.20 "entryDN"
    // 1.3.6.1.1.23.2 "jid" (NOT UNIQUE)
    // entryTtl	1.3.6.1.4.1.1466.101.119.3
    // dynamicSubtrees	1.3.6.1.4.1.1466.101.119.4
    // labeledURI	1.3.6.1.4.1.250.1.57
    // supportedFeatures	1.3.6.1.4.1.4203.1.3.5
    // userPKCS12	2.16.840.1.113730.3.1.216
    // displayName	2.16.840.1.113730.3.1.241

    None
}

pub fn common_attr_type_to_short_name(attr_type: &OBJECT_IDENTIFIER) -> Option<&'static str> {
    let x690_slice = attr_type.as_x690_slice();
    let x690_len = x690_slice.len();
    if x690_len == 3 && x690_slice.starts_with(&[0x55, 4]) {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            3 => Some("cn"),
            4 => Some("sn"),
            5 => Some("serialNumber"),
            6 => Some("c"),
            7 => Some("l"),
            8 => Some("st"),
            9 => Some("street"),
            10 => Some("o"),
            11 => Some("ou"),
            42 => Some("gn"),
            98 => Some("c3"),
            99 => Some("n3"),
            _ => return None,
        };
        // TODO: More attributes
    }

    // Always enabled because it has some important attributes used in naming.
    // 0.9.2342.19200300.100.1. is 0x0992268993F22C6401
    if x690_len == 10
        && x690_slice.starts_with(&[0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 0x01])
    {
        let last_byte = *attr_type.as_x690_slice().last().unwrap();
        return match last_byte {
            1 => Some("uid"),
            3 => Some("mail"),
            4 => Some("info"),
            5 => Some("drink"),
            25 => Some("dc"),
            41 => Some("mobile"),
            42 => Some("pager"),
            43 => Some("co"),
            _ => return None,
        };
    }

    // TODO: One offs:
    // 1.2.840.113549.1.9.1 == "emailAddress" / "email"
    // 1.3.6.1.1.16.4 "entryUUID"
    // 1.3.6.1.1.20 "entryDN"
    // 1.3.6.1.1.23.2 "jid" (NOT UNIQUE)
    // entryTtl	1.3.6.1.4.1.1466.101.119.3
    // dynamicSubtrees	1.3.6.1.4.1.1466.101.119.4
    // labeledURI	1.3.6.1.4.1.250.1.57
    // supportedFeatures	1.3.6.1.4.1.4203.1.3.5
    // userPKCS12	2.16.840.1.113730.3.1.216
    // displayName	2.16.840.1.113730.3.1.241
    None
}
