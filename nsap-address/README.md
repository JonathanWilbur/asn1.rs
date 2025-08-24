# X.213 NSAP Library

## Deviations from IETF RFC 1278

- `IP4`, `IP6`, and `URL` schemes supported in `FromStr` and `Display`

## To Do

- [x] `impl Error`
- [ ] Refactor into separate files so its not all so huge
- [ ] Inline buffer so most strings can be decoded in `no_std`
- [ ] Clean up hard-coded strings
- [x] ~~Support standard macros~~ (Not clear how these actually work)
- [x] `From` implementations
- [ ] Support GOSIP NSAP addressing: https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b
- [ ] A lot more testing
  - [ ] `0xF` padding
- [ ] Fuzz Testing
- [ ] Feature flags
  - [ ] `x25`
  - [ ] `ecma117`
  - [ ] All non-standard things
- [ ] Doc Comments
- [ ] Readme Documentation
