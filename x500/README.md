# X.500

Implementation of the ASN.1 data structures and PDUs defined in the
X.500 series of recommendations published by the
[International Telecommunication Union's (ITU)](https://www.itu.int/en/Pages/default.aspx).

This library only implements the ASN.1 data structures and PDUs defined in these
specifications, as well as subset of the business logic, such as:

- Matching Rules
- Context Matching
- Stringifying
- Comparison
- Filter evaluation

The protocols whose PDUs and data structures are implemented in this library
are (inclusively):

- Directory Access Protocol (DAP) - Which is used for reading X.500 directories.
- Directory Information Shadowing Protocol (DISP) - Which is used for replicating X.500 directories.
- Directory System Protocol (DSP) - Which is used for X.500 directories to read other X.500 directories.
- Directory Operational Binding Management Protocol (DOP) - Which manages rules for how X.500 directories interact.
- Certificate Authority Subscription Protocol (CASP)
- Authorization Validation Management Protocol (AVMP)
- Trust Broker Protocol (TBP)

## Footnote

These libraries were generated entirely or in part by the
[ASN.1 Compilation Service](https://wildboarsoftware.com/asn1-compilation)
offered by [Wildboar Software](https://wildboarsoftware.com). The ASN.1
compiler itself is closed-source and proprietary, but some of the libraries
produced with it are released publicly under the
[MIT license](https://mit-license.org/).

If you would like to see additional ASN.1 libraries in Rust or other
programming languages, or if you have any other questions, please contact us at
[contact@wildboarsoftware.com](mailto:contact@wildboarsoftware.com).
