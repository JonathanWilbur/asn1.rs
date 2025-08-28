# X.213 NSAP Library

ITU-T Recommendation X.213 NSAP address parsing and printing.


## Deviations from IETF RFC 1278

- `ICP` and `IND` AFIs recognized in the `<afi>-<idi>-<dsp>` syntax
- `IP4`, `IP6`, and `URL` schemes supported in `FromStr` and `Display`

## To Do

- [ ] Clamp the `u8` in `get_octets()`
- [x] A lot more testing
  - [x] `BCDBuffer.push_ascii_bytes()`
  - [x] `BCDBuffer.push_str()`
  - [x] `BCDBuffer.len_in_bytes()`
  - [x] `get_address_type_info()`
  - [x] `char_to_local_iso_iec_646_byte`
  - [x] `local_iso_iec_646_byte_to_char`
  - [x] `X213NetworkAddress`
    - [x] `idi_digits()`
      - [x] Truncated tests too
      - [x] Too long tests
      - [x] Non-digits
      - [x] Padding 0xF as MSN
    - [x] `dsp_digits()`
    - [x] `from_ipv4()`
    - [x] `from_ipv6()`
    - [x] `from_itot_url()`
    - [x] `from_itot_socket_addr()`
    - [x] `to_ns_string()`
  - [x] `0xF` paddinggit
- [ ] Fuzz Testing
- [ ] Doc Comments
- [ ] Readme Documentation

## To Do (Future)

- [ ] Support GOSIP NSAP addressing: https://medium.com/@jacstech/jacs-nsap-structure-8cb9a809228b
