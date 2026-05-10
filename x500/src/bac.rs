use wildboar_asn1::{BIT_STRING, OBJECT_IDENTIFIER, read_i64};
use crate::{BasicAccessControl::{
    AuthenticationLevel, GrantsAndDenials, ProtectedItems, UserClasses, ACIItem,
    ACIItem_itemOrUserFirst
}};
use crate::InformationFramework::{AttributeTypeAndValue, Context};
use std::{iter::{ExactSizeIterator, FusedIterator, Iterator}};

// TODO: Implement Display for this
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecognizedGrantsAndDenials(pub u32);

impl RecognizedGrantsAndDenials {

    /// Determine whether permission bits involve granting access
    ///
    /// This is part of the basic access control model described in
    /// [ITU-T Recommendation X.501 (2019)](https://www.itu.int/rec/T-REC-X.501/en)
    /// in section 18.4.2.4.
    ///
    /// Returns `true` if the permissions involve granting access.
    pub fn grants_access(&self) -> bool {
        (self.0 & 0b10101010_10101010_10101010_10101010) > 0
    }

    /// Determine whether permission bits involve denying access
    ///
    /// This is part of the basic access control model described in
    /// [ITU-T Recommendation X.501 (2019)](https://www.itu.int/rec/T-REC-X.501/en)
    /// in section 18.4.2.4.
    ///
    /// Returns `true` if the permissions involve denying access.
    pub fn denies_access(&self) -> bool {
        (self.0 & 0b01010101_01010101_01010101_01010101) > 0
    }

    /// Convert this back into a `BIT STRING`
    pub fn to_bit_string(&self) -> BIT_STRING {
        BIT_STRING::from_bytes(self.0.to_be_bytes().to_vec())
    }

    /// Split into grants and denials, respectively
    pub fn split_into_grants_and_denials(&self) -> (Self, Self) {
        (
            RecognizedGrantsAndDenials(self.0 & 0b10101010_10101010_10101010_10101010),
            RecognizedGrantsAndDenials(self.0 & 0b01010101_01010101_01010101_01010101),
        )
    }

    pub fn check(&self, permission: &PermissionCategory) -> bool {
        let s: u8 = *permission as u8;
        let grant_mask: u32 = 0x8000_0000 >> (s << 1);
        let deny_mask: u32 = 0x8000_0000 >> ((s << 1) + 1);
        let grant_bit_set: bool = (self.0 & grant_mask) > 0;
        let deny_bit_clear: bool = (self.0 & deny_mask) == 0;
        grant_bit_set && deny_bit_clear
    }

}

impl From<&GrantsAndDenials> for RecognizedGrantsAndDenials {

    fn from(value: &GrantsAndDenials) -> Self {
        let b = value.get_bytes_ref();
        // let tb = value.get_trailing_bits_count();
        let x: u32 = match b.len() {
            0 => 0,
            1 => u32::from_be_bytes([ b[0], 0, 0, 0 ]),
            2 => u32::from_be_bytes([ b[0], b[1], 0, 0 ]),
            3 => u32::from_be_bytes([ b[0], b[1], b[2], 0 ]),
            _ => u32::from_be_bytes([ b[0], b[1], b[2], b[3] ]),
        };
        let shift = value.len_in_bits() as u32;
        let mask: u32 = !(0xFFFFFFFFu32.checked_shr(shift).unwrap_or(0));
        RecognizedGrantsAndDenials(x & mask)
    }

}

// impl Display for RecognizedGrantsAndDenials {

//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }

// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UserClassSpecificity {
    /// Did not match
    NonMatch = 0,
    /// Did not match, but unknown if member in group
    NonMatchMaybeInGroup = 5,
    /// Matched `allUsers`
    AllUsers = 10,
    /// Matched `allUsers`, and unknown if member in group
    AllUsersMaybeInGroup = 15,
    /// Matched `subtree`
    Subtree = 20,
    /// Matched `subtree`, and unknown if member in group
    SubtreeMaybeInGroup = 25,
    /// Matched `userGroup`
    UserGroup = 30,
    /// Matched `thisEntry` or `name`
    NamedEntry = 40,
    /// Per ITU Recommendation X.501, Section 18.8.3, bullet point 1.2, if
    /// the specified authentication level is higher than that of the user
    /// and the tuple denies access, we need to include this tuple.
    ///
    /// However, it is unclear in the specification how this should "count"
    /// when comparing the specificity of user classes. Since we presumably
    /// want this to _always_ count, we should return the highest value
    /// possible, which means that this tuple will override all others, at
    /// least in terms of user-class specificity.
    InclusivelySpecific = 50,
}

/// The Access Control Decision Function (ACDF) tuple type documented in
/// ITU Recommendation X.501 (2019), Section 18.8.2.
#[derive(Debug, Clone)]
pub struct ACDFTuple <'a> {
    pub user_classes: &'a UserClasses,
    pub auth_level: &'a AuthenticationLevel,
    pub prot_items: &'a ProtectedItems,
    pub grants_and_denials: RecognizedGrantsAndDenials,
    pub precedence: usize,
    /// `None` variant means "Unknown"
    pub user_class_specificity: Option<UserClassSpecificity>,
}

