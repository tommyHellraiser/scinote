
use std::fmt::{Display, Formatter};
use rust_decimal::{Decimal, RoundingStrategy};
use crate::modules::scientific_notation::error::{SciError, SciResult};

#[derive(Debug, Clone, Copy)]
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
pub struct ScientificNotation {
    pub(crate) coefficient: Decimal,
    pub(crate) exponent: i16,
    pub(crate) display_decimals: usize
}

impl ScientificNotation {
    pub fn build() -> Self {
        ScientificNotation::default()
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
        self.display_decimals = decimals;
        self
    }

    pub fn parse_from_str(input: &str) -> SciResult<Self> {

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

        let display_decimals_unparsed = coefficient.fract();
        let display_decimals = if display_decimals_unparsed == Decimal::ZERO {
            0
        } else {
            display_decimals_unparsed
                .to_string()
                .split('.')
                .collect::<Vec<&str>>()[1]
                .len()
        };

        Ok(Self {
            coefficient,
            exponent,
            display_decimals
        })
    }

    pub fn sum(&mut self, other: Self) -> SciResult<Self> {
        //  First, equalize exponents
        let exponent_diff = self.exponent - other.exponent;
        self.exponent -= exponent_diff;

        //  Second, adapt coefficient to that exponent equalization
        for _ in 0..exponent_diff.abs() {
            //  If exponent difference is negative, then we need to make the number smaller by
            // multiplicating by 0.1
            let exponent_value = if exponent_diff.is_positive() {
                Decimal::from(10)
            } else {
                Decimal::new(1, 1)
            };
            self.coefficient *= exponent_value;
        }

        //  Third, with exponents equalized, sum coefficients
        let mut coefficient_sum_result = self.coefficient + other.coefficient;

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
        let new_exponent = self.exponent + scale_shift_amount;
        let new_display_decimals = if self.coefficient.mantissa().to_string().len() > 3 {
            //  If mantissa is too long, round to a default of 3 decimals
            3
        } else {
            coefficient_sum_result.mantissa().to_string().len() - 1
        };

        Ok(Self {
            coefficient: coefficient_sum_result,
            exponent: new_exponent,
            display_decimals: new_display_decimals
        })
    }
}

impl Default for ScientificNotation {
    fn default() -> Self {
        Self {
            coefficient: Decimal::from(1),
            exponent: i16::default(),
            display_decimals: 2
        }
    }
}

impl Display for ScientificNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x10^{}",
            self.coefficient.round_dp_with_strategy(
                self.display_decimals as u32, RoundingStrategy::MidpointTowardZero
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