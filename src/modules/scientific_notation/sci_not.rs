use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use rust_decimal::{Decimal, RoundingStrategy};
use crate::modules::scientific_notation::error::{SciError, SciResult};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
/// Maximum parsing allowed as coefficient mantissa: i64, meaning, on creation, up to i64::MAX and i64::MIN
/// is allowed.
/// ## Examples:
/// - Valid positive number
/// 9.223372036854775807x10^5 -> The mantissa is equivalent to i64::MAX, hence, it can be parsed
/// - Valid negative number
/// -9.223372036854775807x10^5 -> The mantissa is equivalent to i64::MIN, it'll be parsed as well
/// - Invalid number
/// 9.2233720368547758070x10^5 -> The mantissa contains a value 10 times higher than i64::MAX,
/// conversion will fail
pub struct SciNote {
    pub(crate) coefficient: Decimal,
    pub(crate) exponent: i16,
    pub(crate) display_decimals: Option<usize>
}

impl SciNote {
    pub fn zero() -> Self {
        Self {
            coefficient: Decimal::ZERO,
            exponent: 0,
            display_decimals: Some(0)
        }
    }
    pub fn build() -> Self {
        SciNote::default()
    }

    pub fn coefficient(mut self, coefficient: Decimal) -> Self {
        self.coefficient = coefficient;
        self
    }

    pub fn exponent(mut self, exponent: i16) -> Self {
        self.exponent = exponent;
        self
    }

    pub fn display_decimals(mut self, decimals: usize) -> Self {
        self.display_decimals = Some(decimals);
        self
    }

    pub fn parse_from_str(input: &str) -> SciResult<Self> {

        //  TODO check parsing from zero, if user inputs "0", then coefficient = 0, and exponent = 0

        let input = input.to_string();

        //  If notation x10^ is not found, return error, it's not valid scientific notation
        if !input.contains("x10^") {
            return Err(SciError::FormatError(Some("Input format was incorrect. Expected CCcx10^EEe".to_string())))
        }

        let full_notation = input.split("x10^").collect::<Vec<&str>>();
        if full_notation.len() > 2 {
            return Err(SciError::FormatError(Some("Unexpected format!".to_string())))
        }

        let coefficient = full_notation[0]
            .parse::<Decimal>()?;

        let exponent = full_notation[1]
            .parse::<i16>()
            .map_err(|error| SciError::ParseError(Some(error.to_string())))?;

        Ok(Self {
            coefficient,
            exponent,
            display_decimals: None
        })
    }

    pub fn add(&self, input2: &Self) -> SciResult<Self> {
        let mut input1 = *self;
        //  First, equalize exponents
        let exponent_diff = input1.exponent - input2.exponent;
        input1.exponent -= exponent_diff;

        //  Second, adapt coefficient to that exponent equalization
        for _ in 0..exponent_diff.abs() {
            //  If exponent difference is negative, then we need to make the number smaller by
            // multiplicating by 0.1
            let exponent_value = if exponent_diff.is_positive() {
                Decimal::from(10)
            } else {
                Decimal::new(1, 1)
            };
            input1.coefficient *= exponent_value;
        }

        //  Third, with exponents equalized, sum coefficients
        let mut coefficient_sum_result = input1.coefficient + input2.coefficient;

        //  Scale to always be 1 digit integer. If there's more than one digit, shift right, if there's less, shift left
        //  TODO implement scale function here


        //  Obtaining the mantissa length to convert into a single int, with the needed decimal places
        let old_mantissa = coefficient_sum_result.mantissa();
        let old_scale = coefficient_sum_result.scale();

        //  Rescale the coefficient
        let scale_shift_amount = if old_scale as usize > old_mantissa.to_string().len() {
            //  In this case the shift must be negative, exponent decreases
            // let shift = (old_scale - old_mantissa.to_string().len() as u32) as i16;
            let shift = -(old_scale as i16);
            // coefficient_sum_result.set_scale(shift as u32)
            coefficient_sum_result.set_scale(old_mantissa.to_string().len() as u32 - 1)?;
            shift
        } else {
            //  Shift is positive, exponent increases
            let shift = (old_mantissa.to_string().len() as u32 - old_scale - 1) as i16;
            coefficient_sum_result.set_scale(old_mantissa.to_string().len() as u32 - 1)?;
            shift
        };

        //  Adapt the exponent to the new scale
        let new_exponent = input1.exponent + scale_shift_amount;

        let display_decimals = if let Some(decimals) = input1.display_decimals {
            Some(decimals)
        } else if let Some(decimals) = input2.display_decimals {
            Some(decimals)
        } else {
            None
        };

        Ok(Self {
            coefficient: coefficient_sum_result,
            exponent: new_exponent,
            display_decimals
        })
    }

