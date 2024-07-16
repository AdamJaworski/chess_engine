#[macro_export]
macro_rules! get_bit{
    ($bitboard: expr, $square: expr) => {($bitboard & (1 << $square.get_value())) != 0};
}

#[macro_export]
macro_rules! set_bit{
    ($bitboard: expr, $square: expr) => {$bitboard |= (1 << $square.get_value())};
}

#[macro_export]
macro_rules! pop_bit {
    ($bitboard: expr, $square: expr) => {$bitboard ^= if get_bit!($bitboard, $square) { (1 << $square.get_value()) } else {0};};
}