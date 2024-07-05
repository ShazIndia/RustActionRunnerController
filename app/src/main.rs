fn main() {
    println!("Hello, world!");
}
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}
fn main() {

    divide_non_zero_result(10,5);

}


/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compilable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = doccomments::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #    try_main().unwrap(); // calling try_main and unwrapping
/// #                         // so that test will panic in case of error
/// # }
/// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}