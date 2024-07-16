use std::ffi::c_char;
use rand::random;
use crate::{get_bit, pop_bit, set_bit};
use crate::attack_masks::{bishop_attacks, mask_bishop_attacks, mask_rook_attacks, rook_attacks};
use crate::enums::Square;

pub fn print_board(bitboard: u64) {
    println!("\n  Bitboard: {}\n", bitboard);
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square: u8 = rank * 8 + file;
            if file == 0 { print!("  {} ", rank + 1); }

            print!(" {} ", get_bit!(bitboard, Square::from_raw(square as usize)) as u8);
        }
        println!();
    }

    println!("     A  B  C  D  E  F  G  H\n\n")

}

pub fn count_bits(bitboard: u64) -> u32 {
    bitboard.count_ones()
}

pub fn get_least_significant_bit(bitboard: u64) -> u32 {
    bitboard.trailing_zeros()
}

pub fn set_occupancy(index: usize, bits_in_mask: u32, mut attack_mask: u64) -> u64{
    let mut occupancy: u64 = 0;
    let mut attack_square;
    for count in 0..bits_in_mask {
        attack_square = Square::from_raw(get_least_significant_bit(attack_mask) as usize);

        pop_bit!(attack_mask, attack_square);

        if index & (1 << count) != 0{
            occupancy |= 1 << attack_square.get_value()
        }
    }
    return occupancy;
}


pub fn get_random_u64() -> u64 {
    let mut n1;
    let mut n2;
    let mut n3;
    let mut n4;

    n1 = random::<u32>() as u64 & 0xFFFF;
    n2 = random::<u32>() as u64 & 0xFFFF;
    n3 = random::<u32>() as u64 & 0xFFFF;
    n4 = random::<u32>() as u64 & 0xFFFF;

    n1 | (n2 << 16) | (n3 << 32) | (n4 << 48)
}

pub fn generate_magic_number() -> u64 {
    get_random_u64() & get_random_u64() & get_random_u64()
}

pub fn find_magic_number(square: Square, relevant_bits: u32, bishop: bool) {
    let mut occupancies: [u64;4096] = [0; 4096];
    let mut attacks: [u64;4096] = [0; 4096];
    let mut used_attacks: [u64; 4096];
    let mut attack_mask: u64;
    let mut magic_number: u64;

    attack_mask = if bishop {mask_bishop_attacks(square)} else {mask_rook_attacks(square)};

    let mut occupancy_indicies = 1 << relevant_bits;

    for index in 0..occupancy_indicies {
        occupancies[index] = set_occupancy(index, relevant_bits, attack_mask);

        attacks[index] = if bishop { bishop_attacks(square, occupancies[index]) } else { rook_attacks(square, occupancies[index]) };
    }

    for random_count in 0..10000000 {
         magic_number = generate_magic_number();

        if count_bits(attack_mask * magic_number & 0xFF00000000000000) < 6 {continue }

        used_attacks.fill(0);
    }
}






