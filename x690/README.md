# X.690 Encoding Rules

This library implements the encoding rules defined in
[ITU Recommendation X.690 (2021)](https://www.itu.int/rec/T-REC-X.690/en),
namely:

- The Basic Encoding Rules (BER)
- The Distinguished Encoding Rules (DER)
- ~~The Canonical Encoding Rules (CER)~~ (Currently unsupported)

## To Do

- [ ] Canonical Encoding Rules (CER) on hold pending more field-testing
- [ ] Convert `OSString` directly to `BMPString` on Windows?
- [ ] More efficient `BMPString` and `UniversalString` methods

## Notes

I decided that there were not enough uses cases for `SmallVec` to justify it,
even as a feature flag. Most things stored in a dynamically-allocated array
in this library are too large to fit in a `SmallVec`.

## AI / LLM Usage Statement

All but a small fraction of the code in this library was produced by a human;
ChatGPT generated a very small part of the remainder.

The documentation, on the other hand, was generated largely by AI, but it was
reviewed by the author of the code for correctness. Cursor's AI was used for
this.
