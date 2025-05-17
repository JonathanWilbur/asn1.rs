use atoi_simd::AtoiSimdError;
use smallvec::SmallVec;

use crate::{types::OBJECT_IDENTIFIER, unlikely, write_base_128, OidArcs, OID_ARC, RELATIVE_OID};
use std::{cmp::{self, max, min}, fmt::{Display, Write}, num::{IntErrorKind, ParseIntError}, str::FromStr, sync::Arc};

const PEN_PREFIX: [u32; 6] = [ 1, 3, 6, 1, 4, 1 ];

// TODO: Actually, I don't think you need this. You can check for overflows of u128 directly.
const MAX_ENCODED_ARC_LEN: usize = 19; // Larger than this could exceed the bounds of u128.

// TODO: Limit encoded OIDs to 127 bytes. No reasonable use case would exceed this.

// TODO: Handle on huge arcs, OIDs, etc.

impl OBJECT_IDENTIFIER {

    #[inline]
    pub fn arcs(&self) -> OidArcs<'_> {
        OidArcs{
            encoded: self.0.as_slice(),
            i: 0,
            first_arc_read: false,
            second_arc_read: false,
        }
    }

    // TODO: More efficient implementation
    // TODO: Dedupe from ROID
    // #[inline]
    pub fn to_asn1_string(&self) -> String {
        // I don't think there's really a much more performant way to implement
        // this. itoa is not very helpful here, because we have to clone the
        // stack buffer into an owned string anyway.
        format!(
            "{{ {} }}",
            self.0
                .iter()
                .map(|arc| arc.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }

    // #[inline]
    // pub fn to_iri_string(&self) -> String {
    //     // I don't think there's really a much more performant way to implement
    //     // this. itoa is not very helpful here, because we have to clone the
    //     // stack buffer into an owned string anyway.
    //     format!(
    //         "/{}",
    //         self.0
    //             .iter()
    //             .map(|n| n.to_string())
    //             .collect::<Vec<String>>()
    //             .join("/")
    //     )
    // }

    // #[inline]
    // pub fn extend(&mut self, roid: RELATIVE_OID) -> () {
    //     self.0.extend(roid.0)
    // }

    // #[inline]
    // pub fn starts_with(&mut self, roid: &RELATIVE_OID) -> bool {
    //     self.0.starts_with(roid.0.as_slice())
    // }

    // #[inline]
    // pub fn ends_with(&mut self, roid: &RELATIVE_OID) -> bool {
    //     self.0.ends_with(roid.0.as_slice())
    // }
}

// TODO: Ordering matching rules?
// TODO: Iterator

impl FromStr for OBJECT_IDENTIFIER {
    type Err = (); // TODO: More detailed error type? Overflow, empty, etc.

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
        OBJECT_IDENTIFIER::try_from(nodes).map_err(|_| ())
    }
}

impl TryFrom<Vec<u8>> for OBJECT_IDENTIFIER {
    type Error = std::io::Error; // TODO: Why is this the return type?

    // FIXME: Validate
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        #[cfg(feature = "smallvec")]
        {
            Ok(OBJECT_IDENTIFIER(value.into()))
        }
        #[cfg(not(feature = "smallvec"))]
        {
            Ok(OBJECT_IDENTIFIER(value))
        }
    }

}

impl TryFrom<&[u8]> for OBJECT_IDENTIFIER {
    type Error = std::io::Error;

    // FIXME: Validate
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        #[cfg(feature = "smallvec")]
        {
            Ok(OBJECT_IDENTIFIER(value.into()))
        }
        #[cfg(not(feature = "smallvec"))]
        {
            Ok(OBJECT_IDENTIFIER(value.to_owned()))
        }
    }

}

impl TryFrom<Vec<u32>> for OBJECT_IDENTIFIER {
    type Error = std::io::Error;

    fn try_from(value: Vec<u32>) -> Result<Self, Self::Error> {
        OBJECT_IDENTIFIER::try_from(value.as_slice())
    }

}

// TODO: Can I make this generic over the slice type?
impl TryFrom<&[u32]> for OBJECT_IDENTIFIER {
    type Error = std::io::Error;

