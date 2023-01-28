use asn1::{OBJECT_IDENTIFIER, ASN1Error, OPTIONAL, UTF8String, BOOLEAN, ASN1Result};
use x500::InformationFramework::{ObjectClassKind, AttributeUsage, AttributeType, RelativeDistinguishedName};
use ldap::AttributeValue as LDAPAttributeValue;
use x690::X690Element;

// MAPPING-BASED-MATCHING
//     {SelectedBy, BOOLEAN:combinable, MappingResult, OBJECT IDENTIFIER:matchingRule} ::=
// CLASS {
//     &selectBy             SelectedBy OPTIONAL,
//     &ApplicableTo         ATTRIBUTE,
//     &subtypesIncluded     BOOLEAN DEFAULT TRUE,
//     &combinable           BOOLEAN(combinable),
//     &mappingResults       MappingResult OPTIONAL,
//     &userControl          BOOLEAN DEFAULT FALSE,
//     &exclusive            BOOLEAN DEFAULT TRUE,
//     &matching-rule        MATCHING-RULE.&id(matchingRule),
//     &id                   OBJECT IDENTIFIER UNIQUE }
// WITH SYNTAX {
//     [SELECT BY            &selectBy]
//     APPLICABLE TO         &ApplicableTo
//     [SUBTYPES INCLUDED    &subtypesIncluded]
//     COMBINABLE            &combinable
//     [MAPPING RESULTS      &mappingResults]
//     [USER CONTROL         &userControl]
//     [EXCLUSIVE            &exclusive]
//     MATCHING RULE         &matching-rule
//     ID                    &id }

// ALGORITHM ::= CLASS {
//     &Type          OPTIONAL,
//     &id            OBJECT IDENTIFIER UNIQUE }
// WITH SYNTAX {
//     [PARMS         &Type]
//     IDENTIFIED BY  &id }

// EXTENSION ::= CLASS {
//     &id           OBJECT IDENTIFIER UNIQUE,
//     &ExtnType }
// WITH SYNTAX {
//     SYNTAX        &ExtnType
//     IDENTIFIED BY &id }

// pub trait X500SchemaKnowledgeBase <X500ValueType: Sized, LDAPCodecError: std::error::Error>
//     where Self: Sized {
//     fn get_object_class_by_id (id: OBJECT_IDENTIFIER) -> Option<ObjectClassInfo>;
//     fn get_object_class_by_name (name: &str) -> Option<ObjectClassInfo>;
//     fn get_name_form_by_id (id: OBJECT_IDENTIFIER) -> Option<NameFormInfo>;
//     fn get_name_form_by_name (name: &str) -> Option<NameFormInfo>;
//     fn get_attribute_type_by_id (id: OBJECT_IDENTIFIER) -> Option<AttributeTypeInfo<Self, X500ValueType, X500ValueType>>;
//     fn get_attribute_type_by_name (name: &str) -> Option<AttributeTypeInfo<Self, X500ValueType, X500ValueType>>;
//     fn get_equality_matching_rule_by_id (id: OBJECT_IDENTIFIER) -> Option<MatchingRuleInfo<Self, X500ValueType, X500ValueType, EqualityMatcher<Self, X500ValueType, X500ValueType>>>;
//     fn get_equality_matching_rule_by_name (name: &str) -> Option<MatchingRuleInfo<Self, X500ValueType, X500ValueType, EqualityMatcher<Self, X500ValueType, X500ValueType>>>;
//     fn get_ordering_matching_rule_by_id (id: OBJECT_IDENTIFIER) -> Option<MatchingRuleInfo<Self, X500ValueType, X500ValueType, OrderingMatcher<Self, X500ValueType, X500ValueType>>>;
//     fn get_ordering_matching_rule_by_name (name: &str) -> Option<MatchingRuleInfo<Self, X500ValueType, X500ValueType, OrderingMatcher<Self, X500ValueType, X500ValueType>>>;
//     fn get_substring_matching_rule_by_id (id: OBJECT_IDENTIFIER) -> Option<MatchingRuleInfo<Self, X500ValueType, X500ValueType, SubstringsMatcher<Self, X500ValueType, X500ValueType>>>;
//     fn get_substring_matching_rule_by_name (name: &str) -> Option<MatchingRuleInfo<Self, X500ValueType, X500ValueType, SubstringsMatcher<Self, X500ValueType, X500ValueType>>>;
//     fn get_context_type_by_id (id: OBJECT_IDENTIFIER) -> Option<ContextTypeInfo<Self, X500ValueType, X500ValueType>>;
//     fn get_ldap_syntax_by_id (id: OBJECT_IDENTIFIER) -> Option<LdapSyntaxInfo<Self, X500ValueType, LDAPCodecError>>;
//     fn get_ldap_syntax_by_name (name: &str) -> Option<LdapSyntaxInfo<Self, X500ValueType, LDAPCodecError>>;
// }

