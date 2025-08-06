//! The `OBJECT IDENTIFIER` type
//!
//! You can create, parse, print, compare, check prefixes and suffixes of
//! `OBJECT IDENTIFIER`s:
//!
//! ```rust
//! use wildboar_asn1::oid::OBJECT_IDENTIFIER;
//! use std::str::FromStr;
//! use wildboar_asn1::roid;
//! use wildboar_asn1::oid;
//!
//! let oid1 = OBJECT_IDENTIFIER::from_str("1.3.6.4.1").unwrap();
//! let oid2 = oid!(1,3,6,4,1);
//! assert_eq!(oid1, oid2);
//! assert_eq!(oid1.to_string(), "1.3.6.4.1");
//! assert_eq!(oid1.to_asn1_string(), "{ 1 3 6 4 1 }");
//! assert_eq!(oid1.to_iri_string(), "/1/3/6/4/1");
//! assert!(oid1.starts_with(&oid!(1,3,6,4,1)));
//! assert!(oid1.ends_with(&roid!(6,4,1)));
//! ```
#[cfg(feature = "smallvec")]
use smallvec::{SmallVec, smallvec};
use crate::{unlikely, ASN1Error, ASN1ErrorCode, ASN1Result, X690KnownSize, X690Validate, OID_ARC, RELATIVE_OID};
use std::{cmp::{min, Ordering}, fmt::{Display, Write as FmtWrite}, str::FromStr, u32};
use crate::utils::macros::write_oid_arc;

/// An ASN.1 `OBJECT IDENTIFIER`
#[cfg(not(feature = "smallvec"))]
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct OBJECT_IDENTIFIER (
    /// This contains the DER-encoding of the `OBJECT IDENTIFIER``, per ITU-T
    /// Recommendation X.690. This implementation favors faster comparison and
    /// hashing and lower memory footprint at the expense of slower parsing and
    /// printing.
    ///
    /// Intentionally not exported to library users so as to avoid dependency
    /// on the underlying storage of arcs.
    pub(crate) Vec<u8>
);

/// An ASN.1 `OBJECT IDENTIFIER`
#[cfg(feature = "smallvec")]
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct OBJECT_IDENTIFIER (
    /// This contains the DER-encoding of the `OBJECT IDENTIFIER``, per ITU-T
    /// Recommendation X.690. This implementation favors faster comparison and
    /// hashing and lower memory footprint at the expense of slower parsing and
    /// printing.
    ///
    /// The 16-byte inline vector was chosen because it is more than enough to
    /// accommodate the 12 bytes needed for an object identifier like
    /// 1.3.6.1.4.1.56490.5.4.13000. The vast majority of all object identifiers
    /// will fit without needing _any_ allocation on the heap.
    ///
    /// Intentionally not exported to library users so as to avoid dependency
    /// on the underlying storage of arcs.
    pub(crate) smallvec::SmallVec<[u8; 16]>
);

/// Iterator over the arcs of an `OBJECT IDENTIFIER`
#[derive(Debug, Clone, Copy)]
pub struct OidArcs<'a> {
    /// The full DER-encoding, but optionally with a hack where a single root
    /// arc is stored as a single byte with the most significant bit set.
    pub(crate) encoded: &'a [u8],
    /// Index into the encoded OID. u32 instead of usize so this struct would
    /// still be 24 bytes instead of 32.
    pub(crate) i: u32,
    /// Whether the iterator already handled the first arc. We need this because
    /// both the first and second arcs could be encoded in the first byte, and
    /// i alone would be insufficient to tell us if we iterated over the first
    /// arc already.
    pub(crate) first_arc_read: bool,
    /// This is just used for reverse iteration.
    pub(crate) second_arc_read: bool,
}

impl OBJECT_IDENTIFIER {

