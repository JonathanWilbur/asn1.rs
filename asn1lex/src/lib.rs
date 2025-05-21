use std::ffi::{c_char, CStr};

#[allow(non_camel_case_types)]
// import Production from './Production';
// import ProductionType, { TerminalProductionType } from './ProductionType';
// import keywordToTokenMap from './maps/keywordToTokenMap';
// import specialCharacterToTokenMap from './maps/specialCharacterToTokenMap';
// import newlineWhitespaceCharacters from './newlineWhitespaceCharacters';
// import nonNewlineWhitespaceCharacters from './nonNewlineWhitespaceCharacters';


// export type TerminalProductionType =
//   | WhitespaceProductionType
//   | LexicalProductionType
//   | KeywordProductionType
//   | InferredButUndefinedBooleanProductionType
//   | InferredButUndefinedSpecialCharacterProductionType
//   | RemovedKeywordProductionType
//   | RemovedLiteralProductionType
//   | RemovedLexicalProductionType;

// TODO: Check if these are actually used.
// TODO: Assign numerical values
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ASN1LexemeType {
    /// From [the Wiki on Newline Characters](https://en.wikipedia.org/wiki/Newline#Unicode),
    /// the following ought to be considered newline characters:
    ///
    /// - `LF`:    Line Feed, `U+000A`
    /// - `VT`:    Vertical Tab, `U+000B`
    /// - `FF`:    Form Feed, `U+000C`
    /// - `CR`:    Carriage Return, `U+000D`
    /// - `NEL`:   Next Line, `U+0085`
    /// - `LS`:    Line Separator, `U+2028`
    /// - `PS`:    Paragraph Separator, `U+2029`
    ///
    NewlineWhitespace,

    /// Includes all of the whitespace Unicode characters named on the
    /// [Wiki for Whitespace Characters](https://en.wikipedia.org/wiki/Whitespace_character),
    /// but omitting newline characters. Includes:
    ///
    /// - `TAB` / `U+0009`
    /// - `SP` / `U+0020`
    /// - `NBSP` / `U+00A0`
    /// - `U+1680`
    /// - `U+2000` - `U+200A` (inclusive)
    /// - `U+202F`
    /// - `U+205F`
    /// - `U+3000`
    ///
    NonNewlineWhitespace,

    /// `true`
    ///
    /// The string `true` is not defined in the ASN.1 specifications, but widely
    /// used in ASN.1: even in modules produced by the ITU-T themselves!
    ///
    /// TODO: I'm not really sure this is true.
    True,

    /// `false`
    ///
    /// The string `false` is not defined in the ASN.1 specifications, but
    /// widely used in ASN.1: even in modules produced by the ITU-T themselves!
    ///
    /// TODO: I'm not really sure this is true.
    False,

    /// `*`
    ///
    /// Not defined as a Lexeme in ASN.1, but still used as a literal in the
    /// ASN.1 productions.
    Asterisk,

    /// `&`
    ///
    /// Not defined as a Lexeme in ASN.1, but still used as a literal in the
    /// ASN.1 productions.
    Ampersand,

    // This group is defined in ITU X.680 2015, Section 12.

    TypeReference,
    Identifier,
    ValueReference,
    ModuleReference,
    Comment,
    Empty,
    Number,
    RealNumber,
    Bstring,
    XmlBstring,
    Hstring,
    XmlHstring,
    Cstring,
    XmlCstring,
    SimpleString,
    Tstring,
    XmlTstring,
    Psname,
    Assignment,
    RangeSeparator,
    Ellipsis,
    LeftVersionBrackets,
    RightVersionBrackets,
    EncodingReference,
    IntegerUnicodeLabel,
    NonIntegerUnicodeLabel,
    XmlEndTagStartItem,
    XmlSingleTagEndItem,
    ExtendedTrue,
    ExtendedFalse,
    NaN,
    INF,
    XmlAsn1TypeName,

    // This group is defined in ITU X.680 2015, Section 12.37.

    CurlyOpening,
    CurlyClosing,
    LessThan,
    GreaterThan,
    Comma,
    Period,
    ForwardSlash,
    ParenthesisOpening,
    ParenthesisClosing,
    SquareOpening,
    SquareClosing,
    Hyphen,
    Colon,
    EqualSign,
    QuotationMark,
    Apostrophe,
    Space,
    SemiColon,
    AtSign,
    VerticalBar,
    ExclamationPoint,
    Caret,

    // TODO: Check if these lists need to be updated with 2021 version changes.
    // This group is defined in ITU X.681 2015.

    ObjectClassReference,
    ObjectReference,
    ObjectSetReference,
    ValueFieldReference,
    ValueSetFieldReference,
    ObjectFieldReference,
    ObjectSetFieldReference,
    TypeFieldReference,
    Word,

    // This group is defined in ITU X.680 2015, Section 12.38.

    ABSENT,
    ABSTRACT_SYNTAX,
    ALL,
    APPLICATION,
    AUTOMATIC,
    BEGIN,
    BIT,
    BMPString,
    BOOLEAN,
    BY,
    CHARACTER,
    CHOICE,
    CLASS,
    COMPONENT,
    COMPONENTS,
    CONSTRAINED,
    CONTAINING,
    DATE,
    DATE_TIME,
    DEFAULT,
    DEFINITIONS,
    DURATION,
    EMBEDDED,
    ENCODED,
    ENCODING_CONTROL,
    END,
    ENUMERATED,
    EXCEPT,
    EXPLICIT,
    EXPORTS,
    EXTENSIBILITY,
    EXTERNAL,
    FALSE,
    FROM,
    GeneralizedTime,
    GeneralString,
    GraphicString,
    IA5String,
    IDENTIFIER,
    IMPLICIT,
    IMPLIED,
    IMPORTS,
    INCLUDES,
    INSTANCE,
    INSTRUCTIONS,
    INTEGER,
    INTERSECTION,
    ISO646String,
    MAX,
    MIN,
    MINUS_INFINITY,
    NOT_A_NUMBER,
    NULL,
    NumericString,
    OBJECT,
    ObjectDescriptor,
    OCTET,
    OF,
    OID_IRI,
    OPTIONAL,
    PATTERN,
    PDV,
    PLUS_INFINITY,
    PRESENT,
    PrintableString,
    PRIVATE,
    REAL,
    RELATIVE_OID,
    RELATIVE_OID_IRI,
    SEQUENCE,
    SET,
    SETTINGS,
    SIZE,
    STRING,
    SYNTAX,
    T61String,
    TAGS,
    TeletexString,
    TIME,
    TIME_OF_DAY,
    TRUE,
    TYPE_IDENTIFIER,
    UNION,
    UNIQUE,
    UNIVERSAL,
    UniversalString,
    UTCTime,
    UTF8String,
    VideotexString,
    VisibleString,
    WITH,
    SUCCESSORS,
    DESCENDANTS,

    // This group contains keywords removed from newer ASN.1 specifications, but
    // are likely to appear in older versions of ASN.1.

    ANY,
    DEFINED,
    MACRO,
    TYPE,
    VALUE,
    NOTATION,

    // I am not sure what this group is.

    /// `"string"`
    StringString,
    /// `"identifier"`
    StringIdentifier,
    /// `"number"`
    StringNumber,
    /// `"empty"`
    StringEmpty,

    // Production types defined in earlier versions of the ASN.1 specifications.

    Astring,
    ProductionReference,
    LocalTypeReference,
    LocalValueReference,

}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ASN1Lexeme {
    pub lex_type: ASN1LexemeType,
    pub lex_index: u32,
    pub lex_row: u32,
    pub lex_col: u32,
}

