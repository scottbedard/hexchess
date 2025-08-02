use crate::json;
use hexchess::hexchess::color::Color;
use hexchess::hexchess::hexchess::Hexchess;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    color: Color,
    description: String,
    from: String,
    result: Vec<String>,
}

#[test]
#[ignore]
fn test_get_color() {
    let tests = json::<Test>("get-color.json");

    for test in tests {
        let hexchess = Hexchess::parse(&test.from).unwrap();

        let result = hexchess.get_color(test.color)
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>();

        assert_eq!(result, test.result);
    }
}