    fn try_from(value: &[u32]) -> Result<Self, Self::Error> {
        if value.len() == 0 || value[0] > 2 || value.len() > 255 {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        }
        if value[0] < 2 && value[1] > 39 {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
        }
        let node0: u32 = value[0];
        let node1: u32 = value.get(0).cloned().unwrap_or(0);
        let first_component = node0.checked_add(node1)
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidInput))?;

        #[cfg(feature = "smallvec")]
        {
            let mut inner: SmallVec<[u8; 16]> = SmallVec::new();
            if value.len() == 1 {
                // This is a hack: to represent a single root arc, we create an
                // invalid OID with the first bit set and a length of 1.
                inner.push(value[0] as u8 | 0b1000_0000);
                return Ok(OBJECT_IDENTIFIER(inner));
            }
            write_base_128(&mut inner, first_component)?;
            for arc in value[2..].iter() {
                write_base_128(&mut inner, *arc)?;
            }
            Ok(OBJECT_IDENTIFIER(inner))
        }

        #[cfg(not(feature = "smallvec"))]
        {
            // TODO: To do function to calculate encoded length.
            let pre_alloc_size: usize = 1 + value[1..]
                .iter()
                .map(|arc| if *arc < 128 { 1 } else { 5 })
                .reduce(|total, size| total + size)
                .unwrap();
            let inner: Vec<u8> = Vec::with_capacity(pre_alloc_size);
            if value.len() == 1 {
                // This is a hack: to represent a single root arc, we create an
                // invalid OID with the first bit set and a length of 1.
                inner.push(value[0] as u8 | 0b1000_0000);
                return Ok(OBJECT_IDENTIFIER(inner));
            }
            write_base_128(&mut inner, first_component)?;
            for arc in value[2..].iter() {
                write_base_128(&mut inner, *arc)?;
            }
            Ok(OBJECT_IDENTIFIER(inner))
        }
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
            current_node <<= 7;
            current_node += (byte & 0b0111_1111) as u128;
        }
        // If we are starting on the first byte, it could encode either the
        // first or second arc, so it requires special handling.
        if unlikely(self.i == 0) {
            if self.first_arc_read {
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
    fn count(self) -> usize
        where
            Self: Sized, {
        self.size_hint().0
    }

    /// This performs better than the default implementation of nth() because
    /// it does not bother decoding arcs that are being skipped.
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if unlikely(self.encoded.len() == 0) {
            return None;
        }
        if unlikely(self.i == 0 && self.encoded.len() == 1 && self.encoded[0] >= 0b1000_0000) { // TODO: No masking. Just GTE
            if self.first_arc_read {
                return None; // We're already done iterating.
            } else {
                self.first_arc_read = true;
                return Some(min(self.encoded[0] & 0b0111_1111, 2) as u128);
            }
        }
        let mut iter_debt = n;
        if self.i == 0 {
            if !self.first_arc_read {
                let arc0 = self.next()?;
                if iter_debt == 0 {
                    return Some(arc0);
                }
                iter_debt -= 1;
            }
            let arc1 = self.next()?;
            if iter_debt == 0 {
                return Some(arc1);
            }
            iter_debt -= 1;
        }
        while iter_debt > 0 {
            let i = self.i as usize;
            let arc_len = self.encoded.get(i..)?
                .iter()
                .position(|b| *b < 0b1000_0000)? + 1;
            self.i = self.i.checked_add(arc_len as u32)?;
            iter_debt -= 1;
        }
        // Once iter_debt reaches 0, this is effectively .nth(0), which is .next().
        self.next() // TODO: This still incurs some overhead.
    }

    // #[inline]
    // fn last(mut self) -> Option<Self::Item>
    //     where
    //         Self: Sized, {
    //     // The Rust standard library does this as well.
    //     self.next_back()
    // }

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
            current_node <<= 7; // TODO: Just return wrong arc or None on overflow?
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

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        unimplemented!() // TODO:
    }

}

impl Display for OBJECT_IDENTIFIER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if unlikely(self.0.len() == 0) {
            return Ok(());
        }
        let mut wrote_first: bool = false;
        for arc in self.arcs() {
            if wrote_first {
                f.write_char('.')?;
            } else {
                wrote_first = true;
            }
            if cfg!(feature = "itoa") {
                let mut buf1 = itoa::Buffer::new();
                f.write_str(buf1.format(arc))?;
            } else {
                f.write_str(arc.to_string().as_str())?;
            }
        }
        Ok(())
    }
}

// impl IntoIterator for OBJECT_IDENTIFIER {
//     type Item = u32;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     #[inline]
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

impl PartialEq for OBJECT_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}