    /// Iterate over the arcs of the `OBJECT IDENTIFIER`
    #[inline]
    pub fn arcs(&self) -> OidArcs<'_> {
        OidArcs{
            encoded: self.0.as_slice(),
            i: 0,
            first_arc_read: false,
            second_arc_read: false,
        }
    }

    /// Convert to an ASN.1 `OBJECT IDENTIFIER` string, such as:
    /// `{ 1 3 6 1 4 1 56940 }`.
    pub fn to_asn1_string(&self) -> String {
        #[cfg(feature = "itoa")]
        let mut buf = itoa::Buffer::new();
        let mut out = String::with_capacity(self.0.len() << 3 + 4);
        out.write_str("{ ").unwrap();
        for arc in self.arcs() {
            #[cfg(feature = "itoa")]
            {
                out.write_str(buf.format(arc)).unwrap();
            }
            #[cfg(not(feature = "itoa"))]
            {
                out.write_str(arc.to_string().as_str()).unwrap();
            }
            out.write_char(' ').unwrap();
        }
        out.write_char('}').unwrap();
        out
    }

    /// Convert to an ASN.1 OID Internationalized Resource Identifier (OID-IRI)
    /// string, such as: `/1/3/6/1/4/1/56940`.
    pub fn to_iri_string(&self) -> String {
        #[cfg(feature = "itoa")]
        let mut buf = itoa::Buffer::new();
        let mut out = String::with_capacity(self.0.len() << 3);
        for arc in self.arcs() {
            out.write_char('/').unwrap();
            #[cfg(feature = "itoa")]
            {
                out.write_str(buf.format(arc)).unwrap();
            }
            #[cfg(not(feature = "itoa"))]
            {
                out.write_str(arc.to_string().as_str()).unwrap();
            }
        }
        out
    }

    /// Convert to a dot-delimited string, such as `1.3.6.1.4.1.56940`.
    #[inline]
    pub fn to_dot_delim_string(&self) -> String {
        self.to_string()
    }

    /// Validate that the supplied `content_octets` are a valid X.690 encoding
    /// of an `OBJECT IDENTIFIER`.
    pub fn validate_x690_encoding (content_octets: &[u8]) -> ASN1Result<()> {
        if content_octets.len() == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        if content_octets.len() > 1 && content_octets[content_octets.len() - 1] >= 0b1000_0000 {
            return Err(ASN1Error::new(ASN1ErrorCode::tlv_truncated));
        }
        let mut previous_byte_was_end_of_arc: bool = true;
        for byte in content_octets {
            if previous_byte_was_end_of_arc && *byte == 0b1000_0000 {
                return Err(ASN1Error::new(ASN1ErrorCode::oid_padding));
            }
            previous_byte_was_end_of_arc = *byte < 0b1000_0000;
        }
        Ok(())
    }

    /// Create an `OBJECT IDENTIFIER` directly from a `SmallVec`.
    ///
    /// This is defined so that you can define OIDs as compile-time constants.
    #[cfg(feature = "smallvec")]
    #[inline]
    pub const fn from_smallvec_unchecked (enc: SmallVec<[u8; 16]>) -> Self {
        OBJECT_IDENTIFIER(enc)
    }

    /// Create an `OBJECT IDENTIFIER` directly from the content octets ("value")
    /// of a BER, CER, or DER-encoded Tag-Length-Value (TLV) without checking
    /// for validity. (Other codecs use this encoding as well.)
    ///
    /// This is marked `unsafe` because of the potential to provide an invalid
    /// encoding. Wrong encodings should _never_ panic or read or write
    /// out-of-bounds, but their behavior is _undefined_. In all likelihood,
    /// supplying a wrongly-encoded OID will result in arcs you didn't expect,
    /// aberrant printing, incorrect comparison and ordering, etc.
    ///
    /// If you want to validate the encoding, consider using the safer
    /// [OBJECT_IDENTIFIER::from_x690_encoding_slice] instead.
    #[inline]
    pub unsafe fn from_x690_encoding_slice_unchecked (enc: &[u8]) -> Self {
        #[cfg(feature = "smallvec")]
        {
            OBJECT_IDENTIFIER(enc.into())
        }
        #[cfg(not(feature = "smallvec"))]
        {
            OBJECT_IDENTIFIER(enc.to_owned())
        }
    }

    /// Create an `OBJECT IDENTIFIER` directly from the content octets ("value")
    /// of a BER, CER, or DER-encoded Tag-Length-Value (TLV) without checking
    /// for validity. (Other codecs use this encoding as well.)
    ///
    /// This is marked `unsafe` because of the potential to provide an invalid
    /// encoding. Wrong encodings should _never_ panic or read or write
    /// out-of-bounds, but their behavior is _undefined_. In all likelihood,
    /// supplying a wrongly-encoded OID will result in arcs you didn't expect,
    /// aberrant printing, incorrect comparison and ordering, etc.
    ///
    /// If you want to validate the encoding, consider using the safer
    /// [OBJECT_IDENTIFIER::from_x690_encoding] instead.
    #[inline]
    pub unsafe fn from_x690_encoding_unchecked (enc: Vec<u8>) -> Self {
        #[cfg(feature = "smallvec")]
        {
            OBJECT_IDENTIFIER(enc.into())
        }
        #[cfg(not(feature = "smallvec"))]
        {
            OBJECT_IDENTIFIER(enc)
        }
    }

    /// Create an `OBJECT IDENTIFIER` directly from the content octets ("value")
    /// of a BER, CER, or DER-encoded Tag-Length-Value (TLV). (Other codecs use
    /// this encoding as well.)
    ///
    /// This method validates the encoded data. If you are certain that you do
    /// not need to validate the encoding, consider using the `unsafe`
    /// [OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked] instead.
    pub fn from_x690_encoding_slice (enc: &[u8]) -> ASN1Result<Self> {
        OBJECT_IDENTIFIER::validate_x690_encoding(enc)?;
        unsafe { Ok(OBJECT_IDENTIFIER::from_x690_encoding_slice_unchecked(enc)) }
    }

    /// Create an `OBJECT IDENTIFIER` directly from the content octets ("value")
    /// of a BER, CER, or DER-encoded Tag-Length-Value (TLV). (Other codecs use
    /// this encoding as well.)
    ///
    /// This method validates the encoded data. If you are certain that you do
    /// not need to validate the encoding, consider using the `unsafe`
    /// [OBJECT_IDENTIFIER::from_x690_encoding_unchecked] instead.
    pub fn from_x690_encoding (enc: Vec<u8>) -> ASN1Result<Self> {
        OBJECT_IDENTIFIER::validate_x690_encoding(enc.as_slice())?;
        unsafe { Ok(OBJECT_IDENTIFIER::from_x690_encoding_unchecked(enc)) }
    }

    /// Create a new `OBJECT IDENTIFIER` from another `OBJECT IDENTIFIER` as a
    /// prefix and a single `arc` to append to it. This is an elegant way to
    /// code the ASN.1 equivalent of something like `{ id-at 5 }`.
    pub fn from_prefix_and_arc (mut prefix: OBJECT_IDENTIFIER, arc: OID_ARC) -> ASN1Result<Self> {
        if unlikely(prefix.len() == 0) {
            return OBJECT_IDENTIFIER::try_from([ arc ].as_slice());
        }
        // Handle the single-arc OID case.
        if prefix.0.len() == 1 && prefix.0[0] >= 0b1000_0000 {
            let arc1 = min(prefix.0[0] & 0b0111_1111, 2) as OID_ARC * 40;
            let first_component = arc1.checked_add(arc)
                .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
            #[cfg(feature = "smallvec")]
            {
                let mut inner: SmallVec<[u8; 16]> = smallvec![];
                write_oid_arc!(inner, first_component);
                return Ok(OBJECT_IDENTIFIER::from_smallvec_unchecked(inner));
            }
            #[cfg(not(feature = "smallvec"))]
            {
                let mut inner: Vec<u8> = Vec::with_capacity(16); // Just guess that we'll use more bytes.
                write_oid_arc!(inner, first_component);
                return Ok(OBJECT_IDENTIFIER(inner))
            }
        }
        write_oid_arc!(prefix.0, arc);
        Ok(prefix)
    }

    /// Create an `OBJECT IDENTIFIER` from another `OBJECT IDENTIFIER` and a
    /// suffix. This is useful for encoding something like `{ id-at 5 1 }`.
    pub fn from_prefix_and_suffix (mut prefix: OBJECT_IDENTIFIER, suffix: &[OID_ARC]) -> ASN1Result<Self> {
        if unlikely(suffix.len() == 0) {
            return Ok(prefix);
        }
        if unlikely(prefix.len() == 0) {
            return OBJECT_IDENTIFIER::try_from(suffix);
        }
        // Handle the single-arc OID case.
        if prefix.0.len() == 1 && prefix.0[0] >= 0b1000_0000 {
            let arc1 = min(prefix.0[0] & 0b0111_1111, 2) as OID_ARC * 40;
            let first_component = arc1.checked_add(suffix[0])
                .ok_or(ASN1Error::new(ASN1ErrorCode::value_too_big))?;
            let roid = RELATIVE_OID::try_from(&suffix[1..])?;
            #[cfg(feature = "smallvec")]
            {
                let mut inner: SmallVec<[u8; 16]> = smallvec![];
                write_oid_arc!(inner, first_component);
                inner.extend_from_slice(roid.0.as_slice());
                return Ok(OBJECT_IDENTIFIER::from_smallvec_unchecked(inner));
            }
            #[cfg(not(feature = "smallvec"))]
            {
                let mut inner: Vec<u8> = Vec::with_capacity(16); // Just guess that we'll use more bytes.
                write_oid_arc!(inner, first_component);
                inner.extend_from_slice(roid.0.as_slice());
                return Ok(OBJECT_IDENTIFIER(inner))
            }
        }
        let roid = RELATIVE_OID::try_from(suffix)?;
        prefix.0.extend_from_slice(roid.0.as_slice());
        Ok(prefix)
    }

    /// Create an `OBJECT IDENTIFIER` from another `OBJECT IDENTIFIER` and a
    /// `RELATIVE-OID` that is assumed to be relative to the `prefix`,
    /// essentially by concatenating the two.
    pub fn from_prefix_and_roid (mut prefix: OBJECT_IDENTIFIER, roid: &RELATIVE_OID) -> ASN1Result<Self> {
        if unlikely(roid.len() == 0) {
            return Ok(prefix);
        }
        // Handle the single-arc OID case.
        if prefix.0.len() == 1 && prefix.0[0] >= 0b1000_0000 {
            let mut roid = roid.clone();
            roid.0[0] += (prefix.0[0] & 0b11) * 40;
            prefix.0.extend_from_slice(roid.0.as_slice());
            return Ok(prefix);
        }
        prefix.0.extend_from_slice(roid.0.as_slice());
        Ok(prefix)
    }

    /// Returns the number of arcs in this `OBJECT IDENTIFIER`
    #[inline]
    pub fn len(&self) -> usize {
        self.arcs().count()
    }

    /// Returns the X.690 encoding of this `OBJECT IDENTIFIER`
    #[inline]
    pub fn as_x690_slice(&self) -> &[u8] {
        &self.0
    }

    /// Produces an X.690 encoding of this `OBJECT IDENTIFIER`
    #[inline]
    pub fn to_x690_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Produces an X.690 encoding of this `OBJECT IDENTIFIER` in a `SmallVec`
    #[cfg(feature = "smallvec")]
    #[inline]
    pub fn to_x690_smallvec(self) -> SmallVec<[u8; 16]> {
        self.0
    }

    /// Determine if this `OBJECT IDENTIFIER` starts with the `other`, and is
    /// therefore a prefix of the latter.
    pub fn starts_with(&self, other: &OBJECT_IDENTIFIER) -> bool {
        match (self.0.len(), other.0.len()) {
            // If either is encoded on 1 byte, we iterate over all arcs to
            // handle the single-arc "hack" appropriately.
            (1, _) | (_, 1) => {
                let mut my_arcs = self.arcs();
                for other_arc in other.arcs() {
                    if let Some(my_arc) = my_arcs.next() {
                        if my_arc != other_arc {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            },
            _ => self.0.starts_with(other.0.as_slice())
        }
    }

    /// Determine if this `OBJECT IDENTIFIER` ends with the `RELATIVE-OID`
    /// `roid`.
    #[inline]
    pub fn ends_with(&self, roid: &RELATIVE_OID) -> bool {
        let mut my_arcs = self.arcs();
        for other_arc in roid.arcs().rev() {
            if let Some(my_arc) = my_arcs.next_back() {
                if my_arc != other_arc {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    // Concat may be implemented if the trait is ever stabilized.

}

impl PartialOrd for OBJECT_IDENTIFIER {

    /// This implementation orders OIDs by the numbers in their arcs, and
    /// considers shorter OIDs that are otherwise equal to be "less."
    ///
    /// This means that, when sorted, OIDs should form a "tree-like" ordering,
    /// like so:
    ///
    /// ```text
    /// 1.3.6.1
    /// 1.3.6.1.5
    /// 1.3.6.1.6
    /// 1.3.6.1.6.8
    /// ```
    ///
    /// ## Example
    ///
    /// ```rust
    /// let oid1 = wildboar_asn1::oid!(1,3,6,1);
    /// let oid2 = wildboar_asn1::oid!(1,3,6,1,5);
    /// let oid3 = wildboar_asn1::oid!(1,3,6,1,6);
    /// let oid4 = wildboar_asn1::oid!(1,3,6,1,6,8);
    ///
    /// let mut unordered = Vec::from([
    ///     oid4.clone(),
    ///     oid2.clone(),
    ///     oid1.clone(),
    ///     oid3.clone(),
    /// ].as_slice());
    /// unordered.sort();
    /// assert_eq!(unordered.as_slice(), [ oid1, oid2, oid3, oid4 ].as_slice());
    /// ```
    ///
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

impl Ord for OBJECT_IDENTIFIER {

    /// This implementation orders OIDs by the numbers in their arcs, and
    /// considers shorter OIDs that are otherwise equal to be "less."
    ///
    /// This means that, when sorted, OIDs should form a "tree-like" ordering,
    /// like so:
    ///
    /// ```text
    /// 1.3.6.1
    /// 1.3.6.1.5
    /// 1.3.6.1.6
    /// 1.3.6.1.6.8
    /// ```
    ///
    /// ## Example
    ///
    /// ```rust
    /// let oid1 = wildboar_asn1::oid!(1,3,6,1);
    /// let oid2 = wildboar_asn1::oid!(1,3,6,1,5);
    /// let oid3 = wildboar_asn1::oid!(1,3,6,1,6);
    /// let oid4 = wildboar_asn1::oid!(1,3,6,1,6,8);
    ///
    /// let mut unordered = Vec::from([
    ///     oid4.clone(),
    ///     oid2.clone(),
    ///     oid1.clone(),
    ///     oid3.clone(),
    /// ].as_slice());
    /// unordered.sort();
    /// assert_eq!(unordered.as_slice(), [ oid1, oid2, oid3, oid4 ].as_slice());
    /// ```
    ///
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut iter1 = self.arcs();
        let mut iter2 = other.arcs();
        loop {
            let arc1 = iter1.next();
            let arc2 = iter2.next();
            let cmp_result = match (arc1, arc2) {
                (Some(l), Some(r)) => l.cmp(&r),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => break,
            };
            if cmp_result != Ordering::Equal {
                return cmp_result;
            }
        }
        Ordering::Equal
    }

}

impl FromStr for OBJECT_IDENTIFIER {
    type Err = ();

    /// Parse an `OBJECT IDENTIFIER` from a dot-delimited string, such as `2.5.4.3`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Vec<u32> = Vec::with_capacity(s.len());
        for arc_string in s.split(".") {
            #[cfg(feature = "atoi_simd")]
            {
                let arc = atoi_simd::parse_pos::<OID_ARC>(arc_string.as_bytes()).map_err(|_| ())?;
                nodes.push(arc);
            }
            #[cfg(not(feature = "atoi_simd"))]
            {
                nodes.push(arc_string.parse::<OID_ARC>().map_err(|_| ())?);
            }
        }
        OBJECT_IDENTIFIER::try_from(nodes).map_err(|_| ())
    }
}

impl TryFrom<Vec<OID_ARC>> for OBJECT_IDENTIFIER {
    type Error = ASN1Error;

    /// Create an `OBJECT IDENTIFIER` from arcs
    ///
    /// It is an unfortunate limitation of Rust that it is extremely difficult to
    /// make this generic over all integer types. So this implementation just uses
    /// `Vec<u32>`.
    fn try_from(value: Vec<OID_ARC>) -> Result<Self, Self::Error> {
        OBJECT_IDENTIFIER::try_from(value.as_slice())
    }

}

impl TryFrom<&[OID_ARC]> for OBJECT_IDENTIFIER {
    type Error = ASN1Error;

    /// Create an `OBJECT IDENTIFIER` from arcs
    ///
    /// It is an unfortunate limitation of Rust that it is extremely difficult to
    /// make this generic over all integer types. So this implementation just uses
    /// `u32` slices.
    fn try_from(value: &[OID_ARC]) -> Result<Self, Self::Error> {
        let len = value.len();
        if unlikely(value.len() == 0) {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        if unlikely(value[0] > 2 || (value[0] < 2 && value.get(1).is_some_and(|second_arc| *second_arc > 39))) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_oid_arc));
        }
        let node0: OID_ARC = value[0] * 40; // Checking not needed.
        let node1: OID_ARC = value.get(1).cloned().unwrap_or(0);
        let first_component = node0.checked_add(node1)
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidInput))?;

        #[cfg(feature = "smallvec")]
        {
            let mut inner: SmallVec<[u8; 16]> = SmallVec::new();
            if unlikely(len == 1) {
                // This is a hack: to represent a single root arc, we create an
                // invalid OID with the first bit set and a length of 1.
                inner.push(min(value[0], 2) as u8 | 0b1000_0000);
                return Ok(OBJECT_IDENTIFIER(inner));
            }
            write_oid_arc!(inner, first_component);
            for arc in value[2..].iter() {
                write_oid_arc!(inner, *arc);
            }
            Ok(OBJECT_IDENTIFIER(inner))
        }

        #[cfg(not(feature = "smallvec"))]
        {
            let pre_alloc_size: usize = 1 + value[1..]
                .iter()
                .map(|arc| match *arc {
                    0..=127 => 1,
                    128..=16383 => 2, // Approximate, just in case I have an error
                    _ => 5,
                })
                .reduce(|total, size| total + size)
                .unwrap_or(16);
            let mut inner: Vec<u8> = Vec::with_capacity(pre_alloc_size);
            if unlikely(len == 1) {
                // This is a hack: to represent a single root arc, we create an
                // invalid OID with the first bit set and a length of 1.
                inner.push(min(value[0], 2) as u8 | 0b1000_0000);
                return Ok(OBJECT_IDENTIFIER(inner));
            }
            write_oid_arc!(inner, first_component);
            for arc in value[2..].iter() {
                write_oid_arc!(inner, *arc);
            }
            Ok(OBJECT_IDENTIFIER(inner))
        }
    }

}

impl TryFrom<Vec<i8>> for OBJECT_IDENTIFIER {
    type Error = ASN1Error;

    /// Create an `OBJECT IDENTIFIER` from arcs, with the arcs represented as
    /// `i8`s.
    ///
    /// This is a performance optimizing-hack: as long as an i8 representing an
    /// arc is not negative, it can be written directly into the internal
    /// buffer and still produce an valid encoding (except for the first two
    /// arcs).
    fn try_from(value: Vec<i8>) -> Result<Self, Self::Error> {
        OBJECT_IDENTIFIER::try_from(value.as_slice())
    }

}

impl TryFrom<&[i8]> for OBJECT_IDENTIFIER {
    type Error = ASN1Error;

    /// Create an `OBJECT IDENTIFIER` from arcs, with the arcs represented as
    /// `i8`s.
    ///
    /// This is a performance optimizing-hack: as long as an i8 representing an
    /// arc is not negative, it can be written directly into the internal
    /// buffer and still produce an valid encoding (except for the first two
    /// arcs).
    fn try_from(value: &[i8]) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        if value[0] > 2 || (value[0] < 2 && value.get(1).is_some_and(|second_arc| *second_arc > 39)) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_oid_arc));
        }
        let node0: OID_ARC = (value[0] * 40) as OID_ARC; // Checking not needed.
        let node1: OID_ARC = value.get(1).cloned().unwrap_or(0) as OID_ARC;
        let first_component = node0.checked_add(node1)
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidInput))?;

        if value.iter().any(|b| *b < 0) {
            return Err(ASN1Error::new(ASN1ErrorCode::invalid_oid_arc));
        }

        // Re-interpret the [i8] as a [u8]
        let unsigned: &[u8] = unsafe {
            std::slice::from_raw_parts(
                value.as_ptr() as *const u8,
                value.len(),
            )
        };

        #[cfg(feature = "smallvec")]
        {
            let mut inner: SmallVec<[u8; 16]> = SmallVec::new();
            if value.len() == 1 {
                // This is a hack: to represent a single root arc, we create an
                // invalid OID with the first bit set and a length of 1.
                inner.push(min(value[0], 2) as u8 | 0b1000_0000);
                return Ok(OBJECT_IDENTIFIER(inner));
            }
            write_oid_arc!(inner, first_component);
            inner.extend_from_slice(&unsigned[2..]);
            Ok(OBJECT_IDENTIFIER(inner))
        }

        #[cfg(not(feature = "smallvec"))]
        {
            let mut inner: Vec<u8> = Vec::with_capacity(value.len());
            if value.len() == 1 {
                // This is a hack: to represent a single root arc, we create an
                // invalid OID with the first bit set and a length of 1.
                inner.push(min(value[0], 2) as u8 | 0b1000_0000);
                return Ok(OBJECT_IDENTIFIER(inner));
            }
            write_oid_arc!(inner, first_component);
            inner.extend_from_slice(&unsigned[2..]);
            Ok(OBJECT_IDENTIFIER(inner))
        }
    }

}

