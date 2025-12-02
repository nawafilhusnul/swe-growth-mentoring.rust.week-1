/*
    References: https://leetcode.com/problems/longest-palindrome/
*/

mod utils;

use utils::string_utils::palindrome::longest_palindrome;

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
