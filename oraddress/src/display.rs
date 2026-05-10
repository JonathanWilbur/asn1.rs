use std::fmt::{Debug, Display, Write};
use std::io::BufRead;
use crate::make_string_safe;
use crate::asn1::{
    teletex_common_name,
    BuiltInDomainDefinedAttribute,
    CountryName,
    ExtendedNetworkAddress,
    PDSParameter,
    PersonalName,
    PhysicalDeliveryCountryName,
    PostalCode,
    TeletexDomainDefinedAttribute,
    TeletexPersonalName,
    TerminalType,
    UnformattedPostalAddress,
    UniversalDomainDefinedAttribute,
    UniversalOrBMPString,
    UniversalPersonalName,
};

fn print_teletex(ts: &[u8], delim: char, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if ts.is_ascii() { // Avoid allocation if no diacritics are used.
        let s = unsafe { str::from_utf8_unchecked(ts) };
        f.write_str(make_string_safe(s.as_ref(), delim).as_ref())
    } else {
        let s: String = teletex::teletex_to_utf8(ts).collect();
        f.write_str(make_string_safe(s.as_str(), delim).as_ref())
    }
}

impl Display for PersonalName {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(gn) = self.given_name.as_ref() {
            f.write_str("G=")?;
            f.write_str(make_string_safe(gn.as_str(), ';').as_ref())?;
            f.write_char(';')?;
        }
        if let Some(i) = self.initials.as_ref() {
            f.write_str("I=")?;
            f.write_str(make_string_safe(i.as_str(), ';').as_ref())?;
            f.write_char(';')?;
        }
        f.write_str("S=")?;
        f.write_str(make_string_safe(&self.surname.as_str(), ';').as_ref())?;
        f.write_char(';')?;
        // The order in which the generational qualifier appears is
        // unspecified.
        if let Some(gq) = self.generation_qualifier.as_ref() {
            f.write_str("Q=")?;
            f.write_str(make_string_safe(gq.as_str(), ';').as_ref())?;
            f.write_char(';')?;
        }
        Ok(())
    }

}

impl Display for TeletexPersonalName {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(gn) = self.given_name.as_ref() {
            f.write_str("G=")?;
            print_teletex(gn.as_slice(), ';', f)?;
            f.write_char(';')?;
        }
        if let Some(i) = self.initials.as_ref() {
            f.write_str("I=")?;
            print_teletex(i.as_slice(), ';', f)?;
            f.write_char(';')?;
        }
        f.write_str("S=")?;
        print_teletex(self.surname.as_slice(), ';', f)?;
        f.write_char(';')?;
        // The order in which the generational qualifier appears is
        // unspecified.
        if let Some(gq) = self.generation_qualifier.as_ref() {
            f.write_str("Q=")?;
            print_teletex(gq.as_slice(), ';', f)?;
            f.write_char(';')?;
        }
        Ok(())
    }

}

impl Display for UniversalPersonalName {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(gn) = self.given_name.as_ref() {
            f.write_str("G=")?;
            gn.safe_fmt(';', f)?;
            f.write_char(';')?;
        }
        if let Some(i) = self.initials.as_ref() {
            f.write_str("I=")?;
            i.safe_fmt(';', f)?;
            f.write_char(';')?;
        }
        f.write_str("S=")?;
        self.surname.safe_fmt(';', f)?;
        f.write_char(';')?;
        // The order in which the generational qualifier appears is
        // unspecified.
        if let Some(gq) = self.generation_qualifier.as_ref() {
            f.write_str("Q=")?;
            gq.safe_fmt(';', f)?;
            f.write_char(';')?;
        }
        Ok(())
    }

}

impl Display for CountryName {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountryName::iso_3166_alpha2_code(isocc) => {
                if isocc.as_slice().iter().all(|b| b.is_ascii_alphabetic()) {
                    let s = unsafe { str::from_utf8_unchecked(isocc.as_slice()) };
                    f.write_str(s)?;
                }
            },
            CountryName::x121_dcc_code(dcc) => {
                write!(f, "{:03}", dcc)?;
            },
        };
        Ok(())
    }

}

