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

        let mut moves_strings: Vec<String> = hexchess
            .current_moves()
            .iter()
            .map(|san| san.to_string())
            .collect();
        
        moves_strings.sort();

        let mut expected_result = test.result.clone();

        expected_result.sort();
        
        assert_eq!(moves_strings, expected_result);
    }
}
