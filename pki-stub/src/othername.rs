///! All known `OTHER-NAME` types

/// Prefix of X.690 encoding for the IETF's `OTHER-NAME` OID arc
///
/// The full OID is: `1.3.6.1.5.5.7.8`
pub const IETF_OTHER_NAMES_PREFIX: [u8; 7] = [43, 6, 1, 5, 5, 7, 8];

/// `personalData` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.1`
pub const PersonalData: u8 = 1;

/// `userGroup` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.2`
pub const UserGroup: u8 = 2;

/// `permanentIdentifier` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.3`
///
/// ```asn1
/// id-on-permanentIdentifier OBJECT IDENTIFIER ::= { id-on 3 }
///
/// PermanentIdentifier ::= SEQUENCE {
///     identifierValue UTF8String  OPTIONAL,
///                     -- if absent, use a serialNumber attribute,
///                     -- if there is such an attribute present
///                     -- in the subject DN
///     assigner        OBJECT IDENTIFIER OPTIONAL
///                     -- if absent, the assigner is
///                     -- the certificate issuer
/// }
/// ```
///
/// It is permissible for the `PermanentIdentifier` to have both components
/// absent.
///
pub const PermanentIdentifier: u8 = 3;

/// `hardwareModuleName` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.4`
///
/// HardwareModuleName is described here:
/// <https://www.rfc-editor.org/rfc/rfc4108.html#page-56>
///
/// ```asn1
/// id-on-hardwareModuleName  OBJECT IDENTIFIER ::= { id-on 4 }
///
/// HardwareModuleName ::= SEQUENCE {
///     hwType OBJECT IDENTIFIER,
///     hwSerialNum OCTET STRING }
/// ```
pub const HardwareModuleName: u8 = 4;

/// `xmppAddr` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.5`
///
/// XmppAddr is described here: https://datatracker.ietf.org/doc/html/rfc3920#section-5.1.1
///
/// ```asn1
/// id-on-xmppAddr OBJECT IDENTIFIER ::= { id-on 5 }
/// XmppAddr ::= UTF8String
/// ```
pub const XmppAddr: u8 = 5;

/// `SIM` (Subject Identification Method) `OTHER-NAME` OID X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.6`
///
/// SIM is described here: https://datatracker.ietf.org/doc/html/rfc4683.html#section-5.1
///
/// ```asn1
/// id-on-SIM OBJECT IDENTIFIER ::= { id-on 6 }
/// SIM ::= SEQUENCE {
///     hashAlg          AlgorithmIdentifier,
///     authorityRandom  OCTET STRING,   -- RA-chosen random number
///                                      -- used in computation of
///                                      -- pEPSI
///     pEPSI            OCTET STRING    -- hash of HashContent
///                                      -- with algorithm hashAlg
/// }
/// ```
pub const SIM: u8 = 6;

/// `dnsSRV` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.7`
///
/// SRVName is described here: https://datatracker.ietf.or  1`2g/doc/html/rfc4985
///
/// ```asn1
/// id-on-dnsSRV OBJECT IDENTIFIER ::= { id-on 7 }
/// SRVName ::= IA5String (SIZE (1..MAX))
/// ```
pub const SRVName: u8 = 7;

/// `naiRealm` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.8`
///
/// The NAIRealm OtherName is described here: https://datatracker.ietf.org/doc/html/rfc7585#section-2.2
///
/// ```asn1
/// id-on-naiRealm OBJECT IDENTIFIER ::= { id-on 8 }
/// ub-naiRealm-length INTEGER ::= 255
/// NAIRealm ::= UTF8String (SIZE (1..ub-naiRealm-length))
/// ```
pub const NAIRealm: u8 = 8;

/// `SmtpUTF8Mailbox` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.9`
///
/// SmtpUTF8Mailbox is described here: https://datatracker.ietf.org/doc/html/rfc8398
///
/// ```asn1
/// id-on-SmtpUTF8Mailbox OBJECT IDENTIFIER ::= { id-on 9 }
/// SmtpUTF8Mailbox ::= UTF8String (SIZE (1..MAX))
/// ```
pub const SmtpUTF8Mailbox: u8 = 9;

/// `AcpNodeName` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.10`
///
/// AcpNodeName is described here: https://www.rfc-editor.org/rfc/rfc8994.html
///
/// ```asn1
/// id-on-AcpNodeName OBJECT IDENTIFIER ::= { id-on 10 }
/// AcpNodeName ::= IA5String (SIZE (1..MAX))
/// ```
pub const AcpNodeName: u8 = 10;

/// `AcpNodeName` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.5.5.7.8.11`
///
/// BundleEID is described here: https://www.rfc-editor.org/rfc/rfc9174.html#name-asn1-module
///
/// ```asn1
/// id-on-bundleEID OBJECT IDENTIFIER ::= { id-on 11 }
/// BundleEID ::= IA5String
/// ```
pub const BundleEID: u8 = 11;

/// `UPN` `OTHER-NAME` object identifier X.690 encoding
///
/// OID: `1.3.6.1.4.1.311.20.2.3`
///
/// The syntax for this is a `UTF8String`.
///
/// The UPN OtherName is described here: https://learn.microsoft.com/en-US/troubleshoot/windows-server/windows-security/enabling-smart-card-logon-third-party-certification-authorities
///
pub const UPN: [u8; 10] = [43, 6, 1, 4, 1, 0x82, 0x37, 20, 2, 3];
