use crate::{
    helper::{
        address::{join_address, split_address},
        coordinate::{
            adjustment_insert_coordinate, adjustment_remove_coordinate,
            coordinate_from_index_with_lock, index_from_coordinate,
        },
        range::{get_join_range, get_split_range},
        utils::compile_regex,
    },
    structs::StringValue,
};

/// PARTLY BASED ON: */
/// Copyright (c) 2007 E. W. Bachtal, Inc. */
/// <https://ewbi.blogs.com/develops/2007/03/excel_formula_p.html> */
/// <https://ewbi.blogs.com/develops/2004/12/excel_formula_p.html>

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

macro_rules! token {
    ($value:expr, $typ:expr, $sub:expr) => {{
        let mut obj = FormulaToken::default();
        obj.set_value($value);
        obj.set_token_type($typ);
        obj.set_token_sub_type($sub);
        obj
    }};
}

pub(crate) fn parse_to_tokens<S: Into<String>>(formula: S) -> Vec<FormulaToken> {
    let formula_str = formula.into();
    let formula_length = formula_str.chars().count();

    // quick checks
    if formula_length < 2 || !formula_str.starts_with('=') {
        return Vec::new();
    }

    let (_, tokens2) = parse_into_intermediate_tokens(&formula_str);
    finalize_tokens(&tokens2)
}

fn parse_into_intermediate_tokens(formula: &str) -> (Vec<FormulaToken>, Vec<FormulaToken>) {
    let mut tokens1: Vec<FormulaToken> = Vec::new();
    let mut tokens2: Vec<FormulaToken> = Vec::new();
    let mut stack: Vec<FormulaToken> = Vec::new();

    // state flags
    let mut in_string = false;
    let mut in_path = false;
    let mut in_range = false;
    let mut in_error = false;

    let mut index = 1;
    let mut value = String::new();
    let formula_length = formula.chars().count();

    while index < formula_length {
        // double-quoted strings
        if in_string {
            handle_in_string(
                formula,
                &mut index,
                &mut value,
                &mut tokens1,
                &mut in_string,
            );
            continue;
        }

        // single-quoted strings (links)
        if in_path {
            handle_in_path(formula, &mut index, &mut value, &mut in_path);
            continue;
        }

        // bracketed strings (R1C1 range index or workbook name)
        if in_range {
            handle_in_range(formula, &mut index, &mut value, &mut in_range);
            continue;
        }

        // error values
        if in_error {
            handle_in_error(formula, &mut index, &mut value, &mut tokens1, &mut in_error);
            continue;
        }

        // scientific notation check
        if handle_scientific_notation(formula, &mut index, &mut value) {
            continue;
        }

        // handle special characters
        if handle_special_characters(
            formula,
            &mut index,
            &mut value,
            &mut tokens1,
            &mut stack,
            &mut in_string,
            &mut in_range,
            &mut in_error,
        ) {
            continue;
        }

        // accumulate as normal char
        value.push(formula.chars().nth(index).unwrap());
        index += 1;
    }

    // dump remaining
    if !value.is_empty() {
        let mut obj = FormulaToken::default();
        obj.set_value(value.clone());
        obj.set_token_type(FormulaTokenTypes::Operand);
        tokens1.push(obj);
    }

    // trim whitespace, handle intersection
    cleanup_tokens(&tokens1, &mut tokens2);

    (tokens1, tokens2)
}