// #[derive(Debug)]
// pub enum MatchingError {
//     InvalidAssertion(ASN1Error),
//     InvalidValue(ASN1Error),
//     IoError(std::io::Error),
// }

// // TODO:
// // impl std::error::Error for MatchingError {}

// // TODO: Is this being this generic really worth it?
// pub type EqualityMatcher <S, A, V> = fn(S, A, V) -> Result<Option<bool>, MatchingError>;
// pub type OrderingMatcher <S, A, V> = fn(S, A, V) -> Result<Option<u64>, MatchingError>;
// pub type SubstringsMatcher <S, A, V> = fn(S, A, V) -> Result<bool, MatchingError>;
// pub type ApproxMatcher <S, A, V> = fn(S, A, V, u8) -> Result<Option<bool>, MatchingError>; // TODO: IDK if this makes sense.
// pub type ValueValidator <V> = fn(V) -> ASN1Result<()>;
// pub type ContextValueMatcher <S, A, V> = fn(S, A, V) -> Result<Option<bool>, MatchingError>;

// #[derive(Debug, Clone)]
// pub struct ObjectClassInfo {
//     pub superclasses: OPTIONAL<Vec<ObjectClassInfo>>,
//     pub kind: ObjectClassKind,
//     pub mandatory_attributes: OPTIONAL<Vec<AttributeType>>,
//     pub optional_attributes: OPTIONAL<Vec<AttributeType>>,
//     pub ldap_name: OPTIONAL<Vec<UTF8String>>,
//     pub ldap_desc: OPTIONAL<UTF8String>,
//     pub id: OBJECT_IDENTIFIER,
// }

// pub struct MatchingRuleInfo <S : X500SchemaKnowledgeBase<V>, A: Sized, V: Sized, MatcherType> {
//     pub parent_matching_rules: OPTIONAL<Vec<MatchingRuleInfo<S, A, V, MatcherType>>>,
//     pub unique_match_indicator: OPTIONAL<AttributeTypeInfo<S, A, V>>,
//     pub ldap_syntax: OPTIONAL<OBJECT_IDENTIFIER>,
//     pub ldap_name: OPTIONAL<Vec<UTF8String>>,
//     pub ldap_desc: OPTIONAL<UTF8String>,
//     pub id: OBJECT_IDENTIFIER,
//     pub matcher: MatcherType,
// }

// pub struct AttributeTypeInfo <S : X500SchemaKnowledgeBase<Type>, A: Sized, Type: Sized> {
//     pub derivation: OPTIONAL<Box<AttributeTypeInfo<S, A, Type>>>,
//     pub equality_match: OPTIONAL<Box<MatchingRuleInfo<S, A, Type, EqualityMatcher<S, A, Type>>>>,
//     pub ordering_match: OPTIONAL<Box<MatchingRuleInfo<S, A, Type, OrderingMatcher<S, A, Type>>>>,
//     pub substrings_match: OPTIONAL<Box<MatchingRuleInfo<S, A, Type, SubstringsMatcher<S, A, Type>>>>,
//     pub single_valued: BOOLEAN,
//     pub collective: BOOLEAN,
//     pub dummy: BOOLEAN,
//     pub no_user_modification: BOOLEAN,
//     pub usage: AttributeUsage,
//     pub ldap_syntax: OPTIONAL<OBJECT_IDENTIFIER>,
//     pub ldap_name: OPTIONAL<Vec<UTF8String>>,
//     pub ldap_desc: OPTIONAL<UTF8String>,
//     pub obsolete: BOOLEAN,
//     pub id: OBJECT_IDENTIFIER,

//     /// A function that takes a value of this type and returns Ok(()) if it is syntatically valid.
//     pub validator: OPTIONAL<ValueValidator<Type>>,
// }

// #[derive(Debug, Clone)]
// pub struct NameFormInfo {
//     pub named_object_class: OBJECT_IDENTIFIER,
//     pub mandatory_attributes: Vec<AttributeType>,
//     pub optional_attributes: OPTIONAL<Vec<AttributeType>>,
//     pub ldap_name: OPTIONAL<Vec<UTF8String>>,
//     pub ldap_desc: OPTIONAL<UTF8String>,
//     pub id: OBJECT_IDENTIFIER,
// }

