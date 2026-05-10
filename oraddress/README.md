# X.400 Message Handling System (MHS) Originator-Recipient (O/R) Address

This was split off into a separate crate because it is used in `pki-stub`, and
will be used in X.400-related crates later.

## Multiple Values Handling

ITU-T Rec. X.413 on `oRAddressMatch`:

> Where the value of an element may be a Printable String, Teletex String,
> Universal String, or a combination of these types, any form of presented
> value shall match any form of stored value. Where more than one form is
> present (in the presented value, or stored value, or both) then matching of
> the element succeeds if the matching of at least one form succeeds.

So only one has to match for equality.

## To Do

- [x] (Mostly) Heapless ORAddress
- [ ] Iterators
  - [ ] Ones that extend `BuiltInStandardAttributes`
    - [x] organization-name
    - [ ] personal-name
    - [ ] organizational-unit-names
  - [ ] Domain Defined Attributes
  - [ ] Ones that do not
    - [x] common-name (printable)
    - [x] `PDSParameter`-typed (these are ultimately just strings)
      - [x] physical-delivery-office-name
      - [x] physical-delivery-office-number
      - [x] extension-OR-address-components
      - [x] physical-delivery-personal-name
      - [x] physical-delivery-organization-name
      - [x] extension-physical-delivery-address-components
      - [x] street-address
      - [x] post-office-box-address
      - [x] poste-restante-address
      - [x] unique-postal-name
      - [x] local-postal-attributes
    - [x] pds-name
    - [ ] physical-delivery-country-name (syntax looks like the same as CountryName)
    - [ ] postal-code (numeric or printable)
    - [ ] unformatted-postal-address (printable / teletex)
    - [ ] extended-network-address (complex weird syntax)
    - [ ] terminal-type (u16)
- [ ] RFC 1685 `FromStr` (Should decode to the non-extension syntax, if possible)
- [ ] RFC 1685 `Display` (Should prefer the UniversalString, if present)
- [ ] `Display` for `BuiltInStandardAttributes`
- [ ] `FromStr` for `BuiltInStandardAttributes`
- [ ] Implement `Display` and `FromStr` for:
  - [ ] `BuiltInStandardAttributes`
  - [x] `PersonalName` (and variations of it)
  - [x] `CountryName`
  - [ ] `BuiltInDomainDefinedAttribute` (and variations of it)
  - [x] `UniversalOrBMPString`
  - [ ] `PostalCode`
  - [x] `PhysicalDeliveryCountryName`
  - [ ] `PDSParameter`
  - [ ] `UnformattedPostalAddress`
  - [ ] `ExtendedNetworkAddress`
- [ ] `PartialEq` / `Eq` (Matches if any encoding of the string matches)
- [x] ~~`Hash`~~ (Not possible.)
- [ ] Size limits on DDAs
- [ ] DCC -> ISO (Just map strings to strings instead)
- [ ] Inline
- [ ] `ComponentSpec::req` and `opt`
- [ ] More string size checks
  - [ ] `TeletexOrganizationalUnitName`, for example
- [ ] Fix `with_capacity()`
- [ ] Const
- [ ] Unit Tests
- [ ] Format
- [ ] Fuzz Testing
- [ ] Doc Comments
- [ ] Readme Documentation

## Why Prefer Displaying Printable Characters Only

From [Alvestrand](https://www.alvestrand.no/x400/debate/addressing.html):

> \[X.400(88)\] added an extension mechanism, allowed most attributes to be
> written using the Teletex charset (Japanese characters, and most accents, but
> NOT greek, Arabic or Sanskrit), and then added the proviso that you had to
> make all the mailboxes addressable using the older PrintableString attributes,
> because you couldn't expect everyone in the world to be fluent Kanji typists.

I haven't found this proviso (yet), but I did find this in ITU-T Rec. X.402,
Section 18.2:

> Wherever Printable, Teletex and Universal strings are permitted, one, two or
> all three varieties may be supplied. Where more than one variety is supplied
> for an attribute, each value should unambiguously identify the same user. Many
> systems will not be able to render all possible characters that can be
> represented by Universal Strings (for example, being restricted to that subset
> of Universal String that supports national requirements), and some systems
> will be unable to render Universal Strings at all. Hence Universal Strings
> alone should only be used where it is known that all likely recipients can
> handle the characters concerned (e.g. within a national or regional community
> of users).

I take that to mean that, if the PrintableString value is present, it should be
routable.

Also, IETF RFC 1685 was published before any of the Teletex or Universal string
extension attributes were ever defined...

ITU-T Rec. X.419, Annex B.2.8 describes rules for downgrading O/R addresses to
printable strings for use with 1984 X.400.

Lastly, I asked ChatGPT and it said to prefer displaying the `PrintableString`
syntax, if it is present.

So there you have it. I am going to prioritize printing the
`BuiltInStandardAttributes` over the extension additions equivalents.

Although... maybe `Display` could be an alias to one of two formatting
functions: one that prefers parsing and compatibility, and the other that
displays Universal strings. Then the user has a choice.

## Why Display uses Semi-Colons Only

Because, in the syntax defined in
[IETF RFC 1278](https://datatracker.ietf.org/doc/html/rfc1278), forward slashes
are used to delimit selectors in presentation addresses, we cannot reliably use
forward slashes; but the same syntax does not permit semi-colons anywhere.

Also, semi-colon delimitation seems to be the more common representation.

Finally, semi-colons are outright prohibited in `PrintableString`, whereas the
solidus `/` is not, so it makes accidental parsing and displaying bugs less
likely in using `;`.