    /// Shift either left or right the coefficient to have always one digit as int part
    ///
    /// Return (output_decimal_number, places_shifted)
    ///
    /// A positive shift number means the exponent will grow, and a negative, it'll shrink
    pub(super) fn scale_to_one_integer_digit(input_coefficient: Decimal) -> SciResult<(Decimal, i16)> {

        //  Initialize parser variables
        let input_string = input_coefficient.to_string();

        let integer_string;
        let decimal_string;
        if !input_string.contains('.') {
            //  If there's no decimal point, then the number is an integer
            integer_string = input_string.as_str();
            decimal_string = "";
        } else {
            //  Otherwise, it's decimal
            let parsed = input_string.split('.').collect::<Vec<&str>>();
            integer_string = parsed[0];
            decimal_string = parsed[1];
        }

        //  Parse into Decimal to validate their lengths, and determine if they contain leading zeroes
        let integer_parsed = integer_string.parse::<Decimal>()?;
        let decimal_parsed = decimal_string.parse::<Decimal>().unwrap_or(Decimal::ZERO);

        //  Evaluate if we need to shift right
        match (integer_string.len().cmp(&integer_parsed.to_string().len()), integer_string.len().cmp(&1usize)) {
            (Ordering::Equal, Ordering::Greater) => {
                //  Example case: 2134.xxxx
                //  No leading zeros and need to adjust right, exponent will increase and coefficient will decrease
                let right_shift = integer_string.len() as i16 - 1;
                let new_scale = right_shift as u32 - decimal_string.len() as u32;
                
                let mut output_coefficient = input_coefficient;
                output_coefficient.set_scale(new_scale)?;

                return Ok((output_coefficient, right_shift))
            },
            (Ordering::Equal, Ordering::Equal) => {
                //  Example case: 2.xxxx or 0.xxxx
                //  Don't need to shift in the first case, return as it is with shift = 0
                if integer_parsed != Decimal::ZERO {
                    dbg!("Returned here");
                    return Ok((input_coefficient, 0))
                }
                //  Otherwise, continue and evaluate if we need to shift left
            },
            (Ordering::Less, _) => {
                //  String integer portion cannot be smaller than the Decimal parsed integer portion,
                // it'd be the inverse case of leading zeros
                return Err(SciError::Unexpected("Unexpected parse error".to_string()))
            },
            (_, Ordering::Less) => {
                //  Integer portion in string format cannot have a length smaller than 1, it'd be
                // non-existent. Ex: .xxxx, the zero is missing
                return Err(SciError::Unexpected("Invalid parsed integer number".to_string()))
            }
            (Ordering::Greater, _) => {
                //  Continue and evaluate shift left
            }
        }

        //  If decimal portion is zero, then the number is an integer. If we got to this point
        // without any shifts, there's something wrong, it's best to just return and have a valid number
        if decimal_parsed.is_zero() {
            dbg!("Returned here");
            return Ok((input_coefficient, 0))
        }

        //  Evaluate if we need to shift left
        match decimal_string.len().cmp(&decimal_parsed.to_string().len()) {
            Ordering::Equal | Ordering::Greater => {
                //  We need to shift left, determine the amount of places to shift
                //  Example case: x.000123 -> Shift left by 4 places
                //  Other example: x.1 -> Shift left by 1 space
                let left_shift = decimal_string.len() - decimal_parsed.to_string().len() + 1;
                let new_scale = decimal_parsed.to_string().len() as u32 - 1;
                
                let mut output_coefficient = input_coefficient;
                output_coefficient.set_scale(new_scale)?;

                Ok((output_coefficient, -(left_shift as i16)))
            },
            Ordering::Less => {
                //  In this case, the decimal portion in string format was shorter than the decimal
                // parsed one. Error that should not happen
                Err(SciError::Unexpected("Unexpected parse error".to_string()))
            }
        }
    }
}

impl Default for SciNote {
    fn default() -> Self {
        Self {
            coefficient: Decimal::from(1),
            exponent: i16::default(),
            display_decimals: None
        }
    }
}

impl Display for SciNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x10^{}",
            self.coefficient.round_dp_with_strategy(
                self.display_decimals.unwrap_or(2) as u32, RoundingStrategy::MidpointTowardZero
            ),
            self.exponent
        )
    }
}

/*
impl std::ops::Add for ScientificNotation {
    type Output = ScientificNotation;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Sub for ScientificNotation {
    type Output = ScientificNotation;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Mul for ScientificNotation{
    type Output = ScientificNotation;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Div for ScientificNotation {
    type Output = ScientificNotation;
    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
*/