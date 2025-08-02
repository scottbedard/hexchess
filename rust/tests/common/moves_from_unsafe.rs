use crate::json;
use hexchess::Hexchess;
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
#[ignore]
fn test_moves_from_unsafe() {
    let files = [
        "moves-from-unsafe.json",
    ];

    for file in files {
        let tests = json::<Test>(file);

        for test in tests {
            let from = Position::from_string(&test.from);

            let result = Hexchess::parse(&test.from)
                .unwrap()
                .moves_from_unsafe(from)
                .into_iter()
                .map(|san| san.to_string())
                .collect::<Vec<String>>();

            assert_eq!(result, test.expect, "{}", test.description);
        }
    }
}