impl OidArcs<'_> {

    /// Fast-forward to the end of the iterator, consuming all of it.
    #[inline]
    pub fn end (&mut self) {
        self.i = self.encoded.len().try_into().unwrap_or(u32::MAX);
        self.first_arc_read = true;
        self.second_arc_read = true;
    }

    /// Skip over one arc. This is like calling [OidArcs::next], but it does
    /// not decode the arc or return it.
    pub fn skip_one (&mut self) {
        if unlikely(
            self.encoded.len() == 0
            || self.i as usize >= self.encoded.len()
            || self.i == u32::MAX
        ) {
            return;
        }
        // Handle the single root arc "hack" case
        if unlikely(self.encoded.len() == 1 && self.encoded[0] >= 0b1000_0000) {
            if self.first_arc_read {
                return;
            } else {
                self.first_arc_read = true;
                return;
            }
        }
        let i = self.i as usize;
        let range = match self.encoded.get(i..) {
            Some(r) => r,
            None => return self.end(),
        };
        let maybe_arc_len = range.iter().position(|b| *b < 0b1000_0000).map(|l| l + 1);
        if maybe_arc_len.is_none() {
            return self.end();
        }
        let arc_len = maybe_arc_len.unwrap();
        // If we are starting on the first byte, it could encode either the
        // first or second arc, so it requires special handling.
        if unlikely(self.i == 0) {
            if self.first_arc_read {
                self.second_arc_read = true;
                self.i = match self.i.checked_add(arc_len as OID_ARC) {
                    Some(x) => x,
                    None => return self.end(),
                };
            } else {
                self.first_arc_read = true;
            }
            return;
        }
        if let Some(x) = self.i.checked_add(arc_len as OID_ARC) {
            self.i = x;
        } else {
            self.end();
        }
    }

    /// Skip backwards one arc. This is like calling [OidArcs::next_back],
    /// but it does not decode the arc or return it.
    pub fn skip_one_back (&mut self) {
        if unlikely(self.i as usize >= self.encoded.len()) {
            return;
        }
        // Handle the single OID arc "hack" case.
        if unlikely(self.i == 0 && self.encoded.len() == 1 && self.encoded[0] >= 0b1000_0000) {
            if self.first_arc_read {
                return;
            }
            self.encoded = &self.encoded[0..0];
            return;
        }
        let start = self.encoded[0..self.encoded.len()-1] // Skip the previous byte, because it is the end of the last arc.
            .iter()
            .rposition(|x| (*x & 0b1000_0000) < 0b1000_0000) // Find the byte that terminates the previous arc
            .map(|x| x + 1) // The byte after said terminating byte
            .unwrap_or(0) // Or zero if there is no such terminating byte.
            ;
        if start == 0 {
            if self.second_arc_read {
                // Return first arc
                self.encoded = &self.encoded[0..0];
                if self.first_arc_read {
                    return;
                }
            } else {
                self.second_arc_read = true;
            }
            return;
        }
        self.encoded = &self.encoded[0..start];
    }

}

