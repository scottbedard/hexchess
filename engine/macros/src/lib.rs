extern crate proc_macro;

mod calc_advancement_bonus;

use proc_macro::TokenStream;

/// Macro that generates a compile-time optimized match statement for pawn advancement bonuses.
/// Takes position, color, and bonus multiplier (e.g., 10.0) and generates a match with pre-computed values.
/// All multiplication happens at compile time - only the match lookup happens at runtime.
/// 
/// # Example
/// ```
/// use engine_macros::calc_advancement_bonus;
/// use hexchess::{Position, Color};
/// 
/// let bonus = calc_advancement_bonus!(Position::A6, Color::Black, 10.0);
/// // Expands to a match statement with pre-computed values (0.2 * 10.0 = 2.0)
/// ```
#[proc_macro]
pub fn calc_advancement_bonus(input: TokenStream) -> TokenStream {
    calc_advancement_bonus::calc_advancement_bonus(input)
}
