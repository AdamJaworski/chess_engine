use crate::{not_a_file, not_b_file, not_g_file, not_h_file, set_bit};
use crate::enums::{Side, Square};

pub fn mask_pawn_attacks(side: Side, square: Square) -> u64 {
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    set_bit!(bitboard, square);

    if (side.current() == 0) {
        attacks |= ((bitboard >> 7) & not_a_file);
        attacks |= ((bitboard >> 9) & not_h_file);
    }
    else {
        attacks |= ((bitboard << 7) & not_h_file);
        attacks |= ((bitboard << 9) & not_a_file);
    }

    return attacks;
}

pub fn mask_knight_attacks(square: Square) -> u64 {
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    set_bit!(bitboard, square);

    attacks |= (bitboard >> 17) & not_h_file;
    attacks |= (bitboard >> 15) & not_a_file;

    attacks |= (bitboard >> 10) & (not_h_file & not_g_file);
    attacks |= (bitboard << 6)  & (not_h_file & not_g_file);

    attacks |= (bitboard << 10) & (not_a_file & not_b_file);
    attacks |= (bitboard >> 6)  & (not_a_file & not_b_file);

    attacks |= (bitboard << 17) & not_a_file;
    attacks |= (bitboard << 15) & not_h_file;
    return attacks;
}

pub fn mask_king_attacks(square: Square) -> u64 {
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    set_bit!(bitboard, square);

    attacks |= (bitboard >> 9) & not_h_file;
    attacks |= (bitboard >> 1) & not_h_file;
    attacks |= (bitboard << 7) & not_h_file;

    attacks |= (bitboard << 9) & not_a_file;
    attacks |= (bitboard << 1) & not_a_file;
    attacks |= (bitboard >> 7) & not_a_file;

    attacks |= (bitboard << 8);
    attacks |= (bitboard >> 8);
    return attacks;
}

pub fn mask_bishop_attacks(square: Square) -> u64 {
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    let square_index = square.get_value();
    let target_rank = (square_index / 8) as isize;
    let target_file = (square_index % 8) as isize;

    set_bit!(bitboard, square);

    let mut rank = target_rank + 1;
    let mut file = target_file + 1;

    while rank <= 6 && file <= 6 {
        attacks |= 1 << (rank * 8 + file);
        rank = rank + 1; file = file + 1;
    }

    rank = target_rank - 1; file = target_file + 1;

    while rank >= 1 && file <= 6 {
        attacks |= 1 << (rank * 8 + file);
        rank = rank - 1; file = file + 1;
    }

    rank = target_rank + 1; file = target_file - 1;

    while rank <= 6 && file >= 1 {
        attacks |= 1 << (rank * 8 + file);
        rank = rank + 1; file = file - 1;
    }


    rank = target_rank - 1; file = target_file - 1;

    while rank >= 1 && file <= 6 {
        attacks |= 1 << (rank * 8 + file);
        rank = rank - 1; file = file - 1;
    }

    return attacks;
}


pub fn mask_rook_attacks(square: Square) -> u64 {
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    let square_index = square.get_value();
    let target_rank = (square_index / 8) as isize;
    let target_file = (square_index % 8) as isize;

    set_bit!(bitboard, square);

    let mut rank = target_rank + 1;

    while rank <= 6 {
        attacks |= 1 << (rank * 8 + target_file);
        rank = rank + 1;
    }

    rank = target_rank - 1;
    while rank >= 1 {
        attacks |= 1 << (rank * 8 + target_file);
        rank = rank - 1;
    }

    let mut file = target_file + 1;

    while file <= 6 {
        attacks |= 1 << (target_rank * 8 + file);
        file = file + 1;
    }

    file = target_file - 1;

    while file >= 1 {
        attacks |= 1 << (target_rank * 8 + file);
        file = file - 1;
    }


    return attacks;
}


pub fn bishop_attacks(square: Square, block: u64) -> u64 {
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    let square_index = square.get_value();
    let target_rank = (square_index / 8) as isize;
    let target_file = (square_index % 8) as isize;

    set_bit!(bitboard, square);

    let mut rank = target_rank + 1;
    let mut file = target_file + 1;

    while rank <= 7 && file <= 7 {
        attacks |= 1 << (rank * 8 + file);
        if (1 << (rank * 8 + file) & block) != 0 { break; }
        rank = rank + 1; file = file + 1;
    }

    rank = target_rank - 1; file = target_file + 1;

    while rank >= 0 && file <= 7 {
        attacks |= 1 << (rank * 8 + file);
        if (1 << (rank * 8 + file) & block) != 0 { break; }
        rank = rank - 1; file = file + 1;
    }

    rank = target_rank + 1; file = target_file - 1;

    while rank <= 7 && file >= 0 {
        attacks |= 1 << (rank * 8 + file);
        if (1 << (rank * 8 + file) & block) != 0 { break; }
        rank = rank + 1; file = file - 1;
    }


    rank = target_rank - 1; file = target_file - 1;

    while rank >= 0 && file <= 7 {
        attacks |= 1 << (rank * 8 + file);
        if (1 << (rank * 8 + file) & block) != 0 { break; }
        rank = rank - 1; file = file - 1;
    }

    return attacks;
}

pub fn rook_attacks(square: Square, block: u64) -> u64 {
    let mut  attacks: u64  = 0;
    let mut  bitboard: u64 = 0;

    let square_index = square.get_value();
    let target_rank = (square_index / 8) as isize;
    let target_file = (square_index % 8) as isize;

    set_bit!(bitboard, square);

    let mut rank = target_rank + 1;

    while rank <= 7 {
        attacks |= 1 << (rank * 8 + target_file);
        if (1 << (rank * 8 + target_file) & block) != 0 { break; }
        rank = rank + 1;
    }

    rank = target_rank - 1;
    while rank >= 0 {
        attacks |= 1 << (rank * 8 + target_file);
        if (1 << (rank * 8 + target_file) & block) != 0 { break; }
        rank = rank - 1;
    }

    let mut file = target_file + 1;

    while file <= 7 {
        attacks |= 1 << (target_rank * 8 + file);
        if (1 << (target_rank * 8 + file) & block) != 0 { break; }
        file = file + 1;
    }

    file = target_file - 1;

    while file >= 0 {
        attacks |= 1 << (target_rank * 8 + file);
        if (1 << (target_rank * 8 + file) & block) != 0 { break; }
        file = file - 1;
    }


    return attacks;
}