# X.213 NSAP Library

## To Do

- [ ] Inline buffer so most strings can be decoded in `no_std`
- [ ] Clean up hard-coded strings
- [x] Character encoding is not correct
- [x] `URL` encoding
- [x] Confirm that character encoding prohibits underscores
- [x] `get_ip_addr()`
- [x] `get_url()`
- [ ] Support GOSIP NSAP addressing: https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b
- [x] String parsing errors
- [x] ~~`PartialEq` / `Eq`~~ (Not clear how to do this. Just document the decision.)
- [x] ~~`Hash`~~ (Not clear how to do this. Just document the decision.)
- [ ] A lot more testing
  - [ ] `0xF` padding
- [ ] Fuzz Testing
- [ ] Feature flags
  - [ ] `x25`
  - [ ] `ecma117`
- [ ] Doc Comments
- [ ] Readme Documentation
