use crate::board::{Board, CastlingType, Position, PIECE_STR};
use crate::utils::{self, Color};

pub fn parse(fen: &str) -> Board {
    let mut board: Board = Board::new();
    let mut fen_parts = fen.split_ascii_whitespace().into_iter();

    // Place piece on square
    parse_pieces(fen_parts.next().unwrap(), &mut board.pos);

    // Set side to move
    let side_to_move_str: &str = fen_parts.next().unwrap();
    if side_to_move_str == "w" {
        board.state.set_side(Color::White);
    } else if side_to_move_str == "b" {
        board.state.set_side(Color::Black);
    }

    // Set castling right
    for castling_type in fen_parts.next().unwrap().chars().into_iter() {
        if castling_type == 'K' {
            board
                .state
                .toggle_castling(CastlingType::WhiteKingside as u8);
        } else if castling_type == 'Q' {
            board
                .state
                .toggle_castling(CastlingType::WhiteQueenside as u8);
        } else if castling_type == 'k' {
            board
                .state
                .toggle_castling(CastlingType::BlackKingside as u8);
        } else if castling_type == 'q' {
            board
                .state
                .toggle_castling(CastlingType::BlackQueenside as u8);
        }
    }

    // Set enpassant square
    let enpass_square = fen_parts.next().unwrap();
    if enpass_square != "-" {
        board
            .state
            .set_enpassant(utils::get_sq_from_str(enpass_square));
    }
    // Set 50 move rule
    let rule50: u8 = fen_parts.next().unwrap().parse::<u8>().unwrap();
    board.state.set_rule50(rule50);
    // Set move counter
    let move_count: u8 = fen_parts.next().unwrap().parse::<u8>().unwrap();
    board.state.set_move_count(move_count);

    // Update units bitboard from piece bitboard
    board.pos.update_units_bb();
    board
}

fn parse_pieces(fen_piece: &str, pos: &mut Position) {
    let mut sq: u8 = 0;
    for piece_char in fen_piece.chars().into_iter() {
        if piece_char == '/' {
            continue;
        } else if piece_char.is_ascii_digit() {
            // Retrieve the int value of the offset from the char value
            let offset: u8 = piece_char as u8 - '0' as u8;
            // Add offset value to square counter
            sq += offset;
        } else if piece_char.is_ascii_alphabetic() {
            // Get the piece type and color from the fen character
            let piece_type = PIECE_STR.find(piece_char.to_ascii_uppercase()).unwrap();
            let piece_color = if piece_char.is_ascii_uppercase() {
                Color::White
            } else {
                Color::Black
            };
            pos.add_piece((piece_color as u8, piece_type as u8), sq);
            // Increment the current square
            sq += 1;
        }
    }
}