/// Iterator over Access Control Decision Function (ACDF) Tuples
///
/// The Access Control Decision Function (ACDF) tuple type is documented in
/// ITU Recommendation X.501 (2019), Section 18.8.2. It is used for implementing
/// the Basic Access Control (BAC) scheme.
///
/// There is no point in implementing `DoubleEndedIterator`, though it could be
/// done somewhat easily. It would only complicate and slow down the forward
/// iterator implementation.
pub struct ACDFTuplesIterator <'a> {
    pub(crate) aci: &'a ACIItem,
    pub(crate) i: usize,
}

impl <'a> ACDFTuplesIterator<'a> {

    pub fn new (aci: &'a ACIItem) -> Self {
        ACDFTuplesIterator{
            aci,
            i: 0,
        }
    }

}

impl <'a> Iterator for ACDFTuplesIterator<'a> {
    type Item = ACDFTuple<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.aci.itemOrUserFirst {
            ACIItem_itemOrUserFirst::userFirst(userf) => {
                let user = userf.userPermissions.get(self.i)?;
                self.i = self.i.saturating_add(1);
                let raw_precedence = user.precedence
                    .as_ref()
                    .unwrap_or(self.aci.precedence.as_ref());
                let precedence: i64 = read_i64(raw_precedence.as_slice()).unwrap_or(i64::MAX);
                let precedence: usize = precedence.try_into().unwrap_or(usize::MAX);
                Some(ACDFTuple{
                    user_classes: &userf.userClasses,
                    auth_level: &self.aci.authenticationLevel,
                    grants_and_denials: RecognizedGrantsAndDenials::from(&user.grantsAndDenials),
                    prot_items: &user.protectedItems,
                    precedence,
                    user_class_specificity: None,
                })
            },
            ACIItem_itemOrUserFirst::itemFirst(itemf) => {
                let item = itemf.itemPermissions.get(self.i)?;
                self.i = self.i.saturating_add(1);
                let raw_precedence = item.precedence
                    .as_ref()
                    .unwrap_or(self.aci.precedence.as_ref());
                let precedence: i64 = read_i64(raw_precedence.as_slice()).unwrap_or(i64::MAX);
                let precedence: usize = precedence.try_into().unwrap_or(usize::MAX);
                Some(ACDFTuple{
                    user_classes: &item.userClasses,
                    auth_level: &self.aci.authenticationLevel,
                    grants_and_denials: RecognizedGrantsAndDenials::from(&item.grantsAndDenials),
                    prot_items: &itemf.protectedItems,
                    precedence,
                    user_class_specificity: None,
                })
            },
            ACIItem_itemOrUserFirst::_unrecognized(_) => return None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact_len = match &self.aci.itemOrUserFirst {
            ACIItem_itemOrUserFirst::userFirst(userf) => userf.userPermissions.len(),
            ACIItem_itemOrUserFirst::itemFirst(itemf) => itemf.itemPermissions.len(),
            ACIItem_itemOrUserFirst::_unrecognized(_) => 0,
        };
        (exact_len, Some(exact_len))
    }

}

impl <'a> FusedIterator for ACDFTuplesIterator<'a> {}
impl <'a> ExactSizeIterator for ACDFTuplesIterator<'a> {}

/// Determine whether permission bits involve denying access
///
/// This is part of the basic access control model described in
/// [ITU-T Recommendation X.501 (2019)](https://www.itu.int/rec/T-REC-X.501/en)
/// in section 18.4.2.4.
///
/// Returns `true` if the permissions involve denying access.
pub fn denies_access (permissions: &GrantsAndDenials) -> bool {
    if let Some(last_byte) = permissions.get_bytes_ref().last() {
        let trailing_bits = permissions.get_trailing_bits_count();
        let mask: u8 = 0xFF << (8 - trailing_bits);
        let last_bits = *last_byte & mask;
        if (last_bits & 0b0101_0101) > 0 {
            return true;
        }
    } else {
        return false;
    }
    let bytes = permissions.get_bytes_ref();
    debug_assert!(bytes.len() >= 1);
    for b in bytes[0..bytes.len() - 1].iter() {
        if (*b & 0b0101_0101) > 0 {
            return true;
        }
    }
    false
}

