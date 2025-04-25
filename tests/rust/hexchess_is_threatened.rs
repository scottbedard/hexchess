use crate::json;
use hexchess::{Color, index, Hexchess, position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: String,
    position: String,
    expect: bool,
}

#[test]
fn test_hexchess_is_threatened() {
    let tests = json::<Test>("hexchess-is-threatened.json");

    for test in tests {
        let hexchess = match Hexchess::parse(&test.from) {
            Ok(hexchess) => hexchess,
            Err(e) => {
                panic!("{}", e);
            }
        };

        let i = match index(&test.position) {
            Ok(i) => i,
            Err(_) => continue,
        };

        assert_eq!(hexchess.is_threatened(i), test.expect, "{}", test.description);
    }
}
