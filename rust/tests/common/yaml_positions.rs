use crate::yaml;
use hexchess::Position;

#[test]
fn test_positions() {
    let tests = yaml::<String>("positions.yaml");

    for i in 0..91 {
        let p = Position::from_index(i);

        assert_eq!(p as u8, i);
        assert_eq!(p.to_string(), tests[i as usize]);
    }
}
