use crate::json;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: String,
    result: Vec<String>,
}

#[test]
fn test_current_moves() {
    let tests = json::<Test>("current-moves.json");

    for test in tests {
        let hexchess = Game::parse(&test.from).unwrap();

        let current_moves: Vec<String> = hexchess
            .current_moves()
            .iter()
            .map(|san| san.to_string())
            .collect();

        let expected_sans = test.result.clone();
        
        for san in &current_moves {
            assert!(
                expected_sans.contains(san),
                "Invalid san {} found in test {}",
                san,
                test.description
            );
        }
    }
}
