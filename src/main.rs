mod r#macro;
mod enums;
mod utilities;
mod attack_masks;

use enums::Squares;
use enums::Side;


const not_a_file: u64 = 18374403900871474942;
const not_b_file: u64 = 18302063728033398269;
const not_g_file: u64 = 13816973012072644543;
const not_h_file: u64 = 9187201950435737471;


fn init_leapers_attacks(pawn_attacks: &mut [[u64; 64]; 2]) {
    for square in 0u8..64 {
        pawn_attacks[Side::White as usize][square as usize] = attack_masks::mask_pawn_attacks(Side::White, Squares::from_u8(square).unwrap());
        pawn_attacks[Side::Black as usize][square as usize] = attack_masks::mask_pawn_attacks(Side::Black, Squares::from_u8(square).unwrap());
    }
}

fn main() {
    let mut bitboard: u64 = 0;
    let mut knight_attacks: [u64; 64];
    let mut pawn_attacks: [[u64; 64]; 2] = [[0; 64]; 2];

    println!("BBC engine");

    utilities::print_board(attack_masks::mask_pawn_attacks(Side::Black, Squares::H5));
}