impl Iterator for OidArcs<'_> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if unlikely(self.encoded.len() == 0) { // Handle an empty OID (invalid)
            return None;
        }
        // Handle the single root arc "hack" case
        if unlikely(self.encoded.len() == 1 && self.encoded[0] >= 0b1000_0000) {
            if self.first_arc_read {
                return None;
            } else {
                self.first_arc_read = true;
                return Some(min(self.encoded[0] & 0b0111_1111, 2) as u128);
            }
        }
        // Handle being at the end of the OID
        if unlikely(self.i as usize >= self.encoded.len()) {
            return None;
        }
        let i = self.i as usize;
        let arc_len = self.encoded.get(i..)?
            .iter()
            .position(|b| *b < 0b1000_0000)? + 1;
        let mut current_node: u128 = 0;
        for byte in self.encoded[i..i+arc_len].iter() {
            current_node = current_node.checked_shl(7)?;
            current_node += (byte & 0b0111_1111) as u128;
        }
        // If we are starting on the first byte, it could encode either the
        // first or second arc, so it requires special handling.
        if unlikely(self.i == 0) {
            if self.first_arc_read {
                if self.second_arc_read {
                    return None;
                }
                self.second_arc_read = true;
                self.i = self.i.checked_add(arc_len as u32)?;
                if current_node >= 80 {
                    return Some((current_node - 80) as u128);
                } else {
                    return Some((current_node % 40) as u128);
                }
            } else {
                self.first_arc_read = true;
                if current_node >= 80 {
                    return Some(2);
                } else {
                    return Some((current_node / 40) as u128);
                }
            }
        }
        self.i = self.i.checked_add(arc_len as u32)?;
        Some(current_node)
    }

    /// Non-default implementation: the exact size is known.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.encoded.len();
        match len {
            0 => (0, Some(0)),
            // This is a hack: to represent a single root arc, we create an
            // invalid OID with the first bit set and a length of 1.
            1 => if self.encoded[0] >= 0b1000_0000 {
                (1, Some(1))
            } else {
                (2, Some(2))
            },
            _ => {
                let arcs = self.encoded
                    .iter()
                    .filter(|b| **b < 0b1000_0000).count() + 1;
                (arcs, Some(arcs))
            }
        }
    }

    // This does not need to mutate the iterator.
    // It consumes it by taking ownership and dropping it.
    /// Non-default implementation: the exact size is known.
    #[inline]
    fn count(self) -> usize
        where
            Self: Sized, {
        self.size_hint().0
    }

    /// This performs better than the default implementation of nth() because
    /// it does not bother decoding arcs that are being skipped.
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let mut iter_debt = n;
        while iter_debt > 0 {
            self.skip_one();
            iter_debt -= 1;
        }
        // Once iter_debt reaches 0, this is effectively .nth(0), which is .next().
        self.next()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
        where
            Self: Sized, {
        // The Rust standard library does this as well.
        self.next_back()
    }

}

