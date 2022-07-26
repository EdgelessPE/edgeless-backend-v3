use std::cmp::Ordering;

use crate::utils;

#[test]
pub fn test_version_cmp() {
    let test_array = vec![
        ("2.3.3", "2.2.4", Ordering::Greater),
        ("1.1.0.0", "1.1", Ordering::Equal),
        ("0", "5", Ordering::Less),
    ];
    for node in test_array {
        let s1: Vec<u32> = node
            .0
            .split(".")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let s2: Vec<u32> = node
            .1
            .split(".")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        assert_eq!(utils::version_cmp(&s1, &s2), node.2);
    }
}
