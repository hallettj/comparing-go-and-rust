pub fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

#[test]
fn divides_a_number() {
    let result = checked_division(12, 4);
    match result {
        Some(num) => assert_eq!(3, num),
        None      => assert!(false, "Expected `Some` but got `None`")
    };
}