impl std::iter::FusedIterator for OidArcs<'_> {}

// The provided implementations are sufficient.
impl std::iter::ExactSizeIterator for OidArcs<'_> {}

impl std::iter::DoubleEndedIterator for OidArcs<'_> {

    fn next_back(&mut self) -> Option<Self::Item> {
        if unlikely(self.i as usize >= self.encoded.len()) {
            return None;
        }
        // Handle the single OID arc "hack" case.
        if unlikely(self.i == 0 && self.encoded.len() == 1 && self.encoded[0] >= 0b1000_0000) {
            if self.first_arc_read {
                return None;
            }
            let byte0 = self.encoded[0];
            self.encoded = &self.encoded[0..0];
            return Some(min(byte0 & 0b0111_1111, 2) as u128);
        }
        let start = self.encoded[0..self.encoded.len()-1] // Skip the previous byte, because it is the end of the last arc.
            .iter()
            .rposition(|x| (*x & 0b1000_0000) < 0b1000_0000) // Find the byte that terminates the previous arc
            .map(|x| x + 1) // The byte after said terminating byte
            .unwrap_or(0) // Or zero if there is no such terminating byte.
            ;

        let mut current_node: u128 = 0;
        for byte in self.encoded[start..].iter() {
            current_node = current_node.checked_shl(7)?;
            current_node += (byte & 0b0111_1111) as u128;
        }
        if start == 0 {
            if self.second_arc_read {
                // Return first arc
                self.encoded = &self.encoded[0..0];
                if self.first_arc_read {
                    return None;
                }
                if current_node >= 80 {
                    return Some(2);
                } else {
                    return Some((current_node / 40) as u128);
                }
            } else {
                self.second_arc_read = true;
                // Return second arc
                if current_node >= 80 {
                    return Some((current_node - 80) as u128);
                } else {
                    return Some((current_node % 40) as u128);
                }
            }
        }
        self.encoded = &self.encoded[0..start];
        Some(current_node)
    }

    /// Non-default implementation that does not decode arcs that are skipped
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let mut iter_debt = n;
        while iter_debt > 0 {
            self.skip_one_back();
            iter_debt -= 1;
        }
        // Once iter_debt reaches 0, this is effectively .nth_back(0), which is .next_back().
        self.next_back()
    }

}

impl Display for OBJECT_IDENTIFIER {

