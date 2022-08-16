use std::{cmp::Ordering, collections::HashMap};

use crate::{hash_service::HashService, scanner};

// #[test]
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
        assert_eq!(scanner::version_cmp(&s1, &s2), node.2);
    }
}

#[test]
pub fn test_scanner() {
    let mut map = HashMap::new();
    map.insert(
        String::from("7-Zip美化版_21.7.0.0_Horatio Shaw.7z1643440187"),
        String::from("114514"),
    );
    let hash_service = HashService::new(map);
    let mut scanner = scanner::Scanner::new(hash_service);
    let res = scanner.scan_packages(String::from("./test/packages"));
    println!("{:?}", res);
}
