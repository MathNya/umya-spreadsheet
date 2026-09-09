//! Reproducible probe for repeated access to an already loaded lazy worksheet.
//!
//! Run with `cargo run --release --example loaded_sheet_access_probe --
//! <styles> <accesses>`. The defaults are 10,000 styles and 10,000 accesses.
//! This reports elapsed time but deliberately makes no timing assertion:
//! results depend on hardware and allocator.
use std::{
    env,
    fs,
    path::PathBuf,
    process,
    time::Instant,
};

use umya_spreadsheet::{
    reader,
    writer,
};

fn argument(index: usize, default: usize) -> usize {
    env::args()
        .nth(index)
        .map(|value| value.parse().expect("arguments must be positive integers"))
        .unwrap_or(default)
}

fn main() {
    let styles = argument(1, 10_000);
    let accesses = argument(2, 10_000);
    let path: PathBuf = env::temp_dir().join(format!(
        "umya-loaded-sheet-access-probe-{}.xlsx",
        process::id()
    ));

    let mut source = umya_spreadsheet::new_file();
    let sheet = source.sheet_mut(0).unwrap();
    for row in 1..=styles {
        sheet.cell_mut((1, row as u32)).set_value_number(row as u32);
        sheet
            .style_mut((1, row as u32))
            .font_mut()
            .color_mut()
            .set_argb_str(format!("FF{:06X}", row % 0x01_00_00_00));
    }
    writer::xlsx::write(&source, &path).unwrap();

    let mut book = reader::xlsx::lazy_read(&path).unwrap();
    let initial = Instant::now();
    assert_eq!(book.sheet_mut(0).unwrap().value("A1"), "1");
    let initial_elapsed = initial.elapsed();

    let repeated = Instant::now();
    for _ in 0..accesses {
        assert_eq!(book.sheet_mut(0).unwrap().value("A1"), "1");
    }
    let repeated_elapsed = repeated.elapsed();

    println!("styles={styles} accesses={accesses}");
    println!("initial_deserialize_ms={}", initial_elapsed.as_millis());
    println!("loaded_sheet_access_ms={}", repeated_elapsed.as_millis());
    fs::remove_file(path).unwrap();
}