    /// Display the `OBJECT IDENTIFIER` as a dot-delimited string, such as
    /// `1.3.6.1.4.1.56940`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if unlikely(self.0.len() == 0) {
            return Ok(());
        }
        #[cfg(feature = "itoa")]
        let mut buf1 = itoa::Buffer::new();
        let mut wrote_first: bool = false;
        for arc in self.arcs() {
            if wrote_first {
                f.write_char('.')?;
            } else {
                wrote_first = true;
            }
            #[cfg(feature = "itoa")]
            {
                f.write_str(buf1.format(arc))?;
            }
            #[cfg(not(feature = "itoa"))]
            {
                f.write_str(arc.to_string().as_str())?;
            }
        }
        Ok(())
    }
}

impl X690KnownSize for OBJECT_IDENTIFIER {

    /// Returns the size of the content octets of the X.690-encoded
    /// value.
    fn x690_size (&self) -> usize {
        self.0.len() // The inner value is just DER-encoded
    }

}

impl X690Validate for OBJECT_IDENTIFIER {

    /// Validate the X.690 encoding (BER, CER, or DER) for an
    /// `OBJECT IDENTIFIER` value. This takes the content octets ("value") of
    /// the X.690 Tag-Length-Value.
    fn validate_x690_encoding (content_octets: &[u8]) -> ASN1Result<()> {
        if content_octets.len() == 0 {
            return Err(ASN1Error::new(ASN1ErrorCode::value_too_short));
        }
        if content_octets.len() > 1 && content_octets[content_octets.len() - 1] >= 0b1000_0000 {
            return Err(ASN1Error::new(ASN1ErrorCode::tlv_truncated));
        }
        let mut previous_byte_was_end_of_arc: bool = true;
        for byte in content_octets {
            if previous_byte_was_end_of_arc && *byte == 0b1000_0000 {
                return Err(ASN1Error::new(ASN1ErrorCode::oid_padding));
            }
            previous_byte_was_end_of_arc = *byte < 0b1000_0000;
        }
        Ok(())
    }

}