fn finalize_tokens(tokens_in: &[FormulaToken]) -> Vec<FormulaToken> {
    let mut tokens: Vec<FormulaToken> = Vec::new();
    let token_count = tokens_in.len();
    let mut previous_token: Option<FormulaToken>;

    for i in 0..token_count {
        let mut token = tokens_in[i].clone();
        if i > 0 {
            previous_token = Some(tokens_in[i - 1].clone());
        } else {
            previous_token = None;
        }

        // switch infix "-" to prefix
        if token.get_token_type() == &FormulaTokenTypes::OperatorInfix && token.get_value() == "-" {
            if i == 0 {
                token.set_token_type(FormulaTokenTypes::OperatorPrefix);
            } else if should_token_be_math(&previous_token) {
                token.set_token_sub_type(FormulaTokenSubTypes::Math);
            } else {
                token.set_token_type(FormulaTokenTypes::OperatorPrefix);
            }
            tokens.push(token.clone());
            continue;
        }

        // switch infix "+" to prefix/noop
        if token.get_token_type() == &FormulaTokenTypes::OperatorInfix && token.get_value() == "+" {
            if i == 0 {
                // skip
                continue;
            } else if should_token_be_math(&previous_token) {
                token.set_token_sub_type(FormulaTokenSubTypes::Math);
            } else {
                // skip
                continue;
            }
            tokens.push(token.clone());
            continue;
        }

        // set operator subtypes
        if token.get_token_type() == &FormulaTokenTypes::OperatorInfix
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Nothing
        {
            if let Some(c) = token.get_value().chars().next() {
                if "<>=".contains(c) {
                    token.set_token_sub_type(FormulaTokenSubTypes::Logical);
                } else if c == '&' {
                    token.set_token_sub_type(FormulaTokenSubTypes::Concatenation);
                } else {
                    token.set_token_sub_type(FormulaTokenSubTypes::Math);
                }
            }
            tokens.push(token.clone());
            continue;
        }

        // set operand subtypes
        if token.get_token_type() == &FormulaTokenTypes::Operand
            && token.get_token_sub_type() == &FormulaTokenSubTypes::Nothing
        {
            if token.get_value().parse::<f64>().is_ok() {
                token.set_token_sub_type(FormulaTokenSubTypes::Number);
            } else if ["TRUE", "FALSE"].contains(&token.get_value().to_uppercase().as_str()) {
                token.set_token_sub_type(FormulaTokenSubTypes::Logical);
            } else {
                token.set_token_sub_type(FormulaTokenSubTypes::Range);
            }
            tokens.push(token.clone());
            continue;
        }

        // remove leading '@' from function
        if let FormulaTokenTypes::Function = token.get_token_type() {
            let val = token.get_value();
            if val.starts_with('@') {
                token.set_value(val.trim_start_matches('@').to_string());
            }
        }

        tokens.push(token.clone());
    }
    tokens
}

// --------------------------------------------------------------------------
// Below are the smaller "handler" helper functions referenced above.
// --------------------------------------------------------------------------

fn handle_in_string(
    formula: &str,
    index: &mut usize,
    value: &mut String,
    tokens: &mut Vec<FormulaToken>,
    in_string: &mut bool,
) {
    if formula.chars().nth(*index).unwrap() == QUOTE_DOUBLE {
        // double quote check
        if (*index + 2) <= formula.chars().count()
            && formula.chars().nth(*index + 1).unwrap() == QUOTE_DOUBLE
        {
            *value = format!("{value}{QUOTE_DOUBLE}");
            *index += 1;
        } else {
            *in_string = false;
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Operand);
            obj.set_token_sub_type(FormulaTokenSubTypes::Text);
            tokens.push(obj);
            *value = String::new();
        }
    } else {
        value.push(formula.chars().nth(*index).unwrap());
    }
    *index += 1;
}

fn handle_in_path(formula: &str, index: &mut usize, value: &mut String, in_path: &mut bool) {
    if formula.chars().nth(*index).unwrap() == QUOTE_SINGLE {
        if (*index + 2) <= formula.chars().count()
            && formula.chars().nth(*index + 1).unwrap() == QUOTE_SINGLE
        {
            *value = format!("{value}{QUOTE_SINGLE}");
            *index += 1;
        } else {
            *in_path = false;
        }
    } else {
        value.push(formula.chars().nth(*index).unwrap());
    }
    *index += 1;
}

fn handle_in_range(formula: &str, index: &mut usize, value: &mut String, in_range: &mut bool) {
    if formula.chars().nth(*index).unwrap() == BRACKET_CLOSE {
        *in_range = false;
    }
    value.push(formula.chars().nth(*index).unwrap());
    *index += 1;
}