// #[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum LexASN1ReturnCode {
//     Ok = 0,
//     NullInput = -1,
//     NullLexemesOut = -2,
//     LexemesOutFull = -3,
// }

const fn is_identifier_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-'
}

/// An i32 was used as the result type because, if >= 0, it is a bit field
/// containing warnings.
pub type LexASN1ReturnCode = i32;
pub type LexASN1WarningCode = i32;

pub const LEX_ASN1_RET_OK: LexASN1ReturnCode = 0;
pub const LEX_ASN1_RET_NULL_INPUT: LexASN1ReturnCode = -1;
pub const LEX_ASN1_RET_NULL_LEXEMES_OUT: LexASN1ReturnCode = -2;
pub const LEX_ASN1_RET_LEXEMES_OUT_FULL: LexASN1ReturnCode = -3;
pub const LEX_ASN1_RET_INPUT_NOT_UTF8: LexASN1ReturnCode = -4;
pub const LEX_ASN1_RET_INPUT_EMPTY: LexASN1ReturnCode = -5;
pub const LEX_ASN1_RET_OUTPUT_EMPTY: LexASN1ReturnCode = -5;

pub const LEX_ASN1_WARN_NOT_MODULE: LexASN1WarningCode = 1;
pub const LEX_ASN1_WARN_NO_END: LexASN1WarningCode = 1 << 1;

