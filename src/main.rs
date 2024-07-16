use rand::random;
use crate::attack_masks::mask_rook_attacks;
use crate::enums::{Side, Square};
use crate::utilities::{count_bits, generate_magic_number, get_least_significant_bit, get_random_u64, print_board, set_occupancy};

mod r#macro;
mod enums;
mod utilities;
mod attack_masks;



const not_a_file: u64 = 18374403900871474942;
const not_b_file: u64 = 18302063728033398269;
const not_g_file: u64 = 13816973012072644543;
const not_h_file: u64 = 9187201950435737471;

const bishop_relevant_bits: [u8; 64] = [
6, 5, 5, 5, 5, 5, 5, 6,
5, 5, 5, 5, 5, 5, 5, 5,
5, 5, 7, 7, 7, 7, 5, 5,
5, 5, 7, 9, 9, 7, 5, 5,
5, 5, 7, 9, 9, 7, 5, 5,
5, 5, 7, 7, 7, 7, 5, 5,
5, 5, 5, 5, 5, 5, 5, 5,
6, 5, 5, 5, 5, 5, 5, 6
];

// rook relevant occupancy bit count for every square on board
const rook_relevant_bits: [u8; 64] = [
12, 11, 11, 11, 11, 11, 11, 12,
11, 10, 10, 10, 10, 10, 10, 11,
11, 10, 10, 10, 10, 10, 10, 11,
11, 10, 10, 10, 10, 10, 10, 11,
11, 10, 10, 10, 10, 10, 10, 11,
11, 10, 10, 10, 10, 10, 10, 11,
11, 10, 10, 10, 10, 10, 10, 11,
12, 11, 11, 11, 11, 11, 11, 12
];


// fn init_leapers_attacks(pawn_attacks: &mut [[u64; 64]; 2]) {
//     for square in 0u8..64 {
//         pawn_attacks[Side::WHITE][square as usize] = attack_masks::mask_pawn_attacks(Side::WHITE, Square());
//         pawn_attacks[Side::BLACK][square as usize] = attack_masks::mask_pawn_attacks(Side::BLACK, Square::from_u8(square).unwrap());
//     }
// }

fn main() {
    let mut bitboard: u64 = 0;

    set_bit!(bitboard, Square::E2);
    set_bit!(bitboard, Square::G4);
    set_bit!(bitboard, Square::C4);
    print_board(bitboard);
    print!("index: {}",get_least_significant_bit(bitboard)); print!("   Coordinates {}",  Square::from_raw(get_least_significant_bit(bitboard) as usize).to_string());
    print!("\nBBC engine");

    print_board(generate_magic_number())

}
