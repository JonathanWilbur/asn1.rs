use crate::{
    _decode_Attribute, _decode_GeneralName, AlgorithmIdentifier, Attribute, AttributeCertificate,
    CertAVL, Certificate, Extension, GeneralName, IssuerSerial, IssuerSerialNumber, PKI_Stub,
    SIGNED, SubjectPublicKeyInfo, TBSAttributeCertificate, TBSCertAVL, TBSCertificate,
    _decode_SubjectPublicKeyInfo, _decode_AlgorithmIdentifier,
};
use chrono::DateTime;
use std::{
    collections::{HashMap, HashSet},
    iter::{ExactSizeIterator, FusedIterator, Iterator},
};
use wildboar_asn1::{
    oid, ASN1Error, ASN1ErrorCode, ASN1Result, GeneralizedTime, Tag, TagClass, BIT_STRING, OBJECT_IDENTIFIER, OCTET_STRING, UNIV_TAG_BOOLEAN, UNIV_TAG_INTEGER, UNIV_TAG_SEQUENCE
};
use x690::{RelateTLV, X690Codec, X690Element, X690Value, der::DER};

impl<T> AsRef<T> for SIGNED<T> {
    fn as_ref(&self) -> &T {
        &self.toBeSigned
    }
}

pub trait WithExtensions {

    fn get_extensions<'a>(&'a self) -> &'a [Extension];

    fn get_extension_by_oid<'a>(&'a self, oid: &OBJECT_IDENTIFIER) -> Option<&'a Extension> {
        self.get_extensions()
            .iter()
            .find(|ext| ext.extnId == *oid)
    }

    fn get_decoded_extension_by_oid<'a>(
        &'a self,
        oid: &OBJECT_IDENTIFIER,
    ) -> Option<ASN1Result<X690Element>> {
        let ext = self.get_extension_by_oid(&oid)?;
        let (bytes_read, el) = match DER.decode_from_slice(ext.extnValue.as_slice()) {
            Ok(x) => x,
            Err(e) => return Some(Err(e)),
        };
        if bytes_read != ext.extnValue.len() {
            let mut e = ASN1Error::new(ASN1ErrorCode::trailing_content_octets);
            e.relate_tlv(&el);
            return Some(Err(e));
        }
        Some(Ok(el))
    }

    fn get_extensions_hashmap<'a>(&'a self) -> ASN1Result<HashMap<OBJECT_IDENTIFIER, (bool, X690Element)>> {
        let mut map = HashMap::new();
        let extensions = self.get_extensions();
        for ext in extensions {
            let (bytes_read, el) = match DER.decode_from_slice(ext.extnValue.as_slice()) {
                Ok(x) => x,
                Err(e) => return Err(e),
            };
            if bytes_read != ext.extnValue.len() {
                let mut e = ASN1Error::new(ASN1ErrorCode::trailing_content_octets);
                e.relate_tlv(&el);
                return Err(e);
            }
            if map.insert(ext.extnId.clone(), (ext.critical.unwrap_or(false), el.clone())).is_some() {
                let mut e = ASN1Error::new(ASN1ErrorCode::constraint_violation)
                    .with_component_name("extensions");
                e.relate_tlv(&el);
                return Err(e);
            }
        }
        Ok(map)
    }
}

impl WithExtensions for TBSCertificate {
    #[inline]
    fn get_extensions<'a>(&'a self) -> &'a [Extension] {
        self.extensions
            .as_ref()
            .map(|x| x.as_slice())
            .unwrap_or(NO_EXTENSIONS.as_slice())
    }
}

const NO_EXTENSIONS: [Extension; 0] = [];

impl WithExtensions for Certificate {
    #[inline]
    fn get_extensions<'a>(&'a self) -> &'a [Extension] {
        self.toBeSigned.extensions
            .as_ref()
            .map(|x| x.as_slice())
            .unwrap_or(NO_EXTENSIONS.as_slice())
    }
}

