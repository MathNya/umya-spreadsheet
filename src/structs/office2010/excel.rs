mod data_validation;
pub use self::data_validation::*;

mod data_validations;
pub use self::data_validations::*;

mod data_validation_formula;
use data_validation_formula::data_validation_formula;

data_validation_formula!(DataValidationFormula1, "x14:formula1");
data_validation_formula!(DataValidationFormula2, "x14:formula2");

#[deprecated(since = "4.0.0", note = "Use DataValidationFormula1")]
// spellchecker:ignore-next-line because it is part of public API so needs to be
// allowed for now to avoid unnecessary breakage
pub type DataValidationForumla1 = DataValidationFormula1;

#[deprecated(since = "4.0.0", note = "Use DataValidationFormula2")]
// spellchecker:ignore-next-line because it is part of public API so needs to be
// allowed for now to avoid unnecessary breakage
pub type DataValidationForumla2 = DataValidationFormula2;
