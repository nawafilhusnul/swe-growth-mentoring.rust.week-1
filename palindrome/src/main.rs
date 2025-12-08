/*
    References: https://leetcode.com/problems/longest-palindrome/
*/

use palindrome_string_utils::longest_palindrome;
use palindrome_user_input::get_user_input;

fn main() {
    let user_input = get_user_input();

    let longest = longest_palindrome(user_input);

    println!("The length of longest palindrome is {longest}")
}
