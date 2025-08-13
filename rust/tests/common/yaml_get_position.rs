use crate::yaml;
use hexchess::hexchess::hexchess::Hexchess;
use hexchess::hexchess::position::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Test {
    description: String,
    expected: Option<String>,
    hexchess: String,
    position: String,
}

#[test]
fn test_get_position() {
    let tests = yaml::<Test>("get-position.yaml");

    for test in tests {
        let game = Hexchess::parse(&test.hexchess).unwrap();

        let position = match Position::from_string(&test.position) {
            Ok(p) => p,
            Err(_) => {
                assert!(test.expected.is_none(), "{}", test.description);
                continue;
            },
        };

        if test.expected.is_some() {
            let actual = game.get_position(position).unwrap().to_string();
    
            assert_eq!(actual, test.expected.unwrap(), "{}", test.description);
        } else {
            assert!(test.expected.is_none(), "{}", test.description);
        }
    }
}