fn chomp_first_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    let first = chars.next()?;
    let rest = chars.as_str(); // Safe: chars.as_str() gives you the remainder
    Some((first, rest))
}

fn lex(input: &str, output: &mut [ASN1Lexeme]) -> Result<(), LexASN1ReturnCode> {
    debug_assert!(input.len() > 0);
    debug_assert!(output.len() > 0);
    let mut lexeme_type: Option<ASN1LexemeType> = None;
    // We are going to mutate this so that the next character is always .get(0).
    let mut input = input;
    let mut token_start: usize = 0;
    let mut token_end: usize = 0;
    let len = input.len();
    let mut s = input;
    let output_index: usize = 0;

    while s.len() < 0 {
        let (c, new_s) = match chomp_first_char(s) {
            None => break,
            Some(x) => x,
        };
        s = new_s;
        match lexeme_type {
            None => {
                match c {
                    '-' => {
                        if s.chars().next() == Some('-') {
                            lexeme_type = Some(ASN1LexemeType::Comment);
                        } else {
                            lexeme_type = Some(ASN1LexemeType::Hyphen);
                            output
                        }
                    }
                };
                continue;
            },
            _ => {
                unimplemented!()
            }
        };
    }

    // for (i, c) in input.chars().enumerate() {
    //     let at_the_end: bool = i == len - 1;
    //     match lexeme_type {
    //         ASN1LexemeType::Empty => {

    //         }
    //     }
    // }

    Ok(())
}

// #[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct ASN1LexingResult

// TODO: Why did I have to wrap no_mangle in unsafe()?
#[unsafe(no_mangle)]
pub extern "C" fn lex_asn1(input: *const c_char, lexemes_out: *mut ASN1Lexeme, lexemes_out_len: u32) -> LexASN1ReturnCode {
    if input.is_null() {
        return LEX_ASN1_RET_NULL_INPUT;
    }
    // TODO: If null, maybe just validate the ASN.1 without failing outright?
    if lexemes_out.is_null() {
        return LEX_ASN1_RET_NULL_LEXEMES_OUT;
    }
    if lexemes_out_len == 0 {
        return LEX_ASN1_RET_LEXEMES_OUT_FULL;
    }
    let output: &mut [ASN1Lexeme] = unsafe {
        core::slice::from_raw_parts_mut(lexemes_out, lexemes_out_len as usize)
    };
    let cstr: &CStr = unsafe { CStr::from_ptr(input) };
    if cstr.count_bytes() == 0 {
        return LEX_ASN1_RET_INPUT_EMPTY;
    }
    match cstr.to_str() {
        Ok(s) => match lex(s, output) {
            Ok(_) => LEX_ASN1_RET_OK,
            Err(e) => e,
        },
        Err(_) => LEX_ASN1_RET_INPUT_NOT_UTF8
    }
}