impl WithExtensions for TBSAttributeCertificate {
    #[inline]
    fn get_extensions<'a>(&'a self) -> &'a [Extension] {
        self.extensions
            .as_ref()
            .map(|x| x.as_slice())
            .unwrap_or(NO_EXTENSIONS.as_slice())
    }
}

impl WithExtensions for AttributeCertificate {
    #[inline]
    fn get_extensions<'a>(&'a self) -> &'a [Extension] {
        self.toBeSigned.extensions
            .as_ref()
            .map(|x| x.as_slice())
            .unwrap_or(NO_EXTENSIONS.as_slice())
    }
}

impl WithExtensions for TBSCertAVL {
    #[inline]
    fn get_extensions<'a>(&'a self) -> &'a [Extension] {
        self.avlExtensions
            .as_ref()
            .map(|x| x.as_slice())
            .unwrap_or(NO_EXTENSIONS.as_slice())
    }
}

impl WithExtensions for CertAVL {
    #[inline]
    fn get_extensions<'a>(&'a self) -> &'a [Extension] {
        self.toBeSigned.avlExtensions
            .as_ref()
            .map(|x| x.as_slice())
            .unwrap_or(NO_EXTENSIONS.as_slice())
    }
}

pub struct PKCGenNameIter<'a> {
    cert: &'a TBSCertificate,
    i: usize,
    issuer: bool,
    alt_names_ext: Option<X690Element>,
}

impl<'a> PKCGenNameIter<'a> {

    pub fn for_issuer_names(
        cert: &'a TBSCertificate,
        alt_names_ext: Option<X690Element>,
    ) -> Self {
        PKCGenNameIter {
            cert,
            i: 0,
            alt_names_ext,
            issuer: true,
        }
    }

    pub fn for_subject_names(
        cert: &'a TBSCertificate,
        alt_names_ext: Option<X690Element>,
    ) -> Self {
        PKCGenNameIter {
            cert,
            i: 0,
            alt_names_ext,
            issuer: false,
        }
    }
}

impl<'a> Iterator for PKCGenNameIter<'a> {
    type Item = GeneralName;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i += 1;
            if self.issuer {
                let gn1 = GeneralName::directoryName(self.cert.subject.clone());
                return Some(gn1);
            } else {
                let gn1 = GeneralName::directoryName(self.cert.subject.clone());
                return Some(gn1);
            }
        }
        let ext = self.alt_names_ext.as_ref()?;
        while let Some(comp) = ext
            .components()
            .ok()
            .as_deref()
            .map(|c| c.get(self.i - 1))
            .flatten()
        {
            self.i += 1;
            let maybe_gn = _decode_GeneralName(comp);
            match maybe_gn {
                Ok(gn) => return Some(gn),
                Err(_) => continue,
            };
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let max_total_len = 1
            + self.alt_names_ext
                .as_ref()
                .map(|ext| ext
                    .components()
                    .map(|comps| comps.len())
                    .unwrap_or(0)
                )
                .unwrap_or(0);

        let iters_left = max_total_len.saturating_sub(self.i);
        (iters_left, Some(iters_left))
    }
}

impl<'a> FusedIterator for PKCGenNameIter<'a> {}

pub struct OidsExtIter {
    i: usize,
    ext_el: X690Element,
}

impl OidsExtIter {
    pub fn new(ext_el: X690Element) -> Self {
        OidsExtIter {
            i: 0,
            ext_el,
        }
    }
}

impl Iterator for OidsExtIter {
    type Item = OBJECT_IDENTIFIER;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(comp) = self.ext_el
            .components()
            .ok()
            .as_deref()
            .map(|c| c.get(self.i))
            .flatten()
        {
            self.i += 1;
            let maybe_oid = DER.decode_object_identifier(comp);
            match maybe_oid {
                Ok(oid) => return Some(oid),
                Err(_) => continue,
            };
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let max_total_len = self.ext_el
            .components()
            .map(|comps| comps.len())
            .unwrap_or(0);
        let iters_left = max_total_len.saturating_sub(self.i);
        (iters_left, Some(iters_left))
    }

}