fn handle_in_error(
    formula: &str,
    index: &mut usize,
    value: &mut String,
    tokens: &mut Vec<FormulaToken>,
    in_error: &mut bool,
) {
    value.push(formula.chars().nth(*index).unwrap());
    *index += 1;
    if ERRORS.iter().any(|&x| x == value.as_str()) {
        *in_error = false;
        let mut obj = FormulaToken::default();
        obj.set_value(value.clone());
        obj.set_token_type(FormulaTokenTypes::Operand);
        obj.set_token_sub_type(FormulaTokenSubTypes::Error);
        tokens.push(obj);
        *value = String::new();
    }
}

fn handle_scientific_notation(formula: &str, index: &mut usize, value: &mut String) -> bool {
    if let Some(current_char) = formula.chars().nth(*index) {
        if OPERATORS_SN.contains(current_char)
            && value.len() > 1
            && compile_regex!(r"/^[1-9]{1}(\\.\\d+)?E{1}$/")
                .is_match(&current_char.to_string())
                .unwrap_or(false)
        {
            value.push(current_char);
            *index += 1;
            return true;
        }
    }
    false
}

#[allow(clippy::too_many_arguments)]
fn handle_special_characters(
    formula: &str,
    index: &mut usize,
    value: &mut String,
    tokens1: &mut Vec<FormulaToken>,
    stack: &mut Vec<FormulaToken>,
    in_string: &mut bool,
    in_range: &mut bool,
    in_error: &mut bool,
) -> bool {
    let current_char = formula.chars().nth(*index).unwrap();

    // handle double quote
    if current_char == QUOTE_DOUBLE {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Unknown);
            tokens1.push(obj);
            value.clear();
        }
        *in_string = true;
        *index += 1;
        return true;
    }

    // handle single quote
    if current_char == QUOTE_SINGLE {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Unknown);
            tokens1.push(obj);
            value.clear();
        }
        *in_string = true;
        *index += 1;
        return true;
    }

    // handle bracket open
    if current_char == BRACKET_OPEN {
        *in_range = true;
        value.push(BRACKET_OPEN);
        *index += 1;
        return true;
    }

    // handle error start
    if current_char == ERROR_START {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Unknown);
            tokens1.push(obj);
            value.clear();
        }
        *in_error = true;
        value.push(ERROR_START);
        *index += 1;
        return true;
    }

    // handle braces and semicolons (array constructs)
    if handle_array_chars(current_char, value, tokens1, stack) {
        *index += 1;
        return true;
    }

    // handle whitespace
    if current_char == WHITESPACE {
        handle_whitespace(value, tokens1);
        *index += 1;
        // skip consecutive spaces
        let len = formula.chars().count();
        while *index < len && formula.chars().nth(*index).unwrap() == WHITESPACE {
            *index += 1;
        }
        return true;
    }

    // handle multi-character comparators
    if handle_multi_char_comparator(formula, index, value, tokens1) {
        return true;
    }

    // handle infix operators
    if OPERATORS_INFIX.contains(current_char) {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Operand);
            tokens1.push(obj);
            value.clear();
        }
        let mut obj = FormulaToken::default();
        obj.set_value(current_char);
        obj.set_token_type(FormulaTokenTypes::OperatorInfix);
        tokens1.push(obj);
        *index += 1;
        return true;
    }

    // handle postfix operators
    if OPERATORS_POSTFIX.contains(current_char) {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Operand);
            tokens1.push(obj);
            value.clear();
        }
        let mut obj = FormulaToken::default();
        obj.set_value(current_char);
        obj.set_token_type(FormulaTokenTypes::OperatorPostfix);
        tokens1.push(obj);
        *index += 1;
        return true;
    }

    // handle subexpression / function start
    if current_char == PAREN_OPEN {
        if value.is_empty() {
            let obj = token!(
                "",
                FormulaTokenTypes::Subexpression,
                FormulaTokenSubTypes::Start
            );
            tokens1.push(obj.clone());
            stack.push(obj);
        } else {
            let obj = token!(
                value.clone(),
                FormulaTokenTypes::Function,
                FormulaTokenSubTypes::Start
            );
            tokens1.push(obj.clone());
            stack.push(obj);
            value.clear();
        }
        *index += 1;
        return true;
    }

    // handle function/operand unions
    if current_char == COMMA {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Operand);
            tokens1.push(obj);
            value.clear();
        }

        let mut obj = stack.pop().unwrap();
        obj.set_value("");
        obj.set_token_sub_type(FormulaTokenSubTypes::Stop);
        stack.push(obj.clone());

        if obj.get_token_type() == &FormulaTokenTypes::Function {
            let mut op = FormulaToken::default();
            op.set_value(",");
            op.set_token_type(FormulaTokenTypes::OperatorInfix);
            op.set_token_sub_type(FormulaTokenSubTypes::Union);
            tokens1.push(op);
        } else {
            let mut arg = FormulaToken::default();
            arg.set_value(",");
            arg.set_token_type(FormulaTokenTypes::Argument);
            tokens1.push(arg);
        }
        *index += 1;
        return true;
    }

    // handle subexpression/function stop
    if current_char == PAREN_CLOSE {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Operand);
            tokens1.push(obj);
            value.clear();
        }
        let mut obj = stack.pop().unwrap();
        obj.set_value("");
        obj.set_token_sub_type(FormulaTokenSubTypes::Stop);
        tokens1.push(obj);
        *index += 1;
        return true;
    }

    false
}

