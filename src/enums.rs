use std::slice::Iter;
use std::convert::TryFrom;



pub enum Squares {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

impl TryFrom<u8> for Squares {
    type Error = ();

    fn try_from(value: u8){
        match value {
            0 => Ok(Squares::A8),  1 => Ok(Squares::B8),  2 => Ok(Squares::C8),  3 => Ok(Squares::D8),
            4 => Ok(Squares::E8),  5 => Ok(Squares::F8),  6 => Ok(Squares::G8),  7 => Ok(Squares::H8),
            8 => Ok(Squares::A7),  9 => Ok(Squares::B7), 10 => Ok(Squares::C7), 11 => Ok(Squares::D7),
            12 => Ok(Squares::E7), 13 => Ok(Squares::F7), 14 => Ok(Squares::G7), 15 => Ok(Squares::H7),
            16 => Ok(Squares::A6), 17 => Ok(Squares::B6), 18 => Ok(Squares::C6), 19 => Ok(Squares::D6),
            20 => Ok(Squares::E6), 21 => Ok(Squares::F6), 22 => Ok(Squares::G6), 23 => Ok(Squares::H6),
            24 => Ok(Squares::A5), 25 => Ok(Squares::B5), 26 => Ok(Squares::C5), 27 => Ok(Squares::D5),
            28 => Ok(Squares::E5), 29 => Ok(Squares::F5), 30 => Ok(Squares::G5), 31 => Ok(Squares::H5),
            32 => Ok(Squares::A4), 33 => Ok(Squares::B4), 34 => Ok(Squares::C4), 35 => Ok(Squares::D4),
            36 => Ok(Squares::E4), 37 => Ok(Squares::F4), 38 => Ok(Squares::G4), 39 => Ok(Squares::H4),
            40 => Ok(Squares::A3), 41 => Ok(Squares::B3), 42 => Ok(Squares::C3), 43 => Ok(Squares::D3),
            44 => Ok(Squares::E3), 45 => Ok(Squares::F3), 46 => Ok(Squares::G3), 47 => Ok(Squares::H3),
            48 => Ok(Squares::A2), 49 => Ok(Squares::B2), 50 => Ok(Squares::C2), 51 => Ok(Squares::D2),
            52 => Ok(Squares::E2), 53 => Ok(Squares::F2), 54 => Ok(Squares::G2), 55 => Ok(Squares::H2),
            56 => Ok(Squares::A1), 57 => Ok(Squares::B1), 58 => Ok(Squares::C1), 59 => Ok(Squares::D1),
            60 => Ok(Squares::E1), 61 => Ok(Squares::F1), 62 => Ok(Squares::G1), 63 => Ok(Squares::H1),
            _ => Err(()),
        }
    }
}

impl Squares {
    pub fn from_u8(value: u8) -> Result<Self, ()> {
        Squares::try_from(value)
    }
}


// ###################  SIDES  ##################

pub enum Side {
    White, Black
}