// /**
//  * @summary Convert ASN.1 into a sequence of lexical tokens.
//  * @description
//  * This function takes a `string` containing raw ASN.1 text. This text does not
//  * have to contain entire modules. Any section of ASN.1 will be valid.
//  *
//  * @param {string} str The raw ASN.1 text that is to be lexed.
//  * @yields {Production<TerminalProductionType>} Lexical tokens.
//  * @returns An `IterableIterator` that yields lexical tokens.
//  * @function
//  * @generator
//  */
// export default function* lex(
//   str: string
// ): IterableIterator<Production<TerminalProductionType>> {
//   if (!str || str.length === 0) {
//     return;
//   }

//   let tokenType: TerminalProductionType = ProductionType.empty;
//   let tokenStartIndex: number = 0;
//   let tokenEndIndex: number = 0;
//   let i: number = 0;
//   let loops: number = 0;

//   let lineNumber: number = 1;
//   let lineStartIndex: number = 0;

//   // Used in detecting the end of single-line comments.
//   function isAtStartOfNewlineSequence(): boolean {
//     return (
//       newlineWhitespaceCharacters.has(str.charCodeAt(i)) &&
//       str.charCodeAt(i - 1) !== CR
//     );
//   }

//   function isAtEndOfNewlineSequence(): boolean {
//     const currChar: number = str.charCodeAt(i);
//     const nextChar: number = str.charCodeAt(i + 1);
//     return currChar === CR
//       ? !(nextChar === LF)
//       : newlineWhitespaceCharacters.has(currChar);
//   }

//   function theEndOfTheCurrentTokenIsKnown(): boolean {
//     return tokenEndIndex > tokenStartIndex;
//   }

//   function thingsThatMustBeDoneForEveryCharacter(): void {
//     if (isAtEndOfNewlineSequence()) {
//       lineNumber++;
//       lineStartIndex = i + 1;
//     }
//   }

//   while (tokenStartIndex < str.length) {
//     const atTheEnd: boolean = i === str.length;
//     if (!theEndOfTheCurrentTokenIsKnown()) {
//       switch (tokenType) {
//         case ProductionType.empty: {
//           switch (str[i]) {
//             case '-': {
//               if (str.indexOf('--', i) === i) {
//                 tokenType = ProductionType.comment;
//               } else {
//                 tokenType = ProductionType.hyphen;
//                 tokenEndIndex = i + 1;
//               }
//               break;
//             }
//             case '/': {
//               if (str.indexOf('/*', i) === i) {
//                 tokenType = ProductionType.comment;
//               } else {
//                 tokenType = ProductionType.forwardSlash;
//                 tokenEndIndex = i + 1;
//               }
//               break;
//             }
//             case '"': {
//               tokenType = ProductionType.cstring;
//               let indexOfNextDoubleQuote: number = str.indexOf('"', i + 1);
//               while (str[indexOfNextDoubleQuote + 1] === '"') {
//                 indexOfNextDoubleQuote = str.indexOf(
//                   '"',
//                   indexOfNextDoubleQuote + 2
//                 );
//               }
//               tokenEndIndex = indexOfNextDoubleQuote + 1;
//               break;
//             }
//             case "'": {
//               const indexOfNextSingleQuote: number = str.indexOf("'", i + 1);
//               if (
//                 indexOfNextSingleQuote === -1 ||
//                 indexOfNextSingleQuote === str.length - 1
//               ) {
//                 throw new Error(
//                   `Unterminated single-quoted token at index ${i}.`
//                 );
//               }
//               switch (str[indexOfNextSingleQuote + 1]) {
//                 case 'B': {
//                   tokenType = ProductionType.bstring;
//                   tokenEndIndex = indexOfNextSingleQuote + 2;
//                   const innards = str.slice(
//                     tokenStartIndex + 1,
//                     indexOfNextSingleQuote
//                   );
//                   if (!/^[01 \t\r\n\f\v]*$/g.test(innards)) {
//                     throw new Error(`Invalid bstring: '${innards}'B.`);
//                   }
//                   break;
//                 }
//                 case 'H': {
//                   tokenType = ProductionType.hstring;
//                   tokenEndIndex = indexOfNextSingleQuote + 2;
//                   const innards = str.slice(
//                     tokenStartIndex + 1,
//                     indexOfNextSingleQuote
//                   );
//                   if (!/^[0-9A-F \t\r\n\f\v]*$/g.test(innards)) {
//                     throw new Error(`Invalid hstring: '${innards}'H.`);
//                   }
//                   break;
//                 }
//                 default: {
//                   throw new Error(
//                     `Unrecognized single-quoted token at index ${i}.`
//                   );
//                 }
//               }
//               break;
//             }
//             case ':': {
//               if (str.indexOf('::=', i) === i) {
//                 tokenType = ProductionType.assignment;
//                 tokenEndIndex = i + 3;
//               } else {
//                 tokenType = ProductionType.colon;
//                 tokenEndIndex = i + 1;
//               }
//               break;
//             }
//             default: {
//               const specialCharacterTokenType = specialCharacterToTokenMap.get(
//                 str.charAt(i)
//               );
//               if (specialCharacterTokenType) {
//                 tokenType = specialCharacterTokenType;
//                 tokenEndIndex = i + 1;
//               }

