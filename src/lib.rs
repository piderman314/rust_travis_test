/// Adder
/// 
/// # Examples
/// 
/// ```
/// 
/// assert_eq!(5, travis_test::add(2, 3))
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, add(2, 3));
    }
}
