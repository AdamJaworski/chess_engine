use crate::enums::{Side, Square};

mod r#macro;
mod enums;
mod utilities;
mod attack_masks;



const not_a_file: u64 = 18374403900871474942;
const not_b_file: u64 = 18302063728033398269;
const not_g_file: u64 = 13816973012072644543;
const not_h_file: u64 = 9187201950435737471;


// fn init_leapers_attacks(pawn_attacks: &mut [[u64; 64]; 2]) {
//     for square in 0u8..64 {
//         pawn_attacks[Side::WHITE][square as usize] = attack_masks::mask_pawn_attacks(Side::WHITE, Square());
//         pawn_attacks[Side::BLACK][square as usize] = attack_masks::mask_pawn_attacks(Side::BLACK, Square::from_u8(square).unwrap());
//     }
// }

fn main() {
    let mut bitboard: u64 = 0;

    println!("BBC engine");

    utilities::print_board(attack_masks::mask_bishop_attacks(Square::H1));
}
