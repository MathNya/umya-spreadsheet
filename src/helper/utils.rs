#[macro_export]
macro_rules! from_err {
    ($from:ty, $to:tt, $var:tt) => {
        impl From<$from> for $to {
            fn from(e: $from) -> $to {
                $to::$var(e)
            }
        }
    };
}