// pub struct ContextTypeInfo <S : X500SchemaKnowledgeBase<Type>, Assertion: Sized, Type: Sized> {
//     pub absent_match: BOOLEAN,
//     pub id: OBJECT_IDENTIFIER,
//     pub matcher: ContextValueMatcher<S, Assertion, Type>,
//     pub default_value: OPTIONAL<fn() -> Type>,
//     pub normalize: OPTIONAL<fn(Type) -> Type>,
// }

// pub struct LdapSyntaxInfo <S : X500SchemaKnowledgeBase<X500ValueType>, X500ValueType: Sized, E> {
//     pub ldap_desc: UTF8String,
//     pub id: OBJECT_IDENTIFIER,
//     pub convert_x500_value_to_ldap_value: fn(S, X500ValueType) -> Result<LDAPAttributeValue, E>,
//     pub convert_ldap_value_to_x500_value: fn(S, LDAPAttributeValue) -> Result<X500ValueType, E>,
// }




pub trait X500SchemaKnowledgeBase
    where Self: Sized {
    fn get_object_class_by_id (id: OBJECT_IDENTIFIER) -> Option<ObjectClassInfo>;
    fn get_object_class_by_name (name: &str) -> Option<ObjectClassInfo>;
    fn get_name_form_by_id (id: OBJECT_IDENTIFIER) -> Option<NameFormInfo>;
    fn get_name_form_by_name (name: &str) -> Option<NameFormInfo>;
    fn get_attribute_type_by_id (id: OBJECT_IDENTIFIER) -> Option<AttributeTypeInfo<Self>>;
    fn get_attribute_type_by_name (name: &str) -> Option<AttributeTypeInfo<Self>>;
    fn get_equality_matching_rule_by_id (id: OBJECT_IDENTIFIER) -> Option<MatchingRuleInfo<Self, EqualityMatcher<Self>>>;
    fn get_equality_matching_rule_by_name (name: &str) -> Option<MatchingRuleInfo<Self, EqualityMatcher<Self>>>;
    fn get_ordering_matching_rule_by_id (id: OBJECT_IDENTIFIER) -> Option<MatchingRuleInfo<Self, OrderingMatcher<Self>>>;
    fn get_ordering_matching_rule_by_name (name: &str) -> Option<MatchingRuleInfo<Self, OrderingMatcher<Self>>>;
    fn get_substring_matching_rule_by_id (id: OBJECT_IDENTIFIER) -> Option<MatchingRuleInfo<Self, SubstringsMatcher<Self>>>;
    fn get_substring_matching_rule_by_name (name: &str) -> Option<MatchingRuleInfo<Self, SubstringsMatcher<Self>>>;
    fn get_context_type_by_id (id: OBJECT_IDENTIFIER) -> Option<ContextTypeInfo<Self>>;
    fn get_ldap_syntax_by_id (id: OBJECT_IDENTIFIER) -> Option<LdapSyntaxInfo<Self>>;
    fn get_ldap_syntax_by_name (name: &str) -> Option<LdapSyntaxInfo<Self>>;
}

#[derive(Debug)]
pub enum MatchingError {
    InvalidAssertion(ASN1Error),
    InvalidValue(ASN1Error),
    IoError(std::io::Error),
}

// TODO:
// impl std::error::Error for MatchingError {}

// TODO: Should you define an alias for X690Element, such as `DirectoryValue`?

pub type EqualityMatcher <S> = fn(S, X690Element, X690Element) -> Result<Option<bool>, MatchingError>;
pub type OrderingMatcher <S> = fn(S, X690Element, X690Element) -> Result<Option<u64>, MatchingError>;
pub type SubstringsMatcher <S> = fn(S, X690Element, X690Element) -> Result<bool, MatchingError>;
pub type ApproxMatcher <S> = fn(S, u8) -> Result<Option<bool>, MatchingError>; // TODO: IDK if this makes sense.
pub type ValueValidator <V> = fn(V) -> ASN1Result<()>;
pub type ContextValueMatcher <S> = fn(S, X690Element, X690Element) -> Result<Option<bool>, MatchingError>;

#[derive(Debug, Clone)]
pub struct ObjectClassInfo {
    pub superclasses: OPTIONAL<Vec<ObjectClassInfo>>,
    pub kind: ObjectClassKind,
    pub mandatory_attributes: OPTIONAL<Vec<AttributeType>>,
    pub optional_attributes: OPTIONAL<Vec<AttributeType>>,
    pub ldap_name: OPTIONAL<Vec<UTF8String>>,
    pub ldap_desc: OPTIONAL<UTF8String>,
    pub id: OBJECT_IDENTIFIER,
}

