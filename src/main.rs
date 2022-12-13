mod bitboard;
mod board;
mod fen;
mod moves;
mod pgn_parser;
mod utils;

fn main() {
    println!("Chess program!");

    let b = fen::parse("rnbqkbnr/pppppppp/8/8/P6P/8/PPPPPPPP/RNBQKBNR b Kkq - 4 17");
    b.print_board();
}