/// Determine whether permission bits involve granting access
///
/// This is part of the basic access control model described in
/// [ITU-T Recommendation X.501 (2019)](https://www.itu.int/rec/T-REC-X.501/en)
/// in section 18.4.2.4.
///
/// Returns `true` if the permissions involve granting access.
pub fn grants_access (permissions: &GrantsAndDenials) -> bool {
    if let Some(last_byte) = permissions.get_bytes_ref().last() {
        let trailing_bits = permissions.get_trailing_bits_count();
        let mask: u8 = 0xFF << (8 - trailing_bits);
        let last_bits = *last_byte & mask;
        if (last_bits & 0b1010_1010) > 0 {
            return true;
        }
    } else {
        return false;
    }
    let bytes = permissions.get_bytes_ref();
    debug_assert!(bytes.len() >= 1);
    for b in bytes[0..bytes.len() - 1].iter() {
        if (*b & 0b1010_1010) > 0 {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone)]
pub struct ProtectedEntry {
    pub entry: OBJECT_IDENTIFIER,
    /// for `maxImmSub` support
    pub siblings_count: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ProtectedAttributeType {
    pub attribute_type: OBJECT_IDENTIFIER,
    /// for maxValueCount support
    pub values_count: Option<u64>,
    pub operational: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ProtectedAttributeValue {
    pub value: AttributeTypeAndValue,
    pub contexts: Option<Vec<Context>>,
    pub operational: Option<bool>,
}

/// Some data protected by an X.500 directory
///
/// From ITU Recommendation X.501 (2016), Section 18.2.1:
///
/// > The protected items of the Directory are entries, attributes, attribute
/// > values and names.
///
#[derive(Debug, Clone)]
pub enum ProtectedData {
    Entry(ProtectedEntry),
    AttributeType(ProtectedAttributeType),
    AttributeValue(ProtectedAttributeValue),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PermissionCategory {
    Add = 0,
    DiscloseOnError = 1,
    Read = 2,
    Remove = 3,
    Browse = 4,
    Export = 5,
    Import = 6,
    Modify = 7,
    Rename = 8,
    ReturnDN = 9,
    Compare = 10,
    FilterMatch = 11,
    Invoke = 12,
}

// https://github.com/Wildboar-Software/asn1-typescript-libraries/blob/master/packages/x500/src/lib/bac/operationPermitted.mts


// This looks like a bug:
// https://github.com/Wildboar-Software/asn1-typescript-libraries/blob/b91387a54018aca395c25f6e027a44308496221c/packages/x500/src/lib/bac/splitGrantsAndDenials.mts#L32
// Also a bug by not being the right number.

/// Check if at least one ACDF tuple grants access and none deny
///
/// This is the last step of the Basic Access Control (BAC) Access Control
/// Decision Function (ACDF): this function evaluates whether the
/// operation is permitted by checking that at least one relevant ACDF tuple
/// grants access and that none deny access.
///
/// Returns `true` if the operation is permitted,
///
/// ## Arguments
///
/// - `operations` - An array of permission categories that describe the operation
/// - `tuple` - The Access Control Decision Function (ACDF) tuple.
/// - `request` - The request
pub fn operation_is_permitted(
    operations: &[PermissionCategory],
    tuple: ACDFTuple,
    request: ProtectedData,
) -> bool {
    if tuple.user_class_specificity.is_none() {
        return false;
    }
    let user_class_specificity = tuple.user_class_specificity.unwrap();
    /* Section 18.4.2.5.a of ITU Recommendation X.501 (2016) states that, if
    group membership cannot be determined, a user should be assumed to be a
    member of that group if membership is used to deny access, and the user
    should be assumed to _not_ be a member of that group if membership is
    used to grant access. So here, if membership is the only means by which the
    user matches, and even that could not be determined, we return `false` to
    deny access. */
    let no_confirmed_user_match = [
        UserClassSpecificity::NonMatch,
        UserClassSpecificity::NonMatchMaybeInGroup,
    ].contains(&user_class_specificity);
    if no_confirmed_user_match {
        return false;
    }


    let mut adds: bool = false;
    let mut imports: bool = false;
    for op in operations {
        if *op == PermissionCategory::Add {
            adds = true;
        } else if *op == PermissionCategory::Import {
            imports = true;
        }
    }

    // Check if the user exceeded maxImmSub.
    if let ProtectedData::Entry(pe) = &request {
        let max_imm_sub: u64 = tuple
            .prot_items
            .maxImmSub
            .as_ref()
            .map(|m| std::cmp::max(read_i64(m.as_slice()).unwrap_or(i64::MAX), 0) as u64)
            .unwrap_or(u64::MAX);
        let violated_max_subordinates = match pe.siblings_count {
            Some(s) => s > max_imm_sub,
            _ => false,
        };
        if (adds || imports) && violated_max_subordinates {
            return false;
        }
    }

    // Check if the user exceeded maxValueCount.
    if let ProtectedData::AttributeType(at) = &request {
        let max_count: u64 = tuple
            .prot_items
            .maxValueCount
            .as_ref()
            .map(|mvcs| mvcs
                .iter()
                .find(|mvc| mvc.type_ == at.attribute_type)
                .map(|mvc| std::cmp::max(read_i64(mvc.maxCount.as_slice()).unwrap_or(i64::MAX), 0) as u64)
            )
            .flatten()
            .unwrap_or(u64::MAX);
        let violated_max_values = match at.values_count {
            Some(v) => v > max_count,
            _ => false,
        };
        if adds && violated_max_values {
            return false;
        }
    }

    for op in operations {
        if !tuple.grants_and_denials.check(op) {
            return false;
        }
    }
    return true;
}
