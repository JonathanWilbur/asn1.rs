# X.213 NSAP Library

ITU-T Recommendation X.213 NSAP address parsing and printing.


## Deviations from IETF RFC 1278

- `ICP` and `IND` AFIs recognized in the `<afi>-<idi>-<dsp>` syntax
- `IP4`, `IP6`, and `URL` schemes supported in `FromStr` and `Display`

## To Do

- [x] Check trailing parts
- [ ] A lot more testing
  - [ ] `0xF` padding
- [ ] Fuzz Testing
- [ ] Doc Comments
- [ ] Readme Documentation

## To Do (Future)

- [ ] Support GOSIP NSAP addressing: https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b
