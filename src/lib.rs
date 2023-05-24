use std::fmt::{Debug, Display};

pub mod determinants;
pub mod matrices;

pub use crate::determinants::*;
pub use crate::matrices::*;

/// # Errors
/// * `InappropriateNumberOfItems` - Inappropriate number of items
/// * `TraceExistsOnlyForSquareMatrices` - Traces exists only for square matrices
/// * `IncorrectOrdersForOperation` - Incorret orders of matrices for algebric operations
/// * `IndexOutOfRange` - Index out of range
pub enum Errors {
    InappropriateNumberOfItems,
    TraceExistsOnlyForSquareMatrices,
    IncorrectOrdersForOperation,
    IndexOutOfRange,
}
impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Errors::InappropriateNumberOfItems => "Inappropriate number of items",
            Errors::TraceExistsOnlyForSquareMatrices => "Traces exists only for square matrices",
            Errors::IncorrectOrdersForOperation => {
                "Incorrect orders of matrices for algebric operations"
            }
            Errors::IndexOutOfRange => "Index out of range",
        })
    }
}
impl Debug for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}
