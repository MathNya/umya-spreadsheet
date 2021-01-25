use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;

use super::super::structs::worksheet::Worksheet;
use super::super::structs::comment::Comment;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl/drawings";

pub(crate) fn write(
    worksheet: &Worksheet,
    p_worksheet_id: &str,
    dir: &TempDir
) -> Result<(), XlsxError> {
    if worksheet.get_comments().len() == 0 {
        return Ok(());
    }

    let file_name = format!("vmlDrawing{}.vml", p_worksheet_id);

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // xml
    write_start_tag(&mut writer, "xml", vec![
        ("xmlns:v", "urn:schemas-microsoft-com:vml"),
        ("xmlns:o", "urn:schemas-microsoft-com:office:office"),
        ("xmlns:x", "urn:schemas-microsoft-com:office:excel"),
    ], false);

    // o:shapelayout
    write_start_tag(&mut writer, "o:shapelayout", vec![
        ("v:ext", "edit"),
    ], false);

    // o:idmap
    write_start_tag(&mut writer, "o:idmap", vec![
        ("v:ext", "edit"),
        ("data", "1"),
    ], true);

    write_end_tag(&mut writer, "o:shapelayout");

    // v:shapetype
    write_start_tag(&mut writer, "v:shapetype", vec![
        ("id", "_x0000_t202"),
        ("coordsize", "21600,21600"),
        ("o:spt", "202"),
        ("path", "m,l,21600r21600,l21600,xe"),
    ], false);

    // v:stroke
    write_start_tag(&mut writer, "v:stroke", vec![
        ("joinstyle", "miter"),
    ], true);

    // v:path
    write_start_tag(&mut writer, "v:path", vec![
        ("gradientshapeok", "t"),
        ("o:connecttype", "rect"),
    ], true);

    write_end_tag(&mut writer, "v:shapetype");

    let mut id = 1025;
    for comment in worksheet.get_comments() {
        // v:shape
        write_start_tag(&mut writer, "v:shape", vec![
            ("id", format!("_x0000_s{}", id).to_string().as_str()),
            ("type", "#_x0000_t202"),
            ("style", get_style_string(comment).as_str()),
            ("fillcolor", format!("#{}", comment.get_fill_color().get_argb()).as_str()),
            ("o:insetmode", "auto"),
        ], false);

        // v:fill
        write_start_tag(&mut writer, "v:fill", vec![
            ("color2", format!("#{}", comment.get_fill_color().get_argb()).as_str()),
        ], true);

        // v:shadow
        write_start_tag(&mut writer, "v:shadow", vec![
            ("on", "t"),
            ("color", "black"),
            ("obscured", "t"),
        ], true);

        // v:path
        write_start_tag(&mut writer, "v:path", vec![
            ("o:connecttype", "none"),
        ], true);

        // v:textbox
        write_start_tag(&mut writer, "v:textbox", vec![
            ("style", "mso-direction-alt:auto"),
        ], false);

        // div
        write_start_tag(&mut writer, "div", vec![
            ("style", "text-align:left"),
        ], false);
        write_end_tag(&mut writer, "div");

        write_end_tag(&mut writer, "v:textbox");
        
        // x:ClientData
        write_start_tag(&mut writer, "x:ClientData", vec![
            ("ObjectType", "Note"),
        ], false);

        // x:MoveWithCells
        write_start_tag(&mut writer, "x:MoveWithCells", vec![], true);

        // x:SizeWithCells
        write_start_tag(&mut writer, "x:SizeWithCells", vec![], true);

        // x:Anchor
        let anchor = format!("{}, {}, {}, {}, {}, {}, {}, {}",
            comment.get_anchor().get_left_column(),
            comment.get_anchor().get_left_offset(),
            comment.get_anchor().get_top_row(),
            comment.get_anchor().get_top_offset(),
            comment.get_anchor().get_right_column(),
            comment.get_anchor().get_right_offset(),
            comment.get_anchor().get_bottom_row(),
            comment.get_anchor().get_bottom_offset()
        );
        write_start_tag(&mut writer, "x:Anchor", vec![], false);
        write_text_node(&mut writer, anchor.as_str());
        write_end_tag(&mut writer, "x:Anchor");

        // x:AutoFill
        write_start_tag(&mut writer, "x:AutoFill", vec![], false);
        write_text_node(&mut writer, "False");
        write_end_tag(&mut writer, "x:AutoFill");

        let col = comment.get_coordinate().get_col_num();
        let row = comment.get_coordinate().get_row_num();

        // x:Row
        write_start_tag(&mut writer, "x:Row", vec![], false);
        write_text_node(&mut writer, (row - 1).to_string().as_str());
        write_end_tag(&mut writer, "x:Row");

        // x:Column
        write_start_tag(&mut writer, "x:Column", vec![], false);
        write_text_node(&mut writer, (col -1).to_string().as_str());
        write_end_tag(&mut writer, "x:Column");

        // x:Visible
        if comment.get_visible() == &true {
            write_start_tag(&mut writer, "x:Visible", vec![], true);
        }

        write_end_tag(&mut writer, "x:ClientData");
        write_end_tag(&mut writer, "v:shape");

        id += 1;
    }

    write_end_tag(&mut writer, "xml");

    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}

fn get_style_string(comment: &Comment) -> String {
    let style = format!(
        "position:absolute;margin-left:{};margin-top:{};width:{};height:{};z-index:1;visibility:{}",
        comment.get_margin_left(),
        comment.get_margin_top(),
        comment.get_width(),
        comment.get_height(),
        if comment.get_visible() == &true {"visible"} else {"hidden"}
    );
    style
}
