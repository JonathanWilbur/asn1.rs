# X.213 NSAP Library

## To Do

- [ ] Inline buffer so most strings can be decoded in `no_std`
- [ ] Clean up hard-coded strings
- [x] Character encoding is not correct
- [x] `URL` encoding
- [x] Confirm that character encoding prohibits underscores
- [ ] `get_ip_addr()`
- [x] `get_url()`
- [ ] Support GOSIP NSAP addressing: https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b
- [ ] Is there a separate ATN addressing? It sounds like ATN uses ISO 6523 (ICD)
- [ ] String parsing errors
- [ ] `PartialEq` / `Eq`
  - [ ] `ICP` and `TELEX` both have encodings, although arguably they are different, since one is ITOT only
  - [ ] Compare binary encoding with decimal equally
- [ ] `Hash`
- [ ] A lot more testing
  - [ ] `0xF` padding
- [ ] Fuzz Testing
- [ ] Feature flags
  - [ ] `x25`
  - [ ] `ecma117`
- [ ] Doc Comments
- [ ] Readme Documentation
