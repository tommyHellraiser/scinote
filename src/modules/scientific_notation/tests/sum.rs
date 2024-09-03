use crate::modules::scientific_notation::ScientificNotation;

#[test]
fn test_sum_ok() {
    let mut num1 = ScientificNotation::parse_from_str("5x10^12").unwrap();
    let num2 = ScientificNotation::parse_from_str("2x10^10").unwrap();

    let result = num1.sum(num2).unwrap();

    assert_eq!(result.to_string(), "5.02x10^12".to_string());
}

#[test]
fn test_sum_ok_2() {
    let mut num1 = ScientificNotation::parse_from_str("5x10^12").unwrap();
    let num2 = ScientificNotation::parse_from_str("2x10^12").unwrap();

    let result = num1.sum(num2).unwrap();

    assert_eq!(result.to_string(), "7x10^12".to_string());
}

#[test]
fn test_sum_ok_3() {
    let mut num1 = ScientificNotation::parse_from_str("5x10^8").unwrap();
    let num2 = ScientificNotation::parse_from_str("2x10^10").unwrap();

    let result = num1.sum(num2).unwrap();

    assert_eq!(result.to_string(), "2.05x10^10".to_string());
}

#[test]
fn test_sum_ok_4() {
    let mut num1 = ScientificNotation::parse_from_str("0.0005x10^10").unwrap();
    let num2 = ScientificNotation::parse_from_str("0.0002x10^10").unwrap();

    let result = num1.sum(num2).unwrap();

    assert_eq!(result.to_string(), "7x10^6".to_string());
}
