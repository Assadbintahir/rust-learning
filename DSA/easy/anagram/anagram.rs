use std::collections::HashMap;

fn valid_anagram(first_string: &str, second_string: &str) -> bool {
    if first_string.len() != second_string.len() {
        return false;
    }

    let mut first_hash_map: HashMap<char, u8> = HashMap::new();
    let mut second_hash_map: HashMap<char, u8> = HashMap::new();

    for value in first_string.chars() {
        if first_hash_map.contains_key(&value) {
            first_hash_map.insert(value, first_hash_map.get(&value).unwrap() + 1);
        } else {
            first_hash_map.insert(value, 1);
        }
    }

    for value in second_string.chars() {
        if second_hash_map.contains_key(&value) {
            second_hash_map.insert(value, second_hash_map.get(&value).unwrap() + 1);
        } else {
            second_hash_map.insert(value, 1);
        }
    }

    for (key, value) in first_hash_map.iter() {
        if Some(value) != second_hash_map.get(&key) {
            return false;
        }
    }

    true
}

fn main() {
    assert_eq!(valid_anagram("rat", "car"), false);
    assert_eq!(valid_anagram("anagram", "nagaram"), true);

    println!("All test cases passed!");
}

