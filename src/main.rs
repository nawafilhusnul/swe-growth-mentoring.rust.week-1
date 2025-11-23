/*
    References: https://leetcode.com/problems/longest-palindrome/
*/ 
use std::collections::HashMap;

fn main() {
    println!("{}", longest_palindrome(String::from("abccccdd"))); // 7
    println!("{}", longest_palindrome(String::from("a"))); // 1
    println!("{}", longest_palindrome(String::from("aAbBABba"))); // 8
    println!(
        "{}",
        longest_palindrome(String::from("wasitacaroracatisaw"))
    ); // 19
    println!("{}", longest_palindrome(String::from("bananas"))); // 5
}

fn longest_palindrome(s: String) -> i32 {
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

fn is_odd(n: i32) -> bool {
    n % 2 == 1
}
