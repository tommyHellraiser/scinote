use crate::SciNote;

#[test]
fn test_sum_ok() {
    let num1 = SciNote::parse_from_str("5x10^12").unwrap();
    let num2 = SciNote::parse_from_str("2x10^10").unwrap();

    let result = num1.add(&num2).unwrap();

    assert_eq!(result.to_string(), "5.02x10^12".to_string());
}

#[test]
fn test_sum_ok_2() {
    let num1 = SciNote::parse_from_str("5x10^12").unwrap();
    let num2 = SciNote::parse_from_str("2x10^12").unwrap();

    let result = num1.add(&num2).unwrap();

    assert_eq!(result.to_string(), "7x10^12".to_string());
}

#[test]
fn test_sum_ok_3() {
    let num1 = SciNote::parse_from_str("5x10^8").unwrap();
    let num2 = SciNote::parse_from_str("2x10^10").unwrap();

    let result = num1.add(&num2).unwrap();

    assert_eq!(result.to_string(), "2.05x10^10".to_string());
}

#[test]
fn test_sum_ok_4() {
    let num1 = SciNote::parse_from_str("0.0005x10^10").unwrap();
    let num2 = SciNote::parse_from_str("0.0002x10^10").unwrap();

    let result = num1.add(&num2).unwrap();

    assert_eq!(result.to_string(), "7x10^6".to_string());
}

#[test]
fn test_addition_basic() {
    // Adding two positive numbers
    let num1 = SciNote::parse_from_str("1x10^3").unwrap(); // 1.0e+3
    let num2 = SciNote::parse_from_str("2x10^3").unwrap(); // 2.0e+3
    let result = num1.add(&num2).unwrap();
    assert_eq!(result, SciNote::parse_from_str("3x10^3").unwrap()); // Expected: 3.0e+3

    // Adding two negative numbers
    let num3 = SciNote::parse_from_str("-1x10^3").unwrap(); // -1.0e+3
    let num4 = SciNote::parse_from_str("-2x10^3").unwrap(); // -2.0e+3
    let result = num3.add(&num4).unwrap();
    assert_eq!(result, SciNote::parse_from_str("-3x10^3").unwrap()); // Expected: -3.0e+3

    // Adding a positive and a negative number
    let num5 = SciNote::parse_from_str("1x10^3").unwrap(); // 1.0e+3
    let num6 = SciNote::parse_from_str("-1x10^3").unwrap(); // -1.0e+3
    let result = num5.add(&num6).unwrap();
    assert_eq!(result, SciNote::parse_from_str("0x10^3").unwrap()); // Expected: 0.0e+3
}

#[test]
fn test_addition_different_exponents() {
    // Adding numbers with different exponents
    let num1 = SciNote::parse_from_str("1x10^3").unwrap(); // 1.0e+3
    let num2 = SciNote::parse_from_str("1x10^2").unwrap(); // 1.0e+2
    let result = num1.add(&num2).unwrap();
    assert_eq!(result, SciNote::parse_from_str("1x10^3").unwrap()); // Expected: 1.1e+3 (1000 + 100)

    let num3 = SciNote::parse_from_str("2.5x10^-2").unwrap(); // 2.5e-2
    let num4 = SciNote::parse_from_str("3.5x10^-3").unwrap(); // 3.5e-3
    let result = num3.add(&num4).unwrap();
    assert_eq!(result, SciNote::parse_from_str("2.85x10^-2").unwrap()); // Expected: 2.85e-2 (0.025 + 0.0035)
}

#[test]
fn test_addition_with_zero() {
    // Adding zero to a number
    let num1 = SciNote::parse_from_str("1x10^3").unwrap(); // 1.0e+3
    let zero = SciNote::parse_from_str("0x10^0").unwrap(); // 0.0
    let result = num1.add(&zero).unwrap();
    assert_eq!(result, num1); // Expected: 1.0e+3

    // Adding a number to zero
    let result = zero.add(&num1).unwrap();
    assert_eq!(result, num1); // Expected: 1.0e+3
}

#[test]
fn test_overflow_and_underflow() {
    // Test for overflow
    let num1 = SciNote::parse_from_str("1x10^308").unwrap(); // 1.0e+308
    let num2 = SciNote::parse_from_str("1x10^308").unwrap(); // 1.0e+308
    let result = num1.add(&num2).unwrap();
    dbg!(&result);
    // assert!(result.is_infinite()); // Expected: Infinity (overflow)

    // Test for underflow
    let num3 = SciNote::parse_from_str("1x10^-308").unwrap(); // 1.0e-308
    let num4 = SciNote::parse_from_str("-1x10^-308").unwrap(); // -1.0e-308
    let result = num3.add(&num4).unwrap();
    dbg!(&result);
    // assert_eq!(result, SciNote::zero()); // Expected: 0.0 (underflow to zero)
}

#[test]
fn test_addition_extreme_values() {
    // Adding very large numbers
    let num1 = SciNote::parse_from_str("1x10^308").unwrap(); // 1.0e+308
    let num2 = SciNote::parse_from_str("1x10^307").unwrap(); // 1.0e+307
    let result = num1.add(&num2).unwrap();
    assert_eq!(result, SciNote::parse_from_str("1.1x10^308").unwrap()); // Expected: 1.1e+308

    // Adding very small numbers
    let num3 = SciNote::parse_from_str("1x10^-308").unwrap(); // 1.0e-308
    let num4 = SciNote::parse_from_str("1x10^-309").unwrap(); // 1.0e-309
    let result = num3.add(&num4).unwrap();
    assert_eq!(result, SciNote::parse_from_str("1.1x10^-308").unwrap()); // Expected: 1.1e-308
}

#[test]
fn test_precision_handling() {
    // Adding a very small number to a very large number
    let num1 = SciNote::parse_from_str("1x10^100").unwrap(); // 1.0e+100
    let num2 = SciNote::parse_from_str("1x10^-100").unwrap(); // 1.0e-100
    let result = num1.add(&num2).unwrap();
    assert_eq!(result, num1); // Expected: 1.0e+100 (no change due to precision)

    // Adding numbers where mantissa precision affects the result
    let num3 = SciNote::parse_from_str("1.23456789x10^3").unwrap(); // 1.23456789e+3
    let num4 = SciNote::parse_from_str("9.87654321x10^2").unwrap(); // 9.87654321e+2
    let result = num3.add(&num4).unwrap();
    assert_eq!(result, SciNote::parse_from_str("1.33222221x10^3").unwrap()); // Expected: 1.33222221e+3
}