// #[macro_export]
// macro_rules! oid {
//     ( $( $x:expr ),* ) => {
//         {
//             use $crate::OBJECT_IDENTIFIER;
//             OBJECT_IDENTIFIER::from(Vec::<u32>::from([ $($x,)* ]))
//         }
//     };
// }

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use smallvec::SmallVec;

    use crate::OBJECT_IDENTIFIER;

    // #[test]
    // fn test_oid_parsing () {
    //     let oid1 = OBJECT_IDENTIFIER::from_str("1.3.6.4.1").unwrap();
    //     let oid2 = oid!(1,3,6,4,1);
    //     assert_eq!(oid1, oid2);
    // }

    // #[test]
    // fn test_oid_macro () {
    //     let oid1 = oid!(1,3,6,4,1);
    //     assert_eq!(oid1.to_string(), "1.3.6.4.1");
    // }

    // #[test]
    // fn test_oid_iter_1 () {
    //     let in_arcs: Vec<u8> = vec![ 43, 6, 4, 1, 120 ];
    //     let oid1 = OBJECT_IDENTIFIER::try_from(in_arcs).unwrap();
    //     let out_arcs: Vec<u128> = oid1.arcs().collect();
    //     assert_eq!(out_arcs.len(), 6);
    //     assert_eq!(oid1.to_string(), "1.3.6.4.1.120");

    //     let mut arcs = oid1.arcs();
    //     assert_eq!(arcs.count(), 6);
    //     // arcs.nth(6);
    //     arcs.i = arcs.encoded.len() as u32;
    //     arcs.first_arc_read = true;
    //     assert_eq!(arcs.next_back(), Some(120));
    //     assert_eq!(arcs.next_back(), Some(1));
    //     assert_eq!(arcs.next_back(), Some(4));
    //     assert_eq!(arcs.next_back(), Some(6));
    //     assert_eq!(arcs.next_back(), Some(3));
    //     assert_eq!(arcs.next_back(), Some(1));
    //     assert_eq!(arcs.next_back(), None);
    //     assert_eq!(arcs.next_back(), None);

    //     assert_eq!(arcs.next(), Some(1));
    //     assert_eq!(arcs.next(), Some(3));
    //     assert_eq!(arcs.next(), Some(6));
    //     assert_eq!(arcs.next(), Some(4));
    //     assert_eq!(arcs.next(), Some(1));
    //     assert_eq!(arcs.next(), Some(120));
    //     assert_eq!(arcs.next(), None);
    //     assert_eq!(arcs.next(), None);
    // }

    #[test]
    fn test_empty_oid() {
        // Create an empty OID (invalid but should be handled gracefully)
        let empty_vec: Vec<u8> = vec![];
        let oid = OBJECT_IDENTIFIER::try_from(empty_vec).unwrap();

        let mut arcs = oid.arcs();
        assert_eq!(arcs.next(), None);
        assert_eq!(arcs.next_back(), None);
        assert_eq!(arcs.size_hint(), (0, Some(0)));
        assert_eq!(arcs.count(), 0);
    }

    #[test]
    fn test_single_arc_oid() {
        // Test the "hack" case for single root arc
        let in_arcs: Vec<u8> = vec![0x82]; // 2 with high bit set
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
        let oid = OBJECT_IDENTIFIER::try_from(in_arcs).unwrap();

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
        let oid = OBJECT_IDENTIFIER::try_from(in_arcs).unwrap();

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
        let oid = OBJECT_IDENTIFIER::try_from(in_arcs).unwrap();

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
        let oid = OBJECT_IDENTIFIER::try_from(in_arcs).unwrap();

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
        let oid = OBJECT_IDENTIFIER::try_from(in_arcs).unwrap();

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
            let oid = OBJECT_IDENTIFIER::try_from(encoded).unwrap();
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
            let oid = OBJECT_IDENTIFIER::try_from(encoded.clone()).unwrap();
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
        let oid = OBJECT_IDENTIFIER::try_from(in_arcs).unwrap();

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
        let oid1 = OBJECT_IDENTIFIER::try_from(vec![0x80u8]).unwrap();
        let mut iter = oid1.arcs();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next_back(), None);

        // Mixed forward and backward iteration on larger OID
        let oid2 = OBJECT_IDENTIFIER::try_from(vec![43u8, 6, 4, 1, 120]).unwrap();
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
            let oid = OBJECT_IDENTIFIER::try_from(encoded.clone()).unwrap();

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
}
