mod data_validation;
pub use self::data_validation::*;

mod data_validations;
pub use self::data_validations::*;

mod data_validation_formula1;
pub use self::data_validation_formula1::*;
#[deprecated(since = "4.0.0", note = "Use DataValidationFormula1")]
// spellchecker:ignore-next-line because it is part of public API so needs to be
// allowed for now to avoid unnecessary breakage
pub type DataValidationForumla1 = DataValidationFormula1;

mod data_validation_formula2;
pub use self::data_validation_formula2::*;
#[deprecated(since = "4.0.0", note = "Use DataValidationFormula2")]
// spellchecker:ignore-next-line because it is part of public API so needs to be
// allowed for now to avoid unnecessary breakage
pub type DataValidationForumla2 = DataValidationFormula2;
