use crate::json;
use hexchess::Hexchess;
use hexchess::hexchess::utils::index;
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
        "moves-from.json",
        "moves-king.json",
        "moves-knight.json",
        "moves-pawn.json",
        "moves-straight-line.json",
    ];

    for file in files {
        let tests = json::<Test>(file);

        for test in tests {
            let from = index(&test.position).unwrap();

            let mut result = Hexchess::parse(&test.from)
                .unwrap()
                .moves_from(from)
                .into_iter()
                .map(|san| san.to_string())
                .collect::<Vec<String>>();
            
            result.sort();

            let mut expected = test.expect.clone();

            expected.sort();

            assert_eq!(result, expected, "{}", test.description);
        }
    }
}