impl Display for PhysicalDeliveryCountryName {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhysicalDeliveryCountryName::iso_3166_alpha2_code(isocc) => {
                if isocc.len() != 2
                    || !isocc.bytes().all(|b| b.is_ascii_alphabetic()) {
                    return Ok(());
                }
                f.write_str(isocc.as_str())
            }
            PhysicalDeliveryCountryName::x121_dcc_code(dcc) => {
                if dcc.len() != 3 || !dcc.bytes().all(|b| b.is_ascii_digit()) {
                    return Ok(());
                }
                f.write_str(dcc.as_str())
            }
        }
    }

}

impl Display for PostalCode {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostalCode::numeric_code(nc) => f.write_str(&make_string_safe(nc.as_str(), ';').as_ref()),
            PostalCode::printable_code(pc) => f.write_str(&make_string_safe(pc.as_str(), ';').as_ref()),
        }
    }

}

impl Display for PDSParameter {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ps) = self.printable_string.as_deref() {
            return f.write_str(&make_string_safe(ps, ';').as_ref());
        }
        if let Some(ts) = self.teletex_string.as_deref() {
            return print_teletex(ts, ';', f);
        }
        Ok(())
    }

}

impl Display for UnformattedPostalAddress {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ps) = self.printable_address.as_deref() {
            for (i, line) in ps.iter().take(6).enumerate() {
                if i > 0 {
                    f.write_char(';')?;
                }
                write!(f, "PD-A{}=", i + 1)?;
                f.write_str(&make_string_safe(line.as_str(), ';').as_ref())?;
            }
        }
        if let Some(ts) = self.teletex_string.as_deref() {
            let s: String = teletex::teletex_to_utf8(ts).collect();
            for (i, line) in s.lines().take(6).enumerate() {
                if i > 0 {
                    f.write_char(';')?;
                }
                write!(f, "PD-A{}=", i + 1)?;
                f.write_str(&make_string_safe(line, ';').as_ref())?;
            }
        }
        Ok(())
    }

}

impl Display for ExtendedNetworkAddress {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtendedNetworkAddress::e163_4_address(addr) => {
                f.write_char('+')?;
                f.write_str(&addr.number)?;
                if let Some(ext) = addr.sub_address.as_deref() {
                    f.write_char('x')?;
                    f.write_str(ext)?;
                }
                Ok(())
            },
            ExtendedNetworkAddress::psap_address(psap) => std::fmt::Display::fmt(&psap, f),
        }
    }

}

impl Display for BuiltInDomainDefinedAttribute {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DDA:")?;
        if self.type_.as_str().contains('=') {
            let typestr = self.type_.as_str().replace('=', "==");
            f.write_str(typestr.as_str())?;
        } else {
            f.write_str(&self.type_.as_str())?;
        }
        f.write_char('=')?;
        f.write_str(self.value.as_str())
    }

}

impl Display for TeletexDomainDefinedAttribute {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DDA:")?;
        if self.type_.iter().any(|b| *b == b'=') {
            let typestr: String = teletex::teletex_to_utf8(self.type_.as_slice()).collect();
            let typestr = typestr.replace('=', "==");
            f.write_str(typestr.as_str())?;
        } else {
            if self.type_.iter().all(|b| b.is_ascii_graphic() && *b != b'=') {
                let s = unsafe { str::from_utf8_unchecked(&self.type_) };
                f.write_str(s)?;
            } else {
                let typestr: String = teletex::teletex_to_utf8(self.type_.as_slice()).collect();
                let typestr = typestr.replace('=', "==");
                f.write_str(typestr.as_str())?;
            }
        }
        f.write_char('=')?;
        print_teletex(self.value.as_slice(), ';', f)
    }

}

impl Display for UniversalDomainDefinedAttribute {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DDA:")?;
        let typestr = self.type_.character_encoding.to_string();
        let typestr = typestr.replace('=', "==");
        f.write_str(typestr.as_str())?;
        f.write_char('=')?;
        f.write_str(make_string_safe(self.value.to_string().as_str(), ';').as_ref())
    }

}
