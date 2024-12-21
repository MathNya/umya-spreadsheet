use crate::helper::address::{join_address, split_address};
use crate::helper::coordinate::{
    adjustment_insert_coordinate, adjustment_remove_coordinate, coordinate_from_index_with_lock,
    index_from_coordinate,
};
use crate::helper::range::{get_join_range, get_split_range};
use crate::structs::StringValue;
use fancy_regex::Regex;

/** PARTLY BASED ON: */
/** Copyright (c) 2007 E. W. Bachtal, Inc. */
/** <https://ewbi.blogs.com/develops/2007/03/excel_formula_p.html> */
/** <https://ewbi.blogs.com/develops/2004/12/excel_formula_p.html> */

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
    #[inline]
    fn default() -> Self {
        Self {
            value: StringValue::default(),
            token_type: FormulaTokenTypes::Unknown,
            token_sub_type: FormulaTokenSubTypes::Nothing,
        }
    }
}
impl FormulaToken {
    #[inline]
    #[must_use]
    pub fn get_value(&self) -> &str {
        self.value.get_value_str()
    }

    #[inline]
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_token_type(&self) -> &FormulaTokenTypes {
        &self.token_type
    }

    #[inline]
    pub fn set_token_type(&mut self, value: FormulaTokenTypes) -> &mut Self {
        self.token_type = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_token_sub_type(&self) -> &FormulaTokenSubTypes {
        &self.token_sub_type
    }

    #[inline]
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

pub const ERRORS: &[&str] = &[
    "#NULL!", "#DIV/0!", "#VALUE!", "#REF!", "#NAME?", "#NUM!", "#N/A",
];
const COMPARATORS_MULTI: &[&str] = &[">=", "<=", "<>"];

lazy_static! {
    pub static ref SCIENTIFIC_REGEX: Regex = Regex::new(r"/^[1-9]{1}(\\.\\d+)?E{1}$/").unwrap();
}

pub(crate) fn parse_to_tokens<S: Into<String>>(formula: S) -> Vec<FormulaToken> {
    let mut tokens: Vec<FormulaToken> = Vec::new();

    let formula = formula.into();
    let formula_length = formula.chars().count();
    if formula_length < 2 || !formula.starts_with('=') {
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

    let mut index = 1;
    let mut value = String::new();

    while index < formula_length {
        // double-quoted strings
        // embeds are doubled
        // end marks token
        if in_string {
            if formula.chars().nth(index).unwrap() == QUOTE_DOUBLE {
                if ((index + 2) <= formula_length)
                    && (formula.chars().nth(index + 1).unwrap() == QUOTE_DOUBLE)
                {
                    value = format!("{value}{QUOTE_DOUBLE}");
                    index += 1;
                } else {
                    in_string = false;
                    let mut obj = FormulaToken::default();
                    obj.set_value(value);
                    obj.set_token_type(FormulaTokenTypes::Operand);
                    obj.set_token_sub_type(FormulaTokenSubTypes::Text);
                    tokens1.push(obj);
                    value = String::new();
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
            if formula.chars().nth(index).unwrap() == QUOTE_SINGLE {
                if ((index + 2) <= formula_length)
                    && (formula.chars().nth(index + 1).unwrap() == QUOTE_SINGLE)
                {
                    value = format!("{value}{QUOTE_SINGLE}");
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
            if formula.chars().nth(index).unwrap() == BRACKET_CLOSE {
                in_range = false;
            }
            value = format!("{}{}", value, formula.chars().nth(index).unwrap());

            continue;
        }

        // error values
        // end marks a token, determined from absolute list of values
        if in_error {
            value = format!("{}{}", value, formula.chars().nth(index).unwrap());
            index += 1;
            if ERRORS.iter().any(|&x| x == value.as_str()) {
                in_error = false;
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                obj.set_token_sub_type(FormulaTokenSubTypes::Error);
                tokens1.push(obj);
                value = String::new();
            }

            continue;
        }

        // scientific notation check
        if let Some(current_char) = formula.chars().nth(index) {
            if OPERATORS_SN.contains(current_char)
                && value.len() > 1
                && SCIENTIFIC_REGEX
                    .is_match(&current_char.to_string())
                    .unwrap_or(false)
            {
                value.push(current_char);
                index += 1;
                continue;
            }
        }

        // independent character evaluation (order not important)

        // establish state-dependent character evaluations
        if formula.chars().nth(index).unwrap() == QUOTE_DOUBLE {
            if !value.is_empty() {
                // unexpected
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Unknown);
                tokens1.push(obj);
                value = String::new();
            }
            in_string = true;
            index += 1;

            continue;
        }

        if formula.chars().nth(index).unwrap() == QUOTE_SINGLE {
            if !value.is_empty() {
                // unexpected
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Unknown);
                tokens1.push(obj);
                value = String::new();
            }
            in_string = true;
            index += 1;

            continue;
        }

        if formula.chars().nth(index).unwrap() == BRACKET_OPEN {
            in_range = true;
            value = format!("{value}{BRACKET_OPEN}");
            index += 1;

            continue;
        }

        if formula.chars().nth(index).unwrap() == ERROR_START {
            if !value.is_empty() {
                // unexpected
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Unknown);
                tokens1.push(obj);
                value = String::new();
            }
            in_error = true;
            value = format!("{value}{ERROR_START}");
            index += 1;

            continue;
        }

        // mark start and end of arrays and array rows
        if formula.chars().nth(index).unwrap() == BRACE_OPEN {
            if !value.is_empty() {
                // unexpected
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Unknown);
                tokens1.push(obj);
                value = String::new();
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

        if formula.chars().nth(index).unwrap() == SEMICOLON {
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::new();
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

        if formula.chars().nth(index).unwrap() == BRACE_CLOSE {
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::new();
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
        if formula.chars().nth(index).unwrap() == WHITESPACE {
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::new();
            }
            let mut obj = FormulaToken::default();
            obj.set_value("");
            obj.set_token_type(FormulaTokenTypes::Whitespace);
            tokens1.push(obj);
            index += 1;
            while (formula.chars().nth(index).unwrap() == WHITESPACE) && (index < formula_length) {
                index += 1;
            }

            continue;
        }

        // multi-character comparators
        if (index + 2) <= formula_length
            && COMPARATORS_MULTI
                .iter()
                .any(|&x| x == formula.chars().skip(index).take(2).collect::<String>())
        {
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::new();
            }
            let mut obj = FormulaToken::default();
            obj.set_value(formula.chars().skip(index).take(2).collect::<String>());
            obj.set_token_type(FormulaTokenTypes::OperatorInfix);
            obj.set_token_sub_type(FormulaTokenSubTypes::Logical);
            tokens1.push(obj);
            index += 2;

            continue;
        }

        // standard infix operators
        if OPERATORS_INFIX.contains(formula.chars().nth(index).unwrap()) {
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::new();
            }
            let mut obj = FormulaToken::default();
            obj.set_value(formula.chars().nth(index).unwrap());
            obj.set_token_type(FormulaTokenTypes::OperatorInfix);
            tokens1.push(obj);
            index += 1;

            continue;
        }

        // standard postfix operators (only one)
        if OPERATORS_POSTFIX.contains(formula.chars().nth(index).unwrap()) {
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::new();
            }
            let mut obj = FormulaToken::default();
            obj.set_value(formula.chars().nth(index).unwrap());
            obj.set_token_type(FormulaTokenTypes::OperatorPostfix);
            tokens1.push(obj);
            index += 1;

            continue;
        }

        // start subexpression or function
        if formula.chars().nth(index).unwrap() == PAREN_OPEN {
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Function);
                obj.set_token_sub_type(FormulaTokenSubTypes::Start);
                tokens1.push(obj.clone());
                stack.push(obj);
                value = String::new();
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
        if formula.chars().nth(index).unwrap() == COMMA {
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::new();
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
        if formula.chars().nth(index).unwrap() == PAREN_CLOSE {
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value = String::new();
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
    if !value.is_empty() {
        let mut obj = FormulaToken::default();
        obj.set_value(value.clone());
        obj.set_token_type(FormulaTokenTypes::Operand);
        tokens1.push(obj);
    }

    // move tokenList to new set, excluding unnecessary white-space tokens and converting necessary ones to intersections
    let token_count = tokens1.len();
    #[allow(unused_assignments)]
    let mut previous_token = None;
    #[allow(unused_assignments)]
    let mut next_token = None;
    for i in 0..token_count {
        let token = tokens1.get(i).unwrap();
        if i > 0 {
            match tokens1.get(i - 1) {
                Some(v) => {
                    previous_token = Some(v.clone());
                }
                None => {
                    previous_token = None;
                }
            }
        } else {
            previous_token = None;
        }

        match tokens1.get(i + 1) {
            Some(_) => {
                next_token = Some(tokens1.get(i + 1).unwrap());
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
        value = String::new();
    }

    // move tokens to final list, switching infix "-" operators to prefix when appropriate, switching infix "+" operators
    // to noop when appropriate, identifying operand and infix-operator subtypes, and pulling "@" from function names
    let token_count = tokens2.len();
    #[allow(unused_assignments)]
    let mut previous_token = None;
    for i in 0..token_count {
        let mut token = tokens2.get(i).unwrap().clone();
        if i > 0 {
            match tokens2.get(i - 1) {
                Some(v) => {
                    previous_token = Some(v.clone());
                }
                None => {
                    previous_token = None;
                }
            }
        } else {
            previous_token = None;
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
            if "<>=".contains(token.get_value().chars().next().unwrap()) {
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
            if token.get_value().parse::<f64>().is_err() {
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
        if let FormulaTokenTypes::Function = token.get_token_type() {
            if !token.get_value().is_empty() && token.get_value().starts_with('@') {
                token.set_value(token.get_value().chars().skip(1).collect::<String>());
            }
        }

        tokens.push(token.clone());
    }
    tokens
}

pub(crate) fn render(formula_token_list: &[FormulaToken]) -> String {
    let mut result = String::new();
    for token in formula_token_list {
        if token.get_token_type() == &FormulaTokenTypes::Function
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Start
        {
            result = format!("{}{}", result, token.get_value());
            result = format!("{result}{PAREN_OPEN}");
        } else if token.get_token_type() == &FormulaTokenTypes::Function
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Stop
        {
            result = format!("{result}{PAREN_CLOSE}");
        } else if token.get_token_type() == &FormulaTokenTypes::Subexpression
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Start
        {
            result = format!("{result}{PAREN_OPEN}");
        } else if token.get_token_type() == &FormulaTokenTypes::Subexpression
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Stop
        {
            result = format!("{result}{PAREN_CLOSE}");
        } else if token.get_token_type() == &FormulaTokenTypes::Operand
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Text
        {
            result = format!("{result}{QUOTE_DOUBLE}");
            result = format!("{}{}", result, token.get_value());
            result = format!("{result}{QUOTE_DOUBLE}");
        } else if token.get_token_type() == &FormulaTokenTypes::OperatorInfix
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Intersection
        {
            result = format!("{result}{WHITESPACE}");
        } else {
            result = format!("{}{}", result, token.get_value());
        }
    }
    result
}

pub fn adjustment_formula_coordinate(
    token_list: &mut [FormulaToken],
    offset_col_num: i32,
    offset_row_num: i32,
) {
    for token in token_list.iter_mut() {
        if token.get_token_type() == &FormulaTokenTypes::Operand
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Range
        {
            let (sheet_name, range) = split_address(token.get_value());
            let mut coordinate_list_new: Vec<String> = Vec::new();
            let coordinate_list = get_split_range(range);
            let mut has_error = false;
            for coordinate in &coordinate_list {
                let cell = index_from_coordinate(coordinate);
                if cell.0.is_some() {
                    let mut col_num = cell.0.unwrap();
                    let mut row_num = cell.1.unwrap();
                    let is_lock_col = cell.2.unwrap();
                    let is_lock_row = cell.3.unwrap();
                    if !is_lock_col {
                        let calc_col_num = col_num as i32 + offset_col_num;
                        if calc_col_num < 1 {
                            has_error = true;
                            break;
                        } else {
                            col_num = calc_col_num as u32;
                        }
                    }
                    if !is_lock_row {
                        let calc_row_num = row_num as i32 + offset_row_num;
                        if calc_row_num < 1 {
                            has_error = true;
                            break;
                        } else {
                            row_num = calc_row_num as u32;
                        }
                    }
                    let new_corrdinate =
                        coordinate_from_index_with_lock(col_num, row_num, is_lock_col, is_lock_row);
                    coordinate_list_new.push(new_corrdinate);
                } else {
                    coordinate_list_new.push((*coordinate).to_string());
                }
            }
            if has_error {
                token.set_value("#REF!");
                token.set_token_sub_type(FormulaTokenSubTypes::Error);
            } else {
                let new_value = join_address(sheet_name, &get_join_range(&coordinate_list_new));
                token.set_value(new_value);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn adjustment_insert_formula_coordinate(
    token_list: &mut [FormulaToken],
    root_col_num: u32,
    offset_col_num: u32,
    root_row_num: u32,
    offset_row_num: u32,
    worksheet_name: &str,
    self_worksheet_name: &str,
    ignore_worksheet: bool,
) -> String {
    for token in token_list.iter_mut() {
        if token.get_token_type() == &FormulaTokenTypes::Operand
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Range
        {
            let (sheet_name, range) = split_address(token.get_value());
            if ignore_worksheet
                || (sheet_name.is_empty() && worksheet_name == self_worksheet_name)
                || (sheet_name == worksheet_name)
            {
                let mut coordinate_list_new: Vec<String> = Vec::new();
                let coordinate_list = get_split_range(range);
                for coordinate in &coordinate_list {
                    let cell = index_from_coordinate(coordinate);
                    if cell.0.is_some() {
                        let mut col_num = cell.0.unwrap();
                        let mut row_num = cell.1.unwrap();
                        let is_lock_col = cell.2.unwrap();
                        let is_lock_row = cell.3.unwrap();
                        if !is_lock_col {
                            col_num =
                                adjustment_insert_coordinate(col_num, root_col_num, offset_col_num);
                        }
                        if !is_lock_row {
                            row_num =
                                adjustment_insert_coordinate(row_num, root_row_num, offset_row_num);
                        }
                        let new_corrdinate = coordinate_from_index_with_lock(
                            col_num,
                            row_num,
                            is_lock_col,
                            is_lock_row,
                        );
                        coordinate_list_new.push(new_corrdinate);
                    } else {
                        coordinate_list_new.push((*coordinate).to_string());
                    }
                }
                let new_value = join_address(sheet_name, &get_join_range(&coordinate_list_new));
                token.set_value(new_value);
            }
        }
    }
    render(token_list)
}

#[allow(clippy::too_many_arguments)]
pub fn adjustment_remove_formula_coordinate(
    token_list: &mut [FormulaToken],
    root_col_num: u32,
    offset_col_num: u32,
    root_row_num: u32,
    offset_row_num: u32,
    worksheet_name: &str,
    self_worksheet_name: &str,
    ignore_worksheet: bool,
) -> String {
    for token in token_list.iter_mut() {
        if token.get_token_type() == &FormulaTokenTypes::Operand
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Range
        {
            let (sheet_name, range) = split_address(token.get_value());
            if ignore_worksheet
                || (sheet_name.is_empty() && worksheet_name == self_worksheet_name)
                || (sheet_name == worksheet_name)
            {
                let mut coordinate_list_new: Vec<String> = Vec::new();
                let coordinate_list = get_split_range(range);
                for coordinate in &coordinate_list {
                    let cell = index_from_coordinate(coordinate);
                    if cell.0.is_some() {
                        let mut col_num = cell.0.unwrap();
                        let mut row_num = cell.1.unwrap();
                        let is_lock_col = cell.2.unwrap();
                        let is_lock_row = cell.3.unwrap();
                        if !is_lock_col {
                            col_num =
                                adjustment_remove_coordinate(col_num, root_col_num, offset_col_num);
                        }
                        if !is_lock_row {
                            row_num =
                                adjustment_remove_coordinate(row_num, root_row_num, offset_row_num);
                        }
                        let new_corrdinate = coordinate_from_index_with_lock(
                            col_num,
                            row_num,
                            is_lock_col,
                            is_lock_row,
                        );
                        coordinate_list_new.push(new_corrdinate);
                    } else {
                        coordinate_list_new.push((*coordinate).to_string());
                    }
                }
                let new_value = join_address(sheet_name, &get_join_range(&coordinate_list_new));
                token.set_value(new_value);
            }
        }
    }
    render(token_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let formula = "=10+9";
        assert_eq!(
            format!("={}", render(parse_to_tokens(formula).as_ref())),
            formula
        );

        let formula = "=SUM(E7:I7)";
        assert_eq!(
            format!("={}", render(parse_to_tokens(formula).as_ref())),
            formula
        );

        let formula = "=SUM(Sheet2!E7:I7)";
        assert_eq!(
            format!("={}", render(parse_to_tokens(formula).as_ref())),
            formula
        );

        let formula = "=\"TEST\"";
        assert_eq!(
            format!("={}", render(parse_to_tokens(formula).as_ref())),
            formula
        );
    }
}
