use crate::json;
use hexchess::{position, San};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    description: String,
    error: bool,
    san: String,
    expect: Option<SanStruct>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SanStruct {
    from: String,
    promotion: Option<String>,
    to: String,
}

#[test]
fn test_san_parse() {
    let tests = json::<Test>("san-parse.json");

    for test in tests {
        let result = match San::from(&test.san) {
            Ok(san) => san,
            Err(_) => {
                assert!(test.error, "{}", test.description);
                continue;
            }
        };

        if test.expect.is_some() {
            let expect = test.expect.unwrap();
            assert_eq!(expect.from, position(&result.from));
            assert_eq!(expect.promotion, match result.promotion {
                Some(p) => Some(p.to_string()),
                None => None,
            });
            assert_eq!(expect.to, position(&result.to));
        }
    }
}
