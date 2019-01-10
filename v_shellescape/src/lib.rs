//! # Quick start
//!
//! ```rust
//! extern crate v_shellescape;
//! use v_shellescape::{unix, windows};
//!
//! print!("{}", unix::ShellEscape::from("linker=gcc -L/foo -Wl,bar"));
//! print!("{}", windows::ShellEscape::from("linker=gcc -L/foo -Wl,bar"));
//! ```
//!

#[macro_use]
extern crate v_escape;

use std::env;
use std::fmt::{self, Display, Formatter};

#[path = "unix.rs"]
mod _unix;
#[path = "windows.rs"]
mod _windows;

macro_rules! new_shellescape {
    () => {
        use std::fmt::{self, Display, Formatter};

        pub struct ShellEscape<'a> {
            bytes: &'a [u8],
        }
        _v_escape_escape_new!(ShellEscape);
    };
}

pub mod unix {
    use super::_unix::*;
    new_shellescape!();
    _v_escape_cfg_escape!(false, false, false);
}

pub mod windows {
    use super::_windows::*;
    new_shellescape!();
    _v_escape_cfg_escape!(false, false, false);
}

pub struct Escaped<'a> {
    s: &'a str,
}

impl<'a> Escaped<'a> {
    pub fn new(s: &str) -> Escaped {
        Escaped { s }
    }
}

impl<'a> Display for Escaped<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if cfg!(unix) || env::var("MSYSTEM").is_ok() {
            unix::ShellEscape::from(self.s).fmt(f)
        } else {
            windows::ShellEscape::from(self.s).fmt(f)
        }
    }
}

/// Escape characters that may have special meaning in a shell.
pub fn escape(s: &str) -> Escaped {
    Escaped::new(s)
}

#[cfg(test)]
mod test {
    #[cfg(target_os = "unix")]
    mod unix {
        use super::super::escape;

        #[test]
        fn test() {
            assert_eq!(
                escape("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_=/,.+")
                    .to_string(),
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_=/,.+"
            );
            assert_eq!(escape("--aaa=bbb-ccc").to_string(), "--aaa=bbb-ccc");
            assert_eq!(
                escape("linker=gcc -L/foo -Wl,bar").to_string(),
                r#"'linker=gcc -L/foo -Wl,bar'"#
            );
            assert_eq!(
                escape(r#"--features="default""#).to_string(),
                r#"'--features="default"'"#
            );
            assert_eq!(
                ShellEscape::from(r#"'!\$`\\\n "#).to_string(),
                r#"''\'''\!'\$`\\\n '"#
            );
        }
    }

    #[cfg(target_os = "windows")]
    mod windows {
        use super::super::escape;

        #[test]
        fn test() {
            assert_eq!(escape("--aaa=bbb-ccc").to_string(), "--aaa=bbb-ccc");
            assert_eq!(
                escape("linker=gcc -L/foo -Wl,bar").to_string(),
                r#""linker=gcc -L/foo -Wl,bar""#
            );
            assert_eq!(
                escape(r#"--features="default""#).to_string(),
                r#""--features=\"default\"""#
            );
            assert_eq!(
                escape(r#"\path\to\my documents\"#).to_string(),
                r#""\path\to\my documents\\""#
            );
        }
    }
}
