use fancy_regex::{Captures, Regex};
use helper::coordinate::*;
use structs::StringValue;

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

pub struct FormulaToken
{
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

const QUOTE_DOUBLE: &str = "\"";
const QUOTE_SINGLE: &str = "'";
const BRACKET_CLOSE: &str = "]";
const BRACKET_OPEN: &str = "[";
const BRACE_OPEN: &str = "{";
const BRACE_CLOSE: &str = "}";
const PAREN_OPEN: &str = "(";
const PAREN_CLOSE: &str = ")";
const SEMICOLON: &str = ";";
const WHITESPACE: &str = " ";
const COMMA: &str = ",";
const ERROR_START: &str = "#";

const OPERATORS_SN: &str = "+-";
const OPERATORS_INFIX: &str = "+-*/^&=><";
const OPERATORS_POSTFIX: &str = "%";

const ERRORS: &[String] = ["#NULL!", "#DIV/0!", "#VALUE!", "#REF!", "#NAME?", "#NUM!", "#N/A"];
const COMPARATORS_MULTI: &[String] = [">=", "<=", "<>"];

pub(crate) fn parse_to_tokens<S: Into<String>>(formula: S)-> Vec<FormulaToken>
{
    let formula = formula.into();
    let formula_length = formula.len();
    if formula_length < 2 || formula.chars().next().unwrap() != "=" {
        return;
    }

    // Helper variables
    let mut tokens1: Vec<FormulaToken> = Vec::new();
    let mut tokens2: Vec<Vec<FormulaToken>> = Vec::new();
    let mut stack: Vec<FormulaToken> = Vec::new();
    
    let mut in_string = false;
    let mut in_path = false;
    let mut in_range = false;
    let mut in_error = false;
    let mut next_token:Option<FormulaToken> = None;

    let mut index = 1;
    let mut value = String::from("");

    while index < formula_length {
        // double-quoted strings
        // embeds are doubled
        // end marks token
        if in_string {
            if formula[index] == self::QUOTE_DOUBLE {
                if ((index + 2) <= formula_length) && (formula[index + 1] == self::QUOTE_DOUBLE) {
                    value += self::QUOTE_DOUBLE;
                    index += 1;
                } else {
                    in_string = false;
                    let mut obj = FormulaToken::default();
                    obj.set_value(value);
                    obj.set_token_type(FormulaToken::Operand);
                    obj.set_token_sub_type(FormulaTokenSubTypes::Text);
                    tokens1.push(obj);
                    value = String::from("");
                }
            } else {
                value += formula[index];
            }
            index += 1;

            continue;
        }

        // single-quoted strings (links)
        // embeds are double
        // end does not mark a token
        if in_path {
            if formula[index] == self::QUOTE_SINGLE {
                if ((index + 2) <= formula_length) && (formula[index + 1] == self::QUOTE_SINGLE) {
                    value += self::QUOTE_SINGLE;
                    index += 1;
                } else {
                    in_path = false;
                }
            } else {
                value += formula[index];
            }
            index += 1;

            continue;
        }

        // bracked strings (R1C1 range index or linked workbook name)
        // no embeds (changed to "()" by Excel)
        // end does not mark a token
        if in_range {
            if formula[index] == self::BRACKET_CLOSE {
                in_range = false;
            }
            value += formula[index];
            index;

            continue;
        }

        // error values
        // end marks a token, determined from absolute list of values
        if in_error {
            value += formula[index];
            index += 1;
            if in_array(value, self::ERRORS) {
                in_error = false;
                let mut obj = FormulaToken::default();
                obj.set_value(value);
                obj.set_token_type(FormulaToken::Operand);
                obj.set_token_sub_type(FormulaTokenSubTypes::Error);
                tokens1.push(obj);
                value = String::from("");
            }

            continue;
        }

        // scientific notation check
        if (str_contains(self::OPERATORS_SN, $this->formula[$index])) {
            if (strlen($value) > 1) {
                if (preg_match('/^[1-9]{1}(\\.\\d+)?E{1}$/', $this->formula[$index]) != 0) {
                    $value .= $this->formula[$index];
                    ++$index;

                    continue;
                }
            }
        }

        // independent character evaluation (order not important)

        // establish state-dependent character evaluations
        if ($this->formula[$index] == self::QUOTE_DOUBLE) {
            if ($value !== '') {
                // unexpected
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_UNKNOWN);
                $value = '';
            }
            $inString = true;
            ++$index;

            continue;
        }

        if ($this->formula[$index] == self::QUOTE_SINGLE) {
            if ($value !== '') {
                // unexpected
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_UNKNOWN);
                $value = '';
            }
            $inPath = true;
            ++$index;

            continue;
        }

