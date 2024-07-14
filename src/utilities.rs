use crate::get_bit;


pub fn print_board(bitboard: u64) {
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