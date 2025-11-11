pub struct EvalOptions {
    pub bishop_value: f32,
    pub king_value: f32,
    pub knight_value: f32,
    pub pawn_value: f32,
    pub queen_value: f32,
    pub rook_value: f32,

    pub check_value: f32,
    pub checkmate_value: f32,
    pub stalemate_value: f32,
}

impl Default for EvalOptions {
    fn default() -> Self {
        Self {
            bishop_value: 30.0,
            king_value: 0.0,
            knight_value: 30.0,
            pawn_value: 10.0,
            queen_value: 90.0,
            rook_value: 50.0,

            check_value: 1.0,
            checkmate_value: f32::INFINITY,
            stalemate_value: 2.0,
        }
    }
}