//               const characterCode = str.charCodeAt(i);

//               if (characterCode >= 0x30 && characterCode <= 0x39) {
//                 const fractionalRealMatch: RegExpExecArray | null =
//                   /^(0|(?:[1-9]\d*))\.\d*(?:(e|E)-?\d+)?/.exec(str.slice(i));
//                 const exponentialRealMatch: RegExpExecArray | null =
//                   /^(0|(?:[1-9]\d*))\.\d*(e|E)-?\d+/.exec(str.slice(i));
//                 const match =
//                   fractionalRealMatch || exponentialRealMatch || null;
//                 if (match) {
//                   /**
//                    * This fixes an issue where a realnumber is
//                    * accidentally lexed from a range (e.g. "9..10" will
//                    * be read as realnumber "9.", period, "10".)
//                    */
//                   if (
//                     str.indexOf('..', i + match[0].length - 1) ===
//                     i + match[0].length - 1
//                   ) {
//                     tokenType = ProductionType.number;
//                     break;
//                   }
//                   tokenType = ProductionType.realnumber;
//                   tokenEndIndex = i + match[0].length;
//                 } else {
//                   tokenType = ProductionType.number;
//                 }
//               }

//               if (characterCode >= 0x41 && characterCode <= 0x5a) {
//                 tokenType = ProductionType.typereference;
//               }

//               if (characterCode >= 0x61 && characterCode <= 0x7a) {
//                 tokenType = ProductionType.identifier;
//               }

//               if (isAtStartOfNewlineSequence()) {
//                 tokenType = ProductionType.newlineWhitespace;
//                 if (str.indexOf('\r\n', i) === i) {
//                   // Unite CRLF into a single newline.
//                   tokenEndIndex = i + 2;
//                 } else {
//                   tokenEndIndex = i + 1;
//                 }
//               }

