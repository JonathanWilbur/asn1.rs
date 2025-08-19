# LDAP Postal Address Parser

This Rust crate parses Lightweight Directory Access Protocol (LDAP) postal
addresses according to the syntax in
[IETF RFC 4517](https://datatracker.ietf.org/doc/html/rfc4517#section-3.3.28).
It has no dependencies. This crate can function without a standard library
(`no_std`), but `alloc` is required if you want to actually replace escape
sequences or create new postal address strings. Without `alloc`, this crate
merely detects escape sequences used in postal address lines, but does nothing
about them. This crate has been fuzz-tested.

The LDAP `PostalAddress` syntax is defined in
[IETF RFC 4517, Section 3.3.28](https://datatracker.ietf.org/doc/html/rfc4517#section-3.3.28).
Lines of the postal address are separated by dollar signs. Dollar signs and
backslashes that appear in the postal address are escaped by being translated to
`\24` and `\5C` respectively (case-exact).

## Usage

You can parse and unescape LDAP postal addresses like so:

```rust
use ldappostaladdr::{parse_postal_address, unescape_postal_address_line};
let input = "\\241,000,000 Sweepstakes$PO Box 1000000$Anytown, CA 12345$USA";
let mut postal_address = parse_postal_address(input);
for (line, backslash_escaped, dollar_escaped) in postal_address {
  // This line returns Cow::Borrowed() if the line doesn't contain escape sequences.
  let unescaped_line = unescape_postal_address_line(line, backslash_escaped, dollar_escaped);
  info!(unescaped_line.as_ref());
}
```

You can create LDAP postal addresses like so:

```rust
use ldappostaladdr::escape_postal_address_line;
let lines = vec![
    String::from("$1,000,000 Sweepstakes"),
    String::from("123 Main St."),
    String::from("Anytown, PA 12345"),
    String::from("USA"),
];
let output = lines.iter()
    .map(|line| escape_postal_address_line(line).into_owned())
    .collect::<Vec<String>>()
    .join("$");
assert_eq!(output.as_str(), "\\241,000,000 Sweepstakes$123 Main St.$Anytown, PA 12345$USA");
```

## AI Usage Statement

None of the code in this crate was produced by an AI or LLM of any kind.
