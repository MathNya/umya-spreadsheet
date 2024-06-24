use fancy_regex::{Captures, Regex};
use helper::coordinate::*;
use structs::StringValue;

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaTokenTypes {
    Noop,
    Operand,
    Function,
    Subexpression,
    Argument,
    OperatorPrefix,
    OperatorInfix,
    OperatorPostfix,
    Whitespace,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaTokenSubTypes {
    Nothing,
    Start,
    Stop,
    Text,
    Number,
    Logical,
    Error,
    Range,
    Math,
    Concatenation,
    Intersection,
    Union,
}

#[derive(Clone, Debug)]
pub struct FormulaToken {
    value: StringValue,
    token_type: FormulaTokenTypes,
    token_sub_type: FormulaTokenSubTypes,
}
impl Default for FormulaToken {
    fn default() -> Self {
        Self {
            value: StringValue::default(),
            token_type: FormulaTokenTypes::Unknown,
            token_sub_type: FormulaTokenSubTypes::Nothing,
        }
    }
}
impl FormulaToken {
    pub fn get_value(&self) -> &str {
        self.value.get_value_str()
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub fn get_token_type(&self) -> &FormulaTokenTypes {
        &self.token_type
    }

    pub fn set_token_type(&mut self, value: FormulaTokenTypes) -> &mut Self {
        self.token_type = value;
        self
    }

    pub fn get_token_sub_type(&self) -> &FormulaTokenSubTypes {
        &self.token_sub_type
    }

    pub fn set_token_sub_type(&mut self, value: FormulaTokenSubTypes) -> &mut Self {
        self.token_sub_type = value;
        self
    }
}

const QUOTE_DOUBLE: char = '"';
const QUOTE_SINGLE: char = '\'';
const BRACKET_CLOSE: char = ']';
const BRACKET_OPEN: char = '[';
const BRACE_OPEN: char = '{';
const BRACE_CLOSE: char = '}';
const PAREN_OPEN: char = '(';
const PAREN_CLOSE: char = ')';
const SEMICOLON: char = ';';
const WHITESPACE: char = ' ';
const COMMA: char = ',';
const ERROR_START: char = '#';

const OPERATORS_SN: &str = "+-";
const OPERATORS_INFIX: &str = "+-*/^&=><";
const OPERATORS_POSTFIX: &str = "%";

pub const ERRORS: &'static [&'static str] = &[
    "#NULL!", "#DIV/0!", "#VALUE!", "#REF!", "#NAME?", "#NUM!", "#N/A",
];
const COMPARATORS_MULTI: &'static [&'static str] = &[">=", "<=", "<>"];

