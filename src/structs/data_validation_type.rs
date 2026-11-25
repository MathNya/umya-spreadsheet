use std::borrow::Cow;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub enum DataValidationType {
    Custom(DataValidationFormula),
    Date(DataValidationExpression),
    Decimal(DataValidationExpression),
    List(DataValidationListOrRange),
    #[default]
    None,
    TextLength(DataValidationExpression),
    Time(DataValidationExpression),
    Whole(DataValidationExpression),
}

impl DataValidationType {
    pub(crate) fn with_type_and_operator(
        r#type: Option<&str>,
        operator: Option<&str>,
    ) -> ValidationTypeParser {
        let Some(type_str) = r#type else {
            return ValidationTypeParser::None;
        };
        match type_str {
            "none" => ValidationTypeParser::None,
            "custom" => ValidationTypeParser::Custom,
            "date" => continuation_from_operator(operator, DataValidationType::Date),
            "decimal" => continuation_from_operator(operator, DataValidationType::Decimal),
            "list" => ValidationTypeParser::List,
            "textLength" => continuation_from_operator(operator, DataValidationType::TextLength),
            "time" => continuation_from_operator(operator, DataValidationType::Time),
            "whole" => continuation_from_operator(operator, DataValidationType::Whole),
            other => {
                panic!("Unexpected data validation type {other}")
            }
        }
    }

    pub(crate) fn attrs_and_formulas(&self) -> ValidationTypeWriter<'_> {
        match self {
            DataValidationType::None => ValidationTypeWriter {
                r#type:   "none",
                operator: None,
                formula1: None,
                formula2: None,
            },
            DataValidationType::Custom(formula) => ValidationTypeWriter {
                r#type:   "custom",
                operator: None,
                formula1: Some(Cow::Borrowed(formula)),
                formula2: None,
            },
            DataValidationType::Date(expr) => ValidationTypeWriter::from_expression("date", expr),
            DataValidationType::Decimal(expr) => {
                ValidationTypeWriter::from_expression("decimal", expr)
            }
            DataValidationType::List(list_or_range) => {
                let formula1 = list_or_range.to_formula();
                ValidationTypeWriter {
                    r#type:   "list",
                    operator: None,
                    formula1: Some(Cow::Owned(formula1)),
                    formula2: None,
                }
            }
            DataValidationType::TextLength(expr) => {
                ValidationTypeWriter::from_expression("textLength", expr)
            }
            DataValidationType::Time(expr) => ValidationTypeWriter::from_expression("time", expr),
            DataValidationType::Whole(expr) => ValidationTypeWriter::from_expression("whole", expr),
        }
    }
}

fn continuation_from_operator(
    operator: Option<&str>,
    to_type: fn(DataValidationExpression) -> DataValidationType,
) -> ValidationTypeParser {
    match operator.unwrap_or("between") {
        "between" => ValidationTypeParser::FormulaPair {
            expression_from_formulas: DataValidationExpression::Between,
            to_type,
        },
        "notBetween" => ValidationTypeParser::FormulaPair {
            expression_from_formulas: DataValidationExpression::NotBetween,
            to_type,
        },
        "equal" => ValidationTypeParser::SingleFormula {
            expression_from_formula: DataValidationExpression::Equal,
            to_type,
        },
        "greaterThan" => ValidationTypeParser::SingleFormula {
            expression_from_formula: DataValidationExpression::GreaterThan,
            to_type,
        },
        "greaterThanOrEqual" => ValidationTypeParser::SingleFormula {
            expression_from_formula: DataValidationExpression::GreaterThanOrEqual,
            to_type,
        },
        "lessThan" => ValidationTypeParser::SingleFormula {
            expression_from_formula: DataValidationExpression::LessThan,
            to_type,
        },
        "lessThanOrEqual" => ValidationTypeParser::SingleFormula {
            expression_from_formula: DataValidationExpression::LessThanOrEqual,
            to_type,
        },
        "notEqual" => ValidationTypeParser::SingleFormula {
            expression_from_formula: DataValidationExpression::NotEqual,
            to_type,
        },
        other => {
            panic!("Unexpected data validation operator {other}")
        }
    }
}

