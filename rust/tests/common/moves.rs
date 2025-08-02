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
fn test_piece_movement() {
    let files = [
        // "moves-from.json",
        // "moves-king.json",
        "moves-knight.json",
        // "moves-pawn.json",
        // "moves-straight-line.json",
    ];

    for file in files {
        let tests = json::<Test>(file);

        for test in tests {
            let from = Position::from_string(&test.position);

            let mut actual = Game::parse(&test.from)
                .unwrap()
                .get_moves_unsafe(from)
                .into_iter()
                .map(|san| san.to_string())
                .collect::<Vec<String>>();

            actual.sort();

            let mut expected = test.expect.clone();

            expected.sort();

            assert_eq!(actual, expected, "{}", test.description);

            println!("test {} - {} ... ok", file, test.description);
        }
    }
}