lazy_static! {
    pub static ref SCIENTIFIC_REGEX: Regex = Regex::new(r#"/^[1-9]{1}(\\.\\d+)?E{1}$/"#).unwrap();
}

pub(crate) fn parse_to_tokens<S: Into<String>>(formula: S) -> Vec<FormulaToken> {
    let mut tokens: Vec<FormulaToken> = Vec::new();

    let formula = formula.into();
    let formula_length = formula.len();
    if formula_length < 2 || formula.chars().next().unwrap() != '=' {
        return tokens;
    }

    // Helper variables
    let mut tokens1: Vec<FormulaToken> = Vec::new();
    let mut tokens2: Vec<FormulaToken> = Vec::new();
    let mut stack: Vec<FormulaToken> = Vec::new();

    let mut in_string = false;
    let mut in_path = false;
    let mut in_range = false;
    let mut in_error = false;
    let mut next_token: Option<FormulaToken> = None;

    let mut index = 1;
    let mut value = String::from("");

    while index < formula_length {
        // double-quoted strings
        // embeds are doubled
        // end marks token
        if in_string {
            if formula.chars().nth(index).unwrap() == self::QUOTE_DOUBLE {
                if ((index + 2) <= formula_length)
                    && (formula.chars().nth(index + 1).unwrap() == self::QUOTE_DOUBLE)
                {
                    value = format!("{}{}", value, self::QUOTE_DOUBLE);
                    index += 1;
                } else {
                    in_string = false;
                    let mut obj = FormulaToken::default();
                    obj.set_value(value);
                    obj.set_token_type(FormulaTokenTypes::Operand);
                    obj.set_token_sub_type(FormulaTokenSubTypes::Text);
                    tokens1.push(obj);
                    value = String::from("");
                }
            } else {
                value = format!("{}{}", value, formula.chars().nth(index).unwrap());
            }
            index += 1;

            continue;
        }

        // single-quoted strings (links)
        // embeds are double
        // end does not mark a token
        if in_path {
            if formula.chars().nth(index).unwrap() == self::QUOTE_SINGLE {
                if ((index + 2) <= formula_length)
                    && (formula.chars().nth(index + 1).unwrap() == self::QUOTE_SINGLE)
                {
                    value = format!("{}{}", value, self::QUOTE_SINGLE);
                    index += 1;
                } else {
                    in_path = false;
                }
            } else {
                value = format!("{}{}", value, formula.chars().nth(index).unwrap());
            }
            index += 1;

            continue;
        }

        // bracked strings (R1C1 range index or linked workbook name)
        // no embeds (changed to "()" by Excel)
        // end does not mark a token
        if in_range {
            if formula.chars().nth(index).unwrap() == self::BRACKET_CLOSE {
                in_range = false;
            }
            value = format!("{}{}", value, formula.chars().nth(index).unwrap());
            index;

            continue;
        }

        // error values
        // end marks a token, determined from absolute list of values
        if in_error {
            value = format!("{}{}", value, formula.chars().nth(index).unwrap());
            index += 1;
            if self::ERRORS.iter().any(|&x| x == value.as_str()) {
                in_error = false;
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                obj.set_token_sub_type(FormulaTokenSubTypes::Error);
                tokens1.push(obj);
                value = String::from("");
            }

            continue;
        }

        // scientific notation check
        if self::OPERATORS_SN.contains(formula.chars().nth(index).unwrap()) {
            if value.len() > 1 {
                if !SCIENTIFIC_REGEX
                    .is_match(&formula.chars().nth(index).unwrap().to_string())
                    .unwrap_or(false)
                {
                    value = format!("{}{}", value, formula.chars().nth(index).unwrap());
                    index += 1;

                    continue;
                }
            }
        }

        // independent character evaluation (order not important)

        // establish state-dependent character evaluations
        if formula.chars().nth(index).unwrap() == self::QUOTE_DOUBLE {
            if value != "" {
                // unexpected
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Unknown);
                tokens1.push(obj);
                value = String::from("");
            }
            in_string = true;
            index += 1;

            continue;
        }

        if formula.chars().nth(index).unwrap() == self::QUOTE_SINGLE {
            if value != "" {
                // unexpected
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Unknown);
                tokens1.push(obj);
                value = String::from("");
            }
            in_string = true;
            index += 1;

            continue;
        }

        if formula.chars().nth(index).unwrap() == self::BRACKET_OPEN {
            in_range = true;
            value = format!("{}{}", value, self::BRACKET_OPEN);
            index += 1;

            continue;
        }

        if formula.chars().nth(index).unwrap() == self::ERROR_START {
            if value != "" {
                // unexpected
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Unknown);
                tokens1.push(obj);
                value = String::from("");
            }
            in_error = true;
            value = format!("{}{}", value, self::ERROR_START);
            index += 1;

            continue;
        }

        // mark start and end of arrays and array rows
        if formula.chars().nth(index).unwrap() == self::BRACE_OPEN {
            if value != "" {
                // unexpected
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Unknown);
                tokens1.push(obj);
                value = String::from("");
            }

            let mut obj = FormulaToken::default();
            obj.set_value("ARRAY");
            obj.set_token_type(FormulaTokenTypes::Function);
            obj.set_token_sub_type(FormulaTokenSubTypes::Start);
            tokens1.push(obj.clone());
            stack.push(obj);

            let mut obj = FormulaToken::default();
            obj.set_value("ARRAYROW");
            obj.set_token_type(FormulaTokenTypes::Function);
            obj.set_token_sub_type(FormulaTokenSubTypes::Start);
            tokens1.push(obj.clone());
            stack.push(obj);

            index += 1;

            continue;
        }

        if formula.chars().nth(index).unwrap() == self::SEMICOLON {
            if value != "" {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::from("");
            }

            let mut obj = stack.pop().unwrap();
            obj.set_value("");
            obj.set_token_sub_type(FormulaTokenSubTypes::Stop);
            tokens1.push(obj);

            let mut obj = FormulaToken::default();
            obj.set_value(",");
            obj.set_token_type(FormulaTokenTypes::Argument);
            tokens1.push(obj);

            let mut obj = FormulaToken::default();
            obj.set_value("ARRAYROW");
            obj.set_token_type(FormulaTokenTypes::Function);
            obj.set_token_sub_type(FormulaTokenSubTypes::Start);
            tokens1.push(obj.clone());
            stack.push(obj);

            index += 1;

            continue;
        }

        if formula.chars().nth(index).unwrap() == self::BRACE_CLOSE {
            if value != "" {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::from("");
            }

            let mut obj = stack.pop().unwrap().clone();
            obj.set_value("");
            obj.set_token_sub_type(FormulaTokenSubTypes::Stop);
            tokens1.push(obj);

            let mut obj = stack.pop().unwrap().clone();
            obj.set_value("");
            obj.set_token_sub_type(FormulaTokenSubTypes::Stop);
            tokens1.push(obj);

            index += 1;

            continue;
        }

        // trim white-space
        if formula.chars().nth(index).unwrap() == self::WHITESPACE {
            if value != "" {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::from("");
            }
            let mut obj = FormulaToken::default();
            obj.set_value("");
            obj.set_token_type(FormulaTokenTypes::Whitespace);
            tokens1.push(obj);
            index += 1;
            while ((formula.chars().nth(index).unwrap() == self::WHITESPACE)
                && (index < formula_length))
            {
                index += 1;
            }

            continue;
        }

        // multi-character comparators
        if (index + 2) <= formula_length {
            if COMPARATORS_MULTI
                .iter()
                .any(|&x| x == formula.chars().skip(index).take(2).collect::<String>())
            {
                if value != "" {
                    let mut obj = FormulaToken::default();
                    obj.set_value(value);
                    obj.set_token_type(FormulaTokenTypes::Operand);
                    tokens1.push(obj);
                    value = String::from("");
                }
                let mut obj = FormulaToken::default();
                obj.set_value(formula.chars().skip(index).take(2).collect::<String>());
                obj.set_token_type(FormulaTokenTypes::OperatorInfix);
                obj.set_token_sub_type(FormulaTokenSubTypes::Logical);
                tokens1.push(obj);
                index += 2;

                continue;
            }
        }

        // standard infix operators
        if self::OPERATORS_INFIX.contains(formula.chars().nth(index).unwrap()) {
            if value != "" {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::from("");
            }
            let mut obj = FormulaToken::default();
            obj.set_value(formula.chars().nth(index).unwrap());
            obj.set_token_type(FormulaTokenTypes::OperatorInfix);
            tokens1.push(obj);
            index += 1;

            continue;
        }

        // standard postfix operators (only one)
        if self::OPERATORS_POSTFIX.contains(formula.chars().nth(index).unwrap()) {
            if value != "" {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::from("");
            }
            let mut obj = FormulaToken::default();
            obj.set_value(formula.chars().nth(index).unwrap());
            obj.set_token_type(FormulaTokenTypes::OperatorPostfix);
            tokens1.push(obj);
            index += 1;

            continue;
        }

        // start subexpression or function
        if formula.chars().nth(index).unwrap() == self::PAREN_OPEN {
            if value != "" {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Function);
                obj.set_token_sub_type(FormulaTokenSubTypes::Start);
                tokens1.push(obj.clone());
                stack.push(obj);
                value = String::from("");
            } else {
                let mut obj = FormulaToken::default();
                obj.set_value("");
                obj.set_token_type(FormulaTokenTypes::Subexpression);
                obj.set_token_sub_type(FormulaTokenSubTypes::Start);
                tokens1.push(obj.clone());
                stack.push(obj);
            }
            index += 1;

            continue;
        }

        // function, subexpression, or array parameters, or operand unions
        if formula.chars().nth(index).unwrap() == self::COMMA {
            if value != "" {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::from("");
            }

            let mut obj = stack.pop().unwrap();
            obj.set_value("");
            obj.set_token_sub_type(FormulaTokenSubTypes::Stop);
            stack.push(obj.clone());

            if obj.get_token_type() == &FormulaTokenTypes::Function {
                let mut obj = FormulaToken::default();
                obj.set_value(",");
                obj.set_token_type(FormulaTokenTypes::OperatorInfix);
                obj.set_token_sub_type(FormulaTokenSubTypes::Union);
                tokens1.push(obj);
            } else {
                let mut obj = FormulaToken::default();
                obj.set_value(",");
                obj.set_token_type(FormulaTokenTypes::Argument);
                tokens1.push(obj);
            }
            index += 1;

            continue;
        }

        // stop subexpression
        if formula.chars().nth(index).unwrap() == self::PAREN_CLOSE {
            if value != "" {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::from("");
            }

            let mut obj = stack.pop().unwrap();
            obj.set_value("");
            obj.set_token_sub_type(FormulaTokenSubTypes::Stop);
            tokens1.push(obj);

            index += 1;

            continue;
        }

        // token accumulation
        value = format!("{}{}", value, formula.chars().nth(index).unwrap());
        index += 1;
    }

    // dump remaining accumulation
    if value != "" {
        let mut obj = FormulaToken::default();
        obj.set_value(value.clone());
        obj.set_token_type(FormulaTokenTypes::Operand);
        tokens1.push(obj);
    }

    // move tokenList to new set, excluding unnecessary white-space tokens and converting necessary ones to intersections
    let token_count = tokens1.len();
    let mut previous_token = None;
    let mut next_token = None;
    for i in 0..token_count {
        let token = tokens1.get(i).unwrap();
        match tokens1.get((i - 1)) {
            Some(v) => {
                previous_token = Some(v.clone());
            }
            None => {
                previous_token = None;
            }
        }

        match tokens1.get((i + 1)) {
            Some(v) => {
                next_token = Some(tokens1.get((i + 1)).unwrap());
            }
            None => {
                next_token = None;
            }
        }

        if token.get_token_type() != &FormulaTokenTypes::Whitespace {
            tokens2.push(token.clone());

            continue;
        }

        if previous_token.is_none() {
            continue;
        }

        if !(((previous_token.as_ref().unwrap().get_token_type() == &FormulaTokenTypes::Function)
            && (previous_token.as_ref().unwrap().get_token_sub_type()
                == &FormulaTokenSubTypes::Stop))
            || ((previous_token.as_ref().unwrap().get_token_type()
                == &FormulaTokenTypes::Subexpression)
                && (previous_token.as_ref().unwrap().get_token_sub_type()
                    == &FormulaTokenSubTypes::Stop))
            || (previous_token.as_ref().unwrap().get_token_type() == &FormulaTokenTypes::Operand))
        {
            continue;
        }

        if next_token.is_none() {
            continue;
        }

        if !(((next_token.as_ref().unwrap().get_token_type() == &FormulaTokenTypes::Function)
            && (next_token.as_ref().unwrap().get_token_sub_type() == &FormulaTokenSubTypes::Start))
            || ((next_token.as_ref().unwrap().get_token_type()
                == &FormulaTokenTypes::Subexpression)
                && (next_token.as_ref().unwrap().get_token_sub_type()
                    == &FormulaTokenSubTypes::Start))
            || (next_token.as_ref().unwrap().get_token_type() == &FormulaTokenTypes::Operand))
        {
            continue;
        }

        let mut obj = FormulaToken::default();
        obj.set_value(value);
        obj.set_token_type(FormulaTokenTypes::OperatorInfix);
        obj.set_token_sub_type(FormulaTokenSubTypes::Intersection);
        tokens2.push(obj);
        value = String::from("");
    }

    // move tokens to final list, switching infix "-" operators to prefix when appropriate, switching infix "+" operators
    // to noop when appropriate, identifying operand and infix-operator subtypes, and pulling "@" from function names
    let token_count = tokens2.len();
    let mut previous_token = None;
    for i in 0..token_count {
        let mut token = tokens2.get(i).unwrap().clone();
        match tokens2.get(i - 1) {
            Some(v) => {
                previous_token = Some(v.clone());
            }
            None => {
                previous_token = None;
            }
        }

        if token.get_token_type() == &FormulaTokenTypes::OperatorInfix && token.get_value() == "-" {
            if i == 0 {
                token.set_token_type(FormulaTokenTypes::OperatorPrefix);
            } else if ((previous_token.as_ref().unwrap().get_token_type()
                == &FormulaTokenTypes::Function)
                && (previous_token.as_ref().unwrap().get_token_sub_type()
                    == &FormulaTokenSubTypes::Stop))
                || ((previous_token.as_ref().unwrap().get_token_type()
                    == &FormulaTokenTypes::Subexpression)
                    && (previous_token.as_ref().unwrap().get_token_sub_type()
                        == &FormulaTokenSubTypes::Stop))
                || (previous_token.as_ref().unwrap().get_token_type()
                    == &FormulaTokenTypes::OperatorPostfix)
                || (previous_token.as_ref().unwrap().get_token_type()
                    == &FormulaTokenTypes::Operand)
            {
                token.set_token_sub_type(FormulaTokenSubTypes::Math);
            } else {
                token.set_token_type(FormulaTokenTypes::OperatorPrefix);
            }

            tokens.push(token.clone());

            continue;
        }

        if token.get_token_type() == &FormulaTokenTypes::OperatorInfix && token.get_value() == "+" {
            if i == 0 {
                continue;
            } else if ((previous_token.as_ref().unwrap().get_token_type()
                == &FormulaTokenTypes::Function)
                && (previous_token.as_ref().unwrap().get_token_sub_type()
                    == &FormulaTokenSubTypes::Stop))
                || ((previous_token.as_ref().unwrap().get_token_type()
                    == &FormulaTokenTypes::Subexpression)
                    && (previous_token.as_ref().unwrap().get_token_sub_type()
                        == &FormulaTokenSubTypes::Stop))
                || (previous_token.as_ref().unwrap().get_token_type()
                    == &FormulaTokenTypes::OperatorPostfix)
                || (previous_token.as_ref().unwrap().get_token_type()
                    == &FormulaTokenTypes::Operand)
            {
                token.set_token_sub_type(FormulaTokenSubTypes::Math);
            } else {
                continue;
            }

            tokens.push(token.clone());

            continue;
        }

        if token.get_token_type() == &FormulaTokenTypes::OperatorInfix
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Nothing
        {
            if "<>=".contains(token.get_value().chars().nth(0).unwrap()) {
                token.set_token_sub_type(FormulaTokenSubTypes::Logical);
            } else if token.get_value() == "&" {
                token.set_token_sub_type(FormulaTokenSubTypes::Concatenation);
            } else {
                token.set_token_sub_type(FormulaTokenSubTypes::Math);
            }

            tokens.push(token.clone());

            continue;
        }

        if token.get_token_type() == &FormulaTokenTypes::Operand
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Nothing
        {
            if !token.get_value().parse::<f64>().is_ok() {
                if token.get_value().to_uppercase() == "TRUE"
                    || token.get_value().to_uppercase() == "FALSE"
                {
                    token.set_token_sub_type(FormulaTokenSubTypes::Logical);
                } else {
                    token.set_token_sub_type(FormulaTokenSubTypes::Range);
                }
            } else {
                token.set_token_sub_type(FormulaTokenSubTypes::Number);
            }

            tokens.push(token.clone());

            continue;
        }

        if token.get_token_type() == &FormulaTokenTypes::Function {
            if token.get_value() != "" {
                if token.get_value().chars().nth(0).unwrap() == '@' {
                    token.set_value(token.get_value().chars().skip(1).collect::<String>());
                }
            }
        }

        tokens.push(token.clone());
    }
    tokens
}

