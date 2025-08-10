use crate::json;
use hexchess::hexchess::game::Game;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: String,
    position: String,
    expect: Vec<String>,
}

#[test]
fn test_moves_from_unsafe() {
    let files = [
        "moves-from-unsafe.json",
    ];

    for file in files {
        let tests = json::<Test>(file);

        for test in tests {
            let position = Position::from_string(&test.position).unwrap();

            let result = Game::parse(&test.from)
                .unwrap()
                .get_moves_unsafe(position)
                .into_iter()
                .map(|san| san.to_string())
                .collect::<Vec<String>>();

            assert_eq!(result, test.expect, "{}", test.description);
        }
    }
}
