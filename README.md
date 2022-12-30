# Wildboar Rust ASN.1 Libraries

This monorepo is a sort of "incubator" for ASN.1 / ITU / cryptography-related
projects in Rust. It started off as just an ASN.1-related monorepo, but I
started using it in other libraries to see if it was adequate for its intended
purpose.

This is very much a work in progress. There is almost no documentation now, but
it will be very well documented once it is all stabilized.

## Decisions to make

- Should `BIT STRING` just be a simple array of bytes and a length field, like in Go?
  - I think this should be fine. This would even work well for PER and JER.
  - One difference between X.690 and X.691 encoding is that PER encodes the
    number of bits, whereas BER, CER, and DER encode the number of padding bits.
    - I think the latter is computationally simpler and more secure.
      - It is easy to validate. If you encode the length of bits, it could
        misalign with the number of bytes.
  - This would also probably be more performant, since large bit strings are
    used in cryptographic files.
- Should `chrono` be used at all? It seems like dates should just be represented as integers.
- Is it even possible to have a uniform interface between codecs?
  - I highly doubt it. Even if this is possible, I need to develop all of the
    codecs to discover what this interface even would be.

## How to handle EXPLICIT encoding?

This is a tricky problem, because you have to know the length of the inner value
before you can encode the outer value's length field. We don't want to create
a new buffer, write the inner TLV tuple to that, then copy it all to another
buffer.

...Unless you use indeterminate length encoding.

However, indeterminate length encoding is much higher overhead for lots of
small values, and would almost certainly lead to very serious compatibility
issues. (The Go ASN.1 library only supports DER, for example.)

What if you wrote out the encoded values _backwards_?

This would fix this problem, but the output would be sure to be buggy, and it
would not be reversible. You could not directly use the output of an encoder as
input to a decoder and get the original value.

Also, reversing is still going to do a byte-for-byte copy.

This problem is much worse than I realized. Say you have a value of type:

```asn1
[1] EXPLICIT [2] EXPLICIT [3] EXPLICIT [4] EXPLICIT BOOLEAN
```

The innermost `BOOLEAN` is going to get:

1. Encoded, then...
2. Copied into the value bytes of `[4]`, which gets...
3. Copied into the value bytes of `[3]`, which gets...
4. Copied into the value bytes of `[2]`, which gets...
5. Copied into the value bytes of `[1]`

For a deeply structured type, it is plausible that large strings could get
copied over and over again.

I think the solution would be to produce some sort of intermediary encoding,
like this:

```rust

enum X690Encoding {
    IMPLICIT(Vec<u8>), // the value bytes
    EXPLICIT(X690Element), // the inner TLV tuple
    constructed(Vec<X690Element>) // an array of inner TLV tuples
}

struct X690Element {
    tag_class: TagClass;
    tag_number: TagNumber;
    value: X690Encoding;
}

```

Let's look at this data structure:

```asn1

Person ::= SEQUENCE {
    name        UTF8String,
    age         INTEGER,
    married     BOOLEAN
}

bob Person ::= { name "Bob McGobb", age 43, married FALSE }

```

Under this scheme, the values of the above data structure would get encoded
separately, each producing intermediate values:

```rust
let name = X690Element{
    tag_class: TagClass::UNIVERSAL,
    tag_number: 12,
    value: X690Encoding::IMPLICIT(vec![ 'B', 'o', 'b', ' ', 'M', 'c', 'G', 'o', 'b', 'b' ]),
};
let age = X690Element{
    tag_class: TagClass::UNIVERSAL,
    tag_number: 2,
    value: X690Encoding::IMPLICIT(vec![ 43 ]),
};
let married = X690Element{
    tag_class: TagClass::UNIVERSAL,
    tag_number: 1,
    value: X690Encoding::IMPLICIT(vec![ 0xFF ]),
};
let bob = X690Element{
    tag_class: TagClass::UNIVERSAL,
    tag_number: 16,
    value: X690Encoding::constructed(vec![ name, age, married ]),
};
```

To encode `bob`, we need to know the length of `bob`. What is the length of
`bob`? Well, it's the sum of lengths of all of the constituent elements. What
is the length of `name`? It is the length needed to encode the tag and number,
plus the length of `IMPLICIT` encoding. If it were `EXPLICIT`, this procedure
would just recurse into the inner element.

This is a much more performant alternative, because we don't repeatedly copy
inner values. In fact, they can be written to a buffer just once, since
`.write_vectored()` might be available.

