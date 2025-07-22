use hexchess::hexchess::utils::{index, position};
use crate::json;

#[test]
fn test_positions() {
    let tests = json::<String>("positions.json");

    for i in 0..91 {
        let name = position(&i);

        assert_eq!(index(name).unwrap(), i);
        assert_eq!(name, tests[i as usize]);
    }
}
