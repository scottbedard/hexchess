use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::quote;
use syn::{Expr, parse::Parser};
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// Macro that generates a compile-time optimized match statement for pawn
/// advancement bonuses. Takes position, color, and bonus multiplier (e.g., 10.0)
/// and generates a match with pre-computed values. All multiplication happens at
/// compile time - only the match lookup happens at runtime.
/// 
/// # Example
/// ```
/// use engine_macros::calc_advancement_bonus;
/// use hexchess::{Position, Color};
/// 
/// let bonus = calc_advancement_bonus!(Position::A6, Color::Black, 10.0);
/// // Expands to a match statement with pre-computed values (0.2 * 10.0 = 2.0)
/// ```
pub fn calc_advancement_bonus(input: TokenStream) -> TokenStream {
    // Convert to proc_macro2::TokenStream for parsing
    let input2: TokenStream2 = input.into();
    
    // Parse comma-separated expressions: position, color, bonus
    let parser = Punctuated::<Expr, Comma>::parse_terminated;
    let punctuated = parser.parse2(input2).expect("failed to parse macro input");
    let mut args = punctuated.into_iter();
    
    let position_expr = args.next().expect("expected position argument");
    let color_expr = args.next().expect("expected color argument");
    let bonus_expr = args.next().expect("expected bonus argument");
    
    if args.next().is_some() {
        panic!("calc_advancement_bonus! expects exactly 3 arguments: position, color, bonus");
    }

    // Extract the bonus value (must be a float literal)
    let bonus: f32 = match &bonus_expr {
        Expr::Lit(syn::ExprLit { lit: syn::Lit::Float(lit_float), .. }) => {
            lit_float.base10_digits().parse().expect("invalid float literal")
        }
        _ => panic!("bonus must be a float literal"),
    };

    // Define the scalar values for each position/color combination.
    // 1 = fully advanced, 0 = not advanced at all
    let black_scalars = vec![
        ("A6", 0.2), ("B6", 0.2), ("C6", 0.2), ("D6", 0.2), ("E6", 0.2),
        ("F6", 0.2), ("G6", 0.2), ("H6", 0.2), ("I6", 0.2), ("K6", 0.2),
        ("L6", 0.2), ("A5", 0.4), ("B5", 0.4), ("C5", 0.4), ("D5", 0.4),
        ("E5", 0.4), ("F5", 0.4), ("G5", 0.4), ("H5", 0.4), ("I5", 0.4),
        ("K5", 0.4), ("L5", 0.4), ("A4", 0.6), ("B4", 0.6), ("C4", 0.6),
        ("D4", 0.6), ("E4", 0.6), ("F4", 0.6), ("G4", 0.6), ("H4", 0.6),
        ("I4", 0.6), ("K4", 0.6), ("L4", 0.6), ("A3", 0.8), ("B3", 0.8),
        ("C3", 0.8), ("D3", 0.8), ("E3", 0.8), ("F3", 0.8), ("G3", 0.8),
        ("H3", 0.8), ("I3", 0.8), ("K3", 0.8), ("L3", 0.8), ("A2", 1.0),
        ("B2", 1.0), ("C2", 1.0), ("D2", 1.0), ("E2", 1.0), ("F2", 1.0),
        ("G2", 1.0), ("H2", 1.0), ("I2", 1.0), ("K2", 1.0), ("L2", 1.0),
    ];

    let white_scalars = vec![
        ("F10", 1.0), ("E9", 1.0), ("F9", 0.8), ("G9", 1.0), ("D8", 1.0),
        ("E8", 0.8), ("F8", 0.6), ("G8", 0.8), ("H8", 1.0), ("C7", 1.0),
        ("D7", 0.8), ("E7", 0.6), ("F7", 0.4), ("G7", 0.6), ("H7", 0.8),
        ("I7", 1.0), ("B6", 1.0), ("C6", 0.8), ("D6", 0.6), ("E6", 0.4),
        ("F6", 0.2), ("G6", 0.4), ("H6", 0.6), ("I6", 0.8), ("K6", 1.0),
        ("A5", 1.0), ("B5", 0.8), ("C5", 0.6), ("D5", 0.4), ("E5", 0.2),
        ("G5", 0.2), ("H5", 0.4), ("I5", 0.6), ("K5", 0.8), ("L5", 1.0),
        ("A4", 0.8), ("B4", 0.6), ("C4", 0.4), ("D4", 0.2), ("H4", 0.2),
        ("I4", 0.4), ("K4", 0.6), ("L4", 0.8), ("A3", 0.6), ("B3", 0.4),
        ("C3", 0.2), ("I3", 0.2), ("K3", 0.4), ("L3", 0.6), ("A2", 0.4),
        ("B2", 0.2), ("K2", 0.2), ("L2", 0.4), ("A1", 0.2), ("L1", 0.2),
    ];

    // Generate match arms - pre-compute scalar * bonus at compile time
    let black_arms: Vec<_> = black_scalars.iter().map(|(pos, scalar)| {
        let position_ident = syn::Ident::new(pos, Span::call_site());
        let value = transform_scalar(*scalar) * bonus;
        quote! {
            (hexchess::Position::#position_ident, hexchess::Color::Black) => #value,
        }
    }).collect();

    let white_arms: Vec<_> = white_scalars.iter().map(|(pos, scalar)| {
        let position_ident = syn::Ident::new(pos, Span::call_site());
        let value = transform_scalar(*scalar) * bonus;
        quote! {
            (hexchess::Position::#position_ident, hexchess::Color::White) => #value,
        }
    }).collect();

    // Generate the complete match statement with pre-computed values
    let output = quote! {
        match (#position_expr, #color_expr) {
            #(#black_arms)*
            #(#white_arms)*
            _ => 0.0,
        }
    };

    output.into()
}

/// this transforms the scalar value to give non-linear weight towards pawn
/// advancement. put simply, a pawn that is one more away from promotion is
/// worth significantly more than a pawn that is two away. the value of each
/// step becomes increasingly valuable.
fn transform_scalar(n: f32) -> f32 {
    n * n // <- ease in quad
}
