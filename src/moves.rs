use crate::utils::Piece;
use crate::utils::Square;

pub enum MoveTypes {
    Quiet,
    Capture,
    TwoSquare,
    Enpassant,
    Castling,
    Promotion,
}

pub fn encode(
    source: Square,
    target: Square,
    piece: Piece,
    promoted: Piece,
    flag: MoveTypes,
    captured: Piece,
) -> u32 {
    let mut encoded_move: u32 = 0;
    encoded_move |= source as u32;
    encoded_move |= (target as u32) << 6;
    encoded_move |= (piece as u32) << 12;
    encoded_move |= (promoted as u32) << 16;
    encoded_move |= (flag as u32) << 20;
    encoded_move |= (captured as u32) << 23;
    encoded_move
}

pub fn get_source(moves: u32) -> u32 {
    moves & 0x3F
}

pub fn get_target(moves: u32) -> u32 {
    (moves & 0xFC0) >> 6
}

pub fn get_piece(moves: u32) -> u32 {
    (moves & 0xF000) >> 12
}

pub fn get_promoted(moves: u32) -> u32 {
    (moves & 0xF0000) >> 16
}

pub fn get_type(moves: u32) -> u32 {
    (moves & 0x700000) >> 20
}

pub fn get_captured(moves: u32) -> u32 {
    (moves & 0x7800000) >> 23
}