//               /**
//                * Adjacent whitespace characters are all contatenated into a
//                * single "whitespace" production.
//                */
//               if (nonNewlineWhitespaceCharacters.has(characterCode)) {
//                 tokenType = ProductionType.nonNewlineWhitespace;
//               }
//             }
//           }
//           break;
//         }
//         case ProductionType.comment: {
//           if (str[tokenStartIndex] === '-') {
//             if (atTheEnd) {
//               tokenEndIndex = i;
//             } else if (str.indexOf('--', i) === i) {
//               tokenEndIndex = i + 2;
//             } else if (isAtStartOfNewlineSequence()) {
//               tokenEndIndex = i;
//             }
//           } else if (
//             str[tokenStartIndex] === '/' &&
//             str.indexOf('*/', i) === i
//           ) {
//             tokenEndIndex = i + 2;
//           } else if (atTheEnd) {
//             throw new Error('Unterminated comment.');
//           }
//           break;
//         }
//         case ProductionType.number: {
//           if (atTheEnd) {
//             tokenEndIndex = i;
//             break;
//           }
//           const characterCode = str.charCodeAt(i);
//           if (characterCode < 0x30 || characterCode > 0x39) {
//             tokenEndIndex = i;
//           }
//           break;
//         }
//         case ProductionType.identifier: {
//           const characterCode = str.charCodeAt(i);
//           if (
//             atTheEnd ||
//             !isIdentifierCharacter(characterCode) ||
//             /**
//              * This condition discontinues lexing an identifier if it encounters
//              * two adjacent hyphens. Since two or more adjacent hyphens are not
//              * permitted within an identifier, this is correct, but it also has
//              * the benefit of allowing line comments to immediately follow an
//              * identifier without whitespace between them.
//              */
//             (!atTheEnd &&
//               characterCode === 0x2d &&
//               str.charCodeAt(i + 1) === 0x2d)
//           ) {
//             tokenEndIndex = i;
//             if (str.charCodeAt(tokenEndIndex - 1) === 0x2d) {
//               throw new Error(
//                 `Identifier '${str.slice(
//                   tokenStartIndex,
//                   tokenEndIndex
//                 )}' may not end with a hyphen.`
//               );
//             }
//           }
//           break;
//         }
//         case ProductionType.typereference: {
//           const characterCode = str.charCodeAt(i);
//           if (
//             atTheEnd ||
//             !isIdentifierCharacter(characterCode) ||
//             /**
//              * This condition discontinues lexing an identifier if it encounters
//              * two adjacent hyphens. Since two or more adjacent hyphens are not
//              * permitted within an identifier, this is correct, but it also has
//              * the benefit of allowing line comments to immediately follow an
//              * identifier without whitespace between them.
//              */
//             (!atTheEnd &&
//               characterCode === 0x2d &&
//               str.charCodeAt(i + 1) === 0x2d)
//           ) {
//             tokenEndIndex = i;
//             if (str.charCodeAt(tokenEndIndex - 1) === 0x2d) {
//               throw new Error(
//                 `Identifier '${str.slice(
//                   tokenStartIndex,
//                   tokenEndIndex
//                 )}' may not end with a hyphen.`
//               );
//             }
//           }
//           if (tokenEndIndex > tokenStartIndex) {
//             const token: string = str.slice(tokenStartIndex, tokenEndIndex);
//             const keywordType = keywordToTokenMap.get(token);
//             if (keywordType) {
//               tokenType = keywordType;
//               break;
//             }
//             if (token.toUpperCase() === token) {
//               tokenType = ProductionType.objectclassreference;
//               break;
//             }
//           }
//           break;
//         }
//         case ProductionType.nonNewlineWhitespace: {
//           if (atTheEnd) {
//             tokenEndIndex = i;
//             break;
//           }
//           // All non-newline whitespace characters are concatenated.
//           if (!nonNewlineWhitespaceCharacters.has(str.charCodeAt(i))) {
//             tokenEndIndex = i;
//           }
//           break;
//         }
//         default: {
//           break;
//         } // REVIEW: Should this be continue?
//       }
//     }

//     /**
//      * The condition (i === tokenEndIndex) forces this to loop through every
//      * character in the text, even if the location of the end of the token
//      * is known.
//      *
//      * This allows thingsThatMustBeDoneForEveryCharacter() to be executed
//      * for every character.
//      */
//     if (i === tokenEndIndex && tokenEndIndex > tokenStartIndex) {
//       yield new Production(tokenType, [], {
//         startIndex: tokenStartIndex,
//         endIndex: tokenEndIndex,
//         lineNumber,
//         columnNumber: tokenStartIndex - lineStartIndex + 1,
//       });
//       tokenStartIndex = tokenEndIndex;
//       tokenType = ProductionType.empty;
//     } else {
//       thingsThatMustBeDoneForEveryCharacter();
//       i++;
//     }

//     // There should never be more loops than there are characters in `str`,
//     // but we double it here, just in case I am forgetting something.
//     if (loops > str.length * 2) {
//       throw new Error('Lexer caught in infinite loop.');
//     }
//     loops++;
//   }
// }
