#![allow(dead_code)]

use crate::{bitboard, utils};

pub const PIECE_STR: &str = "PNBRQK ";

pub struct Position {
    piece_bb: [[u64; 6]; 2],
    units_bb: [u64; 3],
}

impl Position {
    pub fn new() -> Position {
        Position {
            piece_bb: [[0; 6]; 2],
            units_bb: [0; 3],
        }
    }
    pub fn get_piece_bb(&self, color: u8, piece: u8) -> u64 {
        self.piece_bb[color as usize][piece as usize]
    }

    pub fn set_piece_bb(&mut self, color: u8, piece: u8, sq: u8) {
        bitboard::set_bit_u64(&mut self.piece_bb[color as usize][piece as usize], sq);
    }

    pub fn get_units_bb(&self, color: u8) -> u64 {
        self.units_bb[color as usize]
    }

    pub fn update_units_bb(&mut self) {
        self.units_bb.fill(0);
        for color in 0..2 {
            for piece in 0..6 {
                self.units_bb[color] |= self.piece_bb[color][piece];
            }
        }
        self.units_bb[2] = self.units_bb[0] | self.units_bb[1];
    }
}

pub struct State {
    side: u8,
    enpassant: u8,
    castling: u8,
    fifty_move: u8,
    full_moves: u8,
    key: u64,
}

impl State {
    pub fn new() -> State {
        State {
            side: 0,
            enpassant: utils::Square::Nosq as u8,
            castling: 0,
            fifty_move: 0,
            full_moves: 0,
            key: 0,
        }
    }

    pub fn change_side(&mut self) {
        self.side ^= 1;
    }

    pub fn set_enpassant(&mut self, sq: u8) {
        self.enpassant = sq;
    }

    pub fn toggle_castling(&mut self, castling_type: u8) {
        if bitboard::get_bit(self.castling as u64, castling_type) {
            bitboard::pop_bit_u8(&mut self.castling, castling_type);
        } else {
            bitboard::set_bit_u8(&mut self.castling, castling_type);
            // self.castling |= 1 << castling_type;

            println!("Updated!");
        }
    }
}

pub struct Board {
    pub pos: Position,
    pub state: State,
}

impl Board {
    pub fn new() -> Board {
        Board {
            pos: Position::new(),
            state: State::new(),
        }
    }
    pub fn find_piece(&self, sq: u8) -> (u8, u8) {
        for color in 0..2 {
            for piece in 0..6 {
                if bitboard::get_bit(self.pos.piece_bb[color][piece], sq as u8) {
                    return (color as u8, piece as u8);
                }
            }
        }
        (utils::Piece::None as u8, utils::Piece::None as u8)
    }
    pub fn print_board(&self) {
        println!("\n    +---+---+---+---+---+---+---+---+");
        for r in 0..8 {
            print!("  {} |", 8 - r);
            for f in 0..8 {
                let (_, piece) = self.find_piece(utils::get_sq(r, f));
                print!(" {} |", PIECE_STR.chars().nth(piece as usize).unwrap());
            }
            println!("\n    +---+---+---+---+---+---+---+---+");
        }
        println!("      a   b   c   d   e   f   g   h\n");
        println!(
            "\n      Side to move: {}",
            if self.state.side == 0 {
                "white"
            } else {
                "black"
            }
        );
        self.print_castling();
        println!(
            "         Enpassant: {}",
            utils::SQ_STR[self.state.enpassant as usize]
        );
        println!("        Full Moves: {}", self.state.full_moves);
    }
    pub fn print_castling(&self) {
        print!("          Castling: ");
        if self.state.castling == 0 {
            println!("none");
            return;
        }
        let mut castling_ltrs: String = String::new();
        if bitboard::get_bit(self.state.castling as u64, 0) {
            castling_ltrs.push('K');
        }
        if bitboard::get_bit(self.state.castling as u64, 1) {
            castling_ltrs.push('Q');
        }
        if bitboard::get_bit(self.state.castling as u64, 2) {
            castling_ltrs.push('k');
        }
        if bitboard::get_bit(self.state.castling as u64, 3) {
            castling_ltrs.push('q');
        }
        println!("{}", castling_ltrs.as_str());
    }
}
