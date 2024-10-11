use std::borrow::Cow;

pub(crate) fn format_as_percentage<'input>(value: &f64, format: &'input str) -> Cow<'input, str> {
    let mut value = value.to_string();
    let mut format = Cow::Borrowed(format);
    format = Cow::Owned(format.replace('%', ""));
    let blocks: Vec<&str> = format.split('.').collect();
    let len = match blocks.get(1) {
        Some(v) => v.len(),
        None => 0,
    };
    value = format!(
        "{:0width$.len$}%",
        (100f64 * &value.parse::<f64>().unwrap()).round(),
        width = 1,
        len = len
    );
    Cow::Owned(value)
}
