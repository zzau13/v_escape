// Returns subtraction of two pointers `b` and `a` as usize,
// checks if value in `a` is larger or equal to the one in `b`
//
// # Arguments
//
// * `a` - *const u8 representing a pointer to u8
// * `b` - *const u8 representing a pointer to u8
//
#[macro_export]
macro_rules! _v_escape_sub {
    ($a:ident, $b:ident) => {{
        debug_assert!($b <= $a);
        ($a as usize) - ($b as usize)
    }};
}

// Writes str in formatter `$fmt` from position `start` to `i`-1
// and substitutes escaped character in position `i` with quote
// and update de index `start`
#[macro_export]
macro_rules! _v_escape_escape_body {
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
#[macro_export]
macro_rules! _v_escape_mask_body {
    ($i:expr, $start:ident, $fmt:ident, $bytes:ident, $quote:expr) => {{
        // Resolve expression `$i`
        let i = $i;
        // Call macro `_v_escape_escape_body!`
        _v_escape_escape_body!(i, $start, $fmt, $bytes, $quote);
    }};
}

// Calls macro `$callback!` passing string representation of a valid
// escaped byte as `$quotes`, only if current value has to be escaped
#[macro_export]
macro_rules! _v_escape_bodies {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $fmt:ident, $bytes:ident, $callback:ident) => {
        // Get usize from 0 to 6 for a given escape character in byte `$b`
        // where 6 is a inescapable character and (0,...,5) are escapeable
        let c = $T[$b as usize] as usize;
        // Check if escape character is valid
        if c < $Q_LEN {
            // Call macro `$callback!` passing `QUOTES[c]` as `$quote` argument
            // `QUOTES[c]` is the string representation of the escaped character
            $callback!($i, $start, $fmt, $bytes, $Q[c]);
        }
    };
}

// Accumulate current escaping byte size
#[macro_export]
macro_rules! _v_escape_size_bodies {
    ($S:ident, $acc:ident, $b:expr) => {
        $acc += $S[$b as usize] as usize;
    };
}
