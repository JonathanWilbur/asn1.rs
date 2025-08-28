#![no_main]

use std::str::FromStr;

use libfuzzer_sys::fuzz_target;

extern crate nsap_address;
use nsap_address::X213NetworkAddress;

fuzz_target!(|input: (&[u8], &str, u64, u8)| {
    let (input_b, input_s, input_u64, input_u8) = input;
    let _ = X213NetworkAddress::try_from(input_b);
    let _ = X213NetworkAddress::from_str(input_s);
    let ss1 =  [ "X121+", input_s ].join("");
    let ss2 =  [ "DCC+", input_s ].join("");
    let ss3 =  [ "TELEX+", input_s ].join("");
    let ss4 =  [ "PSTN+", input_s ].join("");
    let ss5 =  [ "ISDN+", input_s ].join("");
    let ss6 =  [ "ICD+", input_s ].join("");
    let ss7 =  [ "ICP+", input_s ].join("");
    let ss8 =  [ "IND+", input_s ].join("");
    let ss9 =  [ "LOCAL+", input_s ].join("");
    let ss10 = [ "URL+", input_s ].join("");
    let ss11 = [ "IP4+", input_s ].join("");
    let ss12 = [ "IP6+", input_s ].join("");

    let hexstr = hex::encode(input_b);
    let sb1 =  [ "X121+", hexstr.as_str() ].join("");
    let sb2 =  [ "DCC+", hexstr.as_str() ].join("");
    let sb3 =  [ "TELEX+", hexstr.as_str() ].join("");
    let sb4 =  [ "PSTN+", hexstr.as_str() ].join("");
    let sb5 =  [ "ISDN+", hexstr.as_str() ].join("");
    let sb6 =  [ "ICD+", hexstr.as_str() ].join("");
    let sb7 =  [ "ICP+", hexstr.as_str() ].join("");
    let sb8 =  [ "IND+", hexstr.as_str() ].join("");
    let sb9 =  [ "LOCAL+", hexstr.as_str() ].join("");
    let sb10 = [ "URL+", hexstr.as_str() ].join("");
    let sb11 = [ "IP4+", hexstr.as_str() ].join("");
    let sb12 = [ "IP6+", hexstr.as_str() ].join("");

    let url = format!("URL+{}+{}", input_u8, input_s);
    let x25_1 = format!("TELEX+00728722+X.25(80)+02+{}", input_u64);
    let x25_2 = format!("TELEX+00728722+X.25(80)+02+{}+CUDF+{}", input_u64, input_u8);
    let x25_3 = format!("TELEX+00728722+X.25(80)+{}+{}+CUDF+{}", input_u8, input_u8, input_u8);
    let itot1 = format!("TELEX+00728722+RFC-1006+{}+{}.{}.{}.{}", input_u8, input_u8, input_u8, input_u8, input_u8);
    let itot2 = format!("TELEX+00728722+RFC-1006+03+2.3.4.5+{}", input_u8);
    let itot3 = format!("TELEX+00728722+RFC-1006+03+2.3.4.5+{}+{}", input_u8, input_u8);
    let x121 = format!("X121+{}", input_u64);

    let _ = X213NetworkAddress::from_str(ss1.as_str());
    let _ = X213NetworkAddress::from_str(ss2.as_str());
    let _ = X213NetworkAddress::from_str(ss3.as_str());
    let _ = X213NetworkAddress::from_str(ss4.as_str());
    let _ = X213NetworkAddress::from_str(ss5.as_str());
    let _ = X213NetworkAddress::from_str(ss6.as_str());
    let _ = X213NetworkAddress::from_str(ss7.as_str());
    let _ = X213NetworkAddress::from_str(ss8.as_str());
    let _ = X213NetworkAddress::from_str(ss9.as_str());
    let _ = X213NetworkAddress::from_str(ss10.as_str());
    let _ = X213NetworkAddress::from_str(ss11.as_str());
    let _ = X213NetworkAddress::from_str(ss12.as_str());
    let _ = X213NetworkAddress::from_str(sb1.as_str());
    let _ = X213NetworkAddress::from_str(sb2.as_str());
    let _ = X213NetworkAddress::from_str(sb3.as_str());
    let _ = X213NetworkAddress::from_str(sb4.as_str());
    let _ = X213NetworkAddress::from_str(sb5.as_str());
    let _ = X213NetworkAddress::from_str(sb6.as_str());
    let _ = X213NetworkAddress::from_str(sb7.as_str());
    let _ = X213NetworkAddress::from_str(sb8.as_str());
    let _ = X213NetworkAddress::from_str(sb9.as_str());
    let _ = X213NetworkAddress::from_str(sb10.as_str());
    let _ = X213NetworkAddress::from_str(sb11.as_str());
    let _ = X213NetworkAddress::from_str(sb12.as_str());
    let _ = X213NetworkAddress::from_str(url.as_str());
    let _ = X213NetworkAddress::from_str(x25_1.as_str());
    let _ = X213NetworkAddress::from_str(x25_2.as_str());
    let _ = X213NetworkAddress::from_str(x25_3.as_str());
    let _ = X213NetworkAddress::from_str(itot1.as_str());
    let _ = X213NetworkAddress::from_str(itot2.as_str());
    let _ = X213NetworkAddress::from_str(itot3.as_str());
    let _ = X213NetworkAddress::from_str(x121.as_str());
});

