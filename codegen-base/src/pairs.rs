use std::ops::Index;

use crate::switch::Switch;

#[derive(Debug)]
pub(crate) struct Pair {
    pub ch: u8,
    pub quote: String,
}

impl Pair {
    pub fn new<I: Into<String>>(ch: i8, quote: I) -> Self {
        Pair {
            ch: ch as u8,
            quote: quote.into(),
        }
    }
}

pub(crate) struct Pairs<'a>(pub &'a [Pair]);
impl<'a> Into<Switch> for Pairs<'a> {
    fn into(self) -> Switch {
        use Switch::*;
        assert_ne!(self.len(), 0);

        if self.len() == 1 {
            return A { a: self.ch(0) };
        }
        let e = self.len() - 1;

        let mut d = vec![];
        for i in 0..e {
            let diff = self.ch(i + 1) - self.ch(i);
            if 1 < diff {
                d.push((i, diff));
            }
        }
        d.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        match d.len() {
            0 => Ar {
                la: self.ch(0),
                ra: self.ch(e),
            },
            1 => {
                if e == 1 {
                    AB {
                        a: self.ch(0),
                        b: self.ch(e),
                    }
                } else {
                    let i = d[0].0;
                    if i == 0 {
                        ArB {
                            la: self.ch(i + 1),
                            ra: self.ch(e),
                            b: self.ch(0),
                        }
                    } else if i + 1 != e {
                        ArBr {
                            la: self.ch(0),
                            ra: self.ch(i),
                            lb: self.ch(i + 1),
                            rb: self.ch(e),
                        }
                    } else {
                        ArB {
                            la: self.ch(0),
                            ra: self.ch(i),
                            b: self.ch(i + 1),
                        }
                    }
                }
            }
            _ => {
                if e <= 2 {
                    assert_eq!(e, 2);
                    return ABC {
                        a: self.ch(0),
                        b: self.ch(1),
                        c: self.ch(2),
                    };
                }

                let (d, _) = d.split_at_mut(2);
                d.sort_unstable_by_key(|d| d.0);

                let f = d[0].0;
                let l = d[1].0;

                if f + 1 == l {
                    if f == 0 {
                        self.push_1_ranges_2_equals_at_first(f, l, e)
                    } else if l + 1 == e {
                        self.push_1_ranges_2_equals_at_last(f, l, e)
                    } else {
                        self.push_2_ranges_1_equals(f, l, e)
                    }
                } else if f == 0 {
                    if l + 1 == e {
                        self.push_1_ranges_2_equals_at_first_last(f, l, e)
                    } else {
                        self.push_2_ranges_1_equals_at_first(f, l, e)
                    }
                } else if l + 1 == e {
                    self.push_2_ranges_1_equals_at_last(f, l, e)
                } else {
                    self.push_3_ranges(f, l, e)
                }
            }
        }
    }
}

impl<'a> Index<usize> for Pairs<'a> {
    type Output = Pair;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> Pairs<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn ch(&self, i: usize) -> i8 {
        self[i].ch as i8
    }
    #[inline]
    fn push_1_ranges_2_equals_at_first(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBC {
            la: self.ch(l + 1),
            ra: self.ch(e),
            b: self.ch(0),
            c: self.ch(f + 1),
        }
    }

    #[inline]
    fn push_1_ranges_2_equals_at_last(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBC {
            la: self.ch(0),
            ra: self.ch(f),
            b: self.ch(l),
            c: self.ch(e),
        }
    }

    #[inline]
    fn push_1_ranges_2_equals_at_first_last(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBC {
            la: self.ch(f + 1),
            ra: self.ch(l),
            b: self.ch(0),
            c: self.ch(e),
        }
    }

    #[inline]
    fn push_2_ranges_1_equals(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrC {
            la: self.ch(0),
            ra: self.ch(f),
            lb: self.ch(l + 1),
            rb: self.ch(e),
            c: self.ch(f + 1),
        }
    }

    #[inline]
    fn push_2_ranges_1_equals_at_first(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrC {
            la: self.ch(f + 1),
            ra: self.ch(l),
            lb: self.ch(l + 1),
            rb: self.ch(e),
            c: self.ch(0),
        }
    }

    #[inline]
    fn push_2_ranges_1_equals_at_last(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrC {
            la: self.ch(0),
            ra: self.ch(f),
            lb: self.ch(f + 1),
            rb: self.ch(l),
            c: self.ch(e),
        }
    }

