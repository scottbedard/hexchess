use crate::json;
use hexchess::{Color, Hexchess, San, position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    from: String,
    position: String,
    result: Option<String>,
}

#[test]
fn test_get() {
    let tests = json::<Test>("get.json");

    for test in tests {
        let hexchess = Hexchess::parse(&test.from).unwrap();

        let result = hexchess.get(&test.position);

        if result.is_some() {
            assert_eq!(result.unwrap().to_string(), test.result.unwrap(), "{}", test.description);
        } else {
            assert!(test.result.is_none(), "{}", test.description);
        }
    }
}
