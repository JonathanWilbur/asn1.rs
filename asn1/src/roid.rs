use crate::{RELATIVE_OID, RelOidArcs};
use crate::error::{ASN1Error, ASN1ErrorCode, ASN1Result};
use crate::utils::{write_oid_arc, unlikely};
use std::{fmt::Display, str::FromStr};
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::fmt::Write;

impl RELATIVE_OID {

    #[inline]
    pub fn arcs(&self) -> RelOidArcs<'_> {
        RelOidArcs{
            encoded: self.0.as_slice(),
            i: 0,
        }
    }

    pub fn to_asn1_string(&self) -> String {
        let mut out = String::with_capacity(self.0.len() << 2 + 4);
        out.write_str("{ ").unwrap();
        for arc in self.arcs() {
            if cfg!(feature = "itoa") {
                let mut buf = itoa::Buffer::new();
                out.write_str(buf.format(arc)).unwrap();
            } else {
                out.write_str(arc.to_string().as_str()).unwrap();
            }
            out.write_char(' ').unwrap();
        }
        out.write_char('}').unwrap();
        out
    }

    pub fn to_iri_string(&self) -> String {
        let mut out = String::with_capacity(self.0.len() << 2);
        for arc in self.arcs() {
            out.write_char('/').unwrap();
            if cfg!(feature = "itoa") {
                let mut buf = itoa::Buffer::new();
                out.write_str(buf.format(arc)).unwrap();
            } else {
                out.write_str(arc.to_string().as_str()).unwrap();
            }
        }
        out
    }

    pub fn validate_x690_encoding (content_octets: &[u8]) -> ASN1Result<()> {
        if content_octets.len() > 1 && content_octets[content_octets.len() - 1] >= 0b1000_0000 {
            return Err(ASN1Error::new(ASN1ErrorCode::truncated));
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

    /// This is defined so that you can define OIDs as compile-time constants.
    #[cfg(feature = "smallvec")]
    #[inline]
    pub const fn from_smallvec_unchecked (enc: SmallVec<[u8; 16]>) -> Self {
        RELATIVE_OID(enc)
    }

    #[inline]
    pub fn from_x690_encoding_slice_unchecked (enc: &[u8]) -> Self {
        #[cfg(feature = "smallvec")]
        {
            RELATIVE_OID(enc.into())
        }
        #[cfg(not(feature = "smallvec"))]
        {
            RELATIVE_OID(value.to_owned())
        }
    }

    #[inline]
    pub fn from_x690_encoding_unchecked (enc: Vec<u8>) -> Self {
        #[cfg(feature = "smallvec")]
        {
            RELATIVE_OID(enc.into())
        }
        #[cfg(not(feature = "smallvec"))]
        {
            RELATIVE_OID(enc)
        }
    }

    pub fn from_x690_encoding_slice (enc: &[u8]) -> ASN1Result<Self> {
        RELATIVE_OID::validate_x690_encoding(enc)?;
        Ok(RELATIVE_OID::from_x690_encoding_slice_unchecked(enc))
    }

    pub fn from_x690_encoding (enc: Vec<u8>) -> ASN1Result<Self> {
        RELATIVE_OID::validate_x690_encoding(enc.as_slice())?;
        Ok(RELATIVE_OID::from_x690_encoding_unchecked(enc))
    }

    /// Returns the number of arcs in this `RELATIVE-OID`
    #[inline]
    pub fn len(&self) -> usize {
        self.arcs().count()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_x690_slice(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn to_x690_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    #[cfg(feature = "smallvec")]
    #[inline]
    pub fn to_x690_smallvec(self) -> SmallVec<[u8; 16]> {
        self.0
    }

    #[inline]
    pub fn starts_with(&self, roid: &RELATIVE_OID) -> bool {
        self.0.starts_with(roid.0.as_slice())
    }

    #[inline]
    pub fn ends_with(&self, roid: &RELATIVE_OID) -> bool {
        self.0.ends_with(roid.0.as_slice())
    }
}

impl PartialOrd for RELATIVE_OID {

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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

impl Ord for RELATIVE_OID {

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

impl FromStr for RELATIVE_OID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Vec<u32> = Vec::with_capacity(s.len());
        for arc_string in s.split(".") {
            if cfg!(feature = "atoi_simd") {
                let arc = atoi_simd::parse_pos::<u32>(arc_string.as_bytes()).map_err(|_| ())?;
                nodes.push(arc);
            } else {
                nodes.push(arc_string.parse::<u32>().map_err(|_| ())?);
            }
        }
        RELATIVE_OID::try_from(nodes).map_err(|_| ())
    }
}

impl TryFrom<Vec<u32>> for RELATIVE_OID {
    type Error = ASN1Error;

    fn try_from(value: Vec<u32>) -> Result<Self, Self::Error> {
        RELATIVE_OID::try_from(value.as_slice())
    }

}

impl TryFrom<&[u32]> for RELATIVE_OID {
    type Error = ASN1Error;

    /// It is an unfortunate limitation of Rust that it is extremely difficult to
    /// make this generic over all integer types. So this implementation just uses
    /// u32 slices. If you need to append something larger, like a u128, you're
    /// going to have to use a combination of [write_oid_arc] and
    /// [RELATIVE_OID::from_x690_encoding].
    fn try_from(value: &[u32]) -> Result<Self, Self::Error> {
        #[cfg(feature = "smallvec")]
        {
            let mut inner: SmallVec<[u8; 16]> = SmallVec::new();
            for arc in value.iter() {
                write_oid_arc(&mut inner, *arc as u128)?;
            }
            Ok(RELATIVE_OID(inner))
        }

        #[cfg(not(feature = "smallvec"))]
        {
            let pre_alloc_size: usize = value
                .iter()
                .map(|arc| match *arc < 128 {
                    0..=127 => 1,
                    128..=16383 => 2, // Approximate, just in case I have an error
                    _ => 5,
                })
                .reduce(|total, size| total + size)
                .unwrap();
            let inner: Vec<u8> = Vec::with_capacity(pre_alloc_size);
            for arc in value[2..].iter() {
                write_oid_arc(&mut inner, *arc as u128)?;
            }
            Ok(RELATIVE_OID(inner))
        }
    }

}

impl RelOidArcs<'_> {

    #[inline]
    pub fn end (&mut self) {
        self.i = self.encoded.len().try_into().unwrap_or(usize::MAX);
    }

    pub fn skip_one (&mut self) {
        if unlikely(
            self.encoded.len() == 0
            || self.i as usize >= self.encoded.len()
        ) {
            return;
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
        if let Some(x) = self.i.checked_add(arc_len) {
            self.i = x;
        } else {
            self.end();
        }
    }

    pub fn skip_one_back (&mut self) {
        if unlikely(self.i as usize >= self.encoded.len()) {
            return;
        }
        let start = self.encoded[0..self.encoded.len()-1] // Skip the previous byte, because it is the end of the last arc.
            .iter()
            .rposition(|x| (*x & 0b1000_0000) < 0b1000_0000) // Find the byte that terminates the previous arc
            .map(|x| x + 1) // The byte after said terminating byte
            .unwrap_or(0) // Or zero if there is no such terminating byte.
            ;
        self.encoded = &self.encoded[0..start];
    }

}

impl Iterator for RelOidArcs<'_> {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if unlikely(self.i >= self.encoded.len()) {
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
        self.i = self.i.checked_add(arc_len)?;
        Some(current_node)
    }

    /// Non-default implementation: the exact size is known.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let arcs = self.encoded
            .iter()
            .filter(|b| **b < 0b1000_0000).count();
        (arcs, Some(arcs))
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

impl Display for RELATIVE_OID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.0
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(".")
                .as_str(),
        )
    }
}

