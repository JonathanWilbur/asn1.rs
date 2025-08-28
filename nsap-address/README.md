# X.213 NSAP Library

ITU-T Recommendation X.213 Network Service Access Point (NSAP) address parsing
and printing. These address types were designed for use in OSI networking, but
have full compatibility with IP networking. Since OSI networking is ancient
history, this library prioritizes the IP networking aspects, but everything
should be supported.

This module decodes and encodes a Network Service Access Point (NSAP) to and
from the "preferred binary encoding" described in Annex A, Section A.5.3 of
[ITU-T Recommendation X.213 (2001)](https://www.itu.int/rec/T-REC-X.213-200110-I/en).

In addition to this, it displays and decodes NSAPs to and from human-readable
strings according to the procedures defined in
[IETF RFC 1278](https://datatracker.ietf.org/doc/rfc1278/), drawing on
additional information found in
[IETF RFC 1277](https://datatracker.ietf.org/doc/html/rfc1277).

There are some deviations to the above, however. Since the human-friendly string
syntax was defined, new AFIs were added, including one for directly representing
IP addresses and another for representing URLs. As such this library extends the
specification, but should be completely backwards compatible with it.

You should **not** expect an NSAP decoded from a string to encode back into the
same exact string. You should **not** expect an NSAP decoded from bytes to
encode back into the same exact bytes. You should **not** expect all NSAP
syntaxes to be recognized everywhere; your application and dependencies should
handle unrecognized NSAP syntaxes gracefully.

## AI Usage Statement

No code, tests, or documentation was produced by AI or an LLM of any kind,
except for two small internal functions in `utils`, which were reviewed by the
crate's author.

## Deviations from IETF RFC 1278

- `ICP` and `IND` AFIs recognized in the `<afi>-<idi>-<dsp>` syntax
- `IP4`, `IP6`, and `URL` schemes supported in `FromStr` and `Display`

## To Do

- [x] Doc Comments
- [x] Per-module docs
- [ ] Readme Documentation

## To Do (Future)

- [ ] Support GOSIP NSAP addressing: https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b