fn handle_array_chars(
    current_char: char,
    value: &mut String,
    tokens1: &mut Vec<FormulaToken>,
    stack: &mut Vec<FormulaToken>,
) -> bool {
    // handle {
    if current_char == BRACE_OPEN {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Unknown);
            tokens1.push(obj);
            value.clear();
        }
        let arr_start = token!(
            "ARRAY",
            FormulaTokenTypes::Function,
            FormulaTokenSubTypes::Start
        );
        tokens1.push(arr_start.clone());
        stack.push(arr_start);

        let arr_row = token!(
            "ARRAYROW",
            FormulaTokenTypes::Function,
            FormulaTokenSubTypes::Start
        );
        tokens1.push(arr_row.clone());
        stack.push(arr_row);
        return true;
    }

    // handle ;
    if current_char == SEMICOLON {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Operand);
            tokens1.push(obj);
            value.clear();
        }
        let mut obj = stack.pop().unwrap();
        obj.set_value("");
        obj.set_token_sub_type(FormulaTokenSubTypes::Stop);
        tokens1.push(obj);

        let mut comma = FormulaToken::default();
        comma.set_value(",");
        comma.set_token_type(FormulaTokenTypes::Argument);
        tokens1.push(comma);

        let arr_row = token!(
            "ARRAYROW",
            FormulaTokenTypes::Function,
            FormulaTokenSubTypes::Start
        );
        tokens1.push(arr_row.clone());
        stack.push(arr_row);
        return true;
    }

    // handle }
    if current_char == BRACE_CLOSE {
        if !value.is_empty() {
            let mut obj = FormulaToken::default();
            obj.set_value(value.clone());
            obj.set_token_type(FormulaTokenTypes::Operand);
            tokens1.push(obj);
            value.clear();
        }
        let mut obj_end = stack.pop().unwrap();
        obj_end.set_value("");
        obj_end.set_token_sub_type(FormulaTokenSubTypes::Stop);
        tokens1.push(obj_end);

        let mut arr_end = stack.pop().unwrap();
        arr_end.set_value("");
        arr_end.set_token_sub_type(FormulaTokenSubTypes::Stop);
        tokens1.push(arr_end);
        return true;
    }

    false
}

fn handle_whitespace(value: &mut String, tokens1: &mut Vec<FormulaToken>) {
    if !value.is_empty() {
        let mut obj = FormulaToken::default();
        obj.set_value(value.clone());
        obj.set_token_type(FormulaTokenTypes::Operand);
        tokens1.push(obj);
        value.clear();
    }
    let mut space = FormulaToken::default();
    space.set_value("");
    space.set_token_type(FormulaTokenTypes::Whitespace);
    tokens1.push(space);
}

