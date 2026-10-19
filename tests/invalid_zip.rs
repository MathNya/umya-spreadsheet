use std::io::Cursor;

use umya_spreadsheet::{
    reader::xlsx::read_reader,
    structs::XlsxError,
};

#[test]
fn read_reader_returns_zip_error_for_non_zip_input() {
    for input in [&[][..], b"not a zip archive", b"PK\x03\x04"] {
        assert!(
            matches!(
                read_reader(Cursor::new(input), true),
                Err(XlsxError::Zip(_))
            ),
            "expected ZIP input error for {input:?}"
        );
    }
}
