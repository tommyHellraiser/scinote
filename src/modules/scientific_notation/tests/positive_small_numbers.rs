use rust_decimal::Decimal;
use crate::modules::scientific_notation::sci_not::SciNote;

// ---- POSITIVE SMALL NUMBERS ----
#[test]
fn positive_small_parse_scientific_notation_from_string() {
    let input = "5x10^-15";
    let sci_notation = SciNote::parse_from_str(input).unwrap();

    assert_eq!(sci_notation.coefficient, Decimal::from(5));
    assert_eq!(sci_notation.exponent, -15i16);
    assert_eq!(sci_notation.display_decimals, None);
}

#[test]
fn positive_small_parse_scientific_notation_from_string_2() {
    let input = "5.1237514651x10^-38";
    let sci_notation = SciNote::parse_from_str(input).unwrap();

    assert_eq!(sci_notation.coefficient, Decimal::new(51237514651, 10));
    assert_eq!(sci_notation.exponent, -38i16);
    assert_eq!(sci_notation.display_decimals, None);
}

#[test]
fn positive_small_parse_with_error_invalid_exponent() {
    let input = "2x10^-23.4";
    let sci_notation = SciNote::parse_from_str(input);

    assert!(sci_notation.is_err());
}

#[test]
fn positive_small_parse_with_error_invalid_coefficient() {
    let input = "2ax10^-20";
    let sci_notation = SciNote::parse_from_str(input);

    assert!(sci_notation.is_err());
}

#[test]
fn positive_small_parse_with_error_x10_repeated() {
    let input = "2x10^x10^-23";
    let sci_notation = SciNote::parse_from_str(input);

    assert!(sci_notation.is_err());
}

#[test]
fn positive_small_parse_with_error_invalid_input() {
    let input = "asd1234";
    let sci_notation = SciNote::parse_from_str(input);

    assert!(sci_notation.is_err());
}

#[test]
fn positive_small_parse_and_display_back() {
    let input = "315.2x10^-14";
    let sci_notation = SciNote::parse_from_str(input).unwrap();

    assert_eq!(sci_notation.to_string(), input.to_string());
}

#[test]
fn positive_small_parse_i64_max_mantissa() {
    let input = "9.223372036854775807x10^-5";
    let sci_notation = SciNote::parse_from_str(input);

    assert!(sci_notation.is_ok());
    let result = sci_notation.unwrap();
    assert_eq!(result.coefficient, Decimal::new(9223372036854775807, 18));
    assert_eq!(result.exponent, -5i16);
}

#[test]
fn positive_small_from_builder_into_string() {
    let sci_notation = SciNote::build()
        .coefficient(Decimal::new(6278964, 5))
        .exponent(-20i16)
        .display_decimals(3);

    assert_eq!(sci_notation.to_string(), "62.790x10^-20".to_string());
}

#[test]
fn positive_small_parse_min_exponent(){
    let max_exponent = i16::MIN.to_string();
    let input = format!("15.321x10^{}", max_exponent);
    let sci_notation = SciNote::parse_from_str(&input);

    assert!(sci_notation.is_ok());
    let result = sci_notation.unwrap();
    assert_eq!(result.coefficient, Decimal::new(15321, 3));
    assert_eq!(result.exponent, i16::MIN);
}

