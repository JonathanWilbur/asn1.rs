# X.213 NSAP Library

ITU-T Recommendation X.213 Network Service Access Point (NSAP) address parsing
and printing. These address types were designed for use in OSI networking, but
have full compatibility with IP networking. Since OSI networking is ancient
history, this library prioritizes the IP networking aspects, but everything
should be supported.

## AI Usage Statement

No code, tests, or documentation was produced by AI or an LLM of any kind,
except for two small internal functions in `utils`, which were reviewed by the
crate's author.

## Deviations from IETF RFC 1278

- `ICP` and `IND` AFIs recognized in the `<afi>-<idi>-<dsp>` syntax
- `IP4`, `IP6`, and `URL` schemes supported in `FromStr` and `Display`

## To Do

- [x] Format
- [ ] Doc Comments
- [ ] Readme Documentation

## To Do (Future)

- [ ] Support GOSIP NSAP addressing: https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b
