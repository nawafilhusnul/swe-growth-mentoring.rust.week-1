use std::io;

pub fn get_user_input() -> String {
    let mut user_input_string = String::new();

    println!("Please enter a string: ");
    io::stdin()
        .read_line(&mut user_input_string)
        .expect("failed to get user input");

    println!("Your input string is: {user_input_string}");

    return user_input_string.trim().to_string();
}
