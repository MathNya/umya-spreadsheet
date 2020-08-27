use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;

use super::super::structs::worksheet::Worksheet;
use super::super::helper::coordinate::*;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl/drawings";

pub(crate) fn write(
    worksheet: &Worksheet,
    p_worksheet_id: &str,
    chart_start_id: &usize,
    dir: &TempDir
) -> Result<(), XlsxError> 
{
    if worksheet.has_drawing_object() == false {
        return Ok(());
    }

    let file_name = format!("drawing{}.xml", p_worksheet_id);
    let charts = worksheet.get_chart_collection();

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // xdr:wsDr
    write_start_tag(&mut writer, "xdr:wsDr", vec![
        ("xmlns:xdr", "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing"),
        ("xmlns:a", "http://schemas.openxmlformats.org/drawingml/2006/main"),
    ], false);

    let mut id = 1;
    let mut chart_id = chart_start_id.clone();
    for chart in charts {
        // xdr:twoCellAnchor
        write_start_tag(&mut writer, "xdr:twoCellAnchor", vec![], false);

        // xdr:from
        write_start_tag(&mut writer, "xdr:from", vec![], false);

        // xdr:col
        let from = coordinate_from_string(chart.get_top_left_cell());
        write_start_tag(&mut writer, "xdr:col", vec![], false);
        write_text_node(&mut writer, column_index_from_string(from[0]).to_string().as_str());
        write_end_tag(&mut writer, "xdr:col");

        // xdr:colOff
        write_start_tag(&mut writer, "xdr:colOff", vec![], false);
        write_text_node(&mut writer, chart.get_top_left_x_offset().to_string().as_str());
        write_end_tag(&mut writer, "xdr:colOff");
        
        // xdr:row
        write_start_tag(&mut writer, "xdr:row", vec![], false);
        write_text_node(&mut writer, from[1]);
        write_end_tag(&mut writer, "xdr:row");

        // xdr:rowOff
        write_start_tag(&mut writer, "xdr:rowOff", vec![], false);
        write_text_node(&mut writer, chart.get_top_left_y_offset().to_string().as_str());
        write_end_tag(&mut writer, "xdr:rowOff");

        write_end_tag(&mut writer, "xdr:from");

        // xdr:to
        write_start_tag(&mut writer, "xdr:to", vec![], false);

        // xdr:col
        let to = coordinate_from_string(chart.get_bottom_right_cell());
        write_start_tag(&mut writer, "xdr:col", vec![], false);
        write_text_node(&mut writer, column_index_from_string(to[0]).to_string().as_str());
        write_end_tag(&mut writer, "xdr:col");

        // xdr:colOff
        write_start_tag(&mut writer, "xdr:colOff", vec![], false);
        write_text_node(&mut writer, chart.get_bottom_right_x_offset().to_string().as_str());
        write_end_tag(&mut writer, "xdr:colOff");
        
        // xdr:row
        write_start_tag(&mut writer, "xdr:row", vec![], false);
        write_text_node(&mut writer, to[1]);
        write_end_tag(&mut writer, "xdr:row");

        // xdr:rowOff
        write_start_tag(&mut writer, "xdr:rowOff", vec![], false);
        write_text_node(&mut writer, chart.get_bottom_right_y_offset().to_string().as_str());
        write_end_tag(&mut writer, "xdr:rowOff");

        write_end_tag(&mut writer, "xdr:to");

        // xdr:graphicFrame
        write_start_tag(&mut writer, "xdr:graphicFrame", vec![
            ("macro", ""),
        ], false);
        
        // xdr:nvGraphicFramePr
        write_start_tag(&mut writer, "xdr:nvGraphicFramePr", vec![], false);

        // xdr:cNvPr
        write_start_tag(&mut writer, "xdr:cNvPr", vec![
            ("id", chart_id.to_string().as_str()),
            ("name", chart.get_name()),
        ], true);

        // xdr:cNvGraphicFramePr
        write_start_tag(&mut writer, "xdr:cNvGraphicFramePr", vec![], true);

        write_end_tag(&mut writer, "xdr:nvGraphicFramePr");

        // xdr:xfrm
        write_start_tag(&mut writer, "xdr:xfrm", vec![], false);

        // a:off
        write_start_tag(&mut writer, "a:off", vec![
            ("x", "0"),
            ("y", "0"),
        ], true);

        // a:ext
        write_start_tag(&mut writer, "a:ext", vec![
            ("cx", "0"),
            ("cy", "0"),
        ], true);

        write_end_tag(&mut writer, "xdr:xfrm");

        // a:graphic
        write_start_tag(&mut writer, "a:graphic", vec![], false);

        // a:graphicData
        write_start_tag(&mut writer, "a:graphicData", vec![
            ("uri", "http://schemas.openxmlformats.org/drawingml/2006/chart"),
        ], false);

        // c:chart
        write_start_tag(&mut writer, "c:chart", vec![
            ("xmlns:c", "http://schemas.openxmlformats.org/drawingml/2006/chart"),
            ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
            ("r:id", format!("rId{}", id).as_str()),
        ], true);

        write_end_tag(&mut writer, "a:graphicData");

        write_end_tag(&mut writer, "a:graphic");

        write_end_tag(&mut writer, "xdr:graphicFrame");

        // xdr:clientData
        write_start_tag(&mut writer, "xdr:clientData", vec![], true);

        write_end_tag(&mut writer, "xdr:twoCellAnchor");

        id += 1;
        chart_id += 1;
    }

    write_end_tag(&mut writer, "xdr:wsDr");

    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}
