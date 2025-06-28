#![cfg(all(feature = "string", feature = "fmt"))]
use v_escape_base::{Escapes, EscapesBuilder, Vector, escape_builder};

#[derive(Debug, Clone, Copy)]
struct Equal<V: Vector> {
    a: V,
}

struct TodoRemoveBuilder;
impl EscapesBuilder for TodoRemoveBuilder {
    type Escapes<V: Vector> = Equal<V>;

    unsafe fn new<V: Vector>() -> Self::Escapes<V> {
        Equal { a: V::splat(b'a') }
    }
}

impl<V: Vector> Escapes for Equal<V> {
    const ESCAPE_LEN: usize = 1;

    const FALSE_POSITIVE: bool = false;

    type Vector = V;

    #[inline(always)]
    unsafe fn masking(&self, vector2: V) -> V {
        self.a.cmpeq(vector2)
    }

    #[inline(always)]
    fn escape(_: usize) -> &'static str {
        "foo"
    }

    #[inline(always)]
    fn position(_: u8) -> usize {
        0
    }

    #[inline(always)]
    fn byte_byte_compare(c: u8) -> bool {
        c == b'a'
    }
}

escape_builder!(TodoRemoveBuilder);

#[test]
fn test_escape_bytes() {
    let mut buffer = String::new();
    let haystack = "a".repeat(64);
    escape_string(&haystack, &mut buffer);
    assert_eq!(buffer, "foo".repeat(64));
}

#[test]
fn test_escape_fmt() {
    let haystack = "a".repeat(64);
    let result = escape_fmt(&haystack).to_string();
    assert_eq!(result, "foo".repeat(64));
}
