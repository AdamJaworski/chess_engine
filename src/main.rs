macro_rules! get_bit{
    ($bitboard: expr, $square: expr) => {($bitboard & (1 << $square as u8)) != 0};
}

macro_rules! set_bit{
    ($bitboard: expr, $square: expr) => {$bitboard |= (1 << $square as u8)};
}


macro_rules! pop_bit {
    ($bitboard: expr, $square: expr) => {$bitboard ^= if get_bit!($bitboard, $square) { (1 << $square as u8) } else {0};};
}


enum Squares {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1
}

enum Side {
    White, Black
}


fn mask_pawn_attacks(square: Squares, side: Side) -> u64
{
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    set_bit!(bitboard, square);

    if (side as u8 == 0)
    {
        attacks |= (bitboard >> 7);
    }

    else {

    }

    return attacks;
}


fn print_board(bitboard: u64) {
    println!("\n  Bitboard: {}\n", bitboard);
    for rank in 0..8 {
        for file in 0..8 {
            let square: u8 = rank * 8 + file;
            if file == 0 { print!("  {} ", 8 - rank); }
            
            print!(" {} ", get_bit!(bitboard, square) as u8);
        }
        println!();
    }

    println!("     A  B  C  D  E  F  G  H\n\n")

}

fn main() {
    let mut bitboard: u64 = 0;
    let mut pawn_attacks: [[u8; 2]; 64];
    let not_a_file: u64 = 18374403900871474942;
    let not_b_file: u64 = 18302063728033398269;
    let not_g_file: u64 = 13816973012072644543;
    let not_h_file: u64 = 9187201950435737471;

    println!("BBC engine");

    print_board(mask_pawn_attacks(Squares::E4, Side::White));
}
