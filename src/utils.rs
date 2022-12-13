#![allow(dead_code)]

#[rustfmt::skip]
#[derive(Copy, Clone)]
pub enum Square {
	A8, B8, C8, D8, E8, F8, G8, H8,
	A7, B7, C7, D7, E7, F7, G7, H7,
	A6, B6, C6, D6, E6, F6, G6, H6,
	A5, B5, C5, D5, E5, F5, G5, H5,
	A4, B4, C4, D4, E4, F4, G4, H4,
	A3, B3, C3, D3, E3, F3, G3, H3,
	A2, B2, C2, D2, E2, F2, G2, H2,
	A1, B1, C1, D1, E1, F1, G1, H1, Nosq
}

#[rustfmt::skip]
pub const SQ_STR: [&str; 65] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "nosq"
];

pub enum Color {
    White,
    Black,
    Both,
    NoColor,
}

pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    NoPiece,
}

pub fn get_sq(r: u8, f: u8) -> u8 {
    (r * 8) + f
}

pub fn get_row(sq: u8) -> u8 {
    sq >> 3
}

pub fn get_col(sq: u8) -> u8 {
    sq & 7
}

pub fn flip_sq(sq: u8) -> u8 {
    sq ^ 56
}

pub fn is_light_sq(sq: u8) -> bool {
    if (get_row(sq) + get_col(sq) + 1) & 1 == 0 {
        return false;
    }
    true
}

pub fn get_colorless(piece: u8) -> u8 {
    piece % 6
}

pub fn get_sq_from_str(sq_str: &str) -> u8 {
    if sq_str.len() != 2 {
        panic!("Incorrect length!");
    }
    let r = (sq_str.chars().nth(0).unwrap() as u8) - ('0' as u8);
    let f = (sq_str.chars().nth(1).unwrap() as u8) - ('a' as u8);
    get_sq(r, f)
}
