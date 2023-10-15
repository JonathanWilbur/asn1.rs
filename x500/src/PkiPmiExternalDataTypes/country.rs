pub fn x121_dcc_country_code_to_iso_3166 (dcc: u16) -> Option<&'static str> {
    match dcc {
        202 => Some("GR"), // Greece
        204 => Some("NL"), // Netherlands (Kingdom of the)
        205 => Some("NL"), // Netherlands (Kingdom of the)
        206 => Some("BE"), // Belgium
        208 => Some("FR"), // France
        209 => Some("FR"), // France
        210 => Some("FR"), // France
        211 => Some("FR"), // France
        212 => Some("MC"), // Monaco (Principality of)
        213 => Some("AD"), // Andorra (Principality of)
        214 => Some("ES"), // Spain
        215 => Some("ES"), // Spain
        216 => Some("HU"), // Hungary (Republic of)
        218 => Some("BA"), // Bosnia and Herzegovina
        219 => Some("HR"), // Croatia (Republic of)
        220 => Some("YU"), // Yugoslavia (Federal Republic of) [Note: "YU" is outdated since Yugoslavia dissolved; you might want to use "RS" for Serbia or another relevant code]
        222 => Some("IT"), // Italy
        223 => Some("IT"), // Italy
        224 => Some("IT"), // Italy
        225 => Some("VA"), // Vatican City State
        226 => Some("RO"), // Romania
        228 => Some("CH"), // Switzerland (Confederation of)
        229 => Some("CH"), // Switzerland (Confederation of)
        230 => Some("CZ"), // Czech Republic
        231 => Some("SK"), // Slovak Republic
        232 => Some("AT"), // Austria
        233 => Some("AT"), // Austria
        234 => Some("GB"), // United Kingdom of Great Britain and Northern Ireland
        235 => Some("GB"), // United Kingdom of Great Britain and Northern Ireland
        236 => Some("GB"), // United Kingdom of Great Britain and Northern Ireland
        237 => Some("GB"), // United Kingdom of Great Britain and Northern Ireland
        238 => Some("DK"), // Denmark
        239 => Some("DK"), // Denmark
        240 => Some("SE"), // Sweden
        242 => Some("NO"), // Norway
        243 => Some("NO"), // Norway
        244 => Some("FI"), // Finland
        246 => Some("LT"), // Lithuania (Republic of)
        247 => Some("LV"), // Latvia (Republic of)
        248 => Some("EE"), // Estonia (Republic of)
        250 => Some("RU"), // Russian Federation
        251 => Some("RU"), // Russian Federation
        255 => Some("UA"), // Ukraine
        257 => Some("BY"), // Belarus (Republic of)
        259 => Some("MD"), // Moldova (Republic of)
        260 => Some("PL"), // Poland (Republic of)
        261 => Some("PL"), // Poland (Republic of)
        262 => Some("DE"), // Germany (Federal Republic of)
        263 => Some("DE"), // Germany (Federal Republic of)
        264 => Some("DE"), // Germany (Federal Republic of)
        265 => Some("DE"), // Germany (Federal Republic of)
        266 => Some("GI"), // Gibraltar
        268 => Some("PT"), // Portugal
        269 => Some("PT"), // Portugal
        270 => Some("LU"), // Luxembourg
        272 => Some("IE"), // Ireland
        274 => Some("IS"), // Iceland
        276 => Some("AL"), // Albania (Republic of)
        278 => Some("MT"), // Malta
        280 => Some("CY"), // Cyprus (Republic of)
        282 => Some("GE"), // Georgia
        283 => Some("AM"), // Armenia (Republic of)
        284 => Some("BG"), // Bulgaria (Republic of)
        286 => Some("TR"), // Turkey
        288 => Some("FO"), // Faroe Islands
        290 => Some("GL"), // Greenland
        292 => Some("SM"), // San Marino (Republic of)
        293 => Some("SI"), // Slovenia (Republic of)
        294 => Some("MK"), // The Former Yugoslav Republic of Macedonia [Note: This is the old name. Now officially "North Macedonia"]
        295 => Some("LI"), // Liechtenstein (Principality of)
        302 => Some("CA"), // Canada
        303 => Some("CA"), // Canada
        308 => Some("PM"), // Saint Pierre and Miquelon (Collectivité territoriale de la République française)
        310 => Some("US"), // United States of America
        311 => Some("US"), // United States of America
        312 => Some("US"), // United States of America
        313 => Some("US"), // United States of America
        314 => Some("US"), // United States of America
        315 => Some("US"), // United States of America
        316 => Some("US"), // United States of America
        330 => Some("PR"), // Puerto Rico
        332 => Some("VI"), // United States Virgin Islands
        334 => Some("MX"), // Mexico
        335 => Some("MX"), // Mexico
        338 => Some("JM"), // Jamaica
        340 => Some("GP"), // Guadeloupe (This code represents Guadeloupe, separate code might be needed for Martinique)
        342 => Some("BB"), // Barbados
        344 => Some("AG"), // Antigua and Barbuda
        346 => Some("KY"), // Cayman Islands
        348 => Some("VG"), // British Virgin Islands
        350 => Some("BM"), // Bermuda
        352 => Some("GD"), // Grenada
        354 => Some("MS"), // Montserrat
        356 => Some("KN"), // Saint Kitts and Nevis
        358 => Some("LC"), // Saint Lucia
        360 => Some("VC"), // Saint Vincent and the Grenadines
        362 => Some("AN"), // Netherlands Antilles [Note: This code is deprecated since the dissolution of the Netherlands Antilles in 2010]
        363 => Some("AW"), // Aruba
        364 => Some("BS"), // Bahamas (Commonwealth of the)
        365 => Some("AI"), // Anguilla
        366 => Some("DM"), // Dominica (Commonwealth of)
        368 => Some("CU"), // Cuba
        370 => Some("DO"), // Dominican Republic
        372 => Some("HT"), // Haiti (Republic of)
        374 => Some("TT"), // Trinidad and Tobago
        376 => Some("TC"), // Turks and Caicos Islands
        400 => Some("AZ"), // Azerbaijani Republic
        401 => Some("KZ"), // Kazakstan (Republic of)
        404 => Some("IN"), // India (Republic of)
        410 => Some("PK"), // Pakistan (Islamic Republic of)
        411 => Some("PK"), // Pakistan (Islamic Republic of)
        412 => Some("AF"), // Afghanistan (Islamic State of)
        413 => Some("LK"), // Sri Lanka (Democratic Socialist Republic of)
        414 => Some("MM"), // Myanmar (Union of) [Note: Historically referred to as Burma]
        415 => Some("LB"), // Lebanon
        416 => Some("JO"), // Jordan (Hashemite Kingdom of)
        417 => Some("SY"), // Syrian Arab Republic
        418 => Some("IQ"), // Iraq (Republic of)
        419 => Some("KW"), // Kuwait (State of)
        420 => Some("SA"), // Saudi Arabia (Kingdom of)
        421 => Some("YE"), // Yemen (Republic of)
        422 => Some("OM"), // Oman (Sultanate of)
        424 => Some("AE"), // United Arab Emirates
        425 => Some("IL"), // Israel (State of)
        426 => Some("BH"), // Bahrain (State of)
        427 => Some("QA"), // Qatar (State of)
        428 => Some("MN"), // Mongolia
        429 => Some("NP"), // Nepal
        430 => Some("AE"), // United Arab Emirates (Abu Dhabi)
        431 => Some("AE"), // United Arab Emirates (Dubai)
        432 => Some("IR"), // Iran (Islamic Republic of)
        434 => Some("UZ"), // Uzbekistan (Republic of)
        436 => Some("TJ"), // Tajikistan (Republic of)
        437 => Some("KG"), // Kyrgyz Republic
        438 => Some("TM"), // Turkmenistan
        440 => Some("JP"), // Japan
        441 => Some("JP"), // Japan
        442 => Some("JP"), // Japan
        443 => Some("JP"), // Japan
        450 => Some("KR"), // Korea (Republic of)
        452 => Some("VN"), // Viet Nam (Socialist Republic of)
        453 => Some("HK"), // Hongkong
        454 => Some("HK"), // Hongkong
        455 => Some("MO"), // Macau
        456 => Some("KH"), // Cambodia (Kingdom of)
        457 => Some("LA"), // Lao People’s Democratic Republic
        460 => Some("CN"), // China (People’s Republic of)
        466 => Some("TW"), // Taiwan, China
        467 => Some("KP"), // Democratic People’s Republic of Korea
        470 => Some("BD"), // Bangladesh (People’s Republic of)
        472 => Some("MV"), // Maldives (Republic of)
        480 => Some("KR"), // Korea (Republic of)
        481 => Some("KR"), // Korea (Republic of)
        502 => Some("MY"), // Malaysia
        505 => Some("AU"), // Australia
        510 => Some("ID"), // Indonesia (Republic of)
        515 => Some("PH"), // Philippines (Republic of the)
        520 => Some("TH"), // Thailand
        525 => Some("SG"), // Singapore (Republic of)
        528 => Some("BN"), // Brunei Darussalam
        530 => Some("NZ"), // New Zealand
        534 => Some("MP"), // Northern Mariana Islands (Commonwealth of the)
        535 => Some("GU"), // Guam
        536 => Some("NR"), // Nauru (Republic of)
        537 => Some("PG"), // Papua New Guinea
        539 => Some("TO"), // Tonga (Kingdom of)
        540 => Some("SB"), // Solomon Islands
        541 => Some("VU"), // Vanuatu (Republic of)
        542 => Some("FJ"), // Fiji (Republic of)
        543 => Some("WF"), // Wallis and Futuna (French Overseas Territory)
        544 => Some("AS"), // American Samoa
        545 => Some("KI"), // Kiribati (Republic of)
        546 => Some("NC"), // New Caledonia (French Overseas Territory)
        547 => Some("PF"), // French Polynesia (French Overseas Territory)
        548 => Some("CK"), // Cook Islands
        549 => Some("WS"), // Samoa (Independent State of)
        550 => Some("FM"), // Micronesia (Federated States of)
        602 => Some("EG"), // Egypt (Arab Republic of)
        603 => Some("DZ"), // Algeria (People’s Democratic Republic of)
        604 => Some("MA"), // Morocco (Kingdom of)
        605 => Some("TN"), // Tunisia
        606 => Some("LY"), // Libya (Socialist People’s Libyan Arab Jamahiriya)
        607 => Some("GM"), // Gambia (Republic of the)
        608 => Some("SN"), // Senegal (Republic of)
        609 => Some("MR"), // Mauritania (Islamic Republic of)
        610 => Some("ML"), // Mali (Republic of)
        611 => Some("GN"), // Guinea (Republic of)
        612 => Some("CI"), // Côte d’Ivoire (Republic of)
        613 => Some("BF"), // Burkina Faso
        614 => Some("NE"), // Niger (Republic of the)
        615 => Some("TG"), // Togolese Republic
        616 => Some("BJ"), // Benin (Republic of)
        617 => Some("MU"), // Mauritius (Republic of)
        618 => Some("LR"), // Liberia (Republic of)
        619 => Some("SL"), // Sierra Leone
        620 => Some("GH"), // Ghana
        621 => Some("NG"), // Nigeria (Federal Republic of)
        622 => Some("TD"), // Chad (Republic of)
        623 => Some("CF"), // Central African Republic
        624 => Some("CM"), // Cameroon (Republic of)
        625 => Some("CV"), // Cape Verde (Republic of)
        626 => Some("ST"), // Sao Tome and Principe (Democratic Republic of)
        627 => Some("GQ"), // Equatorial Guinea (Republic of)
        628 => Some("GA"), // Gabonese Republic
        629 => Some("CG"), // Congo (Republic of the)
        630 => Some("CD"), // Democratic Republic of the Congo
        631 => Some("AO"), // Angola (Republic of)
        632 => Some("GW"), // Guinea-Bissau (Republic of)
        633 => Some("SC"), // Seychelles (Republic of)
        634 => Some("SD"), // Sudan (Republic of the)
        635 => Some("RW"), // Rwandese Republic
        636 => Some("ET"), // Ethiopia (Federal Democratic Republic of)
        637 => Some("SO"), // Somali Democratic Republic
        638 => Some("DJ"), // Djibouti (Republic of)
        639 => Some("KE"), // Kenya (Republic of)
        640 => Some("TZ"), // Tanzania (United Republic of)
        641 => Some("UG"), // Uganda (Republic of)
        642 => Some("BI"), // Burundi (Republic of)
        643 => Some("MZ"), // Mozambique (Republic of)
        645 => Some("ZM"), // Zambia (Republic of)
        646 => Some("MG"), // Madagascar (Republic of)
        647 => Some("RE"), // Reunion (French Department of)
        648 => Some("ZW"), // Zimbabwe (Republic of)
        649 => Some("NA"), // Namibia (Republic of)
        650 => Some("MW"), // Malawi
        651 => Some("LS"), // Lesotho (Kingdom of)
        652 => Some("BW"), // Botswana (Republic of)
        653 => Some("SZ"), // Swaziland (Kingdom of)
        654 => Some("KM"), // Comoros (Islamic Federal Republic of the)
        655 => Some("ZA"), // South Africa (Republic of)
        658 => Some("ER"), // Eritrea
        702 => Some("BZ"), // Belize
        704 => Some("GT"), // Guatemala (Republic of)
        706 => Some("SV"), // El Salvador (Republic of)
        708 => Some("HN"), // Honduras (Republic of)
        710 => Some("NI"), // Nicaragua
        712 => Some("CR"), // Costa Rica
        714 => Some("PA"), // Panama (Republic of)
        716 => Some("PE"), // Peru
        722 => Some("AR"), // Argentine Republic
        724 => Some("BR"), // Brazil (Federative Republic of)
        725 => Some("BR"), // Brazil (Federative Republic of)
        730 => Some("CL"), // Chile
        732 => Some("CO"), // Colombia (Republic of)
        734 => Some("VE"), // Venezuela (Bolivarian Republic of)
        736 => Some("BO"), // Bolivia (Republic of)
        738 => Some("GY"), // Guyana
        740 => Some("EC"), // Ecuador
        742 => Some("GF"), // Guiana (French Department of)
        744 => Some("PY"), // Paraguay (Republic of)
        746 => Some("SR"), // Suriname (Republic of)
        748 => Some("UY"), // Uruguay (Eastern Republic of)
        _ => None,
    }
}