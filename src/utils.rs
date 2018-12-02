// ASCII codes to reference quotes
#[rustfmt::skip]
pub(crate) static TABLE: [u8; 256] = [
//  \0                            \n
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
//  commands
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
//  \w !  "  #  $  %  &  '  (  )  *  +  ,  -  .  /
    6, 6, 3, 6, 6, 6, 2, 4, 6, 6, 6, 6, 6, 6, 6, 5,
//  0  1  2  3  4  5  6  7  8  9  :  ;  <  =  >  ?
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 6, 1, 6,
//  @  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
//  P  Q  R  S  T  U  V  W  X  Y  Z  [  \  ]  ^  _
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
//  `  a  b  c  d  e  f  g  h  i  j  k  l  m  n  o
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
//  p  q  r  s  t  u  v  w  x  y  z  {  |  }  ~  del
//   ====== Extended ASCII (aka. obs-text) ======
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
];
//                                      #60     #62     #38      #34        #39      #47
pub(crate) static QUOTES: [&str; 6] = ["&lt;", "&gt;", "&amp;", "&quot;", "&#x27;", "&#x2f;"];
pub(crate) const QUOTES_LEN: usize = 6;

/// Returns subtraction of two pointers `b` and `a` as usize,
/// checks if value in `a` is larger or equal to the one in `b`
///
/// # Arguments
///
/// * `a` - *const u8 representing a pointer to u8
/// * `b` - *const u8 representing a pointer to u8
///
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn sub(a: *const u8, b: *const u8) -> usize {
    debug_assert!(b <= a);
    (a as usize) - (b as usize)
}

// Writes str in formatter `$fmt` from position `start` to `i`-1
// and substitutes escaped character in position `i` with quote
// and update de index `start`
macro_rules! escape_body {
    ($i:expr, $start:ident, $fmt:ident, $bytes:ident, $quote:expr) => {{
        use std::str;
        // Test if `start` index is in current position `i`
        if $start < $i {
            // Write slice from `start` to `i`- 1 in formatter
            #[allow(unused_unsafe)]
            $fmt.write_str(unsafe { str::from_utf8_unchecked(&$bytes[$start..$i]) })?;
        }
        // Write $quote to `$fmt` (instead of escape character)
        $fmt.write_str($quote)?;
        // Updates `start` index with the new current position  `i` + 1
        $start = $i + 1;
    }};
}

// Wrap the body of the escape over the body of the mask
#[allow(unused_macros)]
macro_rules! mask_body {
    ($i:expr, $start:ident, $fmt:ident, $bytes:ident, $quote:expr) => {{
        // Resolve expression `$i`
        let i = $i;
        // Call macro `escape_body!`
        escape_body!(i, $start, $fmt, $bytes, $quote);
    }};
}

// Calls macro `$callback!` passing string representation of a valid
// escaped byte as `$quotes`, only if current value has to be escaped
macro_rules! bodies {
    ($i:expr, $b:expr, $start:ident, $fmt:ident, $bytes:ident, $callback:ident) => {
        // Get usize from 0 to 6 for a given escape character in byte `$b`
        // where 6 is a inescapable character and (0,...,5) are escapeable
        let c = TABLE[$b as usize] as usize;
        // Check if escape character is valid
        if c < QUOTES_LEN {
            // Call macro `$callback!` passing `QUOTES[c]` as `$quote` argument
            // `QUOTES[c]` is the string representation of the escaped character
            $callback!($i, $start, $fmt, $bytes, QUOTES[c]);
        }
    };
}
