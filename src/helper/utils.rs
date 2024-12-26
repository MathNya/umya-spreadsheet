#[macro_export]
macro_rules! from_err {
    ($from:ty, $to:tt, $var:tt) => {
        impl From<$from> for $to {
            #[inline]
            fn from(e: $from) -> $to {
                $to::$var(e)
            }
        }
    };
}

macro_rules! compile_regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<fancy_regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| fancy_regex::Regex::new($re).unwrap())
    }};
}

pub(crate) use compile_regex;
