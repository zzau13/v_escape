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
macro_rules! sub {
    ($a:expr, $b:expr) => {{}};
}

fn sub(a: TokenStream, b: TokenStream) -> TokenStream {
    quote! {
        debug_assert!(#b <= #a);
        (#a as usize) - (#b as usize)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! index {
    ($a:ident[$b:expr]) => {{
        unsafe { *$a.as_ptr().add($b) }
    }};
}

#[macro_export]
#[doc(hidden)]
/// Escape body
///
/// Writes str in formatter `$fmt` from position `start` to `i`-1
/// and substitutes escaped character in position `i` with quote
/// and update de index `start`
macro_rules! escape_body {
    ($i:expr, $start:ident, $fmt:ident, $bytes:ident, $quote:expr) => {{
        if $start < $i {
            #[allow(unused_unsafe)]
            $fmt.write_str(unsafe { std::str::from_utf8_unchecked(&$bytes[$start..$i]) })?;
        }
        $fmt.write_str($quote)?;
        $start = $i + 1;
    }};
}

#[macro_export]
#[doc(hidden)]
/// Mask body
///
/// Wrap the body of the escape over the body of the mask
macro_rules! mask_body {
    ($i:expr, $start:ident, $fmt:ident, $bytes:ident, $quote:expr) => {{
        // Resolve expression `$i`
        let i = $i;
        // Call macro `$crate::escape_body!`
        $crate::escape_body!(i, $start, $fmt, $bytes, $quote);
    }};
}

#[macro_export]
#[doc(hidden)]
/// Escape bodies
///
/// Calls macro `$callback!` passing string representation of a valid
/// escaped byte as `$quotes`, only if current value has to be escaped
macro_rules! bodies {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $fmt:ident, $bytes:ident, $callback:path) => {
        // Get usize from 0 to $Q_LEN for a given escape character in byte `$b`
        // where $Q_LEN is a inescapable character and (0,...,$Q_LEN - 1) are escapable
        let c = $crate::index!($T[$b as usize]) as usize;
        // Check if escape character is valid
        if c < $Q_LEN {
            // Call macro `$callback!` passing `QUOTES[c]` as `$quote` argument
            // `QUOTES[c]` is the string representation of the escaped character
            $callback!($i, $start, $fmt, $bytes, $crate::index!($Q[c]));
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape bodies exact
///
/// Calls macro `$callback!` passing string representation of a valid
/// escaped byte as `$quotes`, only if current value has to be escaped
macro_rules! bodies_exact {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $fmt:ident, $bytes:ident, $callback:path) => {
        // Get usize from 0 to $Q_LEN for a given escape character in byte `$b`
        // where $Q_LEN is a inescapable character and (0,...,$Q_LEN - 1) are escapable
        debug_assert_ne!($T[$b as usize] as usize, $Q_LEN as usize);
        // Call macro `$callback!` passing `QUOTES[c]` as `$quote` argument
        // `QUOTES[c]` is the string representation of the escaped character
        $callback!(
            $i,
            $start,
            $fmt,
            $bytes,
            $crate::index!($Q[$crate::index!($T[$b as usize]) as usize])
        );
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape bodies exact one
///
macro_rules! bodies_exact_one {
    ($char:expr, $quote:expr, $_non:expr, $i:expr, $b:expr, $start:ident, $fmt:ident, $bytes:ident, $callback:path) => {
        debug_assert_eq!($char, $b);
        $callback!($i, $start, $fmt, $bytes, $quote);
    };
}

macro_rules! escape_body_bytes {
    ($i:expr, $start:ident, $bytes:ident, $buf:ident, $quote:expr) => {{
        if $start < $i {
            $crate::write_bytes!(&$bytes[$start..$i], $buf);
        }
        $crate::write_bytes!($quote.as_bytes(), $buf);

        $start = $i + 1;
    }};
}
macro_rules! mask_body_bytes {
    ($i:expr, $start:ident, $bytes:ident, $buf:ident, $quote:expr) => {{
        // Resolve expression `$i`
        let i = $i;
        // Call macro `$crate::escape_body_bytes!`
        $crate::escape_body_bytes!(i, $start, $bytes, $buf, $quote);
    }};
}
macro_rules! write_bytes {
    ($bytes:expr, $buf:ident) => {
        $buf.extend_from_slice($bytes);
    };
}

macro_rules! bodies_bytes {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $bytes:ident, $buf:ident, $callback:path) => {
        let c = $crate::index!($T[$b as usize]) as usize;
        if c < $Q_LEN {
            $callback!($i, $start, $bytes, $buf, $crate::index!($Q[c]));
        }
    };
}
macro_rules! bodies_exact_bytes {
    ($T:ident, $Q:ident, $Q_LEN:ident, $i:expr, $b:expr, $start:ident, $bytes:ident, $buf:ident, $callback:path) => {
        debug_assert_ne!($T[$b as usize] as usize, $Q_LEN as usize);
        $callback!(
            $i,
            $start,
            $bytes,
            $buf,
            $crate::index!($Q[$crate::index!($T[$b as usize]) as usize])
        );
    };
}
macro_rules! bodies_exact_one_bytes {
    ($char:expr, $quote:expr, $_non:expr, $i:expr, $b:expr, $start:ident, $bytes:ident, $buf:ident, $callback:path) => {
        debug_assert_eq!($char, $b);
        $callback!($i, $start, $bytes, $buf, $quote);
    };
}
