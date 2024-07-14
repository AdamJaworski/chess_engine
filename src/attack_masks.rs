use crate::enums::{Side, Squares};
use crate::{not_a_file, not_h_file, set_bit};

pub fn mask_pawn_attacks(side: Side, square: Squares) -> u64
{
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    set_bit!(bitboard, square);

    if (side as u8 == 0) {
        attacks |= ((bitboard >> 7) & not_a_file);
        attacks |= ((bitboard >> 9) & not_h_file);
    }
    else {
        attacks |= ((bitboard << 7) & not_h_file);
        attacks |= ((bitboard << 9) & not_a_file);
    }

    return attacks;
}

pub fn mask_knight_attacks(square: Squares) -> u64 {
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    set_bit!(bitboard, square);

}