impl FusedIterator for OidsExtIter {}

pub struct PKCAttrsIter {
    i: usize,
    ext_el: X690Element,
}

impl PKCAttrsIter {
    pub fn new(ext_el: X690Element) -> Self {
        PKCAttrsIter {
            i: 0,
            ext_el,
        }
    }
}

impl Iterator for PKCAttrsIter {
    type Item = Attribute;

    fn next(&mut self) -> Option<Self::Item> {
         while let Some(comp) = self.ext_el
            .components()
            .ok()
            .as_deref()
            .map(|c| c.get(self.i))
            .flatten()
        {
            self.i += 1;
            let maybe_attr = _decode_Attribute(comp);
            match maybe_attr {
                Ok(a) => return Some(a),
                Err(_) => continue,
            };
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let max_total_len = self.ext_el
            .components()
            .map(|comps| comps.len())
            .unwrap_or(0);
        let iters_left = max_total_len.saturating_sub(self.i);
        (iters_left, Some(iters_left))
    }

}

impl FusedIterator for PKCAttrsIter {}

const BOOL_TAG: Tag = Tag::new(TagClass::UNIVERSAL, UNIV_TAG_BOOLEAN);
const INT_TAG: Tag = Tag::new(TagClass::UNIVERSAL, UNIV_TAG_INTEGER);

pub struct PublicKeyCertExtInfo<'a> {
    pub issuer_names: PKCGenNameIter<'a>,
    pub subject_names: PKCGenNameIter<'a>,
    pub ext_key_usage: Option<OidsExtIter>,
    pub subj_dir_attrs: Option<PKCAttrsIter>,
    pub associated_info: Option<PKCAttrsIter>,
    pub ca: bool,
    pub path_len_constraint: u64,
    pub key_usage: Option<BIT_STRING>,
    pub private_key_usage_start: Option<GeneralizedTime>,
    pub private_key_usage_end: Option<GeneralizedTime>,
    pub inhibit_any_policy_skip_certs: Option<u64>,
    pub subject_key_id: Option<Vec<u8>>,
    pub alt_sig_alg: Option<AlgorithmIdentifier>,
    pub alt_sig_value: Option<BIT_STRING>,
    pub alt_spki: Option<SubjectPublicKeyInfo>,
    pub acceptable_priv_pols: Option<OidsExtIter>,
}

pub struct AttrCertExtInfo {
    pub aa: bool,
    pub path_len_constraint: u64,
    pub no_assertion: bool,
    pub is_soa: bool,
    pub is_single_use: bool,
    pub is_group_ac: bool,
    pub has_rev_info_avail: bool,
    pub is_indirect_issuer: bool,
    pub issued_on_behalf_of: Option<GeneralName>,
    pub alt_sig_alg: Option<AlgorithmIdentifier>,
    pub alt_sig_value: Option<BIT_STRING>,
    pub acceptable_cert_policies: Option<OidsExtIter>,
    pub acceptable_priv_policies: Option<OidsExtIter>,
}

pub struct AVLExtInfo {
    pub alt_sig_alg: Option<AlgorithmIdentifier>,
    pub alt_sig_value: Option<BIT_STRING>,
}

