pub fn is_odd(n: i32) -> bool {
    n % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_odd(2);
        assert_eq!(result, false);
    }
}
