use nom::{self, AsBytes};
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

named!(parse_syntax<Input, Vec<Pair>>, many1!(do_parse!(
    pair: parse_pair >>
    tag!(" || ") >>
    (pair)
)));

named!(parse_pair<Input, Pair>, map!(
    separated_pair!(is_char, tag!("->"), take_until!(" || ")),
    |s| Pair::new(s.0, &s.1)
));

named!(is_char<Input, u8>, alt!(
    map_res!(take_while!(nom::is_digit), is_digit) |
    map!(take!(1),
    |s| {
        let b = s.as_bytes();
        assert_eq!(b.len(), 1);
        // TODO: use try_from
        i8::from_str_radix(&b.first().unwrap().to_string(), 10).unwrap() as u8
    })
));

fn is_digit(s: Input) -> Result<u8, nom::Err<Input>> {
    if s.is_empty() {
        Err(nom::Err::Failure(error_position!(s, nom::ErrorKind::IsNot)))
    } else {
        Ok(i8::from_str_radix(str::from_utf8(&s.as_bytes()).unwrap(), 10).unwrap() as u8)
    }
}

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
        parse("1->f");
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
