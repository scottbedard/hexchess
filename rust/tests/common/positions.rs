use hexchess::hexchess::position::Position;

use crate::json;

#[test]
#[ignore]
fn test_positions() {
    let tests = json::<String>("positions.json");

    for i in 0..91 {
        let p = Position::from_fen_index(i);

        assert_eq!(p.to_fen_index(), i);
        assert_eq!(p.to_string(), tests[i as usize]);
    }
}