pub fn adjustment_insert_formula_coordinate(
    formula: &str,
    root_col_num: &u32,
    offset_col_num: &u32,
    root_row_num: &u32,
    offset_row_num: &u32,
    worksheet_name: &str,
    self_worksheet_name: &str,
) -> String {
    let re = Regex::new(r"[^\(]*!*[A-Z]+[0-9]+\:[A-Z]+[0-9]+").unwrap();
    let result = re.replace_all(formula, |caps: &Captures| {
        let caps_string = caps.get(0).unwrap().as_str().to_string();
        let split_str: Vec<&str> = caps_string.split('!').collect();
        let with_wksheet: bool;
        let wksheet: &str;
        let range: &str;

        if split_str.len() == 2 {
            with_wksheet = true;
            wksheet = split_str.first().unwrap();
            range = split_str.get(1).unwrap();
        } else {
            with_wksheet = false;
            wksheet = self_worksheet_name;
            range = split_str.first().unwrap();
        }

        if wksheet != worksheet_name {
            return caps_string;
        }

        let split_range: Vec<&str> = range.split(':').collect();
        let mut result = String::from("");

        for coordinate in split_range {
            let index_coordinate = index_from_coordinate(coordinate);
            let is_lock_col = index_coordinate.2.unwrap();
            let is_lock_row = index_coordinate.3.unwrap();
            let col_num = adjustment_insert_coordinate(
                &index_coordinate.0.unwrap(),
                root_col_num,
                offset_col_num,
            );
            let row_num = adjustment_insert_coordinate(
                &index_coordinate.1.unwrap(),
                root_row_num,
                offset_row_num,
            );
            let new_corrdinate =
                coordinate_from_index_with_lock(&col_num, &row_num, &is_lock_col, &is_lock_row);

            if !&result.is_empty() {
                result = format!("{}:", result);
            }
            result = format!("{}{}", result, new_corrdinate);
        }

        if with_wksheet {
            result = format!("{}!{}", wksheet, result);
        }

        result
    });

    result.into()
}

