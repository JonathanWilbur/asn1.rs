# X.690 Encoding Rules

This library implements the encoding rules defined in
[ITU Recommendation X.690 (2021)](https://www.itu.int/rec/T-REC-X.690/en),
namely:

- The Basic Encoding Rules (BER)
- The Distinguished Encoding Rules (DER)
- ~~The Canonical Encoding Rules (CER)~~ (Currently unsupported)

Much of the functionality of this crate depends upon
[`wildboar-asn1`](https://crates.io/crates/wildboar-asn1), which is a "core"
crate for all aspects of ASN.1 that are mostly "codec-agnostic" so that it can
be used by other crates for implementing the Packed Encoding Rules,
Octet Encoding Rules, BACNet, etc.

## Usage

The ITU-T Recommendation X.690 codecs use Tag-Length-Value (TLV) triplets to
encode ASN.1 values. These triplets are encoded on an integral number of bytes
(what the specifications call "octets" for disambiguation). In this library,
this TLV triplet is referred to as an `X690Element`, and generally, throughout
the library, the word "element" refers to one of these triplets.

The `X690Element` itself may be composed of constituent `X690Element`s, allowing
for the representation of "structured types."

For your convenience, the `X690Element` is:

```rust,ignore
pub struct X690Element {
    /// The tag identifying the type and class of this element
    pub tag: Tag,
    /// The value content of this element
    pub value: X690Value,
}
```

The length is determined at encoding time by the `value`. It is not stored
anywhere, but you can read it using `X690Value::len()`.

The `X690Value`, which represents the value octets ("content octets") of the
TLV triplet is defined like:

```rust,ignore
pub enum X690Value {
    /// A primitive value stored as raw bytes
    Primitive(Bytes),
    /// A constructed value containing child elements
    Constructed(Arc<Vec<X690Element>>),
    /// A value that has been serialized to bytes (for lazy decoding or faster encoding)
    Serialized(Bytes),
}
```

If the `Serialized` alternative is used, the tag, length, and value can be
written in one single call to `write()` when used with `x690_write_tlv()`. This
is a performance hack that exists only for when you are serializing trustworthy
data.

You can serialize `X690Element`s to bytes using `x690_write_tlv()`. To read
`X690Element`s from bytes, you must use one of the codec-specific methods:
`X690Codec::decode_from_slice()` or or `X690Codec::decode_from_bytes()`.

### Codec API

The Basic Encoding Rules (BER) and Distinguished Encoding Rules (DER) are
implemented as structs that implement the `X690Codec` trait. These structs
currently do not have any fields, but they may, in the future, to support
Encoding Control Notation (ECN), if that is ever supported.

This crate exports `BER` and `DER`, which are compile-time constructed instances
of the `BasicEncodingRules` and `DistinguishedEncodingRules` respectively. You
can import them and invoke them like so:

```rust,ignore
let b = true;
let el: X690Element = BER.encode_boolean(&b)?;
let decoded = BER.decode_boolean(&el)?;
assert_eq!(b, decoded);
```

### Functional API

This library also provides some functionality independently of any codec,
simply for ease-of-use. Most of these start with `x690_read_*` or
`x690_write_*` for decoding and encoding, respectively. These functions try to
encode in such a way that is compliant with DER, but no such guarantees are
made.

### Parsing API - for constructed types

For structured types:

| Use...                    | ...to parse         |
|===========================|=====================|
| `_parse_set()`            | `SET`               |
| `_parse_sequence()`       | `SEQUENCE`          |
| `_decode_sequence_of()`   | `SEQUENCE OF`       |
| `_decode_set_of()`        | `SET OF`            |
| `_encode_explicit()`      | `EXPLICIT` tagged   |
| `_encode_implicit()`      | `IMPLICIT` tagged   |

### Feature Flags

- `der` - Enable the Distinguished Encoding Rules
- `simdutf8` - Use SIMD instructions to accelerate UTF-8 operations
- `likely_stable` - Use branch prediction hint macros so your CPU speculatively
  executes the branches more likely to be taken.

The feature flags have not been benchmarked thus far. It is unknown if they
actually improve performance.

The above functions start with underscores just because they

## Examples

### Encoding, Decoding, and Validating `AlgorithmIdentifier` (a `SEQUENCE`)

This data structure is used in X.509 certificates and in many other
cryptographic artifacts.

```rust
use wildboar_asn1::*;
use x690::*;

struct AlgorithmIdentifier {
    pub algorithm: OBJECT_IDENTIFIER,
    pub parameters: Option<ASN1Value>,
}

const _rctl1_components_for_AlgorithmIdentifier: &[ComponentSpec; 2] = &[
    ComponentSpec::new(
        "algorithm",
        false,
        TagSelector::tag((
            TagClass::UNIVERSAL,
            UNIV_TAG_OBJECT_IDENTIFIER,
        )),
        None,
        None,
    ),
    ComponentSpec::new("parameters", true, TagSelector::any, None, None),
];

const _eal_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];
const _rctl2_components_for_AlgorithmIdentifier: &[ComponentSpec; 0] = &[];

fn decode_AlgorithmIdentifier(el: &X690Element) -> ASN1Result<AlgorithmIdentifier> {
    let elements = el.components()?;
    let seq_iter = X690StructureIterator::new(
        &elements,
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
    );
    let mut maybe_algorithm: Option<OBJECT_IDENTIFIER> = None;
    let mut parameters: Option<ASN1Value> = None;
    for (i, component) in seq_iter.enumerate() {
        match component.unwrap() {
            "algorithm" => {
                maybe_algorithm = Some(DER.decode_object_identifier(&elements[i])?);
            },
            "parameters" => {
                parameters = Some(DER.decode_any(&elements[i])?);
            }
            _ => {
                return Err(ASN1Error::new(ASN1ErrorCode::unrecognized_components_in_inextensible_type));
            }
        }
    }
    let algorithm: OBJECT_IDENTIFIER = maybe_algorithm.unwrap();
    let parameters: Option<ASN1Value> = parameters;
    Ok(AlgorithmIdentifier {
        algorithm,
        parameters,
    })
}

pub fn encode_AlgorithmIdentifier(value_: &AlgorithmIdentifier) -> ASN1Result<X690Element> {
    let mut components_: Vec<X690Element> = Vec::with_capacity(12);
    components_.push(DER.encode_object_identifier(&value_.algorithm)?);
    if let Some(v_) = &value_.parameters {
        components_.push(x690_identity(&v_)?);
    }
    Ok(X690Element::new(
        Tag::new(TagClass::UNIVERSAL, UNIV_TAG_SEQUENCE),
        X690Value::Constructed(Arc::new(
            [components_, value_._unrecognized.clone()].concat(),
        )),
    ))
}

pub fn validate_AlgorithmIdentifier(el: &X690Element) -> ASN1Result<()> {
    let _elements = match &el.value {
        X690Value::Constructed(children) => children,
        _ => {
            return Err(
                el.to_asn1_err_named(ASN1ErrorCode::invalid_construction, "AlgorithmIdentifier")
            )
        }
    };
    let _seq_iter = X690StructureIterator::new(
        _elements.as_slice(),
        _rctl1_components_for_AlgorithmIdentifier,
        _eal_components_for_AlgorithmIdentifier,
        _rctl2_components_for_AlgorithmIdentifier,
    );
    let mut _i: usize = 0;
    for _fallible_component_name in _seq_iter {
        let _component_name = _fallible_component_name?;
        let _maybe_el = _elements.get(_i);
        _i += 1;
        let _el = _maybe_el.unwrap();
        match _component_name {
            "algorithm" => DER.validate_object_identifier(_el)?,
            "parameters" => DER.validate_any(_el)?,
            _ => (),
        }
    }
    Ok(())
}

let alg = AlgorithmIdentifier{
  algorithm: OBJECT_IDENTIFIER::from_str("2.5.4.3").unwrap(),
  parameters: None,
};

let encoded = encode_AlgorithmIdentifier(&alg)?;
assert!(validate_AlgorithmIdentifier(&encoded).is_ok()); // For validating without fully decoding.
let decoded = decode_AlgorithmIdentifier(&encoded)?;
assert_eq(alg.algorithm, decoded.algorithm);
```

### Encoding, Validating, and Decoding `UnboundedDirectoryString` (a `CHOICE`)

This type is used in X.500 directories.

```rust
use wildboar_asn1::*;
use x690::*;

pub enum UnboundedDirectoryString {
    teletexString(TeletexString),
    printableString(PrintableString),
    bmpString(BMPString),
    universalString(UniversalString),
    uTF8String(UTF8String),
}

pub fn _decode_UnboundedDirectoryString(el: &X690Element) -> ASN1Result<UnboundedDirectoryString> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => Ok(UnboundedDirectoryString::teletexString(
            BER.decode_t61_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 19) => Ok(UnboundedDirectoryString::printableString(
            BER.decode_printable_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 30) => Ok(UnboundedDirectoryString::bmpString(
            BER.decode_bmp_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 28) => Ok(UnboundedDirectoryString::universalString(
            BER.decode_universal_string(&el)?,
        )),
        (TagClass::UNIVERSAL, 12) => Ok(UnboundedDirectoryString::uTF8String(
            BER.decode_utf8_string(&el)?,
        )),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UnboundedDirectoryString",
            ))
        }
    }
}

pub fn _encode_UnboundedDirectoryString(
    value_: &UnboundedDirectoryString,
) -> ASN1Result<X690Element> {
    match value_ {
        UnboundedDirectoryString::teletexString(v) => BER.encode_t61_string(&v),
        UnboundedDirectoryString::printableString(v) => BER.encode_printable_string(&v),
        UnboundedDirectoryString::bmpString(v) => BER.encode_bmp_string(&v),
        UnboundedDirectoryString::universalString(v) => BER.encode_universal_string(&v),
        UnboundedDirectoryString::uTF8String(v) => BER.encode_utf8_string(&v),
    }
}

pub fn _validate_UnboundedDirectoryString(el: &X690Element) -> ASN1Result<()> {
    match (el.tag.tag_class, el.tag.tag_number) {
        (TagClass::UNIVERSAL, 20) => BER.validate_t61_string(&el),
        (TagClass::UNIVERSAL, 19) => BER.validate_printable_string(&el),
        (TagClass::UNIVERSAL, 30) => BER.validate_bmp_string(&el),
        (TagClass::UNIVERSAL, 28) => BER.validate_universal_string(&el),
        (TagClass::UNIVERSAL, 12) => BER.validate_utf8_string(&el),
        _ => {
            return Err(el.to_asn1_err_named(
                ASN1ErrorCode::unrecognized_alternative_in_inextensible_choice,
                "UnboundedDirectoryString",
            ))
        }
    }
}

let ds = UnboundedDirectoryString::UTF8String("hi mom".to_owned());
let encoded = _encode_UnboundedDirectoryString(&ds)?;
assert!(_validate_UnboundedDirectoryString(&encoded).is_ok()); // For validating without fully decoding.
let decoded = _decode_UnboundedDirectoryString(&encoded)?;
match (ds, decoded) {
  (UnboundedDirectoryString::UTF8String(a), UnboundedDirectoryString::UTF8String(b)) => assert_eq!(a, b),
  (_, _) => panic!(),
};
```

## To Do

- [ ] Canonical Encoding Rules (CER) on hold pending more field-testing
- [ ] Convert `OSString` directly to `BMPString` on Windows?
- [ ] More efficient `BMPString` and `UniversalString` methods

## Notes

### Why start with underscores?

Many of the functions in this library start with underscores because they are
expected to be used in generated code. Valid ASN.1 identifiers are not allowed
to start with hyphens, so code generation that converts hyphens to underscores
are guaranteed to produce no symbols that conflict with those that start with
underscores in this library. This library is already in use by the
[ASN.1 Compilation Service](https://wildboarsoftware.com/asn1-compilation)
offered by [Wildboar Software](https://wildboarsoftware.com), which was used to
compile the X.500 directory-related ASN.1 modules to what you see in `x500`.

### Why not SmallVec?

I decided that there were not enough uses cases for `SmallVec` to justify it,
even as a feature flag. Most things stored in a dynamically-allocated array
in this library are too large to fit in a `SmallVec`.

## AI / LLM Usage Statement

All but a small fraction of the code in this library was produced by a human;
ChatGPT generated a very small part of the remainder.

The documentation, on the other hand, was generated largely by AI, but it was
reviewed by the author of the code for correctness. Cursor's AI was used for
this.
