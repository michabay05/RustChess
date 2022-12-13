#![allow(dead_code)]

use crate::utils::*;

pub fn print_bitboard(num: u64) {
    for r in 0u8..8 {
        print!(" {} |", 8 - r);
        for f in 0u8..8 {
            print!(
                " {}",
                if get_bit(num, get_sq(r, f) as u8) {
                    "1"
                } else {
                    "."
                }
            );
        }
        println!();
    }
    println!("     - - - - - - - -\n     a b c d e f g h\n");
    println!("     Decimal: {}", num);
    println!(" Hexadecimal: {:#x}\n", num);
}

pub fn get_bit(num: u64, index: u8) -> bool {
    if (num & (1 << (index))) != 0 {
        return true;
    }
    false
}

pub fn set_bit_u8(num: &mut u8, index: u8) {
    (*num) |= 1 << index;
}

pub fn set_bit_u64(num: &mut u64, index: u8) {
    (*num) |= 1 << index;
}

pub fn pop_bit_u8(num: &mut u8, index: u8) {
    (*num) &= !(1 << index);
}

pub fn pop_bit_u64(num: &mut u64, index: u8) {
    (*num) &= !(1 << index);
}
