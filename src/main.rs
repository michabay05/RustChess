mod bitboard;
mod board;
mod moves;
mod pgn_parser;
mod utils;

fn main() {
    println!("Chess program!");

    pgn_parser::read_pgn("src/test.pgn");
}
