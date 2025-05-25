#![no_main]

use libfuzzer_sys::fuzz_target;

extern crate asn1;

fuzz_target!(|data: wildboar_asn1::BIT_STRING| {
    if data.get_trailing_bits_count() >= 8 || data.len_in_bytes() == 0 {
        return;
    }
    let _ = wildboar_asn1::join_bit_strings(&[ data.to_owned(), wildboar_asn1::BIT_STRING::new() ]);
    let _ = wildboar_asn1::join_bit_strings(&[ wildboar_asn1::BIT_STRING::new(), data.to_owned() ]);
    let _ = wildboar_asn1::join_bit_strings(&[ data.to_owned(), data.to_owned() ]);
    let _ = wildboar_asn1::join_bit_strings(&[ data.to_owned(), data.to_owned(), data.to_owned() ]);
    let _ = wildboar_asn1::join_bit_strings(&[ data.to_owned(), data.to_owned(), wildboar_asn1::BIT_STRING::new(), data.to_owned() ]);
    let _ = wildboar_asn1::join_bit_strings(&[ data.to_owned(), data.to_owned(), wildboar_asn1::BIT_STRING::from_bin("1"), data.to_owned() ]);
    let _ = wildboar_asn1::join_bit_strings(&[ wildboar_asn1::BIT_STRING::from_bin("0"), data.to_owned(), wildboar_asn1::BIT_STRING::from_bin("1") ]);

    let bin = data.to_string();
    let decoded = wildboar_asn1::BIT_STRING::from_bin(&bin[1..bin.len()-2]);
    assert_eq!(data, decoded);
    assert_eq!(decoded, data);
    assert_eq!(decoded, decoded);

    let _ = data == wildboar_asn1::BIT_STRING::new();
    let _ = wildboar_asn1::BIT_STRING::new() == data;
    let _ = data == data;

    let len = data.len_in_bits();
    let mut data2 = data.to_owned();
    for i in 0..len {
        let _ = data2.get(i);
        data2.set(i, true);
    }
});
