pub fn sub_numbers(a: i32, b: i32) -> i32 {
    return a - b;
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_sub () {
        assert_eq!(5, sub_numbers(15, 10));
    }
}