pub fn adjustment_remove_formula_coordinate(
    formula: &str,
    root_col_num: &u32,
    offset_col_num: &u32,
    root_row_num: &u32,
    offset_row_num: &u32,
    worksheet_name: &str,
    self_worksheet_name: &str,
) -> String {
    let re = Regex::new(r"[^\(]*!*[A-Z]+[0-9]+\:[A-Z]+[0-9]+").unwrap();
    let result = re.replace_all(formula, |caps: &Captures| {
        let caps_string = caps.get(0).unwrap().as_str().to_string();
        let split_str: Vec<&str> = caps_string.split('!').collect();
        let with_wksheet: bool;
        let wksheet: String;
        let range: String;

        if split_str.len() == 2 {
            with_wksheet = true;
            wksheet = split_str.first().unwrap().to_string();
            range = split_str.get(1).unwrap().to_string();
        } else {
            with_wksheet = false;
            wksheet = self_worksheet_name.to_string();
            range = split_str.first().unwrap().to_string();
        }

        if wksheet != worksheet_name {
            return caps_string;
        }

        let split_range: Vec<&str> = range.split(':').collect();
        let mut result = String::from("");

        for coordinate in split_range {
            let index_coordinate = index_from_coordinate(coordinate);
            let is_lock_col = index_coordinate.2.unwrap();
            let is_lock_row = index_coordinate.3.unwrap();
            let col_num = adjustment_remove_coordinate(
                &index_coordinate.0.unwrap(),
                root_col_num,
                offset_col_num,
            );
            let row_num = adjustment_remove_coordinate(
                &index_coordinate.1.unwrap(),
                root_row_num,
                offset_row_num,
            );
            let new_corrdinate =
                coordinate_from_index_with_lock(&col_num, &row_num, &is_lock_col, &is_lock_row);

            if !&result.is_empty() {
                result = format!("{}:", result);
            }

            result = format!("{}{}", result, new_corrdinate);
        }

        if with_wksheet {
            result = format!("{}!{}", wksheet, result);
        }

        result
    });

    result.into()
}
