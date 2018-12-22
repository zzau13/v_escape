use nom::{self, AsBytes, Needed};
use std::{i8, str};

type Input<'a> = nom::types::CompleteByteSlice<'a>;

#[allow(non_snake_case)]
fn Input(input: &[u8]) -> Input {
    nom::types::CompleteByteSlice(input)
}

#[derive(Debug, PartialEq)]
pub struct Pair<'a> {
    pub char: u8,
    pub quote: &'a [u8],
}

impl<'a> Pair<'a> {
    pub fn new(char: u8, quote: &[u8]) -> Pair {
        Pair { char, quote }
    }
}

named!(parse_syntax<Input, Vec<Pair>>, many1!(parse_pair));

named!(parse_pair<Input, Pair>, map!(
    separated_pair!(is_char, tag!("->"), alt!(take_until_and_consume!(" || ") | nom::rest)),
    |s| Pair::new(s.0, &s.1)
));

macro_rules! is_digit {
    ($name:ident, $base:expr) => {
        fn $name(s: Input) -> Result<u8, nom::Err<Input>> {
            if s.is_empty() {
                Err(nom::Err::Incomplete(Needed::Size(1)))
            } else {
                Ok(
                    i8::from_str_radix(str::from_utf8(&s.as_bytes()).unwrap(), $base)
                        .expect("overflow at i8") as u8,
                )
            }
        }
    };
}

is_digit!(is_digit_8, 8);
is_digit!(is_digit_10, 10);
is_digit!(is_digit_16, 16);

fn try_into_i8(s: Input) -> Result<u8, nom::Err<Input>> {
    let b = s.as_bytes();
    if b.len() == 1 {
        Ok(i8::from_str_radix(&b.first().unwrap().to_string(), 10).unwrap() as u8)
    } else {
        Err(nom::Err::Incomplete(Needed::Size(1)))
    }
}

named!(is_char<Input, u8>, alt!(
    map_res!(preceded!(tag!("0x"), take_while!(nom::is_hex_digit)), is_digit_16) |
    map_res!(preceded!(tag!("0o"), take_while!(nom::is_oct_digit)), is_digit_8) |
    map_res!(preceded!(tag!("#"), take_while!(nom::is_digit)), try_into_i8) |
    map_res!(take_while!(nom::is_digit), is_digit_10) |
    map_res!(take!(1), try_into_i8)
));

pub fn parse(src: &str) -> Vec<Pair> {
    let mut pairs = match parse_syntax(Input(src.as_bytes())) {
        Ok((left, res)) => {
            if !left.is_empty() {
                let s = str::from_utf8(left.0).unwrap();
                panic!("unable to parse syntax:\n\n{:?}", s);
            } else {
                res
            }
        }
        Err(nom::Err::Error(err)) => panic!("problems parsing syntax source: {:?}", err),
        Err(nom::Err::Failure(err)) => panic!("problems parsing syntax source: {:?}", err),
        Err(nom::Err::Incomplete(_)) => panic!("parsing incomplete"),
    };

    // need order for calculate ranges
    pairs.sort_by(|a, b| a.char.cmp(&b.char));
    pairs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse("123->&lt; || "), vec![Pair::new(123, b"&lt;")]);
        assert_eq!(
            parse("123->&lt; || 10->&  || "),
            vec![Pair::new(10, b"& "), Pair::new(123, b"&lt;"),]
        );
    }

    #[test]
    fn test_syntax() {
        assert_eq!(parse("b->& || "), vec![Pair::new(b'b', b"&")]);
        assert_eq!(parse("b->&"), vec![Pair::new(b'b', b"&")]);
        assert_eq!(parse("#->& || "), vec![Pair::new(b'#', b"&")]);

        assert_eq!(parse("#6->& || "), vec![Pair::new(b'6', b"&")]);
        assert_eq!(parse("0x34->& || "), vec![Pair::new(0x34, b"&")]);
        assert_eq!(parse("0o34->& || "), vec![Pair::new(0o34, b"&")]);

        assert_eq!(parse(" ->- || "), vec![Pair::new(b' ', b"-")]);
        assert_eq!(
            parse("<->& || >->- || "),
            vec![Pair::new(b'<', b"&"), Pair::new(b'>', b"-"),]
        );
        assert_eq!(
            parse("\"->& || a->- || "),
            vec![Pair::new(b'"', b"&"), Pair::new(b'a', b"-"),]
        );
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_a() {
        parse("-f");
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_b() {
        parse("->f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_c() {
        parse("1>f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_d() {
        parse("1-f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_bad_syntax_e() {
        parse("1-f ||");
    }

    #[should_panic]
    #[test]
    fn test_panic_overflow_u8() {
        parse("256->f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_overflow_i8() {
        parse("128->f || ");
    }

    #[should_panic]
    #[test]
    fn test_panic_overflow_negative() {
        parse("-1->f || ");
    }
}
