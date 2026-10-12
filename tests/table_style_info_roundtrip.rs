use std::io::Cursor;

use umya_spreadsheet::{
    ShowColumn,
    ShowStripes,
    Table,
    TableColumn,
    TableStyleInfo,
    Workbook,
    reader,
    writer,
};

fn add_table(book: &mut Workbook, name: &str, area: (&str, &str), style_info: TableStyleInfo) {
    let mut table = Table::new(name, area);
    table.add_column(TableColumn::new("Name"));
    table.add_column(TableColumn::new("Value"));
    table.set_style_info(Some(style_info));
    book.sheet_mut(0).unwrap().add_table(table);
}

fn roundtrip(book: &Workbook) -> Workbook {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    reader::xlsx::read_reader(Cursor::new(out), true).unwrap()
}

#[test]
fn table_style_info_flags_survive_roundtrip() {
    let mut book = umya_spreadsheet::new_file();
    // Excel's default for a new table: a named style with banded rows
    add_table(
        &mut book,
        "Banded",
        ("A1", "B3"),
        TableStyleInfo::new(
            "TableStyleMedium2",
            ShowColumn::Hide,
            ShowColumn::Hide,
            ShowStripes::Show,
            ShowStripes::Hide,
        ),
    );
    add_table(
        &mut book,
        "Columns",
        ("D1", "E3"),
        TableStyleInfo::new(
            "TableStyleLight9",
            ShowColumn::Show,
            ShowColumn::Show,
            ShowStripes::Hide,
            ShowStripes::Show,
        ),
    );

    let book = roundtrip(&book);
    let tables = book.sheet(0).unwrap().tables();
    assert_eq!(tables.len(), 2);

    let banded = tables[0].style_info().unwrap();
    assert_eq!(banded.name(), "TableStyleMedium2");
    assert!(!banded.is_show_first_col());
    assert!(!banded.is_show_last_col());
    assert!(banded.is_show_row_stripes());
    assert!(!banded.is_show_col_stripes());

    let columns = tables[1].style_info().unwrap();
    assert_eq!(columns.name(), "TableStyleLight9");
    assert!(columns.is_show_first_col());
    assert!(columns.is_show_last_col());
    assert!(!columns.is_show_row_stripes());
    assert!(columns.is_show_col_stripes());
}