const EXT_SDA: u8 = 9;
const EXT_SKID: u8 = 14;
const EXT_KEY_USAGE: u8 = 15;
const EXT_PKUP: u8 = 16;
const EXT_SAN: u8 = 17;
const EXT_IAN: u8 = 18;
const EXT_BCONS: u8 = 19;
const EXT_EKU: u8 = 37;
const EXT_BACONS: u8 = 41;
const EXT_SOAID: u8 = 50;
const EXT_ACC_CERT_POL: u8 = 52;
const EXT_INH_ANY_POL: u8 = 54;
const EXT_NO_REV: u8 = 56;
const EXT_ACC_PRIV_POL: u8 = 57;
const EXT_NO_ASS: u8 = 62;
const EXT_SINGLE_USE: u8 = 65;
const EXT_GROUP_AC: u8 = 66;
const EXT_SAPKI: u8 = 72;
const EXT_ALT_SIG_ALG: u8 = 73;
const EXT_ALT_SIG_VAL: u8 = 74;
const EXT_ASSOC_INFO: u8 = 75;

impl TBSCertificate {
    pub fn matches_issuer_serial(&self) {}

    pub fn get_info_from_extensions<'a>(&'a self) -> ASN1Result<PublicKeyCertExtInfo<'a>> {
        let mut issuer_names = PKCGenNameIter::for_issuer_names(self, None);
        let mut subject_names = PKCGenNameIter::for_subject_names(self, None);
        let mut ext_key_usage: Option<OidsExtIter> = None;
        let mut subj_dir_attrs: Option<PKCAttrsIter> = None;
        let mut associated_info: Option<PKCAttrsIter> = None;
        let mut ca: bool = false;
        let mut path_len_constraint: u64 = u64::MAX;
        let mut key_usage: Option<BIT_STRING> = None;
        let mut private_key_usage_start: Option<GeneralizedTime> = None;
        let mut private_key_usage_end: Option<GeneralizedTime> = None;
        let mut inhibit_any_policy_skip_certs: Option<u64> = None;
        let mut subject_key_id: Option<OCTET_STRING> = None;
        let mut alt_sig_alg: Option<AlgorithmIdentifier> = None;
        let mut alt_sig_value: Option<BIT_STRING> = None;
        let mut alt_spki: Option<SubjectPublicKeyInfo> = None;
        let mut acceptable_priv_pols: Option<OidsExtIter> = None;

        let extensions = match self.extensions.as_ref() {
            Some(exts) => exts,
            None => return Ok(PublicKeyCertExtInfo{
                issuer_names,
                subject_names,
                ext_key_usage,
                subj_dir_attrs,
                associated_info,
                ca,
                path_len_constraint,
                key_usage,
                private_key_usage_start,
                private_key_usage_end,
                inhibit_any_policy_skip_certs,
                subject_key_id,
                alt_sig_alg,
                alt_sig_value,
                alt_spki,
                acceptable_priv_pols,
            }),
        };

        let mut seen_exts: HashSet<OBJECT_IDENTIFIER> = HashSet::new();
        for ext in extensions {
            if seen_exts.insert(ext.extnId.clone()) {
                let mut e = ASN1Error::new(ASN1ErrorCode::constraint_violation)
                    .with_component_name("extensions");
                return Err(e);
            }
            let bytes = ext.extnId.as_x690_slice();
            if bytes.len() != 3 {
                // All of the extensions we recognize are 3 bytes long.
                continue;
            }
            let last_byte = bytes[2];
            let (bytes_read, el) = DER.decode_from_slice(ext.extnValue.as_slice())?;
            if bytes_read != ext.extnValue.len() {
                let mut e = ASN1Error::new(ASN1ErrorCode::trailing_content_octets);
                e.relate_tlv(&el);
                return Err(e);
            }
            match last_byte {
                EXT_SDA => {
                    subj_dir_attrs = Some(PKCAttrsIter::new(el));
                },
                EXT_SKID => {
                    subject_key_id = Some(DER.decode_octet_string(&el)?);
                },
                EXT_KEY_USAGE => {
                    key_usage = Some(DER.decode_bit_string(&el)?);
                },
                EXT_EKU => {
                    ext_key_usage = Some(OidsExtIter::new(el));
                },
                EXT_PKUP => {
                    let components = match &el.value {
                        X690Value::Constructed(c) => c,
                        _ => continue,
                    };
                    let mut components = components.as_slice();
                    let first = match components.first() {
                        Some(c) => c,
                        None => continue, // Malformed: at least one component is required.
                    };
                    if first.tag.tag_class != TagClass::CONTEXT {
                        let mut e = ASN1Error::new(ASN1ErrorCode::invalid_construction)
                            .with_component_name("notBefore");
                        e.relate_tlv(&el);
                        return Err(e);
                    }
                    match first.tag.tag_number {
                        0 => private_key_usage_start = Some(DER.decode_generalized_time(first)?),
                        1 => private_key_usage_end = Some(DER.decode_generalized_time(first)?),
                        _ => continue, // Invalid first tag.
                    };
                    let second = match components.get(1) {
                        Some(c) => c,
                        None => continue,
                    };
                    if second.tag == Tag::new(TagClass::CONTEXT, 1) {
                        if private_key_usage_end.is_some() {
                            let mut e = ASN1Error::new(ASN1ErrorCode::constraint_violation)
                                .with_component_name("notAfter");
                            e.relate_tlv(&el);
                            return Err(e);
                        }
                        private_key_usage_end = Some(DER.decode_generalized_time(second)?);
                    }
                },
                EXT_SAN => {
                    subject_names = PKCGenNameIter::for_subject_names(self, Some(el));
                },
                EXT_IAN => {
                    issuer_names = PKCGenNameIter::for_issuer_names(self, Some(el));
                },
                EXT_BCONS => {
                    let components = match &el.value {
                        X690Value::Constructed(c) => c,
                        _ => continue,
                    };
                    let mut components = components.as_slice();
                    let first = match components.first() {
                        Some(f) => f,
                        None => continue,
                    };
                    if first.tag == BOOL_TAG {
                        ca = DER.decode_boolean(first).ok().unwrap_or(false);
                    } else if first.tag == INT_TAG {
                        path_len_constraint = DER.decode_u64(first).ok().unwrap_or(u64::MAX);
                        continue;
                    }
                    let second = match components.get(1) {
                        Some(c) => c,
                        None => continue,
                    };
                    if second.tag == INT_TAG {
                        path_len_constraint = DER.decode_u64(second).ok().unwrap_or(u64::MAX);
                    }
                },
                EXT_INH_ANY_POL => {
                    inhibit_any_policy_skip_certs = Some(DER.decode_u64(&el)?);
                },
                EXT_ACC_PRIV_POL => {
                    acceptable_priv_pols = Some(OidsExtIter::new(el));
                },
                EXT_SAPKI => {
                    alt_spki = Some(_decode_SubjectPublicKeyInfo(&el)?);
                },
                EXT_ALT_SIG_ALG => {
                    alt_sig_alg = Some(_decode_AlgorithmIdentifier(&el)?);
                },
                EXT_ALT_SIG_VAL => {
                    alt_sig_value = Some(DER.decode_bit_string(&el)?);
                },
                EXT_ASSOC_INFO => {
                    associated_info = Some(PKCAttrsIter::new(el));
                },
                _ => (),
            };
        }

        Ok(PublicKeyCertExtInfo{
            issuer_names,
            subject_names,
            ext_key_usage,
            subj_dir_attrs,
            associated_info,
            ca,
            path_len_constraint,
            key_usage,
            private_key_usage_start,
            private_key_usage_end,
            inhibit_any_policy_skip_certs,
            subject_key_id,
            alt_sig_alg,
            alt_sig_value,
            alt_spki,
            acceptable_priv_pols,
        })
    }

    fn iter_subject_names<'a>(&'a self) -> PKCGenNameIter<'a> {
        let maybe_ext = self.get_decoded_extension_by_oid(&oid!(2, 5, 29, 17))
            .map(|x| x.ok())
            .flatten(); // subjectAltName
        PKCGenNameIter::for_subject_names(self, maybe_ext)
    }

    fn iter_issuer_names<'a>(&'a self) -> PKCGenNameIter<'a> {
        let maybe_ext = self.get_decoded_extension_by_oid(&oid!(2, 5, 29, 18))
            .map(|x| x.ok())
            .flatten(); // issuerAltName
        PKCGenNameIter::for_issuer_names(self, maybe_ext)
    }

    fn iter_ext_key_usage(&self) -> Option<OidsExtIter> {
        let maybe_ext = self.get_decoded_extension_by_oid(&oid!(2, 5, 29, 37))?.ok()?; // extendedKeyUsage
        Some(OidsExtIter::new(maybe_ext))
    }

    fn iter_subject_directory_attributes(&self) -> Option<PKCAttrsIter> {
        let maybe_ext = self.get_decoded_extension_by_oid(&oid!(2, 5, 29, 9))?.ok()?; // subjectDirectoryAttributes
        Some(PKCAttrsIter::new(maybe_ext))
    }

    fn iter_associated_information(&self) -> Option<PKCAttrsIter> {
        let maybe_ext = self.get_decoded_extension_by_oid(&oid!(2, 5, 29, 75))?.ok()?; // associatedInformation
        Some(PKCAttrsIter::new(maybe_ext))
    }

    fn claims_to_be_issued_by_cert<C: AsRef<TBSCertificate>>(&self, issuer: &C) -> bool {
        let issuer_tbs = issuer.as_ref();
        let mut names_from_issuer = issuer_tbs.iter_subject_names().take(10);
        names_from_issuer.any(|niss| self.iter_issuer_names().take(10).any(|nsub| niss == nsub))
    }

    fn was_valid_as_of(&self) {
        todo!() // TODO: Implement
    }

}

