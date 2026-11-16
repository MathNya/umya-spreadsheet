//! A chart on a sheet whose name has an apostrophe used to panic on write.

use umya_spreadsheet::{
    Chart,
    ChartType,
    drawing::spreadsheet::MarkerType,
    new_file,
    writer,
};

#[test]
fn chart_on_sheet_with_apostrophe_writes() {
    let mut book = new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.set_name("O'Brien");
    sheet.cell_mut("A1").set_value_number(1);
    sheet.cell_mut("A2").set_value_number(2);

    let mut from_marker = MarkerType::default();
    let mut to_marker = MarkerType::default();
    from_marker.set_coordinate("C1");
    to_marker.set_coordinate("H10");
    let mut chart = Chart::default();
    chart.new_chart(
        &ChartType::LineChart,
        from_marker,
        to_marker,
        vec!["'O''Brien'!$A$1:$A$2"],
    );
    book.sheet_mut(0).unwrap().add_chart(chart);

    let mut out = Vec::new();
    writer::xlsx::write_writer(&book, &mut out).unwrap();
}
