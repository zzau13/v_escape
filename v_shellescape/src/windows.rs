/// Windows-specific escaping.
///

// ASCII codes blacklist characters
#[rustfmt::skip]
static BLACKLIST: [bool; 256] = byte_map![
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
//  \0                            \n
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//  commands
    1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//  \w !  "  #  $  %  &  '  (  )  *  +  ,  -  .  /
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//  0  1  2  3  4  5  6  7  8  9  :  ;  <  =  >  ?
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//  @  A  B  C  D  E  F  G  H  I  J  K  L  M  N  O
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//  P  Q  R  S  T  U  V  W  X  Y  Z  [  \  ]  ^  _
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//  `  a  b  c  d  e  f  g  h  i  j  k  l  m  n  o
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//  p  q  r  s  t  u  v  w  x  y  z  {  |  }  ~  del
//   ====== Extended ASCII (aka. obs-text) ======
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

#[inline]
fn blacklisted(c: u8) -> bool {
    BLACKLIST[c as usize]
}

pub mod scalar {
    use super::blacklisted;

    #[inline]
    /// Escape for the windows cmd.exe shell.
    ///
    /// See [here][msdn] for more information.
    ///
    /// [msdn]: http://blogs.msdn.com/b/twistylittlepassagesallalike/archive/2011/04/23/everyone-quotes-arguments-the-wrong-way.aspx
    pub fn escape(bytes: &[u8], fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use std::str::from_utf8_unchecked;

        let mut first = None;

        for (i, b) in bytes.iter().enumerate() {
            if blacklisted(*b) {
                first = Some(i);
                break;
            }
        }

        if let Some(first) = first {
            let mut nslashes = 0;
            let mut start = 0;

            fmt.write_str("\"")?;
            fmt.write_str(unsafe { from_utf8_unchecked(&bytes[..first]) })?;

            let bytes = &bytes[first..];

            for (i, b) in bytes.iter().enumerate() {
                match *b {
                    b'"' => {
                        if start < i {
                            fmt.write_str(unsafe { from_utf8_unchecked(&bytes[start..i]) })?;
                        }

                        fmt.write_fmt(format_args!("{}\"", "\\".repeat(nslashes + 1)))?;
                        start = i + 1;
                    }
                    b'\\' => {
                        if start < i {
                            fmt.write_str(unsafe { from_utf8_unchecked(&bytes[start..i]) })?;
                        }

                        fmt.write_str(&"\\")?;
                        nslashes += 1;
                        start = i + 1;
                    }
                    _ => {
                        if nslashes != 0 {
                            nslashes = 0;
                        }
                    }
                }
            }

            fmt.write_fmt(format_args!(
                "{}{}\"",
                unsafe { from_utf8_unchecked(&bytes[start..]) },
                "\\".repeat(nslashes)
            ))?;
        } else {
            fmt.write_str(unsafe { from_utf8_unchecked(bytes) })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    mod scalar {
        use super::super::scalar::escape as _escape;
        use std::fmt::{self, Display, Formatter};

        pub struct ShellEscape<'a> {
            bytes: &'a [u8],
        }
        _v_escape_escape_new!(ShellEscape);

        #[test]
        fn test() {
            assert_eq!(
                ShellEscape::from("--aaa=bbb-ccc").to_string(),
                "--aaa=bbb-ccc"
            );
            assert_eq!(
                ShellEscape::from("linker=gcc -L/foo -Wl,bar").to_string(),
                r#""linker=gcc -L/foo -Wl,bar""#
            );
            assert_eq!(
                ShellEscape::from(r#"--features="default""#).to_string(),
                r#""--features=\"default\"""#
            );
            assert_eq!(
                ShellEscape::from(r#"\path\to\my documents\"#).to_string(),
                r#""\path\to\my documents\\""#
            );
        }
    }
}
