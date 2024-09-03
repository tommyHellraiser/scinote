use rust_decimal::Error;

pub type SciResult<R> = Result<R, SciError>;

#[derive(Debug)]
pub enum SciError {
    CoefficientError(Option<String>),
    ConversionError(Option<String>),
    ExponentError(Option<String>),
    FormatError(Option<String>),
    GenericError(Option<String>),
    OperationError(Option<String>),
    OutOfRangeError(Option<String>),
    ParseError(Option<String>),
    PrecisionError(Option<String>),
    ScaleError(Option<String>),
}

impl From<Error> for SciError {
    fn from(value: Error) -> Self {
        match value {
            Error::ErrorString(_) => Self::GenericError(Some(value.to_string())),
            Error::ExceedsMaximumPossibleValue => Self::OutOfRangeError(Some(value.to_string())),
            Error::LessThanMinimumPossibleValue => Self::OutOfRangeError(Some(value.to_string())),
            Error::Underflow => Self::PrecisionError(Some(value.to_string())),
            Error::ScaleExceedsMaximumPrecision(_) => Self::ScaleError(Some(value.to_string())),
            Error::ConversionTo(_) => Self::ConversionError(Some(value.to_string()))
        }
    }
}