fn handle_multi_char_comparator(
    formula: &str,
    index: &mut usize,
    value: &mut String,
    tokens1: &mut Vec<FormulaToken>,
) -> bool {
    let formula_length = formula.chars().count();
    if (*index + 2) <= formula_length {
        let next_two = formula.chars().skip(*index).take(2).collect::<String>();
        if COMPARATORS_MULTI.iter().any(|&x| x == next_two) {
            // flush current value
            if !value.is_empty() {
                let mut obj = FormulaToken::default();
                obj.set_value(value.clone());
                obj.set_token_type(FormulaTokenTypes::Operand);
                tokens1.push(obj);
                value.clear();
            }
            let mut obj = FormulaToken::default();
            obj.set_value(next_two);
            obj.set_token_type(FormulaTokenTypes::OperatorInfix);
            obj.set_token_sub_type(FormulaTokenSubTypes::Logical);
            tokens1.push(obj);
            *index += 2;
            return true;
        }
    }
    false
}

fn cleanup_tokens(tokens1: &[FormulaToken], tokens2: &mut Vec<FormulaToken>) {
    let token_count = tokens1.len();
    let mut value = String::new();
    for i in 0..token_count {
        let token = &tokens1[i];
        if token.get_token_type() != &FormulaTokenTypes::Whitespace {
            tokens2.push(token.clone());
            continue;
        }

        let mut previous_token: Option<&FormulaToken> = None;
        let mut next_token: Option<&FormulaToken> = None;

        if i > 0 {
            previous_token = tokens1.get(i - 1);
        }
        if i + 1 < token_count {
            next_token = tokens1.get(i + 1);
        }

        if let Some(p) = previous_token {
            if !is_operand_or_close(p) {
                continue;
            }
        }
        if let Some(n) = next_token {
            if !is_operand_or_open(n) {
                continue;
            }
        }
        tokens2.push(token!(
            value.clone(),
            FormulaTokenTypes::OperatorInfix,
            FormulaTokenSubTypes::Intersection
        ));
        value.clear();

        if let Some(n) = next_token {
            if !is_operand_or_open(n) {
                continue;
            }
        }
        tokens2.push(token!(
            value.clone(),
            FormulaTokenTypes::OperatorInfix,
            FormulaTokenSubTypes::Intersection
        ));
        value.clear();
    }
}

#[allow(clippy::ref_option)]
fn should_token_be_math(previous_token: &Option<FormulaToken>) -> bool {
    if previous_token.is_none() {
        return false;
    }
    let t = previous_token.as_ref().unwrap();
    if (t.get_token_type() == &FormulaTokenTypes::Function
        && t.get_token_sub_type() == &FormulaTokenSubTypes::Stop)
        || (t.get_token_type() == &FormulaTokenTypes::Subexpression
            && t.get_token_sub_type() == &FormulaTokenSubTypes::Stop)
        || (t.get_token_type() == &FormulaTokenTypes::OperatorPostfix)
        || (t.get_token_type() == &FormulaTokenTypes::Operand)
    {
        return true;
    }
    false
}

fn is_operand_or_close(token: &FormulaToken) -> bool {
    ((token.get_token_type() == &FormulaTokenTypes::Function)
        && (token.get_token_sub_type() == &FormulaTokenSubTypes::Stop))
        || ((token.get_token_type() == &FormulaTokenTypes::Subexpression)
            && (token.get_token_sub_type() == &FormulaTokenSubTypes::Stop))
        || (token.get_token_type() == &FormulaTokenTypes::Operand)
}

fn is_operand_or_open(token: &FormulaToken) -> bool {
    ((token.get_token_type() == &FormulaTokenTypes::Function)
        && (token.get_token_sub_type() == &FormulaTokenSubTypes::Start))
        || ((token.get_token_type() == &FormulaTokenTypes::Subexpression)
            && (token.get_token_sub_type() == &FormulaTokenSubTypes::Start))
        || (token.get_token_type() == &FormulaTokenTypes::Operand)
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
                        let calc_col_num =
                            num_traits::cast::<_, i32>(col_num).unwrap() + offset_col_num;
                        if calc_col_num < 1 {
                            has_error = true;
                            break;
                        }
                        col_num = num_traits::cast::<_, u32>(calc_col_num).unwrap();
                    }
                    if !is_lock_row {
                        let calc_row_num =
                            num_traits::cast::<_, i32>(row_num).unwrap() + offset_row_num;
                        if calc_row_num < 1 {
                            has_error = true;
                            break;
                        }
                        row_num = num_traits::cast::<_, u32>(calc_row_num).unwrap();
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
