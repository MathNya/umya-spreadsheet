use std::collections::HashMap;

lazy_static! {
    pub(crate) static ref FILL_BUILT_IN_FORMAT_CODES: HashMap<usize, String> = {
        let mut map:HashMap<usize, String> = HashMap::new();
        // General
        map.insert(0, NumberFormat::FORMAT_GENERAL.to_string());
        map.insert(1, "0".to_string());
        map.insert(2, "0.00".to_string());
        map.insert(3, "#,##0".to_string());
        map.insert(4, "#,##0.00".to_string());

        map.insert(9, "0%".to_string());
        map.insert(10, "0.00%".to_string());
        map.insert(11, "0.00E+00".to_string());
        map.insert(12, "# ?/?".to_string());
        map.insert(13, "# ??/??".to_string());
        map.insert(14, "m/d/yyyy".to_string()); // Despite ECMA 'mm-dd-yy");
        map.insert(15, "d-mmm-yy".to_string());
        map.insert(16, "d-mmm".to_string());
        map.insert(17, "mmm-yy".to_string());
        map.insert(18, "h:mm AM/PM".to_string());
        map.insert(19, "h:mm:ss AM/PM".to_string());
        map.insert(20, "h:mm".to_string());
        map.insert(21, "h:mm:ss".to_string());
        map.insert(22, "m/d/yyyy h:mm".to_string()); // Despite ECMA 'm/d/yy h:mm");

        map.insert(37, "#,##0_);(#,##0)".to_string()); //  Despite ECMA '#,##0 ;(#,##0)");
        map.insert(38, "#,##0_);[Red](#,##0)".to_string()); //  Despite ECMA '#,##0 ;[Red](#,##0)");
        map.insert(39, "#,##0.00_);(#,##0.00)".to_string()); //  Despite ECMA '#,##0.00;(#,##0.00)");
        map.insert(40, "#,##0.00_);[Red](#,##0.00)".to_string()); //  Despite ECMA '#,##0.00;[Red](#,##0.00)");

        map.insert(44, r###"_("$"* #,##0.00_);_("$"* \(#,##0.00\);_("$"* "-"??_);_(@_)"###.to_string());
        map.insert(45, "mm:ss".to_string());
        map.insert(46, "[h]:mm:ss".to_string());
        map.insert(47, "mm:ss.0".to_string()); //  Despite ECMA 'mmss.0");
        map.insert(48, "##0.0E+0".to_string());
        map.insert(49, "@".to_string());

        // CHT
        map.insert(27, "[$-404]e/m/d".to_string());
        map.insert(30, "m/d/yy".to_string());
        map.insert(36, "[$-404]e/m/d".to_string());
        map.insert(50, "[$-404]e/m/d".to_string());
        map.insert(57, "[$-404]e/m/d".to_string());

        // THA
        map.insert(59, "t0".to_string());
        map.insert(60, "t0.00".to_string());
        map.insert(61, "t#,##0".to_string());
        map.insert(62, "t#,##0.00".to_string());
        map.insert(67, "t0%".to_string());
        map.insert(68, "t0.00%".to_string());
        map.insert(69, "t# ?/?".to_string());
        map.insert(70, "t# ??/??".to_string());

        // JPN
        map.insert(28, r###"[$-411]ggge"年"m"月"d"日""###.to_string());
        map.insert(29, r###"[$-411]ggge"年"m"月"d"日""###.to_string());
        map.insert(31, r###"yyyy"年"m"月"d"日""###.to_string());
        map.insert(32, r###"h"時"mm"分""###.to_string());
        map.insert(33, r###"h"時"mm"分"ss"秒""###.to_string());
        map.insert(34, r###"yyyy"年"m"月""###.to_string());
        map.insert(35, r###"m"月"d"日""###.to_string());
        map.insert(51, r###"[$-411]ggge"年"m"月"d"日""###.to_string());
        map.insert(52, r###"yyyy"年"m"月""###.to_string());
        map.insert(53, r###"m"月"d"日""###.to_string());
        map.insert(54, r###"[$-411]ggge"年"m"月"d"日""###.to_string());
        map.insert(55, r###"yyyy"年"m"月""###.to_string());
        map.insert(56, r###"m"月"d"日""###.to_string());
        map.insert(58, r###"[$-411]ggge"年"m"月"d"日""###.to_string());

        map
    };
}

#[derive(Clone, Debug)]
pub struct NumberFormat {
    flipped_built_in_formats: Vec<String>,
    format_code: String,
    built_in_format_code: Option<usize>,
}
impl Default for NumberFormat {
    fn default() -> Self {
        Self {
            flipped_built_in_formats: Vec::new(),
            format_code: NumberFormat::FORMAT_GENERAL.to_string(),
            built_in_format_code: Some(0)
        }
    }
}
impl NumberFormat {
    // Pre-defined formats
    pub const FORMAT_GENERAL: &'static str = "General";

    pub const FORMAT_TEXT: &'static str = "@";

    pub const FORMAT_NUMBER: &'static str = "0";
    pub const FORMAT_NUMBER_00: &'static str = "0.00";
    pub const FORMAT_NUMBER_COMMA_SEPARATED1: &'static str = "#,##0.00";
    pub const FORMAT_NUMBER_COMMA_SEPARATED2: &'static str = "#,##0.00_-";
    
    pub const FORMAT_PERCENTAGE: &'static str = "0%";
    pub const FORMAT_PERCENTAGE_00: &'static str = "0.00%";
   
    pub const FORMAT_DATE_YYYYMMDD2: &'static str = "yyyy-mm-dd";
    pub const FORMAT_DATE_YYYYMMDD: &'static str = "yyyy-mm-dd";
    pub const FORMAT_DATE_DDMMYYYY: &'static str = "dd/mm/yyyy";
    pub const FORMAT_DATE_DMYSLASH: &'static str = "d/m/yy";
    pub const FORMAT_DATE_DMYMINUS: &'static str = "d-m-yy";
    pub const FORMAT_DATE_DMMINUS: &'static str = "d-m";
    pub const FORMAT_DATE_MYMINUS: &'static str = "m-yy";
    pub const FORMAT_DATE_XLSX14: &'static str = "mm-dd-yy";
    pub const FORMAT_DATE_XLSX15: &'static str = "d-mmm-yy";
    pub const FORMAT_DATE_XLSX16: &'static str = "d-mmm";
    pub const FORMAT_DATE_XLSX17: &'static str = "mmm-yy";
    pub const FORMAT_DATE_XLSX22: &'static str = "m/d/yy h:mm";
    pub const FORMAT_DATE_DATETIME: &'static str = "d/m/yy h:mm";
    pub const FORMAT_DATE_TIME1: &'static str = "h:mm AM/PM";
    pub const FORMAT_DATE_TIME2: &'static str = "h:mm:ss AM/PM";
    pub const FORMAT_DATE_TIME3: &'static str = "h:mm";
    pub const FORMAT_DATE_TIME4: &'static str = "h:mm:ss";
    pub const FORMAT_DATE_TIME5: &'static str = "mm:ss";
    pub const FORMAT_DATE_TIME6: &'static str = "h:mm:ss";
    pub const FORMAT_DATE_TIME7: &'static str = "i:s.S";
    pub const FORMAT_DATE_TIME8: &'static str = "h:mm:ss;@";
    pub const FORMAT_DATE_YYYYMMDDSLASH: &'static str = "yyyy/mm/dd;@";
    
    pub const FORMAT_CURRENCY_USD_SIMPLE: &'static str = r###""$"#,##0.00_-"###;
    pub const FORMAT_CURRENCY_USD: &'static str = r###"$#,##0_-"###;
    pub const FORMAT_CURRENCY_EUR_SIMPLE: &'static str = r###"#,##0.00_-"€""###;
    pub const FORMAT_CURRENCY_EUR: &'static str = r###"#,##0_-"€""###;
    pub const FORMAT_ACCOUNTING_USD: &'static str = r###"_("$"* #,##0.00_);_("$"* \(#,##0.00\);_("$"* "-"??_);_(@_)"###;
    pub const FORMAT_ACCOUNTING_EUR: &'static str = r###"_("€"* #,##0.00_);_("€"* \(#,##0.00\);_("€"* "-"??_);_(@_)"###;

    pub fn get_flipped_built_in_formats(&self)-> &Vec<String> {
        &self.flipped_built_in_formats
    }
    pub(crate) fn set_flipped_built_in_formats(&mut self, value:Vec<String>) {
        self.flipped_built_in_formats = value;
    }
    pub(crate) fn add_flipped_built_in_formats<S: Into<String>>(&mut self, value:S) {
        self.flipped_built_in_formats.push(value.into());
    }
    pub fn get_format_code(&self)-> &str {
        &self.format_code
    }

    /// Set the format code.
    /// # Arguments
    /// * `value` - format code. (umya_spreadsheet::NumberFormat)
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(0);
    /// let _ = worksheet.get_style_mut("C30")
    /// .get_number_format_mut()
    /// .set_format_code(umya_spreadsheet::NumberFormat::FORMAT_DATE_XLSX17);
    /// ```
    pub fn set_format_code<S: Into<String>>(&mut self, value:S) {
        self.format_code = value.into();
        let check:&str = &self.format_code;
        self.built_in_format_code = FILL_BUILT_IN_FORMAT_CODES.iter().find_map(|(key, val)| if val == check { Some(key.clone()) } else { None });
    }

    pub fn get_built_in_format_code(&self)-> &Option<usize> {
        &self.built_in_format_code
    }
    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}",
            &self.format_code,
            match &self.built_in_format_code{Some(v) =>{v.to_string()},None=>{"None".into()}}
        )))
    }
}

