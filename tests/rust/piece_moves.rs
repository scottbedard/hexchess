use crate::json;
use hexchess::Hexchess;
use hexchess::hexchess::utils::index;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: String,
    hexchess: String,
    result: Vec<String>,
}

#[test]
fn test_piece_movement() {
    let suites = [
        json::<Test>("piece-moves-king.json"),
        json::<Test>("piece-moves-knight.json"),
        json::<Test>("piece-moves-pawn.json"),
        json::<Test>("piece-moves-straight-line.json"),
    ];

    for suite in suites {
        for test in suite {
            let from = index(&test.from).unwrap();

            let result = Hexchess::parse(&test.hexchess)
                .unwrap()
                .moves_from(from)
                .into_iter()
                .map(|san| san.to_string())
                .collect::<Vec<String>>();

            assert_eq!(result, test.result, "{}", test.description);
        }
    }
}
