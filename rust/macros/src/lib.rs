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
                result |= 1u128 << bitmask_index(fen_index);
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

/// Convert a fen index to a bitmap index
fn bitmask_index(index: u8) -> u8 {
    match index {
        0 => 5, // f11
        1 => 15, // e10
        2 => 16, // f10
        3 => 6, // g10
        4 => 25, // d9
        5 => 26, // e9
        6 => 27, // f9
        7 => 17, // g9
        8 => 7, // h9
        9 => 35, // c8
        10 => 36, // d8
        11 => 37, // e8
        12 => 38, // f8
        13 => 28, // g8
        14 => 18, // h8
        15 => 8, // i8
        16 => 45, // b7
        17 => 46, // c7
        18 => 47, // d7
        19 => 48, // e7
        20 => 49, // f7
        21 => 39, // g7
        22 => 29, // h7
        23 => 19, // i7
        24 => 9, // k7
        25 => 55, // a6
        26 => 56, // b6
        27 => 57, // c6
        28 => 58, // d6
        29 => 59, // e6
        30 => 60, // f6
        31 => 50, // g6
        32 => 40, // h6
        33 => 30, // i6
        34 => 20, // k6
        35 => 10, // l6
        36 => 66, // a5
        37 => 67, // b5
        38 => 68, // c5
        39 => 69, // d5
        40 => 70, // e5
        41 => 71, // f5
        42 => 61, // g5
        43 => 51, // h5
        44 => 41, // i5
        45 => 31, // k5
        46 => 21, // l5
        47 => 77, // a4
        48 => 78, // b4
        49 => 79, // c4
        50 => 80, // d4
        51 => 81, // e4
        52 => 82, // f4
        53 => 72, // g4
        54 => 62, // h4
        55 => 52, // i4
        56 => 42, // k4
        57 => 32, // l4
        58 => 88, // a3
        59 => 89, // b3
        60 => 90, // c3
        61 => 91, // d3
        62 => 92, // e3
        63 => 93, // f3
        64 => 83, // g3
        65 => 73, // h3
        66 => 63, // i3
        67 => 53, // k3
        68 => 43, // l3
        69 => 99, // a2
        70 => 100, // b2
        71 => 101, // c2
        72 => 102, // d2
        73 => 103, // e2
        74 => 104, // f2
        75 => 94, // g2
        76 => 84, // h2
        77 => 74, // i2
        78 => 64, // k2
        79 => 54, // l2
        80 => 110, // a1
        81 => 111, // b1
        82 => 112, // c1
        83 => 113, // d1
        84 => 114, // e1
        85 => 115, // f1
        86 => 105, // g1
        87 => 95, // h1
        88 => 85, // i1
        89 => 75, // k1
        90 => 65, // l1
        _ => panic!("invalid fen index: {}", index),
    }
}

/// Convert a position to a bitmap index
fn bitmask_index_from_position(position: &str) -> u8 {
    match position {
        "f11" => 5,
        "g10" => 6,
        "h9" => 7,
        "i8" => 8,
        "k7" => 9,
        "l6" => 10,
        "e10" => 15,
        "f10" => 16,
        "g9" => 17,
        "h8" => 18,
        "i7" => 19,
        "k6" => 20,
        "l5" => 21,
        "d9" => 25,
        "e9" => 26,
        "f9" => 27,
        "g8" => 28,
        "h7" => 29,
        "i6" => 30,
        "k5" => 31,
        "l4" => 32,
        "c8" => 35,
        "d8" => 36,
        "e8" => 37,
        "f8" => 38,
        "g7" => 39,
        "h6" => 40,
        "i5" => 41,
        "k4" => 42,
        "l3" => 43,
        "b7" => 45,
        "c7" => 46,
        "d7" => 47,
        "e7" => 48,
        "f7" => 49,
        "g6" => 50,
        "h5" => 51,
        "i4" => 52,
        "k3" => 53,
        "l2" => 54,
        "a6" => 55,
        "b6" => 56,
        "c6" => 57,
        "d6" => 58,
        "e6" => 59,
        "f6" => 60,
        "g5" => 61,
        "h4" => 62,
        "i3" => 63,
        "k2" => 64,
        "l1" => 65,
        "a5" => 66,
        "b5" => 67,
        "c5" => 68,
        "d5" => 69,
        "e5" => 70,
        "f5" => 71,
        "g4" => 72,
        "h3" => 73,
        "i2" => 74,
        "k1" => 75,
        "a4" => 77,
        "b4" => 78,
        "c4" => 79,
        "d4" => 80,
        "e4" => 81,
        "f4" => 82,
        "g3" => 83,
        "h2" => 84,
        "i1" => 85,
        "a3" => 88,
        "b3" => 89,
        "c3" => 90,
        "d3" => 91,
        "e3" => 92,
        "f3" => 93,
        "g2" => 94,
        "h1" => 95,
        "a2" => 99,
        "b2" => 100,
        "c2" => 101,
        "d2" => 102,
        "e2" => 103,
        "f2" => 104,
        "g1" => 105,
        "a1" => 110,
        "b1" => 111,
        "c1" => 112,
        "d1" => 113,
        "e1" => 114,
        "f1" => 115,
        _ => panic!("invalid position: {}", position),
    }
}