/// Convenience macro for creating object identifiers
///
/// #### Example
///
/// ```rust
/// let oid1 = wildboar_asn1::oid!(1,3,6,4,1);
/// ```
#[macro_export]
macro_rules! oid {
    ( $( $x:expr ),+ ) => {
        {
            use $crate::OBJECT_IDENTIFIER;
            OBJECT_IDENTIFIER::try_from([ $($x as u32,)* ].as_slice()).unwrap()
        }
    };
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::{OBJECT_IDENTIFIER, OID_ARC};

    #[test]
    fn test_oid_parsing () {
        let oid1 = OBJECT_IDENTIFIER::from_str("1.3.6.4.1").unwrap();
        let oid2 = oid!(1,3,6,4,1);
        assert_eq!(oid1, oid2);
    }

    #[test]
    fn test_oid_macro () {
        let oid1 = oid!(1,3,6,4,1);
        assert_eq!(oid1.to_string(), "1.3.6.4.1");
    }

    #[test]
    fn test_empty_oid() {
        // Create an empty OID (invalid but should be handled gracefully)
        let empty_vec: Vec<u8> = vec![];
        assert!(OBJECT_IDENTIFIER::from_x690_encoding(empty_vec).is_err());
    }

    #[test]
    fn test_single_arc_oid() {
        // Test the "hack" case for single root arc
        let in_arcs: Vec<OID_ARC> = vec![2]; // 2 with high bit set
        let oid = OBJECT_IDENTIFIER::try_from(in_arcs).unwrap();

        let mut arcs = oid.arcs();
        assert_eq!(arcs.size_hint(), (1, Some(1)));
        assert_eq!(arcs.next(), Some(2));
        assert_eq!(arcs.next(), None);

        let mut arcs = oid.arcs();
        assert_eq!(arcs.next_back(), Some(2));
        assert_eq!(arcs.next_back(), None);
    }

    #[test]
    fn test_minimal_two_arc_oid() {
        // Test minimal OID with just 2 arcs: 0.1
        let in_arcs: Vec<u8> = vec![1]; // 0*40 + 1 = 1
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(in_arcs).unwrap();

        let out_arcs: Vec<u128> = oid.arcs().collect();
        assert_eq!(out_arcs, vec![0, 1]);
        assert_eq!(oid.to_string(), "0.1");

        let mut arcs = oid.arcs();
        assert_eq!(arcs.size_hint(), (2, Some(2)));
        assert_eq!(arcs.next(), Some(0));
        assert_eq!(arcs.next(), Some(1));
        assert_eq!(arcs.next(), None);

        let mut arcs = oid.arcs();
        assert_eq!(arcs.next_back(), Some(1));
        assert_eq!(arcs.next_back(), Some(0));
        assert_eq!(arcs.next_back(), None);
    }

    #[test]
    fn test_large_oid() {
        // Test OID with many arcs
        let in_arcs: Vec<u8> = vec![
            43,                                         // 1.3
            6, 1, 4, 1, 187, 23, 220, 200, 0x81, 1,     // many arcs
            0x82, 0x1F, 0x8F, 0x10,                     // large arc (multi-byte)
        ];
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(in_arcs).unwrap();

        let arcs: Vec<u128> = oid.arcs().collect();
        assert_eq!(arcs.len(), 10);
        let iter = oid.arcs();
        // Test size_hint and count
        assert_eq!(iter.size_hint(), (10, Some(10)));

        // Create new iterator since count consumes it
        let iter = oid.arcs();
        assert_eq!(iter.count(), 10);

        // Test nth to skip elements
        let mut iter = oid.arcs();
        assert_eq!(iter.nth(3), Some(1));
        assert_eq!(iter.next(), Some(4));

        // Test iterating from both ends simultaneously
        let mut iter = oid.arcs();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(1936));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(287));
        assert_eq!(iter.next(), Some(6));
    }

    #[test]
    fn test_huge_arc() {
        // Test OID with a huge arc value approaching u128 limits
        let in_arcs: Vec<u8> = vec![
            43, // 1.3
            // Encode a massive number using base 128
            0xAF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0x7F, // Final byte without continuation bit
        ];
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(in_arcs).unwrap();

        let mut iter = oid.arcs();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));

        // The huge arc should decode properly to a u128 value
        let huge_arc = iter.next().unwrap();
        assert_eq!(huge_arc, u128::MAX);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_bidirectional_iteration() {
        let in_arcs: Vec<u8> = vec![43, 6, 4, 1, 0x81, 0x10];
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(in_arcs).unwrap();

        let mut iter = oid.arcs();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(144));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_fused_iterator() {
        // Test that the iterator behaves as a fused iterator
        // (continues to return None after exhaustion)
        let in_arcs: Vec<u8> = vec![43, 6];
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(in_arcs).unwrap();

        let mut iter = oid.arcs();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), None);
        // Fused iterator should keep returning None
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_oid_exact_size_iterator() {
        // Test exact size iterator implementation
        let test_cases: Vec<(Vec<u8>, usize)> = vec![
            (vec![43, 6, 4], 4),        // 1.3.6.4
            (vec![0x81, 0x01], 2),      // 2.129
            (vec![0x82], 1),            // Single-arc hack: 2
            (vec![40], 2),              // 1.0
        ];

        for (encoded, expected_len) in test_cases {
            let oid = OBJECT_IDENTIFIER::from_x690_encoding(encoded).unwrap();
            let iter = oid.arcs();

            // ExactSizeIterator provides len()
            assert_eq!(iter.len(), expected_len);

            // Verify it matches the actual count
            let arcs: Vec<u128> = oid.arcs().collect();
            assert_eq!(arcs.len(), expected_len);
        }
    }

    #[test]
    fn test_special_oid_cases() {
        // Test the edge cases between first and second arc
        // These test cases specifically check the encoding of the first two arcs
        let test_cases: Vec<(Vec<u8>, Vec<u128>)> = vec![
            (vec![0], vec![0, 0]),            // 0.0
            (vec![1], vec![0, 1]),            // 0.1
            (vec![39], vec![0, 39]),          // 0.39
            (vec![40], vec![1, 0]),           // 1.0
            (vec![79], vec![1, 39]),          // 1.39
            (vec![80], vec![2, 0]),           // 2.0
            (vec![119], vec![2, 39]),         // 2.39
            (vec![120], vec![2, 40]),         // 2.40
            (vec![0x81, 0x00], vec![2, 48]),  // 2.48
        ];

        for (encoded, expected_arcs) in test_cases {
            let oid = OBJECT_IDENTIFIER::from_x690_encoding(encoded.clone()).unwrap();
            let arcs: Vec<u128> = oid.arcs().collect();
            assert_eq!(arcs, expected_arcs, "Failed for encoding: {:?}", encoded);

            // Also check string representation
            let expected_str = expected_arcs.iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(".");
            assert_eq!(oid.to_string(), expected_str);
        }
    }

    #[test]
    fn test_oid_nth_method() {
        // Create an OID with several arcs
        let in_arcs: Vec<u8> = vec![43, 6, 4, 1, 120, 67];
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(in_arcs).unwrap();

        // Test the nth method directly
        let mut iter1 = oid.arcs();
        assert_eq!(iter1.nth(0), Some(1));  // First element
        assert_eq!(iter1.nth(0), Some(3));  // Next element after previous
        assert_eq!(iter1.nth(2), Some(1));  // Skip 2, get 3rd element after previous
        assert_eq!(iter1.nth(1), Some(67)); // Skip 1, get 2nd element after previous
        assert_eq!(iter1.nth(0), None);     // No more elements

        // Test nth with larger skip counts
        let mut iter2 = oid.arcs();
        assert_eq!(iter2.nth(4), Some(1)); // Skip 4, get 5th element
        assert_eq!(iter2.nth(0), Some(120)); // Next element
        assert_eq!(iter2.nth(0), Some(67));
        assert_eq!(iter2.nth(0), None);

        // Test nth with a skip that exceeds the remaining elements
        let mut iter3 = oid.arcs();
        assert_eq!(iter3.nth(10), None);    // Skip beyond available elements
    }

    #[test]
    fn test_oid_bidirectional_edge_cases() {
        // Test edge cases for bidirectional iteration
        // First arc only
        let oid1 = OBJECT_IDENTIFIER::try_from(vec![0u32]).unwrap();
        let mut iter = oid1.arcs();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next_back(), None);

        // Mixed forward and backward iteration on larger OID
        let oid2 = OBJECT_IDENTIFIER::from_x690_encoding(vec![43u8, 6, 4, 1, 120]).unwrap();
        let mut iter = oid2.arcs();
        assert_eq!(iter.next(), Some(1));      // Forward: get first arc
        assert_eq!(iter.next_back(), Some(120)); // Backward: get last arc
        assert_eq!(iter.next(), Some(3));      // Forward: get second arc
        assert_eq!(iter.next(), Some(6));      // Forward: get third arc
        assert_eq!(iter.next_back(), Some(1));  // Backward: get second-to-last arc
        assert_eq!(iter.next_back(), Some(4));  // Backward: get third-to-last arc
        assert_eq!(iter.next(), None);         // No more elements forward
        assert_eq!(iter.next_back(), None);    // No more elements backward
    }

    #[test]
    fn test_oid_validation_with_large_arcs() {
        // Test various encoding combinations, including some with large arcs
        let test_cases = vec![
            // id-pkix = 1.3.6.1.5.5.7
            (vec![43u8, 6, 1, 5, 5, 7], "1.3.6.1.5.5.7"),

            // OID with arcs that require multiple bytes
            (vec![43u8, 0x87, 0x67, 0x81, 0x33], "1.3.999.179"),

            // PEN prefix (1.3.6.1.4.1) + large final arc
            (vec![43u8, 6, 1, 4, 1, 0x83, 0xAA, 0x1B], "1.3.6.1.4.1.54555"),
        ];

        for (encoded, expected_str) in test_cases {
            let oid = OBJECT_IDENTIFIER::from_x690_encoding(encoded.clone()).unwrap();

            // Check string representation
            assert_eq!(oid.to_string(), expected_str);

            // Check consistency when converting to arcs and back
            let arcs: Vec<u128> = oid.arcs().collect();

            // Check arc count matches the number of dots plus 1
            let expected_arc_count = expected_str.chars().filter(|c| *c == '.').count() + 1;
            assert_eq!(arcs.len(), expected_arc_count);

            // Test ExactSizeIterator implementation
            let iter = oid.arcs();
            assert_eq!(iter.len(), expected_arc_count);

            // Validate each iterator method
            let mut forward_iter = oid.arcs();
            forward_iter.next_back();
            for _ in 0..expected_arc_count-1 {
                assert!(forward_iter.next().is_some());
            }

            assert_eq!(forward_iter.next(), None);
            assert_eq!(forward_iter.next_back(), None);
        }
    }

    #[test]
    fn test_nth_back_1() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![43u8, 6, 1, 4, 1, 0x83, 0xAA, 0x1B]).unwrap();

        let mut arcs1 = oid.arcs();
        assert_eq!(arcs1.next_back(), Some(54555));
        assert_eq!(arcs1.next_back(), Some(1));
        assert_eq!(arcs1.next_back(), Some(4));
        assert_eq!(arcs1.next_back(), Some(1));
        assert_eq!(arcs1.next_back(), Some(6));
        assert_eq!(arcs1.next_back(), Some(3));
        assert_eq!(arcs1.next_back(), Some(1));
        assert_eq!(arcs1.next_back(), None);
        assert_eq!(arcs1.next(), None);

        let mut arcs2 = oid.arcs();
        assert_eq!(arcs2.nth_back(0), Some(54555));
        assert_eq!(arcs2.nth_back(0), Some(1));
        assert_eq!(arcs2.nth_back(0), Some(4));
        assert_eq!(arcs2.nth_back(0), Some(1));
        assert_eq!(arcs2.nth_back(0), Some(6));
        assert_eq!(arcs2.nth_back(0), Some(3));
        assert_eq!(arcs2.nth_back(0), Some(1));
        assert_eq!(arcs2.nth_back(0), None);
        assert_eq!(arcs2.next(), None);

        let mut arcs3 = oid.arcs();
        assert_eq!(arcs3.nth_back(1), Some(1));
        assert_eq!(arcs3.nth_back(2), Some(6));
        assert_eq!(arcs3.nth_back(1), Some(1));
        assert_eq!(arcs3.nth_back(3), None);
        assert_eq!(arcs3.next(), None);

        let mut arcs4 = oid.arcs();
        assert_eq!(arcs4.nth_back(6), Some(1));
        assert_eq!(arcs4.nth_back(0), None);
        assert_eq!(arcs4.next(), None);

        let mut arcs5 = oid.arcs();
        assert_eq!(arcs5.nth_back(5), Some(3));
        assert_eq!(arcs5.nth_back(0), Some(1));
        assert_eq!(arcs5.nth_back(0), None);
        assert_eq!(arcs5.next(), None);
    }

    #[test]
    fn test_nth_back_2() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();

        let mut arcs1 = oid.arcs();
        assert_eq!(arcs1.next_back(), Some(1));
        assert_eq!(arcs1.next_back(), Some(54555));
        assert_eq!(arcs1.next_back(), Some(127));
        assert_eq!(arcs1.next_back(), Some(3));
        assert_eq!(arcs1.next_back(), Some(0));
        assert_eq!(arcs1.next_back(), None);
        assert_eq!(arcs1.next(), None);

        let mut arcs2 = oid.arcs();
        assert_eq!(arcs2.nth_back(0), Some(1));
        assert_eq!(arcs2.nth_back(0), Some(54555));
        assert_eq!(arcs2.nth_back(0), Some(127));
        assert_eq!(arcs2.nth_back(0), Some(3));
        assert_eq!(arcs2.nth_back(0), Some(0));
        assert_eq!(arcs2.nth_back(0), None);
        assert_eq!(arcs2.next(), None);

        let mut arcs3 = oid.arcs();
        assert_eq!(arcs3.nth_back(1), Some(54555));
        assert_eq!(arcs3.nth_back(1), Some(3));
        assert_eq!(arcs3.nth_back(1), None);
        assert_eq!(arcs3.next(), None);

        let mut arcs4 = oid.arcs();
        assert_eq!(arcs4.nth_back(4), Some(0));
        assert_eq!(arcs4.next(), None);

        let mut arcs5 = oid.arcs();
        assert_eq!(arcs5.nth_back(3), Some(3));
        assert_eq!(arcs5.next(), Some(0));
        assert_eq!(arcs5.next(), None);
    }

    // This is to make sure that, if an arc is so large that it fails to be
    // processed, it does not cause a panic when collected.
    #[test]
    fn test_arc_too_large_for_u128() {
        // Test OID with a huge arc value exceeding u128 limits
        let in_arcs: Vec<u8> = vec![
            43, // 1.3
            0xFF, // One byte plus a few bits too many.
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0x7F, // Final byte without continuation bit
        ];
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(in_arcs).unwrap();
        let _: Vec<u128> = oid.arcs().collect();
        let _: std::collections::HashSet<u128> = oid.arcs().collect();
    }

    #[test]
    fn test_oid_starts_with_1() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        let prefix = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8, 127, 0x83, 0xAA, 0x1B]).unwrap();
        assert!(oid.starts_with(&prefix));
    }

    #[test]
    fn test_oid_starts_with_2() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        let prefix = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8]).unwrap();
        assert!(oid.starts_with(&prefix));
    }

    #[test]
    fn test_oid_starts_with_3() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        let prefix = OBJECT_IDENTIFIER::try_from([ 0u32 ].as_slice()).unwrap();
        assert!(oid.starts_with(&prefix));
    }

    #[test]
    fn test_oid_starts_with_4() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![43]).unwrap();
        let prefix = OBJECT_IDENTIFIER::try_from([ 1u32 ].as_slice()).unwrap();
        assert!(oid.starts_with(&prefix));
    }

    #[test]
    fn test_oid_starts_with_5() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![43]).unwrap();
        let prefix = OBJECT_IDENTIFIER::try_from([ 2u32 ].as_slice()).unwrap();
        assert!(!oid.starts_with(&prefix));
    }

    #[test]
    fn test_oid_starts_with_6() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![43]).unwrap();
        let prefix = OBJECT_IDENTIFIER::try_from([ 1u32, 4 ].as_slice()).unwrap();
        assert!(!oid.starts_with(&prefix));
    }

    #[test]
    fn test_oid_ends_with_1() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        let suffix = crate::RELATIVE_OID::from_x690_encoding(vec![127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        assert!(oid.ends_with(&suffix));
    }

    #[test]
    fn test_oid_ends_with_2() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        let suffix = crate::RELATIVE_OID::from_x690_encoding(vec![3, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        assert!(oid.ends_with(&suffix));
    }

    #[test]
    fn test_oid_ends_with_3() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        let suffix = crate::RELATIVE_OID::from_x690_encoding(vec![0, 3, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        assert!(oid.ends_with(&suffix));
    }

    #[test]
    fn test_oid_ends_with_4() {
        let oid = OBJECT_IDENTIFIER::from_x690_encoding(vec![03u8, 127, 0x83, 0xAA, 0x1B, 1]).unwrap();
        let suffix = crate::RELATIVE_OID::from_x690_encoding(vec![0, 3, 126, 0x83, 0xAA, 0x1B, 1]).unwrap();
        assert!(!oid.ends_with(&suffix));
    }

    #[test]
    fn test_from_prefix_and_arc_1() {
        let prefix = oid!(1,3,6,1);
        let arc = 4;
        let oid1 = OBJECT_IDENTIFIER::from_prefix_and_arc(prefix, arc).unwrap();
        assert_eq!(oid1.to_string(), "1.3.6.1.4");
    }

    #[test]
    fn test_from_prefix_and_arc_2() {
        let prefix = oid!(1);
        let arc = 4;
        let oid1 = OBJECT_IDENTIFIER::from_prefix_and_arc(prefix, arc).unwrap();
        assert_eq!(oid1.to_string(), "1.4");
    }

    #[test]
    fn test_from_prefix_and_suffix_1() {
        let prefix = oid!(1,3,6,1);
        let oid1 = OBJECT_IDENTIFIER::from_prefix_and_suffix(prefix, [4, 9].as_slice()).unwrap();
        assert_eq!(oid1.to_string(), "1.3.6.1.4.9");
    }

    #[test]
    fn test_from_prefix_and_suffix_2() {
        let prefix = oid!(1);
        let oid1 = OBJECT_IDENTIFIER::from_prefix_and_suffix(prefix, [4, 9].as_slice()).unwrap();
        assert_eq!(oid1.to_string(), "1.4.9");
    }

    #[test]
    fn test_from_i8_slice() {
        let oid = OBJECT_IDENTIFIER::try_from([ 2i8, 5, 4, 3 ].as_slice()).unwrap();
        assert_eq!(oid.to_string(), "2.5.4.3");
    }

}
