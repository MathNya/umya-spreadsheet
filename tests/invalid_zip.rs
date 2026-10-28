use std::io::Cursor;

use umya_spreadsheet::{
    reader::xlsx::read_reader,
    structs::XlsxErrorKind,
};

#[test]
fn read_reader_returns_zip_error_for_non_zip_input() {
    for input in [&[][..], b"not a zip archive", b"PK\x03\x04"] {
        assert_eq!(
            read_reader(Cursor::new(input), true)
                .map(|_| ())
                .unwrap_err()
                .kind(),
            XlsxErrorKind::Zip,
            "expected ZIP input error for {input:?}"
        );
    }
}
