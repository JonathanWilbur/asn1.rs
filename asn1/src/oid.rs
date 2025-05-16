use atoi_simd::AtoiSimdError;
use smallvec::SmallVec;

use crate::{types::OBJECT_IDENTIFIER, unlikely, write_base_128, OidArcs, OID_ARC, RELATIVE_OID};
use std::{cmp::max, fmt::{Display, Write}, num::{IntErrorKind, ParseIntError}, str::FromStr, sync::Arc};

const PEN_PREFIX: [u32; 6] = [ 1, 3, 6, 1, 4, 1 ];

impl OBJECT_IDENTIFIER {

    #[inline]
    pub fn arcs(&self) -> OidArcs<'_> {
        OidArcs{
            encoded: self.0.as_slice(),
            i: 0,
        }
    }

    // TODO: Dedupe from ROID
    // #[inline]
    // pub fn to_asn1_string(&self) -> String {
    //     // I don't think there's really a much more performant way to implement
    //     // this. itoa is not very helpful here, because we have to clone the
    //     // stack buffer into an owned string anyway.
    //     format!(
    //         "{{ {} }}",
    //         self.0
    //             .iter()
    //             .map(|n| n.to_string())
    //             .collect::<Vec<String>>()
    //             .join(" ")
    //     )
    // }

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
        // This is a hack: to represent a single root arc, we create an
        // invalid OID with the first bit set and a length of 1.
        if unlikely(self.encoded.len() == 1 && self.encoded[0] > 0b1000_0000) {
            if self.i == 0 {
                self.i = 1;
                return Some(max(self.encoded[0] & 0b0111_1111, 2) as u128);
            } else {
                return None;
            }
        }
        let i = self.i.saturating_sub(1); // both i=0 and i=1 will start at [0].
        let arc_len = self.encoded[i..]
            .iter()
            .position(|b| *b < 0b1000_0000)?;
        let mut current_node: u128 = 0;
        for byte in self.encoded[i..i+arc_len].iter() {
            current_node <<= 7;
            current_node += (byte & 0b0111_1111) as u128;
        }
        if self.i == 0 {
            self.i = 1;
            if current_node >= 80 {
                return Some(2);
            } else {
                return Some((current_node / 40) as u128);
            }
        } else if self.i == 1 {
            self.i += arc_len;
            if current_node >= 80 {
                return Some((current_node - 80) as u128);
            } else {
                return Some((current_node % 40) as u128);
            }
        }
        self.i += arc_len;
        Some(current_node)
    }

    /// Non-default implementation: the exact size is known.
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.encoded.len();
        match len {
            0 => (0, Some(0)),
            // This is a hack: to represent a single root arc, we create an
            // invalid OID with the first bit set and a length of 1.
            1 => if self.encoded[0] > 0b1000_0000 {
                (1, Some(1))
            } else {
                (2, Some(2))
            },
            _ => {
                let arcs = self.encoded[1..]
                    .iter()
                    .filter(|b| **b < 0b1000_0000).count() + 2;
                (arcs, Some(arcs))
            }
        }
    }

    /// Non-default implementation: the exact size is known.
    fn count(mut self) -> usize
        where
            Self: Sized, {
        self.i = self.encoded.len();
        self.size_hint().0
    }

    // FIXME: This is supposed to move the iterator
    /// This implementation optimizes over the default by not decoding any OID
    /// arcs other than the one requested.
    // fn nth(&mut self, n: usize) -> Option<Self::Item> {
    //     // This is a hack: to represent a single root arc, we create an
    //     // invalid OID with the first bit set and a length of 1.
    //     if unlikely(self.i == 0 && self.encoded.len() == 1 && self.encoded[0] > 0b1000_0000) {
    //         if n == 0 {
    //             return Some(max(self.encoded[0] & 0b0111_1111, 2) as u128);
    //         } else {
    //             return None;
    //         }
    //     }

    //     let mut iter_debt = n;
    //     let mut start_of_arc = self.i;

    //     // TODO: I think I have overcomplicated this.
    //     // Just turn the n into a coordinate.


    //     // if n == 0 {
    //     //     let first_byte = self.encoded.get(0)?;
    //     //     if *first_byte >= 80 {
    //     //         return Some(2);
    //     //     } else {
    //     //         return Some((*first_byte / 40) as u128);
    //     //     }
    //     // } else if n == 1 {
    //     //     let first_byte = self.encoded.get(0)?;
    //     //     if *first_byte >= 80 {
    //     //         return Some((first_byte - 80) as u128);
    //     //     } else {
    //     //         return Some((first_byte % 40) as u128);
    //     //     }
    //     // }
    //     for (i, b) in self.encoded[self.i..].iter().enumerate() {
    //         if *b > 0b1000_0000 {
    //             continue;
    //         }
    //         // We hit the end of an encoded arc.
    //         iter_debt = iter_debt.saturating_sub(1);
    //         if self.i == 0 && n == 2 {
    //             // If we skip the first encoded subcomponent, it skips two arcs, not one.
    //             iter_debt = iter_debt.saturating_sub(1);
    //         }
    //         if iter_debt == 0 { // If it's the nth arc, start decoding and return it.
    //             let mut current_node: u128 = 0;
    //             for byte in self.encoded[start_of_arc..i+1].iter() {
    //                 current_node <<= 7;
    //                 current_node += (byte & 0b0111_1111) as u128;
    //             }
    //             // TODO: Handle arc 0 and arc 1
    //             // TODO: Increment self.i

    //             return Some(current_node);
    //         } else {
    //             // Otherwise record the start of the arc as being the next byte.
    //             start_of_arc = self.i + i + 1; // +1 for next byte, +1 since i starts at index 1.
    //         }
    //     }
    //     None
    // }

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
        let len = self.encoded.len();
        if len == 0 {
            return None;
        }
        if len == 1 {
            let first_byte = self.encoded[0];
            if (first_byte & 0b1000_0000) > 0 {
                self.encoded = &[];
                if first_byte >= 80 {
                    return Some(2);
                } else {
                    return Some((first_byte / 40) as u128);
                }
            } else {
                // self.second_arc_popped = true;
                // FIXME:
                if first_byte >= 80 {
                    return Some((first_byte - 80) as u128);
                } else {
                    return Some((first_byte % 40) as u128);
                }
            }
        }
        // TODO: Test an OID like 2.1.525502
        let start_of_last = self.encoded[1..len-1]
            .iter()
            .rposition(|b| *b < 0b1000_0000) // Find the last "end byte"
            .map(|p| p + 1 + 1) // +1 to get the index in array, not slice; +1 to get index of subsequent byte
            .unwrap_or(1); // If we didn't find a concluded previous arc, the first byte ended it.

        // This is to make the reverse iterator respect the progress made by the
        // forward iterator. This is required behavior.
        if start_of_last < self.i {
            return None;
        }

        let mut current_node: u128 = 0;
        for byte in self.encoded[start_of_last..].iter() {
            // TODO: Everywhere you do this, check that the encoded OID is not too large.
            current_node <<= 7;
            current_node += (byte & 0b0111_1111) as u128;
        }
        self.encoded = &self.encoded[0..start_of_last];
        Some(current_node)
    }

    // // FIXME: This is supposed to move the iterator
    // fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
    //     let len = self.encoded.len();
    //     if len == 0 {
    //         return None;
    //     }
    //     if len == 1 {
    //         if n > 2 {
    //             return None;
    //         }
    //         let first_byte = self.encoded[0];
    //         if self.second_arc_popped {
    //             if n > 1 {
    //                 return None;
    //             }
    //             if first_byte >= 80 {
    //                 return Some((first_byte - 80) as u128);
    //             } else {
    //                 return Some((first_byte % 40) as u128);
    //             }
    //         } else if n == 1 {
    //             if first_byte >= 80 {
    //                 return Some((first_byte - 80) as u128);
    //             } else {
    //                 return Some((first_byte % 40) as u128);
    //             }
    //         } else { // n == 0

    //         }
    //     }
    //     todo!()
    // }

}

// TODO: Dedupe by converting into ROID or vice versa
// impl Display for OBJECT_IDENTIFIER {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if unlikely(self.0.len() == 0) {
//             return Ok(());
//         }
//         if cfg!(feature = "itoa") {
//             let mut buf1 = itoa::Buffer::new();
//             f.write_str(buf1.format(self.0[0]))?;
//         } else {
//             f.write_str(self.0[0].to_string().as_str())?;
//         }
//         for arc in self.0[1..].iter() {
//             f.write_char('.')?;
//             if cfg!(feature = "itoa") {
//                 let mut buf1 = itoa::Buffer::new();
//                 f.write_str(buf1.format(*arc))?;
//             } else {
//                 f.write_str(arc.to_string().as_str())?;
//             }
//         }
//         Ok(())
//     }
// }

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


#[macro_export]
macro_rules! oid {
    ( $( $x:expr ),* ) => {
        {
            use $crate::OBJECT_IDENTIFIER;
            OBJECT_IDENTIFIER::from(Vec::<u32>::from([ $($x,)* ]))
        }
    };
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::OBJECT_IDENTIFIER;

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

}