impl std::iter::FusedIterator for RelOidArcs<'_> {}

// The provided implementations are sufficient.
impl std::iter::ExactSizeIterator for RelOidArcs<'_> {}

impl std::iter::DoubleEndedIterator for RelOidArcs<'_> {

    fn next_back(&mut self) -> Option<Self::Item> {
        if unlikely(self.i as usize >= self.encoded.len()) {
            return None;
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
        self.encoded = &self.encoded[0..start];
        Some(current_node)
    }

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

#[macro_export]
macro_rules! roid {
    ( $( $x:expr ),* ) => {
        {
            use $crate::RELATIVE_OID;
            RELATIVE_OID::try_from([ $($x as u32,)* ].as_slice()).unwrap()
        }
    };
}

// The code for [RELATIVE_OID] was copied from [OBJECT_IDENTIFIER], but with
// just a few modifications, so the tests there transitively cover this to some
// extent.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roid_parsing () {
        let roid1 = RELATIVE_OID::from_str("1.3.6.4.1").unwrap();
        let roid2 = roid!(1,3,6,4,1);
        assert_eq!(roid1, roid2);
    }

    #[test]
    fn test_roid_macro () {
        let roid1 = roid!(1,3,6,4,1);
        assert_eq!(roid1.to_string(), "1.3.6.4.1");
    }

    #[test]
    fn test_large_roid() {
        // Test OID with many arcs
        let in_arcs: Vec<u8> = vec![
            43, 6, 1, 4, 1, 187, 23, 220, 200, 0x81, 1,
            0x82, 0x1F, 0x8F, 0x10,
        ];
        let oid = RELATIVE_OID::from_x690_encoding(in_arcs).unwrap();

        let arcs: Vec<u128> = oid.arcs().collect();
        assert_eq!(arcs.len(), 9);
        let iter = oid.arcs();
        // Test size_hint and count
        assert_eq!(iter.size_hint(), (9, Some(9)));

        // Create new iterator since count consumes it
        let iter = oid.arcs();
        assert_eq!(iter.count(), 9);

        // Test nth to skip elements
        let mut iter = oid.arcs();
        assert_eq!(iter.nth(3), Some(4));
        assert_eq!(iter.next(), Some(1));

        // Test iterating from both ends simultaneously
        let mut iter = oid.arcs();
        assert_eq!(iter.next(), Some(43));
        assert_eq!(iter.next_back(), Some(1936));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next_back(), Some(287));
        assert_eq!(iter.next(), Some(1));
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
        let oid = RELATIVE_OID::from_x690_encoding(in_arcs).unwrap();
        let _: Vec<u128> = oid.arcs().collect();
        let _: std::collections::HashSet<u128> = oid.arcs().collect();
    }

}
