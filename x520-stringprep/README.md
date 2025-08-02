# X.520 String Preparation

String preparation / normalization per the procedures defined in
[ITU-T Recommendation X.520 (2019)](https://www.itu.int/rec/T-REC-X.520),
Section 7. These procedures were defined for usage in X.500 directories,
but they may be used elsewhere.

This library is `no_std`: it does not require the standard library and can be
used in embedded systems.

## Example Usage

A basic example, using the most basic building-block of this library:

```rust
# extern crate alloc;
# use alloc::string::String;
# use x520_stringprep::{
#     x520_stringprep_case_exact_str,
#     x520_stringprep_case_ignore_str
# };

let input = "  Hello\te\u{0301}\u{2000}Ä\u{FB03}n  ";

let output: String = x520_stringprep_case_exact_str(input)
    .map(|maybe_c| maybe_c.unwrap())
    .collect();
assert_eq!(output, " Hello é Äffin ");
let output: String = x520_stringprep_case_ignore_str(input)
    .map(|maybe_c| maybe_c.unwrap())
    .collect();
assert_eq!(output, " hello é äffin ");
```

You might notice that the outputs above are not trimmed, even though this is
a requirement in the specification. This is done because this might be a
performance-critical operation, so it is left to the caller to trim strings
only if it is believed that they need it. Trimming is a pretty trivial step
that can be performed prior to storage in a database; once it is done,
subsequent retrievals from this database won't need trimming.

Since this library is likely to be used with ASN.1, there are also functions
meant for preparing `BMPString` and `UniversalString`:

- `x520_stringprep_case_exact_bmp`
- `x520_stringprep_case_ignore_bmp`
- `x520_stringprep_case_exact_univ_str`
- `x520_stringprep_case_ignore_univ_str`

These functions allow you to check if a string is already prepared:

- `is_x520_stringprepped_case_exact_str`
- `is_x520_stringprepped_case_ignore_str`

These allow you to compare strings:

- `x520_stringprep_case_exact_compare`, which can be used in implementations of
  `caseExactMatch` and other such matching rules.
- `x520_stringprep_case_ignore_compare`, which can be used in implementations of
  `caseIgnoreMatch` and other such matching rules.

If you enable the `alloc` feature flag, you also get these convenience functions
that return owned strings from the preparation procedure:

- `x520_stringprep_to_case_exact_string`
- `x520_stringprep_to_case_ignore_string`

## AI Usage Statement

None of the library code was produced by an AI or LLM of any kind, but the unit
tests were written by the Cursor editor, and LLM-based autocomplete was used in
producing documentation comments. The unit tests were individually reviewed by
the author.
