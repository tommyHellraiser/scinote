use rust_decimal::Decimal;
use crate::SciNote;

#[test]
fn shift_right_1() {
    let input = Decimal::from(5000);
    let (output, shift) = SciNote::scale_to_one_integer_digit(input).unwrap();

    assert_eq!(output, Decimal::from(5));
    assert_eq!(shift, 3);
}
#[test]
fn shift_right_2(){
    let input = Decimal::from(1000000000);
    let (output, shift) = SciNote::scale_to_one_integer_digit(input).unwrap();

    assert_eq!(output, Decimal::from(1));
    assert_eq!(shift, 9);
}
#[test]
fn shift_right_3(){
    let input = Decimal::from(10);
    let (output, shift) = SciNote::scale_to_one_integer_digit(input).unwrap();

    assert_eq!(output, Decimal::from(1));
    assert_eq!(shift, 1);
}
#[test]
fn no_shift() {
    let input = Decimal::from(9);
    let (output, shift) = SciNote::scale_to_one_integer_digit(input).unwrap();

    assert_eq!(output, Decimal::from(9));
    assert_eq!(shift, 0);
}
#[test]
fn shift_left_1() {
    let input = Decimal::new(3, 7); // 0.0000003
    let (output, shift) = SciNote::scale_to_one_integer_digit(input).unwrap();

    assert_eq!(output, Decimal::from(3));
    assert_eq!(shift, -7);
}
#[test]
fn shift_left_2() {
    let input = Decimal::new(3, 26);
    let (output, shift) = SciNote::scale_to_one_integer_digit(input).unwrap();

    assert_eq!(output, Decimal::from(3));
    assert_eq!(shift, -26);
}
#[test]
fn shift_left_3() {
    let input = Decimal::new(3, 1);
    let (output, shift) = SciNote::scale_to_one_integer_digit(input).unwrap();

    assert_eq!(output, Decimal::from(3));
    assert_eq!(shift, -1);
}