pub type DataValidationFormula = String;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DataValidationExpression {
    Between(DataValidationFormula, DataValidationFormula),
    Equal(DataValidationFormula),
    GreaterThan(DataValidationFormula),
    GreaterThanOrEqual(DataValidationFormula),
    LessThan(DataValidationFormula),
    LessThanOrEqual(DataValidationFormula),
    NotBetween(DataValidationFormula, DataValidationFormula),
    NotEqual(DataValidationFormula),
}

impl DataValidationExpression {
    pub(crate) fn to_operator_and_formulas(&self) -> (&'static str, Option<&str>, Option<&str>) {
        match self {
            DataValidationExpression::Between(f1, f2) => ("between", Some(f1), Some(f2)),
            DataValidationExpression::Equal(f) => ("equal", Some(f), None),
            DataValidationExpression::GreaterThan(f) => ("greaterThan", Some(f), None),
            DataValidationExpression::GreaterThanOrEqual(f) => {
                ("greaterThanOrEqual", Some(f), None)
            }
            DataValidationExpression::LessThan(f) => ("lessThan", Some(f), None),
            DataValidationExpression::LessThanOrEqual(f) => ("lessThanOrEqual", Some(f), None),
            DataValidationExpression::NotBetween(f1, f2) => ("notBetween", Some(f1), Some(f2)),
            DataValidationExpression::NotEqual(f) => ("notEqual", Some(f), None),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DataValidationListOrRange {
    List(Vec<String>),
    Range(String),
}

impl DataValidationListOrRange {
    fn from_formula(formula: &str) -> Result<Self, &'static str> {
        let formula = formula.trim();

        if let Some(items) = formula.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            // Preserve item whitespace in line with OOXML's xml:space rules
            // (§18.3.1.44).
            Ok(Self::List(items.split(',').map(str::to_owned).collect()))
        } else if formula.is_empty() {
            Err("validation of type \"list\" with empty formula list")
        } else {
            Ok(Self::Range(formula.to_owned()))
        }
    }

    fn to_formula(&self) -> String {
        match self {
            DataValidationListOrRange::List(items) => format!("\"{}\"", items.join(",")),
            DataValidationListOrRange::Range(range) => range.clone(),
        }
    }
}

pub(crate) enum ValidationTypeParser {
    None,
    Custom,
    List,
    SingleFormula {
        expression_from_formula: fn(String) -> DataValidationExpression,
        to_type:                 fn(DataValidationExpression) -> DataValidationType,
    },
    FormulaPair {
        expression_from_formulas: fn(String, String) -> DataValidationExpression,
        to_type:                  fn(DataValidationExpression) -> DataValidationType,
    },
}

impl ValidationTypeParser {
    pub(crate) fn apply(
        self,
        formula1: Option<String>,
        formula2: Option<String>,
    ) -> Result<DataValidationType, &'static str> {
        match (self, formula1, formula2) {
            (Self::None, None, None) => Ok(DataValidationType::None),
            (Self::List, Some(formula), None) => {
                DataValidationListOrRange::from_formula(&formula).map(DataValidationType::List)
            }
            (Self::Custom, Some(formula), None) => Ok(DataValidationType::Custom(formula)),
            (
                Self::SingleFormula {
                    expression_from_formula,
                    to_type,
                },
                Some(formula1),
                None,
            ) => Ok(to_type(expression_from_formula(formula1))),
            (
                Self::FormulaPair {
                    expression_from_formulas,
                    to_type,
                },
                Some(formula1),
                Some(formula2),
            ) => Ok(to_type(expression_from_formulas(formula1, formula2))),
            _ => Err("Missing or unexpected validation formulas"),
        }
    }
}

pub(crate) struct ValidationTypeWriter<'a> {
    pub r#type:   &'static str,
    pub operator: Option<&'static str>,
    pub formula1: Option<Cow<'a, str>>,
    pub formula2: Option<&'a str>,
}

impl<'a> ValidationTypeWriter<'a> {
    fn from_expression(r#type: &'static str, expression: &'a DataValidationExpression) -> Self {
        let (operator, formula1, formula2) = expression.to_operator_and_formulas();
        Self {
            r#type,
            operator: Some(operator),
            formula1: formula1.map(Cow::Borrowed),
            formula2,
        }
    }

    pub fn has_formulas(&self) -> bool {
        self.formula1.is_some() || self.formula2.is_some()
    }
}