impl TBSAttributeCertificate {
    // .claims_to_be_issued_by_cert(cert) and .claims_to_be_held_by_cert(cert) are
    // intentionally not implemented, because these require cryptography-related
    // dependencies, which this crate is avoiding depending on.

    pub fn get_info_from_extensions<'a>(&'a self) -> ASN1Result<AttrCertExtInfo> {
        let mut aa: bool = false;
        let mut path_len_constraint: u64 = u64::MAX;
        let mut no_assertion: bool = false;
        let mut is_soa: bool = false;
        let mut is_single_use: bool = false;
        let mut is_group_ac: bool = false;
        let mut has_rev_info_avail: bool = true;
        let mut is_indirect_issuer: bool = false;
        let mut issued_on_behalf_of: Option<GeneralName> = None;
        let mut alt_sig_alg: Option<AlgorithmIdentifier> = None;
        let mut alt_sig_value: Option<BIT_STRING> = None;
        let mut acceptable_cert_policies: Option<OidsExtIter> = None;
        let mut acceptable_priv_policies: Option<OidsExtIter> = None;

        let extensions = match self.extensions.as_ref() {
            Some(exts) => exts,
            None => return Ok(AttrCertExtInfo{
                aa,
                path_len_constraint,
                no_assertion,
                is_soa,
                is_single_use,
                is_group_ac,
                has_rev_info_avail,
                is_indirect_issuer,
                issued_on_behalf_of,
                alt_sig_alg,
                alt_sig_value,
                acceptable_cert_policies,
                acceptable_priv_policies,
            }),
        };

        let mut seen_exts: HashSet<OBJECT_IDENTIFIER> = HashSet::new();
        for ext in extensions {
            if seen_exts.insert(ext.extnId.clone()) {
                let mut e = ASN1Error::new(ASN1ErrorCode::constraint_violation)
                    .with_component_name("extensions");
                return Err(e);
            }
            let bytes = ext.extnId.as_x690_slice();
            if bytes.len() != 3 {
                // All of the extensions we recognize are 3 bytes long.
                continue;
            }
            let last_byte = bytes[2];
            let (bytes_read, el) = DER.decode_from_slice(ext.extnValue.as_slice())?;
            if bytes_read != ext.extnValue.len() {
                let mut e = ASN1Error::new(ASN1ErrorCode::trailing_content_octets);
                e.relate_tlv(&el);
                return Err(e);
            }
            match last_byte {
                EXT_BACONS => {
                    let components = match &el.value {
                        X690Value::Constructed(c) => c,
                        _ => continue,
                    };
                    let mut components = components.as_slice();
                    let first = match components.first() {
                        Some(f) => f,
                        None => continue,
                    };
                    if first.tag == BOOL_TAG {
                        aa = DER.decode_boolean(first).ok().unwrap_or(false);
                    } else if first.tag == INT_TAG {
                        path_len_constraint = DER.decode_u64(first).ok().unwrap_or(u64::MAX);
                        continue;
                    }
                    let second = match components.get(1) {
                        Some(c) => c,
                        None => continue,
                    };
                    if second.tag == INT_TAG {
                        path_len_constraint = DER.decode_u64(second).ok().unwrap_or(u64::MAX);
                    }
                },
                EXT_SOAID => is_soa = true,
                EXT_ACC_CERT_POL => acceptable_cert_policies = Some(OidsExtIter::new(el)),
                EXT_NO_REV => has_rev_info_avail = false,
                EXT_ACC_PRIV_POL => acceptable_priv_policies = Some(OidsExtIter::new(el)),
                EXT_NO_ASS => no_assertion = true,
                EXT_SINGLE_USE => is_single_use = true,
                EXT_GROUP_AC => is_group_ac = true,
                EXT_ALT_SIG_ALG => alt_sig_alg = Some(_decode_AlgorithmIdentifier(&el)?),
                EXT_ALT_SIG_VAL => alt_sig_value = Some(DER.decode_bit_string(&el)?),
                _ => (),
            }
        }

        Ok(AttrCertExtInfo{
            aa,
            path_len_constraint,
            no_assertion,
            is_soa,
            is_single_use,
            is_group_ac,
            has_rev_info_avail,
            is_indirect_issuer,
            issued_on_behalf_of,
            alt_sig_alg,
            alt_sig_value,
            acceptable_cert_policies,
            acceptable_priv_policies,
        })
    }
}