There is still one small problem, and I say "small" because the performance hit
would probably be almost unnoticeable, even to profiling: calculating the length
bytes is a recursive procedure, and with the algorithm as it is, the length has
to be recalculated for every single element every time it is used in the
calculation of the length of another element. It would be much more performant
if we could do a "single-sweep." For that reason, we will add one more field to
the `X690Element`: `last_calculated_length` of type `Option<usize>`. This will
be set once an element and all of its subordinates' length are calculated, so
that every subordinate, recursively, does not have to have its length
recalculated.

Finally, there are some circumstances where users will want to simply relay raw
bytes that are known to be a valid BER-encoded (or CER / DER) element without
first decoding them. If we were to decode a raw sequence of bytes into such an
element, we would, at minimum, have to parse the tag number, determine the
length, and copy the value bytes of the element into a new vector. The copy,
in particular, could be computationally expensive. Instead, we will add a fourth
variant of `X690Encoding`, called `already_encoded`, which contains a vector of
bytes. When this variant is used, the tag class and number are ignored, the
length need not be calculated, and the raw bytes it contains are simply
concatenated to the end of the encoding.

There is one more performance enhancement we'll use: sometimes, for the sake of
memory resourcefulness, you do not want to load the entire contents of the
encoded value into memory. Fortunately, since the only thing we need to do with
the `already_encoded` variant is read it, we can abstract it out our need to use
the `Vec<u8>` and just use a `Read` type instead, so that encoded values can be
streamed from the filesystem, say, 10000 bytes at a time, so that memory won't
be consumed by a single gigantic ASN.1 element.

Finally, we may want to save the name of a value, if there is one, so we add a
`name` field. This just helps with debugging: if a particular element is the
cause of some error, we can display it's name, such as `subjectPublicKeyInfo`.

The final element looks like this:

```rust

enum X690Encoding {
    IMPLICIT(Vec<u8>), // the value bytes
    EXPLICIT(X690Element), // the inner TLV tuple
    constructed(Vec<X690Element>) // an array of inner TLV tuples
    already_encoded(Read), // the already-encoded TLV
}

struct X690Element {
    pub name: Option<String>;
    pub tag_class: TagClass;
    pub tag_number: TagNumber;
    pub value: X690Encoding;
    last_calculated_length: Option<usize>; // Maybe not pub
}


```

## Encoding Instructions

Abstract ASN.1 values are translated to their `X690Element` equivalents. The
"root" element that needs encoding is passed into `ber_encode()`, `cer_encode()`
or `der_encode()`. The input is immediately passed into `x690_cst()` which
returns a single `X690Element`, which may use the `constructed` variant of
`value` to contain substituent `X690Element`s. This CST is passed into
`ber_encode_cst()`.

TODO: Do I need to record which encoding was used to encode a given value?
I don't think so.

TODO: Do I need to record which nodes have been visited to avoid infinite loops?

## Attributes

Eventually, we will define attribute macros that can be used to control how
structured types are encoded.

Usage should look something like this:

```rust

struct AlgorithmIdentifier {
    #[asn1_component(UNIVERSAL, 6)]
    pub algorithm: OBJECT_IDENTIFIER;
    pub parameters: OPTIONAL<ANY>;
}

```

Which would translate to this:

```rust
struct AlgorithmIdentifier {
    pub const _tag_class_for_algorithm: TagClass = UNIVERSAL;
    pub const _tag_number_for_algorithm: TagNumber = 6;
    pub algorithm: OBJECT_IDENTIFIER;
    pub parameters: OPTIONAL<ANY>;
}

```

Actually, that might not work well, because some component types are CHOICE
types, which can have multiple tags. You also need to handle EXPLICIT,
IMPLICIT, and sometimes constraints too.

https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros

## Encoding Control Notation

Encoding controls are passed in as parameters to the `*er_encode()` functions,
which control how these elements are encoded.

## Future libraries

- `parser`
- `ber`
- `der`
- `cer`
- `per`
- `bacnet`
- `gser` (https://datatracker.ietf.org/doc/html/rfc3641)

## TODO

- [ ] `asn1`
  - [ ] `oid!` macro
  - [ ] `bits!` macro
  - [ ] `octs!` macro
  - [ ] `ASN1Error::relate_tlv(X690Element)`
- [ ] `x690`
  - [ ] Deconstruct
  - [ ] Hash trait for `X690Element`
  - [ ] Print `asn1parse` output;
- [ ] Is there some way to abstract ROSE out of X.500, so it can be recycled among projects?
- [ ] Just an idea: if you are using trait type parameters, such as `X690Element`
      to constrain ROSE-related values to `X690Element`s instead of `ASN1Value`,
      couldn't you implement an IDM layer that accepts only BER and DER encoding
      for `impl ROSEReceiver<X690Element>`?
- [ ] Shouldn't the API implement a lazy technique? As, just decode PDUs as they
      are needed? In other words, pull, rather than eagerly parse and enqueue.
