extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn bitmask(input: TokenStream) -> TokenStream {
    let source = parse_macro_input!(input as LitStr).value();
    let mut fen_index: u8 = 0;
    let mut result: u128 = 0;

    for (index, current) in source.chars().enumerate() {
        match current {
            '/' | '0' => continue,
            '1' => match source.chars().nth(index as usize + 1) {
                Some('0') | Some('1') => fen_index += 10,
                _ => fen_index += 1,
            },
            '2' => fen_index += 2,
            '3' => fen_index += 3,
            '4' => fen_index += 4,
            '5' => fen_index += 5,
            '6' => fen_index += 6,
            '7' => fen_index += 7,
            '8' => fen_index += 8,
            '9' => fen_index += 9,
            'x' => {
                result |= 1u128 << fen_index;
                fen_index += 1;
            }
            _ => panic!("invalid fen character at index {}: {}", index, current),
        }
    }

    if fen_index < 91 {
        panic!("board underflow: {} -> {}", source, fen_index);
    } else if fen_index > 91 {
        panic!("board overflow: {} -> {}", source, fen_index);
    }

    let output = quote! {
        #result
    };
    
    output.into()
}

#[proc_macro]
pub fn bitmask_csv(input: TokenStream) -> TokenStream {
    let source = parse_macro_input!(input as LitStr).value();
    let mut result: u128 = 0;

    let positions: Vec<String> = source
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    for position in positions {
        result |= 1u128 << bitmask_index_from_position(&position);
    }

    let output = quote! {
        #result
    };
    
    output.into()
}

/// Convert a position to a bitmap index
fn bitmask_index_from_position(position: &str) -> u8 {
    match position {
        "f11" => 0,
        "e10" => 1,
        "f10" => 2,
        "g10" => 3,
        "d9" => 4,
        "e9" => 5,
        "f9" => 6,
        "g9" => 7,
        "h9" => 8,
        "c8" => 9,
        "d8" => 10,
        "e8" => 11,
        "f8" => 12,
        "g8" => 13,
        "h8" => 14,
        "i8" => 15,
        "b7" => 16,
        "c7" => 17,
        "d7" => 18,
        "e7" => 19,
        "f7" => 20,
        "g7" => 21,
        "h7" => 22,
        "i7" => 23,
        "k7" => 24,
        "a6" => 25,
        "b6" => 26,
        "c6" => 27,
        "d6" => 28,
        "e6" => 29,
        "f6" => 30,
        "g6" => 31,
        "h6" => 32,
        "i6" => 33,
        "k6" => 34,
        "l6" => 35,
        "a5" => 36,
        "b5" => 37,
        "c5" => 38,
        "d5" => 39,
        "e5" => 40,
        "f5" => 41,
        "g5" => 42,
        "h5" => 43,
        "i5" => 44,
        "k5" => 45,
        "l5" => 46,
        "a4" => 47,
        "b4" => 48,
        "c4" => 49,
        "d4" => 50,
        "e4" => 51,
        "f4" => 52,
        "g4" => 53,
        "h4" => 54,
        "i4" => 55,
        "k4" => 56,
        "l4" => 57,
        "a3" => 58,
        "b3" => 59,
        "c3" => 60,
        "d3" => 61,
        "e3" => 62,
        "f3" => 63,
        "g3" => 64,
        "h3" => 65,
        "i3" => 66,
        "k3" => 67,
        "l3" => 68,
        "a2" => 69,
        "b2" => 70,
        "c2" => 71,
        "d2" => 72,
        "e2" => 73,
        "f2" => 74,
        "g2" => 75,
        "h2" => 76,
        "i2" => 77,
        "k2" => 78,
        "l2" => 79,
        "a1" => 80,
        "b1" => 81,
        "c1" => 82,
        "d1" => 83,
        "e1" => 84,
        "f1" => 85,
        "g1" => 86,
        "h1" => 87,
        "i1" => 88,
        "k1" => 89,
        "l1" => 90,
        _ => panic!("invalid position: {}", position),
    }
}