impl TBSCertAVL {
    pub fn get_info_from_extensions<'a>(&'a self) -> ASN1Result<AVLExtInfo> {
        let mut alt_sig_alg: Option<AlgorithmIdentifier> = None;
        let mut alt_sig_value: Option<BIT_STRING> = None;

        let extensions = match self.avlExtensions.as_ref() {
            Some(exts) => exts,
            None => return Ok(AVLExtInfo{
                alt_sig_alg,
                alt_sig_value,
            }),
        };

        let mut seen_exts: HashSet<OBJECT_IDENTIFIER> = HashSet::new();
        for ext in extensions {
            if seen_exts.insert(ext.extnId.clone()) {
                let mut e = ASN1Error::new(ASN1ErrorCode::constraint_violation)
                    .with_component_name("extensions");
                return Err(e);
            }
            let bytes = ext.extnId.as_x690_slice();
            if bytes.len() != 3 {
                // All of the extensions we recognize are 3 bytes long.
                continue;
            }
            let last_byte = bytes[2];
            let (bytes_read, el) = DER.decode_from_slice(ext.extnValue.as_slice())?;
            if bytes_read != ext.extnValue.len() {
                let mut e = ASN1Error::new(ASN1ErrorCode::trailing_content_octets);
                e.relate_tlv(&el);
                return Err(e);
            }
            match last_byte {
                EXT_ALT_SIG_ALG => alt_sig_alg = Some(_decode_AlgorithmIdentifier(&el)?),
                EXT_ALT_SIG_VAL => alt_sig_value = Some(DER.decode_bit_string(&el)?),
                _ => (),
            };
        }

        Ok(AVLExtInfo{
            alt_sig_alg,
            alt_sig_value,
        })
    }
}