//! # Quick start
//!
//! ```rust
//! extern crate v_latexescape;
//! use v_latexescape::LateXEscape;
//!
//! print!("{}", LateXEscape::from("# Header"));
//! ```
//!

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate v_escape;

cfg_if! {
    if #[cfg(all(v_latexescape_simd, v_latexescape_avx))] {
        new_escape!(
            LateXEscape,
            "35->\\# || 36->\\$ || 37->\\% || 38->\\& || 92->\\textbackslash{} || \
            94->\\textasciicircum{} || 95->\\_ || 123->\\{ || 125->\\} || 126->\\textasciitilde{}",
            avx = true, simd = true
        );

        pub mod sized {
            new_escape_sized!(
                LateXEscape,
                "35->\\# || 36->\\$ || 37->\\% || 38->\\& || 92->\\textbackslash{} || \
                94->\\textasciicircum{} || 95->\\_ || 123->\\{ || 125->\\} || 126->\\textasciitilde{}",
                avx = true, simd = true
            );
        }
    } else if #[cfg(all(v_latexescape_simd, v_latexescape_sse))] {
        new_escape!(
            LateXEscape,
            "35->\\# || 36->\\$ || 37->\\% || 38->\\& || 92->\\textbackslash{} || \
            94->\\textasciicircum{} || 95->\\_ || 123->\\{ || 125->\\} || 126->\\textasciitilde{}",
            avx = false, simd = true
        );

        pub mod sized {
            new_escape_sized!(
                LateXEscape,
                "35->\\# || 36->\\$ || 37->\\% || 38->\\& || 92->\\textbackslash{} || \
                94->\\textasciicircum{} || 95->\\_ || 123->\\{ || 125->\\} || 126->\\textasciitilde{}",
                avx = false, simd = true
            );
        }
    } else {
        new_escape!(
            LateXEscape,
            "35->\\# || 36->\\$ || 37->\\% || 38->\\& || 92->\\textbackslash{} || \
            94->\\textasciicircum{} || 95->\\_ || 123->\\{ || 125->\\} || 126->\\textasciitilde{}",
            avx = false, simd = false
        );

        pub mod sized {
            new_escape_sized!(
                LateXEscape,
                "35->\\# || 36->\\$ || 37->\\% || 38->\\& || 92->\\textbackslash{} || \
                94->\\textasciicircum{} || 95->\\_ || 123->\\{ || 125->\\} || 126->\\textasciitilde{}",
                avx = false, simd = false
            );
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_escape() {
        use super::LateXEscape;

        let empty = "";
        assert_eq!(LateXEscape::from(empty).to_string(), empty);

        assert_eq!(LateXEscape::from("").to_string(), "");
        assert_eq!(LateXEscape::from("#$%&").to_string(), "\\#\\$\\%\\&");
        assert_eq!(
            LateXEscape::from("bar_^").to_string(),
            "bar\\_\\textasciicircum{}"
        );
        assert_eq!(LateXEscape::from("{foo}").to_string(), "\\{foo\\}");
        assert_eq!(
            LateXEscape::from("~\\").to_string(),
            "\\textasciitilde{}\\textbackslash{}"
        );
        assert_eq!(
            LateXEscape::from("_% of do$llar an&d #HASHES {I} have in ~ \\ latex").to_string(),
            "\\_\\% of do\\$llar an\\&d \\#HASHES \\{I\\} have in \\textasciitilde{} \
             \\textbackslash{} latex"
        );
        assert_eq!(
            LateXEscape::from(
                "_% of do$llar an&d #HASHES {I} have in ~ \\ latex"
                    .repeat(10_000)
                    .as_ref()
            )
            .to_string(),
            "\\_\\% of do\\$llar an\\&d \\#HASHES \\{I\\} have in \\textasciitilde{} \
             \\textbackslash{} latex"
                .repeat(10_000)
        );
    }

    #[test]
    fn test_size() {
        use super::sized::LateXEscape;

        let empty = "";
        assert_eq!(LateXEscape::from(empty).size(), empty.len());

        assert_eq!(LateXEscape::from("").size(), "".len());
        assert_eq!(LateXEscape::from("#$%&").size(), "\\#\\$\\%\\&".len());
        assert_eq!(
            LateXEscape::from("bar_^").size(),
            "bar\\_\\textasciicircum{}".len()
        );
        assert_eq!(LateXEscape::from("{foo}").size(), "\\{foo\\}".len());
        assert_eq!(
            LateXEscape::from("~\\").size(),
            "\\textasciitilde{}\\textbackslash{}".len()
        );
        assert_eq!(
            LateXEscape::from("_% of do$llar an&d #HASHES {I} have in ~ \\ latex").size(),
            "\\_\\% of do\\$llar an\\&d \\#HASHES \\{I\\} have in \\textasciitilde{} \
             \\textbackslash{} latex"
                .len()
        );
        assert_eq!(
            LateXEscape::from(
                "_% of do$llar an&d #HASHES {I} have in ~ \\ latex"
                    .repeat(10_000)
                    .as_ref()
            )
            .size(),
            "\\_\\% of do\\$llar an\\&d \\#HASHES \\{I\\} have in \\textasciitilde{} \
             \\textbackslash{} latex"
                .repeat(10_000)
                .len()
        );
    }

}
