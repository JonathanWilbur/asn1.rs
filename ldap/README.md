# Lightweight Directory Access Protocol (LDAP)

ASN.1 data structures and PDUs based on the ASN.1 definitions in
[IETF RFC 4511](https://datatracker.ietf.org/doc/html/rfc4511/), which defines
the Lightweight Directory Access Protocol (LDAP).

These libraries were generated entirely or in part by the
[ASN.1 Compilation Service](https://wildboarsoftware.com/asn1-compilation)
offered by [Wildboar Software](https://wildboarsoftware.com). The ASN.1
compiler itself is closed-source and proprietary, but some of the libraries
produced with it are released publicly under the
[MIT license](https://mit-license.org/).

If you would like to see additional ASN.1 libraries in Rust or other
programming languages, or if you have any other questions, please contact us at
[contact@wildboarsoftware.com](mailto:contact@wildboarsoftware.com).

## Example Usage

```rust
use x690::X690Codec;
use x690::ber::BER;
use wildboar_ldap::{
    LDAPMessage,
    BindRequest,
    AuthenticationChoice,
    LDAPMessage_protocolOp,
    _encode_LDAPMessage,
};

let bind_req = BindRequest::new(
    vec![ 1 ],
    vec![], // Empty DN
    AuthenticationChoice::simple(vec![]),
    vec![],
);

let msg = LDAPMessage::new(
    vec![ 1 ], // messageID
    LDAPMessage_protocolOp::bindRequest(bind_req),
    None, // no controls
    vec![], // no ASN.1 extensions
);

let encoded = _encode_LDAPMessage(&msg).unwrap();
let mut bytes = Vec::new();
let _ = BER.write(&mut bytes, &encoded).unwrap();
// Now the encoded LDAPMessage is in bytes.
assert!(bytes.len() > 2);
```
