use crate::h;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Initial game position
pub const INITIAL_POSITION: &str = "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1";

/// This graph represents the positions on a hexboard, and their relationship
/// to one another. Each child in the graph represents a position, with it's
/// neighboring positions listed clockwise starting from the position directly
/// above it.
/// 
/// Think of it like the hands of a clock, with 12 o'clock being index 0...
/// For example, to find the position directly below f6, we'd first go to that
/// position in the fen (index 30), then look at the 6th index of that array,
/// which is 41. The 41st fen index is f5.
pub const HEXBOARD_GRAPH: [[Option<u8>; 12]; 91] = [
    [ 
        /* f11 */
        None, 
        None, 
        None,
        None, 
        Some(h!("g10")),
        Some(h!("g9")),
        Some(h!("f10")),
        Some(h!("e9")),
        Some(h!("e10")),
        None, 
        None, 
        None
   ],
    [ 
        /* e10 */
        None, 
        None, 
        Some(h!("f11")),
        Some(h!("g10")),
        Some(h!("f10")),
        Some(h!("f9")),
        Some(h!("e9")),
        Some(h!("d8")),
        Some(h!("d9")),
        None, 
        None, 
        None
   ],
    [ 
        /* f10 */
        Some(h!("f11")),
        None, 
        Some(h!("g10")),
        Some(h!("h9")),
        Some(h!("g9")),
        Some(h!("g8")),
        Some(h!("f9")),
        Some(h!("e8")),
        Some(h!("e9")),
        Some(h!("d9")),
        Some(h!("e10")),
        None,
   ],
    [ 
        /* g10 */
        None, 
        None, 
        None,
        None, 
        Some(h!("h9")),
        Some(h!("h8")),
        Some(h!("g9")),
        Some(h!("f9")),
        Some(h!("f10")),
        Some(h!("e10")),
        Some(h!("f11")),
        None,
   ],
    [ 
        /* d9 */
        None, 
        None,
        Some(h!("e10")),
        Some(h!("f10")),
        Some(h!("e9")),
        Some(h!("e8")),
        Some(h!("d8")),
        Some(h!("c7")),
        Some(h!("c8")),
        None, 
        None,
        None
   ],
    [ 
        /* e9 */
        Some(h!("e10")),
        Some(h!("f11")),
        Some(h!("f10")),
        Some(h!("g9")),
        Some(h!("f9")),
        Some(h!("f8")),
        Some(h!("e8")),
        Some(h!("d7")),
        Some(h!("d8")),
        Some(h!("c8")),
        Some(h!("d9")), 
        None
   ],
    [ 
        /* f9 */
        Some(h!("f10")),
        Some(h!("g10")),
        Some(h!("g9")),
        Some(h!("h8")),
        Some(h!("g8")),
        Some(h!("g7")),
        Some(h!("f8")),
        Some(h!("e7")),
        Some(h!("e8")),
        Some(h!("d8")),
        Some(h!("e9")),
        Some(h!("e10"))
   ],
    [ 
        /* g9 */
        Some(h!("g10")),
        None, 
        Some(h!("h9")),
        Some(h!("i8")),
        Some(h!("h8")),
        Some(h!("h7")),
        Some(h!("g8")),
        Some(h!("f8")),
        Some(h!("f9")),
        Some(h!("e9")),
        Some(h!("f10")),
        Some(h!("f11"))
   ],
    [ 
        /* h9 */
        None, 
        None, 
        None,
        None, 
        Some(h!("i8")),
        Some(h!("i7")),
        Some(h!("h8")),
        Some(h!("g8")),
        Some(h!("g9")),
        Some(h!("f10")),
        Some(h!("g10")),
        None,
   ],
    [ 
        /* c8 */
        None,
        None,
        Some(h!("d9")),
        Some(h!("e9")),
        Some(h!("d8")),
        Some(h!("d7")),
        Some(h!("c7")),
        Some(h!("b6")),
        Some(h!("b7")),
        None,
        None,
        None
   ],
    [ 
        /* d8 */
        Some(h!("d9")),
        Some(h!("e10")),
        Some(h!("e9")),
        Some(h!("f9")),
        Some(h!("e8")),
        Some(h!("e7")),
        Some(h!("d7")),
        Some(h!("c6")),
        Some(h!("c7")),
        Some(h!("b7")),
        Some(h!("c8")), 
        None
   ],
    [ 
        /* e8 */
        Some(h!("e9")),
        Some(h!("f10")),
        Some(h!("f9")),
        Some(h!("g8")),
        Some(h!("f8")),
        Some(h!("f7")),
        Some(h!("e7")),
        Some(h!("d6")),
        Some(h!("d7")),
        Some(h!("c7")),
        Some(h!("d8")),
        Some(h!("d9"))
   ],
    [ 
        /* f8 */
        Some(h!("f9")),
        Some(h!("g9")),
        Some(h!("g8")),
        Some(h!("h7")),
        Some(h!("g7")),
        Some(h!("g6")),
        Some(h!("f7")),
        Some(h!("e6")),
        Some(h!("e7")),
        Some(h!("d7")),
        Some(h!("e8")),
        Some(h!("e9"))
   ],
    [ 
        /* g8 */
        Some(h!("g9")),
        Some(h!("h9")),
        Some(h!("h8")),
        Some(h!("i7")),
        Some(h!("h7")),
        Some(h!("h6")),
        Some(h!("g7")),
        Some(h!("f7")),
        Some(h!("f8")),
        Some(h!("e8")),
        Some(h!("f9")),
        Some(h!("f10"))
   ],
    [ 
        /* h8 */
        Some(h!("h9")),
        None,
        Some(h!("i8")),
        Some(h!("k7")),
        Some(h!("i7")),
        Some(h!("i6")),
        Some(h!("h7")),
        Some(h!("g7")),
        Some(h!("g8")),
        Some(h!("f9")),
        Some(h!("g9")),
        Some(h!("g10"))
   ],
    [ 
        /* i8 */
        None,
        None,
        None,
        None,
        Some(h!("k7")),
        Some(h!("k6")),
        Some(h!("i7")),
        Some(h!("h7")),
        Some(h!("h8")),
        Some(h!("g9")),
        Some(h!("h9")),
        None,
   ],
    [ 
        /* b7 */
        None,
        None,
        Some(h!("c8")),
        Some(h!("d8")),
        Some(h!("c7")),
        Some(h!("c6")),
        Some(h!("b6")),
        Some(h!("a5")),
        Some(h!("a6")),
        None,
        None,
        None
   ],
    [ 
        /* c7 */
        Some(h!("c8")),
        Some(h!("d9")),
        Some(h!("d8")),
        Some(h!("e8")),
        Some(h!("d7")),
        Some(h!("d6")),
        Some(h!("c6")),
        Some(h!("b5")),
        Some(h!("b6")),
        Some(h!("a6")),
        Some(h!("b7")),
        None,
   ],
    [ 
        /* d7 */
        Some(h!("d8")),
        Some(h!("e9")),
        Some(h!("e8")),
        Some(h!("f8")),
        Some(h!("e7")),
        Some(h!("e6")),
        Some(h!("d6")),
        Some(h!("c5")),
        Some(h!("c6")),
        Some(h!("b6")),
        Some(h!("c7")),
        Some(h!("c8"))
   ],
    [ 
        /* e7 */
        Some(h!("e8")),
        Some(h!("f9")),
        Some(h!("f8")),
        Some(h!("g7")),
        Some(h!("f7")),
        Some(h!("f6")),
        Some(h!("e6")),
        Some(h!("d5")),
        Some(h!("d6")),
        Some(h!("c6")),
        Some(h!("d7")),
        Some(h!("d8"))
   ],
    [ 
        /* f7 */
        Some(h!("f8")),
        Some(h!("g8")),
        Some(h!("g7")),
        Some(h!("h6")),
        Some(h!("g6")),
        Some(h!("g5")),
        Some(h!("f6")),
        Some(h!("e5")),
        Some(h!("e6")),
        Some(h!("d6")),
        Some(h!("e7")),
        Some(h!("e8"))
   ],
    [ 
        /* g7 */
        Some(h!("g8")),
        Some(h!("h8")),
        Some(h!("h7")),
        Some(h!("i6")),
        Some(h!("h6")),
        Some(h!("h5")),
        Some(h!("g6")),
        Some(h!("f6")),
        Some(h!("f7")),
        Some(h!("e7")),
        Some(h!("f8")),
        Some(h!("f9"))
   ],
    [ 
        /* h7 */
        Some(h!("h8")),
        Some(h!("i8")),
        Some(h!("i7")),
        Some(h!("k6")),
        Some(h!("i6")),
        Some(h!("i5")),
        Some(h!("h6")),
        Some(h!("g6")),
        Some(h!("g7")),
        Some(h!("f8")),
        Some(h!("g8")),
        Some(h!("g9"))
   ],
    [ 
        /* i7 */
        Some(h!("i8")),
        None,
        Some(h!("k7")),
        Some(h!("l6")),
        Some(h!("k6")),
        Some(h!("k5")),
        Some(h!("i6")),
        Some(h!("h6")),
        Some(h!("h7")),
        Some(h!("g8")),
        Some(h!("h8")),
        Some(h!("h9"))
   ],
    [ 
        /* k7 */
        None,
        None,
        None,
        None,
        Some(h!("l6")),
        Some(h!("l5")),
        Some(h!("k6")),
        Some(h!("i6")),
        Some(h!("i7")),
        Some(h!("h8")),
        Some(h!("i8")),
        None,
   ],
    [ 
        /* a6 */
        None,
        None,
        Some(h!("b7")),
        Some(h!("c7")),
        Some(h!("b6")),
        Some(h!("b5")),
        Some(h!("a5")),
        None,
        None,
        None,
        None,
        None
   ],
    [ 
        /* b6 */
        Some(h!("b7")),
        Some(h!("c8")),
        Some(h!("c7")),
        Some(h!("d7")),
        Some(h!("c6")),
        Some(h!("c5")),
        Some(h!("b5")),
        Some(h!("a4")),
        Some(h!("a5")),
        None,
        Some(h!("a6")),
        None,
   ],
    [ 
        /* c6 */
        Some(h!("c7")),
        Some(h!("d8")),
        Some(h!("d7")),
        Some(h!("e7")),
        Some(h!("d6")),
        Some(h!("d5")),
        Some(h!("c5")),
        Some(h!("b4")),
        Some(h!("b5")),
        Some(h!("a5")),
        Some(h!("b6")),
        Some(h!("b7"))
   ],
    [ 
        /* d6 */
        Some(h!("d7")),
        Some(h!("e8")),
        Some(h!("e7")),
        Some(h!("f7")),
        Some(h!("e6")),
        Some(h!("e5")),
        Some(h!("d5")),
        Some(h!("c4")),
        Some(h!("c5")),
        Some(h!("b5")),
        Some(h!("c6")),
        Some(h!("c7"))
   ],
    [ 
        /* e6 */
        Some(h!("e7")),
        Some(h!("f8")),
        Some(h!("f7")),
        Some(h!("g6")),
        Some(h!("f6")),
        Some(h!("f5")),
        Some(h!("e5")),
        Some(h!("d4")),
        Some(h!("d5")),
        Some(h!("c5")),
        Some(h!("d6")),
        Some(h!("d7"))
   ],
    [ 
        /* f6 */
        Some(h!("f7")),
        Some(h!("g7")),
        Some(h!("g6")),
        Some(h!("h5")),
        Some(h!("g5")),
        Some(h!("g4")),
        Some(h!("f5")),
        Some(h!("e4")),
        Some(h!("e5")),
        Some(h!("d5")),
        Some(h!("e6")),
        Some(h!("e7"))
   ],
    [ 
        /* g6 */
        Some(h!("g7")),
        Some(h!("h7")),
        Some(h!("h6")),
        Some(h!("i5")),
        Some(h!("h5")),
        Some(h!("h4")),
        Some(h!("g5")),
        Some(h!("f5")),
        Some(h!("f6")),
        Some(h!("e6")),
        Some(h!("f7")),
        Some(h!("f8"))
   ],
    [ 
        /* h6 */
        Some(h!("h7")),
        Some(h!("i7")),
        Some(h!("i6")),
        Some(h!("k5")),
        Some(h!("i5")),
        Some(h!("i4")),
        Some(h!("h5")),
        Some(h!("g5")),
        Some(h!("g6")),
        Some(h!("f7")),
        Some(h!("g7")),
        Some(h!("g8"))
   ],
    [ 
        /* i6 */
        Some(h!("i7")),
        Some(h!("k7")),
        Some(h!("k6")),
        Some(h!("l5")),
        Some(h!("k5")),
        Some(h!("k4")),
        Some(h!("i5")),
        Some(h!("h5")),
        Some(h!("h6")),
        Some(h!("g7")),
        Some(h!("h7")),
        Some(h!("h8"))
   ],
    [ 
        /* k6 */
        Some(h!("k7")),
        None,
        Some(h!("l6")),
        None,
        Some(h!("l5")),
        Some(h!("l4")),
        Some(h!("k5")),
        Some(h!("i5")),
        Some(h!("i6")),
        Some(h!("h7")),
        Some(h!("i7")),
        Some(h!("i8"))
   ],
    [ 
        /* l6 */
        None,
        None,
        None,
        None,
        None,
        None,
        Some(h!("l5")),
        Some(h!("k5")),
        Some(h!("k6")),
        Some(h!("i7")),
        Some(h!("k7")),
        None,
   ],
    [ 
        /* a5 */
        Some(h!("a6")),
        Some(h!("b7")),
        Some(h!("b6")),
        Some(h!("c6")),
        Some(h!("b5")),
        Some(h!("b4")),
        Some(h!("a4")),
        None,
        None,
        None,
        None,
        None
   ],
    [ 
        /* b5 */
        Some(h!("b6")),
        Some(h!("c7")),
        Some(h!("c6")),
        Some(h!("d6")),
        Some(h!("c5")),
        Some(h!("c4")),
        Some(h!("b4")),
        Some(h!("a3")),
        Some(h!("a4")),
        None,
        Some(h!("a5")),
        Some(h!("a6"))
   ],
    [ 
        /* c5 */
        Some(h!("c6")),
        Some(h!("d7")),
        Some(h!("d6")),
        Some(h!("e6")),
        Some(h!("d5")),
        Some(h!("d4")),
        Some(h!("c4")),
        Some(h!("b3")),
        Some(h!("b4")),
        Some(h!("a4")),
        Some(h!("b5")),
        Some(h!("b6"))
   ],
    [ 
        /* d5 */
        Some(h!("d6")),
        Some(h!("e7")),
        Some(h!("e6")),
        Some(h!("f6")),
        Some(h!("e5")),
        Some(h!("e4")),
        Some(h!("d4")),
        Some(h!("c3")),
        Some(h!("c4")),
        Some(h!("b4")),
        Some(h!("c5")),
        Some(h!("c6"))
   ],
    [ 
        /* e5 */
        Some(h!("e6")),
        Some(h!("f7")),
        Some(h!("f6")),
        Some(h!("g5")),
        Some(h!("f5")),
        Some(h!("f4")),
        Some(h!("e4")),
        Some(h!("d3")),
        Some(h!("d4")),
        Some(h!("c4")),
        Some(h!("d5")),
        Some(h!("d6"))
   ],
    [ 
        /* f5 */
        Some(h!("f6")),
        Some(h!("g6")),
        Some(h!("g5")),
        Some(h!("h4")),
        Some(h!("g4")),
        Some(h!("g3")),
        Some(h!("f4")),
        Some(h!("e3")),
        Some(h!("e4")),
        Some(h!("d4")),
        Some(h!("e5")),
        Some(h!("e6"))
   ],
    [ 
        /* g5 */
        Some(h!("g6")),
        Some(h!("h6")),
        Some(h!("h5")),
        Some(h!("i4")),
        Some(h!("h4")),
        Some(h!("h3")),
        Some(h!("g4")),
        Some(h!("f4")),
        Some(h!("f5")),
        Some(h!("e5")),
        Some(h!("f6")),
        Some(h!("f7"))
   ],
    [ 
        /* h5 */
        Some(h!("h6")),
        Some(h!("i6")),
        Some(h!("i5")),
        Some(h!("k4")),
        Some(h!("i4")),
        Some(h!("i3")),
        Some(h!("h4")),
        Some(h!("g4")),
        Some(h!("g5")),
        Some(h!("f6")),
        Some(h!("g6")),
        Some(h!("g7"))
   ],
    [ 
        /* i5 */
        Some(h!("i6")),
        Some(h!("k6")),
        Some(h!("k5")),
        Some(h!("l4")),
        Some(h!("k4")),
        Some(h!("k3")),
        Some(h!("i4")),
        Some(h!("h4")),
        Some(h!("h5")),
        Some(h!("g6")),
        Some(h!("h6")),
        Some(h!("h7"))
   ],
    [ 
        /* k5 */
        Some(h!("k6")),
        Some(h!("l6")),
        Some(h!("l5")),
        None,
        Some(h!("l4")),
        Some(h!("l3")),
        Some(h!("k4")),
        Some(h!("i4")),
        Some(h!("i5")),
        Some(h!("h6")),
        Some(h!("i6")),
        Some(h!("i7"))
   ],
    [ 
        /* l5 */
        Some(h!("l6")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("l4")),
        Some(h!("k4")),
        Some(h!("k5")),
        Some(h!("i6")),
        Some(h!("k6")),
        Some(h!("k7"))
   ],
    [ 
        /* a4 */
        Some(h!("a5")),
        Some(h!("b6")),
        Some(h!("b5")),
        Some(h!("c5")),
        Some(h!("b4")),
        Some(h!("b3")),
        Some(h!("a3")),
        None,
        None,
        None,
        None,
        None
   ],
    [ 
        /* b4 */
        Some(h!("b5")),
        Some(h!("c6")),
        Some(h!("c5")),
        Some(h!("d5")),
        Some(h!("c4")),
        Some(h!("c3")),
        Some(h!("b3")),
        Some(h!("a2")),
        Some(h!("a3")),
        None,
        Some(h!("a4")),
        Some(h!("a5"))
   ],
    [ 
        /* c4 */
        Some(h!("c5")),
        Some(h!("d6")),
        Some(h!("d5")),
        Some(h!("e5")),
        Some(h!("d4")),
        Some(h!("d3")),
        Some(h!("c3")),
        Some(h!("b2")),
        Some(h!("b3")),
        Some(h!("a3")),
        Some(h!("b4")),
        Some(h!("b5"))
   ],
    [ 
        /* d4 */
        Some(h!("d5")),
        Some(h!("e6")),
        Some(h!("e5")),
        Some(h!("f5")),
        Some(h!("e4")),
        Some(h!("e3")),
        Some(h!("d3")),
        Some(h!("c2")),
        Some(h!("c3")),
        Some(h!("b3")),
        Some(h!("c4")),
        Some(h!("c5"))
   ],
    [ 
        /* e4 */
        Some(h!("e5")),
        Some(h!("f6")),
        Some(h!("f5")),
        Some(h!("g4")),
        Some(h!("f4")),
        Some(h!("f3")),
        Some(h!("e3")),
        Some(h!("d2")),
        Some(h!("d3")),
        Some(h!("c3")),
        Some(h!("d4")),
        Some(h!("d5"))
   ],
    [ 
        /* f4 */
        Some(h!("f5")),
        Some(h!("g5")),
        Some(h!("g4")),
        Some(h!("h3")),
        Some(h!("g3")),
        Some(h!("g2")),
        Some(h!("f3")),
        Some(h!("e2")),
        Some(h!("e3")),
        Some(h!("d3")),
        Some(h!("e4")),
        Some(h!("e5"))
   ],
    [ 
        /* g4 */
        Some(h!("g5")),
        Some(h!("h5")),
        Some(h!("h4")),
        Some(h!("i3")),
        Some(h!("h3")),
        Some(h!("h2")),
        Some(h!("g3")),
        Some(h!("f3")),
        Some(h!("f4")),
        Some(h!("e4")),
        Some(h!("f5")),
        Some(h!("f6"))
   ],
    [ 
        /* h4 */
        Some(h!("h5")),
        Some(h!("i5")),
        Some(h!("i4")),
        Some(h!("k3")),
        Some(h!("i3")),
        Some(h!("i2")),
        Some(h!("h3")),
        Some(h!("g3")),
        Some(h!("g4")),
        Some(h!("f5")),
        Some(h!("g5")),
        Some(h!("g6"))
   ],
    [ 
        /* i4 */
        Some(h!("i5")),
        Some(h!("k5")),
        Some(h!("k4")),
        Some(h!("l3")),
        Some(h!("k3")),
        Some(h!("k2")),
        Some(h!("i3")),
        Some(h!("h3")),
        Some(h!("h4")),
        Some(h!("g5")),
        Some(h!("h5")),
        Some(h!("h6"))
   ],
    [ 
        /* k4 */
        Some(h!("k5")),
        Some(h!("l5")),
        Some(h!("l4")),
        None,
        Some(h!("l3")),
        Some(h!("l2")),
        Some(h!("k3")),
        Some(h!("i3")),
        Some(h!("i4")),
        Some(h!("h5")),
        Some(h!("i5")),
        Some(h!("i6"))
   ],
    [ 
        /* l4 */
        Some(h!("l5")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("l3")),
        Some(h!("k3")),
        Some(h!("k4")),
        Some(h!("i5")),
        Some(h!("k5")),
        Some(h!("k6"))
   ],
    [ 
        /* a3 */
        Some(h!("a4")),
        Some(h!("b5")),
        Some(h!("b4")),
        Some(h!("c4")),
        Some(h!("b3")),
        Some(h!("b2")),
        Some(h!("a2")),
        None,
        None,
        None,
        None,
        None
   ],
    [ 
        /* b3 */
        Some(h!("b4")),
        Some(h!("c5")),
        Some(h!("c4")),
        Some(h!("d4")),
        Some(h!("c3")),
        Some(h!("c2")),
        Some(h!("b2")),
        Some(h!("a1")),
        Some(h!("a2")),
        None,
        Some(h!("a3")),
        Some(h!("a4"))
   ],
    [ 
        /* c3 */
        Some(h!("c4")),
        Some(h!("d5")),
        Some(h!("d4")),
        Some(h!("e4")),
        Some(h!("d3")),
        Some(h!("d2")),
        Some(h!("c2")),
        Some(h!("b1")),
        Some(h!("b2")),
        Some(h!("a2")),
        Some(h!("b3")),
        Some(h!("b4"))
   ],
    [ 
        /* d3 */
        Some(h!("d4")),
        Some(h!("e5")),
        Some(h!("e4")),
        Some(h!("f4")),
        Some(h!("e3")),
        Some(h!("e2")),
        Some(h!("d2")),
        Some(h!("c1")),
        Some(h!("c2")),
        Some(h!("b2")),
        Some(h!("c3")),
        Some(h!("c4"))
   ],
    [ 
        /* e3 */
        Some(h!("e4")),
        Some(h!("f5")),
        Some(h!("f4")),
        Some(h!("g3")),
        Some(h!("f3")),
        Some(h!("f2")),
        Some(h!("e2")),
        Some(h!("d1")),
        Some(h!("d2")),
        Some(h!("c2")),
        Some(h!("d3")),
        Some(h!("d4"))
   ],
    [ 
        /* f3 */
        Some(h!("f4")),
        Some(h!("g4")),
        Some(h!("g3")),
        Some(h!("h2")),
        Some(h!("g2")),
        Some(h!("g1")),
        Some(h!("f2")),
        Some(h!("e1")),
        Some(h!("e2")),
        Some(h!("d2")),
        Some(h!("e3")),
        Some(h!("e4"))
   ],
    [ 
        /* g3 */
        Some(h!("g4")),
        Some(h!("h4")),
        Some(h!("h3")),
        Some(h!("i2")),
        Some(h!("h2")),
        Some(h!("h1")),
        Some(h!("g2")),
        Some(h!("f2")),
        Some(h!("f3")),
        Some(h!("e3")),
        Some(h!("f4")),
        Some(h!("f5"))
   ],
    [ 
        /* h3 */
        Some(h!("h4")),
        Some(h!("i4")),
        Some(h!("i3")),
        Some(h!("k2")),
        Some(h!("i2")),
        Some(h!("i1")),
        Some(h!("h2")),
        Some(h!("g2")),
        Some(h!("g3")),
        Some(h!("f4")),
        Some(h!("g4")),
        Some(h!("g5"))
   ],
    [ 
        /* i3 */
        Some(h!("i4")),
        Some(h!("k4")),
        Some(h!("k3")),
        Some(h!("l2")),
        Some(h!("k2")),
        Some(h!("k1")),
        Some(h!("i2")),
        Some(h!("h2")),
        Some(h!("h3")),
        Some(h!("g4")),
        Some(h!("h4")),
        Some(h!("h5"))
   ],
    [ 
        /* k3 */
        Some(h!("k4")),
        Some(h!("l4")),
        Some(h!("l3")),
        None,
        Some(h!("l2")),
        Some(h!("l1")),
        Some(h!("k2")),
        Some(h!("i2")),
        Some(h!("i3")),
        Some(h!("h4")),
        Some(h!("i4")),
        Some(h!("i5"))
   ],
    [ 
        /* l3 */
        Some(h!("l4")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("l2")),
        Some(h!("k2")),
        Some(h!("k3")),
        Some(h!("i4")),
        Some(h!("k4")),
        Some(h!("k5"))
   ],
    [ 
        /* a2 */
        Some(h!("a3")),
        Some(h!("b4")),
        Some(h!("b3")),
        Some(h!("c3")),
        Some(h!("b2")),
        Some(h!("b1")),
        Some(h!("a1")),
        None,
        None,
        None,
        None,
        None
   ],
    [ 
        /* b2 */
        Some(h!("b3")),
        Some(h!("c4")),
        Some(h!("c3")),
        Some(h!("d3")),
        Some(h!("c2")),
        Some(h!("c1")),
        Some(h!("b1")),
        None,
        Some(h!("a1")),
        None,
        Some(h!("a2")),
        Some(h!("a3"))
   ],
    [ 
        /* c2 */
        Some(h!("c3")),
        Some(h!("d4")),
        Some(h!("d3")),
        Some(h!("e3")),
        Some(h!("d2")),
        Some(h!("d1")),
        Some(h!("c1")),
        None,
        Some(h!("b1")),
        Some(h!("a1")),
        Some(h!("b2")),
        Some(h!("b3"))
   ],
    [ 
        /* d2 */
        Some(h!("d3")),
        Some(h!("e4")),
        Some(h!("e3")),
        Some(h!("f3")),
        Some(h!("e2")),
        Some(h!("e1")),
        Some(h!("d1")),
        None,
        Some(h!("c1")),
        Some(h!("b1")),
        Some(h!("c2")),
        Some(h!("c3"))
   ],
    [ 
        /* e2 */
        Some(h!("e3")),
        Some(h!("f4")),
        Some(h!("f3")),
        Some(h!("g2")),
        Some(h!("f2")),
        Some(h!("f1")),
        Some(h!("e1")),
        None,
        Some(h!("d1")),
        Some(h!("c1")),
        Some(h!("d2")),
        Some(h!("d3"))
   ],
    [ 
        /* f2 */
        Some(h!("f3")),
        Some(h!("g3")),
        Some(h!("g2")),
        Some(h!("h1")),
        Some(h!("g1")),
        None,
        Some(h!("f1")),
        None,
        Some(h!("e1")),
        Some(h!("d1")),
        Some(h!("e2")),
        Some(h!("e3"))
   ],
    [ 
        /* g2 */
        Some(h!("g3")),
        Some(h!("h3")),
        Some(h!("h2")),
        Some(h!("i1")),
        Some(h!("h1")),
        None,
        Some(h!("g1")),
        Some(h!("f1")),
        Some(h!("f2")),
        Some(h!("e2")),
        Some(h!("f3")),
        Some(h!("f4"))
   ],
    [ 
        /* h2 */
        Some(h!("h3")),
        Some(h!("i3")),
        Some(h!("i2")),
        Some(h!("k1")),
        Some(h!("i1")),
        None,
        Some(h!("h1")),
        Some(h!("g1")),
        Some(h!("g2")),
        Some(h!("f3")),
        Some(h!("g3")),
        Some(h!("g4"))
   ],
    [ 
        /* i2 */
        Some(h!("i3")),
        Some(h!("k3")),
        Some(h!("k2")),
        Some(h!("l1")),
        Some(h!("k1")),
        None,
        Some(h!("i1")),
        Some(h!("h1")),
        Some(h!("h2")),
        Some(h!("g3")),
        Some(h!("h3")),
        Some(h!("h4"))
   ],
    [ 
        /* k2 */
        Some(h!("k3")),
        Some(h!("l3")),
        Some(h!("l2")),
        None,
        Some(h!("l1")),
        None,
        Some(h!("k1")),
        Some(h!("i1")),
        Some(h!("i2")),
        Some(h!("h3")),
        Some(h!("i3")),
        Some(h!("i4"))
   ],
    [ 
        /* l2 */
        Some(h!("l3")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("l1")),
        Some(h!("k1")),
        Some(h!("k2")),
        Some(h!("i3")),
        Some(h!("k3")),
        Some(h!("k4"))
   ],
    [ 
        /* a1 */
        Some(h!("a2")),
        Some(h!("b3")),
        Some(h!("b2")),
        Some(h!("c2")),
        Some(h!("b1")),
        None,
        None,
        None,
        None,
        None,
        None,
        None
   ],
    [ 
        /* b1 */
        Some(h!("b2")),
        Some(h!("c3")),
        Some(h!("c2")),
        Some(h!("d2")),
        Some(h!("c1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("a1")),
        Some(h!("a2"))
   ],
    [ 
        /* c1 */
        Some(h!("c2")),
        Some(h!("d3")),
        Some(h!("d2")),
        Some(h!("e2")),
        Some(h!("d1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("b1")),
        Some(h!("b2"))
   ],
    [ 
        /* d1 */
        Some(h!("d2")),
        Some(h!("e3")),
        Some(h!("e2")),
        Some(h!("f2")),
        Some(h!("e1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("c1")),
        Some(h!("c2"))
   ],
    [ 
        /* e1 */
        Some(h!("e2")),
        Some(h!("f3")),
        Some(h!("f2")),
        Some(h!("g1")),
        Some(h!("f1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("d1")),
        Some(h!("d2"))
   ],
    [ 
        /* f1 */
        Some(h!("f2")),
        Some(h!("g2")),
        Some(h!("g1")),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(h!("e1")),
        Some(h!("e2"))
   ],
    [ 
        /* g1 */
        Some(h!("g2")),
        Some(h!("h2")),
        Some(h!("h1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("f1")),
        Some(h!("e1")),
        Some(h!("f2")),
        Some(h!("f3"))
   ],
    [ 
        /* h1 */
        Some(h!("h2")),
        Some(h!("i2")),
        Some(h!("i1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("g1")),
        Some(h!("f2")),
        Some(h!("g2")),
        Some(h!("g3"))
   ],
    [ 
        /* i1 */
        Some(h!("i2")),
        Some(h!("k2")),
        Some(h!("k1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("h1")),
        Some(h!("g2")),
        Some(h!("h2")),
        Some(h!("h3"))
   ],
    [ 
        /* k1 */
        Some(h!("k2")),
        Some(h!("l2")),
        Some(h!("l1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("i1")),
        Some(h!("h2")),
        Some(h!("i2")),
        Some(h!("i3"))
   ],
    [ 
        /* l1 */
        Some(h!("l2")),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(h!("k1")),
        Some(h!("i2")),
        Some(h!("k2")),
        Some(h!("k3"))
    ]
];

/// Reachable positions for a knight from each position on the board
pub const KNIGHT_GRAPH: [&[u8]; 91] = [
    &[14, 13, 11, 10],
    &[8, 13, 12, 18, 17],
    &[15, 22, 21, 19, 18, 9],
    &[23, 22, 12, 11, 4],
    &[3, 7, 12, 19, 27, 26],
    &[8, 14, 21, 20, 28, 27, 16],
    &[15, 23, 32, 31, 29, 28, 17, 9],
    &[24, 33, 32, 20, 19, 10, 4],
    &[34, 33, 21, 12, 5, 1],
    &[2, 6, 19, 28, 37, 36],
    &[0, 7, 13, 20, 29, 38, 37, 25],
    &[0, 3, 14, 22, 31, 30, 39, 38, 26, 16],
    &[3, 8, 23, 33, 43, 42, 40, 39, 27, 17, 4, 1],
    &[24, 34, 44, 43, 30, 29, 18, 10, 1, 0],
    &[35, 45, 44, 31, 20, 11, 5, 0],
    &[46, 45, 32, 21, 6, 2],
    &[5, 11, 28, 38, 47],
    &[1, 6, 12, 29, 39, 48, 47],
    &[1, 2, 13, 21, 30, 40, 49, 48, 36, 25],
    &[2, 7, 22, 32, 42, 41, 50, 49, 37, 26, 9, 4],
    &[7, 14, 33, 44, 54, 53, 51, 50, 38, 27, 10, 5],
    &[8, 15, 34, 45, 55, 54, 41, 40, 28, 18, 5, 2],
    &[35, 46, 56, 55, 42, 30, 19, 11, 2, 3],
    &[57, 56, 43, 31, 12, 6, 3],
    &[57, 44, 32, 13, 7],
    &[10, 18, 38, 48],
    &[4, 11, 19, 39, 49, 58],
    &[4, 5, 12, 20, 40, 50, 59, 58],
    &[5, 6, 21, 31, 41, 51, 60, 59, 47, 36, 16, 9],
    &[6, 13, 32, 43, 53, 52, 61, 60, 48, 37, 17, 10],
    &[13, 22, 44, 55, 65, 64, 62, 61, 49, 38, 18, 11],
    &[14, 23, 45, 56, 66, 65, 52, 51, 39, 28, 11, 6],
    &[15, 24, 46, 57, 67, 66, 53, 41, 29, 19, 6, 7],
    &[68, 67, 54, 42, 20, 12, 7, 8],
    &[68, 55, 43, 21, 13, 8],
    &[56, 44, 22, 14],
    &[9, 18, 28, 49, 59],
    &[9, 10, 19, 29, 50, 60, 69],
    &[10, 11, 20, 30, 51, 61, 70, 69, 25, 16],
    &[11, 12, 31, 42, 52, 62, 71, 70, 58, 47, 26, 17],
    &[12, 21, 43, 54, 64, 63, 72, 71, 59, 48, 27, 18],
    &[21, 32, 55, 66, 76, 75, 73, 72, 60, 49, 28, 19],
    &[22, 33, 56, 67, 77, 76, 63, 62, 50, 39, 19, 12],
    &[23, 34, 57, 68, 78, 77, 64, 52, 40, 29, 12, 13],
    &[24, 35, 79, 78, 65, 53, 30, 20, 13, 14],
    &[79, 66, 54, 31, 21, 14, 15],
    &[67, 55, 32, 22, 15],
    &[16, 17, 28, 39, 60, 70],
    &[17, 18, 29, 40, 61, 71, 80, 25],
    &[18, 19, 30, 41, 62, 72, 81, 80, 36, 26],
    &[19, 20, 42, 53, 63, 73, 82, 81, 69, 58, 37, 27],
    &[20, 31, 54, 65, 75, 74, 83, 82, 70, 59, 38, 28],
    &[31, 43, 66, 77, 87, 86, 84, 83, 71, 60, 39, 29],
    &[32, 44, 67, 78, 88, 87, 74, 73, 61, 50, 29, 20],
    &[33, 45, 68, 79, 89, 88, 75, 63, 51, 40, 20, 21],
    &[34, 46, 90, 89, 76, 64, 41, 30, 21, 22],
    &[35, 90, 77, 65, 42, 31, 22, 23],
    &[78, 66, 43, 32, 23, 24],
    &[26, 27, 39, 50, 71, 81],
    &[27, 28, 40, 51, 72, 82, 36],
    &[28, 29, 41, 52, 73, 83, 47, 37],
    &[29, 30, 53, 64, 74, 84, 80, 69, 48, 38],
    &[30, 42, 65, 76, 86, 85, 81, 70, 49, 39],
    &[42, 54, 77, 88, 82, 71, 50, 40],
    &[43, 55, 78, 89, 85, 84, 72, 61, 40, 30],
    &[44, 56, 79, 90, 86, 74, 62, 51, 30, 31],
    &[45, 57, 87, 75, 52, 41, 31, 32],
    &[46, 88, 76, 53, 42, 32, 33],
    &[89, 77, 54, 43, 33, 34],
    &[37, 38, 50, 61, 82],
    &[38, 39, 51, 62, 83, 47],
    &[39, 40, 52, 63, 84, 58, 48],
    &[40, 41, 64, 75, 85, 80, 59, 49],
    &[41, 53, 76, 87, 81, 60, 50],
    &[53, 65, 88, 82, 61, 51],
    &[54, 66, 89, 83, 72, 51, 41],
    &[55, 67, 90, 85, 73, 62, 41, 42],
    &[56, 68, 86, 63, 52, 42, 43],
    &[57, 87, 64, 53, 43, 44],
    &[88, 65, 54, 44, 45],
    &[48, 49, 61, 72],
    &[49, 50, 62, 73, 58],
    &[50, 51, 63, 74, 69, 59],
    &[51, 52, 75, 86, 70, 60],
    &[52, 64, 87, 71, 61],
    &[64, 76, 72, 62],
    &[65, 77, 83, 62, 52],
    &[66, 78, 84, 73, 52, 53],
    &[67, 79, 74, 63, 53, 54],
    &[68, 75, 64, 54, 55],
    &[76, 65, 55, 56],
];

/// Promotion pieces
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum PromotionPiece {
    #[serde(rename(deserialize = "b", serialize = "b"))]
    Bishop,

    #[serde(rename(deserialize = "n", serialize = "n"))]
    Knight,

    #[serde(rename(deserialize = "q", serialize = "q"))]
    Queen,

    #[serde(rename(deserialize = "r", serialize = "r"))]
    Rook,
}

impl fmt::Display for PromotionPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            PromotionPiece::Bishop => 'b',
            PromotionPiece::Knight => 'n',
            PromotionPiece::Queen => 'q',
            PromotionPiece::Rook => 'r',
        };
        write!(f, "{}", printable)
    }
}