pub struct MatchingRuleInfo <S : X500SchemaKnowledgeBase, MatcherType> {
    pub parent_matching_rules: OPTIONAL<Vec<MatchingRuleInfo<S, MatcherType>>>,
    pub unique_match_indicator: OPTIONAL<AttributeTypeInfo<S>>,
    pub ldap_syntax: OPTIONAL<OBJECT_IDENTIFIER>,
    pub ldap_name: OPTIONAL<Vec<UTF8String>>,
    pub ldap_desc: OPTIONAL<UTF8String>,
    pub id: OBJECT_IDENTIFIER,
    pub matcher: MatcherType,
}

pub struct AttributeTypeInfo <S : X500SchemaKnowledgeBase> {
    pub derivation: OPTIONAL<Box<AttributeTypeInfo<S>>>,
    pub equality_match: OPTIONAL<Box<MatchingRuleInfo<S, EqualityMatcher<S>>>>,
    pub ordering_match: OPTIONAL<Box<MatchingRuleInfo<S, OrderingMatcher<S>>>>,
    pub substrings_match: OPTIONAL<Box<MatchingRuleInfo<S, SubstringsMatcher<S>>>>,
    pub single_valued: BOOLEAN,
    pub collective: BOOLEAN,
    pub dummy: BOOLEAN,
    pub no_user_modification: BOOLEAN,
    pub usage: AttributeUsage,
    pub ldap_syntax: OPTIONAL<OBJECT_IDENTIFIER>,
    pub ldap_name: OPTIONAL<Vec<UTF8String>>,
    pub ldap_desc: OPTIONAL<UTF8String>,
    pub obsolete: BOOLEAN,
    pub id: OBJECT_IDENTIFIER,

    /// A function that takes a value of this type and returns Ok(()) if it is syntatically valid.
    pub validator: OPTIONAL<ValueValidator<X690Element>>,
}

#[derive(Debug, Clone)]
pub struct NameFormInfo {
    pub named_object_class: OBJECT_IDENTIFIER,
    pub mandatory_attributes: Vec<AttributeType>,
    pub optional_attributes: OPTIONAL<Vec<AttributeType>>,
    pub ldap_name: OPTIONAL<Vec<UTF8String>>,
    pub ldap_desc: OPTIONAL<UTF8String>,
    pub id: OBJECT_IDENTIFIER,
}

pub struct ContextTypeInfo <S : X500SchemaKnowledgeBase> {
    pub absent_match: BOOLEAN,
    pub id: OBJECT_IDENTIFIER,
    pub matcher: ContextValueMatcher<S>,
    pub default_value: OPTIONAL<fn() -> X690Element>,
    pub normalize: OPTIONAL<fn(X690Element) -> X690Element>,
}

pub struct LdapSyntaxInfo <S : X500SchemaKnowledgeBase> {
    pub ldap_desc: UTF8String,
    pub id: OBJECT_IDENTIFIER,

    // TODO: Are these really the best error types for this purpose?
    pub convert_x500_value_to_ldap_value: fn(S, X690Element) -> Result<LDAPAttributeValue, ASN1Error>,
    pub convert_ldap_value_to_x500_value: fn(S, LDAPAttributeValue) -> Result<X690Element, ASN1Error>,
}

pub struct Vertex<T> {
    pub superior: Option<T>,
    pub item: T,
}

pub type DseDatabaseId = u32;

pub struct DSE {
    pub id: DseDatabaseId,
    pub rdn: RelativeDistinguishedName,
}

pub type DITVertex = Vertex<DSE>;

/*

Format of values stored in ReDB:

RDNs Table:
Key: | u32 superior entry ID | DER-encoded, normalized RDN |
Value: | u32 DSE ID | Alias bit, FromEntry bit | (Byte including the two bits, list operations can run with a single query*)
*Then again, you still have to query ACI, object classes, DSE type, and probably more to actually perform a list anyway.
*Still, having the whole DSE type in this table might save a lot of computing. DSE type would be a 24-bit field. Subentries could be ignored, for instance.

DSEs Table:
Key: u32 entry ID
Value: ?
  subordinate_completeness Boolean?
  attribute_completeness   Boolean?
  subordinates_driver u16 or u32? (Probably u16)
You could probably get rid of this table entirely if you just define operational attributes for the above values.

Format of values:
Key: | Entry ID u32 | 1 byte: type OID length, N | N bytes: DER-encoded type OID, reversed* | X.690 tag class + constructed flag | X.690 tag number | DER-encoded, normalized value
Value: Non-normalized value with contexts
*Reversing gives better performance because the last arc varies more than the first.
Does this table need to have a bit flag for operational?
The key innately limits the value size. However, this might not be a problem,
because directories are not good with handling large values anyway. You might
WANT a low limit (e.g. 65535 bytes) on value sizes.
Values that don't have a normalizer will just have to be queried by attribute, then filtered in-memory.

This seems like it could be really simple and elegant.

*/
