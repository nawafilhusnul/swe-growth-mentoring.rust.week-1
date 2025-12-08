use std::collections::HashMap;

use palindrome_number_utils::is_odd;

pub fn longest_palindrome(s: String) -> i32 {
    let mut hash_map: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        let count = hash_map.entry(c).or_insert(0);
        *count += 1
    }

    let mut total_count: i32 = 0;

    for (_, value) in hash_map.into_iter() {
        let mut added = value;
        if is_odd(value) && is_odd(total_count) {
            added -= 1;
        }

        total_count += added;
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = longest_palindrome(String::from("wasitacaroracatisaw"));
        assert_eq!(result, 19);
    }
}
