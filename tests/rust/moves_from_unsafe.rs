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
fn test_moves_from_unsafe() {
    let files = [
        "moves-from-unsafe.json",
    ];

    for file in files {
        let tests = json::<Test>(file);

        for test in tests {
            let from = index(&test.from).unwrap();

            let result = Hexchess::parse(&test.hexchess)
                .unwrap()
                .moves_from_unsafe(from)
                .into_iter()
                .map(|san| san.to_string())
                .collect::<Vec<String>>();

            assert_eq!(result, test.result, "{}", test.description);
        }
    }
}