        if ($this->formula[$index] == self::BRACKET_OPEN) {
            $inRange = true;
            $value .= self::BRACKET_OPEN;
            ++$index;

            continue;
        }

        if ($this->formula[$index] == self::ERROR_START) {
            if ($value !== '') {
                // unexpected
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_UNKNOWN);
                $value = '';
            }
            $inError = true;
            $value .= self::ERROR_START;
            ++$index;

            continue;
        }

        // mark start and end of arrays and array rows
        if ($this->formula[$index] == self::BRACE_OPEN) {
            if ($value !== '') {
                // unexpected
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_UNKNOWN);
                $value = '';
            }

            $tmp = new FormulaToken('ARRAY', FormulaToken::TOKEN_TYPE_FUNCTION, FormulaToken::TOKEN_SUBTYPE_START);
            $tokens1[] = $tmp;
            $stack[] = clone $tmp;

            $tmp = new FormulaToken('ARRAYROW', FormulaToken::TOKEN_TYPE_FUNCTION, FormulaToken::TOKEN_SUBTYPE_START);
            $tokens1[] = $tmp;
            $stack[] = clone $tmp;

            ++$index;

            continue;
        }

        if ($this->formula[$index] == self::SEMICOLON) {
            if ($value !== '') {
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERAND);
                $value = '';
            }

            /** @var FormulaToken $tmp */
            $tmp = array_pop($stack);
            $tmp->setValue('');
            $tmp->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_STOP);
            $tokens1[] = $tmp;

            $tmp = new FormulaToken(',', FormulaToken::TOKEN_TYPE_ARGUMENT);
            $tokens1[] = $tmp;

            $tmp = new FormulaToken('ARRAYROW', FormulaToken::TOKEN_TYPE_FUNCTION, FormulaToken::TOKEN_SUBTYPE_START);
            $tokens1[] = $tmp;
            $stack[] = clone $tmp;

            ++$index;

            continue;
        }

        if ($this->formula[$index] == self::BRACE_CLOSE) {
            if ($value !== '') {
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERAND);
                $value = '';
            }

            /** @var FormulaToken $tmp */
            $tmp = array_pop($stack);
            $tmp->setValue('');
            $tmp->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_STOP);
            $tokens1[] = $tmp;

            /** @var FormulaToken $tmp */
            $tmp = array_pop($stack);
            $tmp->setValue('');
            $tmp->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_STOP);
            $tokens1[] = $tmp;

            ++$index;

            continue;
        }

        // trim white-space
        if ($this->formula[$index] == self::WHITESPACE) {
            if ($value !== '') {
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERAND);
                $value = '';
            }
            $tokens1[] = new FormulaToken('', FormulaToken::TOKEN_TYPE_WHITESPACE);
            ++$index;
            while (($this->formula[$index] == self::WHITESPACE) && ($index < $formulaLength)) {
                ++$index;
            }

            continue;
        }

        // multi-character comparators
        if (($index + 2) <= $formulaLength) {
            if (in_array(substr($this->formula, $index, 2), $COMPARATORS_MULTI)) {
                if ($value !== '') {
                    $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERAND);
                    $value = '';
                }
                $tokens1[] = new FormulaToken(substr($this->formula, $index, 2), FormulaToken::TOKEN_TYPE_OPERATORINFIX, FormulaToken::TOKEN_SUBTYPE_LOGICAL);
                $index += 2;

                continue;
            }
        }

        // standard infix operators
        if (str_contains(self::OPERATORS_INFIX, $this->formula[$index])) {
            if ($value !== '') {
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERAND);
                $value = '';
            }
            $tokens1[] = new FormulaToken($this->formula[$index], FormulaToken::TOKEN_TYPE_OPERATORINFIX);
            ++$index;

            continue;
        }

        // standard postfix operators (only one)
        if (str_contains(self::OPERATORS_POSTFIX, $this->formula[$index])) {
            if ($value !== '') {
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERAND);
                $value = '';
            }
            $tokens1[] = new FormulaToken($this->formula[$index], FormulaToken::TOKEN_TYPE_OPERATORPOSTFIX);
            ++$index;

            continue;
        }

        // start subexpression or function
        if ($this->formula[$index] == self::PAREN_OPEN) {
            if ($value !== '') {
                $tmp = new FormulaToken($value, FormulaToken::TOKEN_TYPE_FUNCTION, FormulaToken::TOKEN_SUBTYPE_START);
                $tokens1[] = $tmp;
                $stack[] = clone $tmp;
                $value = '';
            } else {
                $tmp = new FormulaToken('', FormulaToken::TOKEN_TYPE_SUBEXPRESSION, FormulaToken::TOKEN_SUBTYPE_START);
                $tokens1[] = $tmp;
                $stack[] = clone $tmp;
            }
            ++$index;

            continue;
        }

        // function, subexpression, or array parameters, or operand unions
        if ($this->formula[$index] == self::COMMA) {
            if ($value !== '') {
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERAND);
                $value = '';
            }

            /** @var FormulaToken $tmp */
            $tmp = array_pop($stack);
            $tmp->setValue('');
            $tmp->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_STOP);
            $stack[] = $tmp;

            if ($tmp->getTokenType() == FormulaToken::TOKEN_TYPE_FUNCTION) {
                $tokens1[] = new FormulaToken(',', FormulaToken::TOKEN_TYPE_OPERATORINFIX, FormulaToken::TOKEN_SUBTYPE_UNION);
            } else {
                $tokens1[] = new FormulaToken(',', FormulaToken::TOKEN_TYPE_ARGUMENT);
            }
            ++$index;

            continue;
        }

        // stop subexpression
        if ($this->formula[$index] == self::PAREN_CLOSE) {
            if ($value !== '') {
                $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERAND);
                $value = '';
            }

            /** @var FormulaToken $tmp */
            $tmp = array_pop($stack);
            $tmp->setValue('');
            $tmp->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_STOP);
            $tokens1[] = $tmp;

            ++$index;

            continue;
        }

        // token accumulation
        $value .= $this->formula[$index];
        ++$index;
    }

    // dump remaining accumulation
    if ($value !== '') {
        $tokens1[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERAND);
    }

    // move tokenList to new set, excluding unnecessary white-space tokens and converting necessary ones to intersections
    $tokenCount = count($tokens1);
    for ($i = 0; $i < $tokenCount; ++$i) {
        $token = $tokens1[$i];
        if (isset($tokens1[$i - 1])) {
            $previousToken = $tokens1[$i - 1];
        } else {
            $previousToken = null;
        }
        if (isset($tokens1[$i + 1])) {
            $nextToken = $tokens1[$i + 1];
        } else {
            $nextToken = null;
        }

        if ($token->getTokenType() != FormulaToken::TOKEN_TYPE_WHITESPACE) {
            $tokens2[] = $token;

            continue;
        }

        if ($previousToken === null) {
            continue;
        }

        if (
            !(
                (($previousToken->getTokenType() == FormulaToken::TOKEN_TYPE_FUNCTION) && ($previousToken->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_STOP))
            || (($previousToken->getTokenType() == FormulaToken::TOKEN_TYPE_SUBEXPRESSION) && ($previousToken->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_STOP))
            || ($previousToken->getTokenType() == FormulaToken::TOKEN_TYPE_OPERAND)
            )
        ) {
            continue;
        }

        if ($nextToken === null) {
            continue;
        }

        if (
            !(
                (($nextToken->getTokenType() == FormulaToken::TOKEN_TYPE_FUNCTION) && ($nextToken->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_START))
            || (($nextToken->getTokenType() == FormulaToken::TOKEN_TYPE_SUBEXPRESSION) && ($nextToken->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_START))
            || ($nextToken->getTokenType() == FormulaToken::TOKEN_TYPE_OPERAND)
            )
        ) {
            continue;
        }

        $tokens2[] = new FormulaToken($value, FormulaToken::TOKEN_TYPE_OPERATORINFIX, FormulaToken::TOKEN_SUBTYPE_INTERSECTION);
    }

    // move tokens to final list, switching infix "-" operators to prefix when appropriate, switching infix "+" operators
    // to noop when appropriate, identifying operand and infix-operator subtypes, and pulling "@" from function names
    $this->tokens = [];

    $tokenCount = count($tokens2);
    for ($i = 0; $i < $tokenCount; ++$i) {
        $token = $tokens2[$i];
        if (isset($tokens2[$i - 1])) {
            $previousToken = $tokens2[$i - 1];
        } else {
            $previousToken = null;
        }

        if ($token->getTokenType() == FormulaToken::TOKEN_TYPE_OPERATORINFIX && $token->getValue() == '-') {
            if ($i == 0) {
                $token->setTokenType(FormulaToken::TOKEN_TYPE_OPERATORPREFIX);
            } elseif (
                (($previousToken?->getTokenType() == FormulaToken::TOKEN_TYPE_FUNCTION)
                    && ($previousToken?->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_STOP))
                || (($previousToken?->getTokenType() == FormulaToken::TOKEN_TYPE_SUBEXPRESSION)
                    && ($previousToken?->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_STOP))
                || ($previousToken?->getTokenType() == FormulaToken::TOKEN_TYPE_OPERATORPOSTFIX)
                || ($previousToken?->getTokenType() == FormulaToken::TOKEN_TYPE_OPERAND)
            ) {
                $token->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_MATH);
            } else {
                $token->setTokenType(FormulaToken::TOKEN_TYPE_OPERATORPREFIX);
            }

            $this->tokens[] = $token;

            continue;
        }

        if ($token->getTokenType() == FormulaToken::TOKEN_TYPE_OPERATORINFIX && $token->getValue() == '+') {
            if ($i == 0) {
                continue;
            } elseif (
                (($previousToken?->getTokenType() == FormulaToken::TOKEN_TYPE_FUNCTION)
                    && ($previousToken?->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_STOP))
                || (($previousToken?->getTokenType() == FormulaToken::TOKEN_TYPE_SUBEXPRESSION)
                    && ($previousToken?->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_STOP))
                || ($previousToken?->getTokenType() == FormulaToken::TOKEN_TYPE_OPERATORPOSTFIX)
                || ($previousToken?->getTokenType() == FormulaToken::TOKEN_TYPE_OPERAND)
            ) {
                $token->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_MATH);
            } else {
                continue;
            }

            $this->tokens[] = $token;

            continue;
        }

        if (
            $token->getTokenType() == FormulaToken::TOKEN_TYPE_OPERATORINFIX
            && $token->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_NOTHING
        ) {
            if (str_contains('<>=', substr($token->getValue(), 0, 1))) {
                $token->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_LOGICAL);
            } elseif ($token->getValue() == '&') {
                $token->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_CONCATENATION);
            } else {
                $token->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_MATH);
            }

            $this->tokens[] = $token;

            continue;
        }

        if (
            $token->getTokenType() == FormulaToken::TOKEN_TYPE_OPERAND
            && $token->getTokenSubType() == FormulaToken::TOKEN_SUBTYPE_NOTHING
        ) {
            if (!is_numeric($token->getValue())) {
                if (strtoupper($token->getValue()) == 'TRUE' || strtoupper($token->getValue()) == 'FALSE') {
                    $token->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_LOGICAL);
                } else {
                    $token->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_RANGE);
                }
            } else {
                $token->setTokenSubType(FormulaToken::TOKEN_SUBTYPE_NUMBER);
            }

            $this->tokens[] = $token;

            continue;
        }

        if ($token->getTokenType() == FormulaToken::TOKEN_TYPE_FUNCTION) {
            if ($token->getValue() !== '') {
                if (str_starts_with($token->getValue(), '@')) {
                    $token->setValue(substr($token->getValue(), 1));
                }
            }
        }

        $this->tokens[] = $token;
    }
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
