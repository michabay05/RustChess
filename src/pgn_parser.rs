use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;

const MOVE_PATTERN: &str = r"((([KQRBN])?([a-h])?x?([a-h])([0-8])(=[QRBN])?|(O-O-O|O-O)))";

const MOVE_WITH_ANNOT_PATTERN: &str =
    r"((([KQRBN])?([a-h])?x?([a-h])([0-8])(=[QRBN])?|(O-O-O|O-O))([+#])?)";

const RESULT_PATTERN: &str = r"((1-0)|(1\/2-1\/2)|(0-1))";

const TAG_PATTERN: &str = r#"\[(\w+) "([\w ,.:\-\+/]+)"\]"#;

struct PGNInfo {
    tags: HashMap<String, String>,
    moves: Vec<u32>,
    result: u8,
}

// PGN result constants
const NO_RESULT: u8 = 0;
const WHITE_WINS: u8 = 1;
const BLACK_WINS: u8 = 2;
const DRAW: u8 = 3;

impl PGNInfo {
    fn new() -> Self {
        PGNInfo {
            tags: HashMap::new(),
            moves: Vec::new(),
            result: 10,
        }
    }
}

fn open_file(path_of_file: &str) -> File {
    let opened_file: File = File::open(path_of_file).expect("Couldn't open file");
    opened_file
}

pub fn read_pgn(path_of_file: &str) {
    let mut pgn_info = PGNInfo::new();
    let mut pgn_file: File = open_file(path_of_file);

    let mut file_contents = String::new();
    pgn_file
        .read_to_string(&mut file_contents)
        .expect("Can't read file at the moment...");

    let rgx_tag = regex::Regex::new(TAG_PATTERN).unwrap();
    for caps in rgx_tag.captures_iter(file_contents.as_str()) {
        pgn_info
            .tags
            .insert(String::from(&caps[1]), String::from(&caps[2]));
    }

    if pgn_info.tags.contains_key("Result") {
        let result_value = pgn_info.tags.get("Result").unwrap();
        if result_value == "1-0" {
            pgn_info.result = WHITE_WINS;
        } else if result_value == "0-1" {
            pgn_info.result = BLACK_WINS;
        } else {
            pgn_info.result = DRAW;
        }
    }
    println!("Result: {}", pgn_info.result);
}
