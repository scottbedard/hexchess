use crate::json;
use hexchess::Hexchess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: String,
    result: Vec<String>,
}

#[test]
fn test_hexchess_apply_move_unsafe() {
    let tests = json::<Test>("current-moves.json");

    for test in tests {
        let hexchess = Hexchess::parse(&test.from).unwrap();

        let moves = hexchess
            .current_moves()
            .iter()
            .map(|san| san.to_string())
            .collect::<Vec<String>>();

        assert_eq!(moves, test.result);
    }
}
