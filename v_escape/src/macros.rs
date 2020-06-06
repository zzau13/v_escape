#[macro_export]
#[doc(hidden)]
/// Assert and subtraction
///
/// Returns subtraction of two pointers `b` and `a` as usize,
/// checks if value in `a` is larger or equal to the one in `b`
///
/// # Arguments
///
/// * `a` - *const u8 representing a pointer to u8
/// * `b` - *const u8 representing a pointer to u8
///
macro_rules! _v_escape_sub {
    ($a:ident, $b:ident) => {{
        debug_assert!($b <= $a);
        ($a as usize) - ($b as usize)
    }};
}

#[macro_export]
#[doc(hidden)]
/// Escape body
///
/// Writes str in formatter `$fmt` from position `start` to `i`-1
/// and substitutes escaped character in position `i` with quote
/// and update de index `start`
macro_rules! _v_escape_escape_body {
    ($i:expr, $start:ident, $fmt:ident, $bytes:ident, $quote:expr) => {{
        // Test if `start` index is in current position `i`
        if $start < $i {
            // Write slice from `start` to `i`- 1 in formatter
            #[allow(unused_unsafe)]
            $fmt.write_str(unsafe { ::std::str::from_utf8_unchecked(&$bytes[$start..$i]) })?;
        }
        // Write $quote to `$fmt` (instead of escape character)
        $fmt.write_str($quote)?;
        // Updates `start` index with the new current position  `i` + 1
        $start = $i + 1;
    }};
}

#[macro_export]
#[doc(hidden)]
/// Mask body
///
/// Wrap the body of the escape over the body of the mask
macro_rules! _v_escape_mask_body {
    ($i:expr, $start:ident, $fmt:ident, $bytes:ident, $quote:expr) => {{
        // Resolve expression `$i`
        let i = $i;
        // Call macro `_v_escape_escape_body!`
        _v_escape_escape_body!(i, $start, $fmt, $bytes, $quote);
    }};
}

#[macro_export]
#[doc(hidden)]
/// Escape bodies
///
/// Calls macro `$callback!` passing string representation of a valid
/// escaped byte as `$quotes`, only if current value has to be escaped
macro_rules! _v_escape_bodies {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $fmt:ident, $bytes:ident, $callback:ident) => {
        // Get usize from 0 to $Q_LEN for a given escape character in byte `$b`
        // where $Q_LEN is a inescapable character and (0,...,$Q_LEN - 1) are escapable
        let c = $T[$b as usize] as usize;
        // Check if escape character is valid
        if c < $Q_LEN {
            // Call macro `$callback!` passing `QUOTES[c]` as `$quote` argument
            // `QUOTES[c]` is the string representation of the escaped character
            $callback!($i, $start, $fmt, $bytes, $Q[c]);
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape bodies exact
///
/// Calls macro `$callback!` passing string representation of a valid
/// escaped byte as `$quotes`, only if current value has to be escaped
macro_rules! _v_escape_bodies_exact {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $fmt:ident, $bytes:ident, $callback:ident) => {
        // Get usize from 0 to $Q_LEN for a given escape character in byte `$b`
        // where $Q_LEN is a inescapable character and (0,...,$Q_LEN - 1) are escapable
        debug_assert_ne!($T[$b as usize] as usize, $Q_LEN as usize);
        // Call macro `$callback!` passing `QUOTES[c]` as `$quote` argument
        // `QUOTES[c]` is the string representation of the escaped character
        $callback!($i, $start, $fmt, $bytes, $Q[$T[$b as usize] as usize]);
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape bodies exact one
///
macro_rules! _v_escape_bodies_exact_one {
    ($char:expr, $quote:expr, $_non:expr, $i:expr, $b:expr, $start:ident, $fmt:ident, $bytes:ident, $callback:ident) => {
        debug_assert_eq!($char, $b);
        $callback!($i, $start, $fmt, $bytes, $quote);
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape body
///
/// Writes str in formatter `$fmt` from position `start` to `i`-1
/// and substitutes escaped character in position `i` with quote
/// and update de index `start`
macro_rules! _v_escape_escape_body_ptr {
    ($i:expr, $start:ident, $cur:ident, $buf:ident, $bytes:ident, $quote:expr) => {{
        // Test if `start` index is in current position `i`
        if $start < $i {
            // Write slice from `start` to `i`- 1 in a buffer pointer
            _v_escape_write_ptr!($cur, $buf, &$bytes[$start..$i], $i - $start)
        }
        let quote = $quote;
        _v_escape_write_ptr!($cur, $buf, quote.as_bytes(), quote.len());
        // Updates `start` index with the new current position  `i` + 1
        $start = $i + 1;
    }};
}

#[macro_export]
#[doc(hidden)]
/// Mask body
///
/// Wrap the body of the escape over the body of the mask
macro_rules! _v_escape_mask_body_ptr {
    ($i:expr, $start:ident, $cur:ident, $buf:ident, $bytes:ident, $quote:expr) => {{
        // Resolve expression `$i`
        let i = $i;
        // Call macro `_v_escape_escape_body_ptr!`
        _v_escape_escape_body_ptr!(i, $start, $cur, $buf, $bytes, $quote);
    }};
}

#[macro_export]
#[doc(hidden)]
/// Write in pointer with max bound
macro_rules! _v_escape_write_ptr {
    ($cur:ident, $buf:ident, $bytes:expr, $len:expr) => {
        if $buf.len() < $cur + ($len) {
            return None;
        } else {
            (&mut $buf[$cur..$cur + ($len)]).copy_from_slice($bytes);
            $cur += $len;
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape bodies
///
/// Calls macro `$callback!` passing string representation of a valid
/// escaped byte as `$quotes`, only if current value has to be escaped
macro_rules! _v_escape_bodies_ptr {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $cur:ident, $buf:ident, $bytes:ident, $callback:ident) => {
        let c = $T[$b as usize] as usize;
        if c < $Q_LEN {
            $callback!($i, $start, $cur, $buf, $bytes, $Q[c]);
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape bodies exact
///
/// Calls macro `$callback!` passing string representation of a valid
/// escaped byte as `$quotes`, only if current value has to be escaped
macro_rules! _v_escape_bodies_exact_ptr {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $cur:ident, $buf:ident, $bytes:ident, $callback:ident) => {
        // Get usize from 0 to $Q_LEN for a given escape character in byte `$b`
        // where $Q_LEN is a inescapable character and (0,...,$Q_LEN - 1) are escapable
        debug_assert_ne!($T[$b as usize] as usize, $Q_LEN as usize);
        // Call macro `$callback!` passing `QUOTES[c]` as `$quote` argument
        // `QUOTES[c]` is the string representation of the escaped character
        $callback!($i, $start, $cur, $buf, $bytes, $Q[$T[$b as usize] as usize]);
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape bodies exact one
///
macro_rules! _v_escape_bodies_exact_one_ptr {
    ($char:expr, $quote:expr, $_non:expr, $i:expr, $b:expr, $start:ident, $cur:ident, $buf:ident, $bytes:ident, $callback:ident) => {
        debug_assert_eq!($char, $b);
        $callback!($i, $start, $cur, $buf, $bytes, $quote);
    };
}
