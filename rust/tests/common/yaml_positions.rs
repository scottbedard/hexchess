use crate::yaml;
use hexchess::hexchess::position::Position;

#[test]
fn test_positions() {
    let tests = yaml::<String>("positions.yaml");

    for i in 0..91 {
        let p = Position::from_fen_index(i);

        assert_eq!(p.to_fen_index(), i);
        assert_eq!(p.to_string(), tests[i as usize]);
    }
}
