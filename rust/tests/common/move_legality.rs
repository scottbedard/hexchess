use crate::json;
use hexchess::{Hexchess, San};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: String,
    result: bool,
    san: String,
}

#[test]
#[ignore]
fn test_move_legality() {
    let tests = json::<Test>("move-legality.json");

    for test in tests {
        let hexchess = Hexchess::parse(&test.from).unwrap();
        let san = San::from_string(&test.san);

        // if san.is_err() {
        //     assert_eq!(test.result, false, "{}", test.description);
        //     continue;
        // }

        let result = hexchess.is_legal(&san);

        assert_eq!(result, test.result, "{}", test.description);
    }
}
