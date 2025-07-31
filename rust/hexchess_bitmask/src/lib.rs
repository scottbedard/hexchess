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
