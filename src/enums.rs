use std::slice::Iter;
use std::convert::TryFrom;



#[derive(Copy, Clone, PartialEq)]
pub struct Square {
    value: usize,
}
impl Square {
    pub const A1: Self = Self { value: 0 };
    pub const B1: Self = Self { value: 1 };
    pub const C1: Self = Self { value: 2 };
    pub const D1: Self = Self { value: 3 };
    pub const E1: Self = Self { value: 4 };
    pub const F1: Self = Self { value: 5 };
    pub const G1: Self = Self { value: 6 };
    pub const H1: Self = Self { value: 7 };
    pub const A2: Self = Self { value: 8 };
    pub const B2: Self = Self { value: 9 };
    pub const C2: Self = Self { value: 10 };
    pub const D2: Self = Self { value: 11 };
    pub const E2: Self = Self { value: 12 };
    pub const F2: Self = Self { value: 13 };
    pub const G2: Self = Self { value: 14 };
    pub const H2: Self = Self { value: 15 };
    pub const A3: Self = Self { value: 16 };
    pub const B3: Self = Self { value: 17 };
    pub const C3: Self = Self { value: 18 };
    pub const D3: Self = Self { value: 19 };
    pub const E3: Self = Self { value: 20 };
    pub const F3: Self = Self { value: 21 };
    pub const G3: Self = Self { value: 22 };
    pub const H3: Self = Self { value: 23 };
    pub const A4: Self = Self { value: 24 };
    pub const B4: Self = Self { value: 25 };
    pub const C4: Self = Self { value: 26 };
    pub const D4: Self = Self { value: 27 };
    pub const E4: Self = Self { value: 28 };
    pub const F4: Self = Self { value: 29 };
    pub const G4: Self = Self { value: 30 };
    pub const H4: Self = Self { value: 31 };
    pub const A5: Self = Self { value: 32 };
    pub const B5: Self = Self { value: 33 };
    pub const C5: Self = Self { value: 34 };
    pub const D5: Self = Self { value: 35 };
    pub const E5: Self = Self { value: 36 };
    pub const F5: Self = Self { value: 37 };
    pub const G5: Self = Self { value: 38 };
    pub const H5: Self = Self { value: 39 };
    pub const A6: Self = Self { value: 40 };
    pub const B6: Self = Self { value: 41 };
    pub const C6: Self = Self { value: 42 };
    pub const D6: Self = Self { value: 43 };
    pub const E6: Self = Self { value: 44 };
    pub const F6: Self = Self { value: 45 };
    pub const G6: Self = Self { value: 46 };
    pub const H6: Self = Self { value: 47 };
    pub const A7: Self = Self { value: 48 };
    pub const B7: Self = Self { value: 49 };
    pub const C7: Self = Self { value: 50 };
    pub const D7: Self = Self { value: 51 };
    pub const E7: Self = Self { value: 52 };
    pub const F7: Self = Self { value: 53 };
    pub const G7: Self = Self { value: 54 };
    pub const H7: Self = Self { value: 55 };
    pub const A8: Self = Self { value: 56 };
    pub const B8: Self = Self { value: 57 };
    pub const C8: Self = Self { value: 58 };
    pub const D8: Self = Self { value: 59 };
    pub const E8: Self = Self { value: 60 };
    pub const F8: Self = Self { value: 61 };
    pub const G8: Self = Self { value: 62 };
    pub const H8: Self = Self { value: 63 };
    pub const NULL: Self = Self { value: 64 };

    #[inline]
    pub const fn get_value(&self) -> usize {
        self.value
    }
}
// ###################  SIDES  ##################

#[derive(Copy, Clone, PartialEq)]
pub struct Side(usize);
impl Side {
    pub const WHITE: Side = Side::from_raw(0);
    pub const BLACK: Side = Side::from_raw(1);
    #[inline]
    pub const fn from_raw(value: usize) -> Self {
        Self { 0: value }
    }
    #[inline]
    pub const fn current(&self) -> usize {
        self.0
    }

}