    #[inline]
    fn push_3_ranges(&self, f: usize, l: usize, e: usize) -> Switch {
        Switch::ArBrCr {
            la: self.ch(0),
            ra: self.ch(f),
            lb: self.ch(f + 1),
            rb: self.ch(l),
            lc: self.ch(l + 1),
            rc: self.ch(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::switch::{self, Switch::*};

    static E: &str = "f";
    macro_rules! pair {
        ($($l:literal),*) => { &[$(Pair::new($l, E)),*] };
    }

    #[test]
    fn test_1_escape() {
        let pairs = pair!(0);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(switch, A { a: 0 })
    }

    #[test]
    fn test_2_escape() {
        let pairs = pair!(0, 2);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(switch, AB { a: 0, b: 2 })
    }

    #[test]
    fn test_3_escape() {
        let pairs = pair!(0, 2, 4);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(switch, ABC { a: 0, b: 2, c: 4 })
    }

    #[test]
    fn test_1_range() {
        let pairs = pair!(0, 1);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(switch, Ar { la: 0, ra: 1 })
    }

    #[test]
    fn test_2_range() {
        let pairs = pair!(0, 1, 3, 4);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBr {
                la: 0,
                ra: 1,
                lb: 3,
                rb: 4
            }
        )
    }

    #[test]
    fn test_3_range() {
        let pairs = pair!(0, 1, 3, 4, 6, 7);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrCr {
                la: 0,
                ra: 1,
                lb: 3,
                rb: 4,
                lc: 6,
                rc: 7
            }
        );

        let pairs = pair!(0, 1, 3, 5, 7, 9, 50, 52, 55, 60, 61, 62, 63, 64, 126, 127);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrCr {
                la: 0,
                ra: 9,
                lb: 50,
                rb: 64,
                lc: 126,
                rc: 127
            }
        )
    }

    #[test]
    fn test_1_range_1_escape() {
        let pairs = pair!(0, 1, 3);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(switch, ArB { la: 0, ra: 1, b: 3 });

        let pairs = pair!(0, 2, 3);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(switch, ArB { la: 2, ra: 3, b: 0 });

        let pairs = pair!(0, 1, 2, 4);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(switch, ArB { la: 0, ra: 2, b: 4 });

        let pairs = pair!(50, 51, 52, 53, 54, 55, 67);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArB {
                la: 50,
                ra: 55,
                b: 67
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_a() {
        let pairs = pair!(0, 1, 3, 4, 6);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrC {
                la: 0,
                ra: 1,
                lb: 3,
                rb: 4,
                c: 6
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_b() {
        let pairs = pair!(0, 4, 5, 7, 8);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrC {
                la: 4,
                ra: 5,
                lb: 7,
                rb: 8,
                c: 0
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_c() {
        let pairs = pair!(14, 15, 16, 50, 51, 52, 98);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrC {
                la: 14,
                ra: 16,
                lb: 50,
                rb: 52,
                c: 98
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_d() {
        let pairs = pair!(14, 15, 16, 50, 51, 52, 98);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrC {
                la: 14,
                ra: 16,
                lb: 50,
                rb: 52,
                c: 98
            }
        );
        let pairs = pair!(
            14, 15, 16, 17, 18, 19, 50, 51, 52, 53, 54, 55, 56, 57, 58, 98
        );
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrC {
                la: 14,
                ra: 19,
                lb: 50,
                rb: 58,
                c: 98
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_e() {
        let pairs = pair!(14, 16, 50, 51, 52, 98);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrC {
                la: 14,
                ra: 16,
                lb: 50,
                rb: 52,
                c: 98
            }
        );

        let pairs = pair!(14, 16, 17, 18, 19, 50, 52, 53, 56, 57, 58, 98);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrC {
                la: 14,
                ra: 19,
                lb: 50,
                rb: 58,
                c: 98
            }
        );
    }

    #[test]
    fn test_2_range_1_escape_f() {
        let pairs = pair!(60, 61, 65, 80, 81);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrC {
                la: 60,
                ra: 61,
                lb: 80,
                rb: 81,
                c: 65
            }
        );

        let pairs = pair!(
            52, 53, 56, 58, 60, 61, 62, 80, 101, 102, 104, 105, 108, 110, 120
        );
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBrC {
                la: 52,
                ra: 62,
                lb: 101,
                rb: 120,
                c: 80
            }
        );
    }

    #[test]
    fn test_1_range_2_escape_a() {
        let pairs = pair!(0, 1, 4, 6);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBC {
                la: 0,
                ra: 1,
                b: 4,
                c: 6
            }
        );

        let pairs = pair!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 73, 127);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBC {
                la: 0,
                ra: 14,
                b: 73,
                c: 127
            }
        );
    }

    #[test]
    fn test_1_range_2_escape_b() {
        let pairs = pair!(0, 2, 5, 6);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBC {
                la: 5,
                ra: 6,
                b: 0,
                c: 2
            }
        );

        let pairs = pair!(0, 2, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBC {
                la: 5,
                ra: 18,
                b: 0,
                c: 2
            }
        );
    }

    #[test]
    fn test_1_range_2_escape_c() {
        let pairs = pair!(0, 2, 3, 8);
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBC {
                la: 2,
                ra: 3,
                b: 0,
                c: 8
            }
        );

        let pairs = pair!(
            0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 127
        );
        let switch: Switch = Pairs(pairs).into();

        assert_eq!(
            switch,
            ArBC {
                la: 2,
                ra: 17,
                b: 0,
                c: 127
            }
        );
    }
}
