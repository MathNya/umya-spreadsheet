use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;

use ::structs::Spreadsheet;
use ::structs::Borders;
use ::structs::Alignment;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl";
const FILE_NAME: &'static str = "styles.xml";

pub(crate) fn write(spreadsheet: &Spreadsheet, dir: &TempDir) -> Result<(), XlsxError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // styleSheet
    write_start_tag(&mut writer, "styleSheet", vec![
        ("xmlns", "http://schemas.openxmlformats.org/spreadsheetml/2006/main"),
        ("xmlns:mc", "http://schemas.openxmlformats.org/markup-compatibility/2006"),
        ("mc:Ignorable", "x14ac"),
        ("xmlns:x14ac", "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac"),
    ], false);

    let all_nmfmt = spreadsheet.get_all_number_format();

    // numFmts
    if all_nmfmt.len() > 0 {
        write_start_tag(&mut writer, "numFmts", vec![
            ("count", all_nmfmt.len().to_string().as_str()),
        ], false);
        let mut i = 0;
        for (_, nmfmt) in &all_nmfmt {
            // numFmt
            write_start_tag(&mut writer, "numFmt", vec![
                ("numFmtId", (177+i).to_string().as_str()),
                ("formatCode", nmfmt.get_format_code()),
            ], true);
            i += 1;
        }
        write_end_tag(&mut writer, "numFmts");
    }

    let all_font = spreadsheet.get_all_font();

    // fonts
    if all_font.len() > 0 {
        write_start_tag(&mut writer, "fonts", vec![
            ("count", all_font.len().to_string().as_str()),
            ("x14ac:knownFonts", "1"),
        ], false);

        for (_, font) in &all_font {
            // font
            write_start_tag(&mut writer, "font", vec![], false);

            // italic
            if font.get_italic() == &true {
                write_start_tag(&mut writer, "i", vec![], true);
            }

            // strike
            if font.get_strikethrough() == &true {
                write_start_tag(&mut writer, "strike", vec![], true);
            }

            // sz
            write_start_tag(&mut writer, "sz", vec![
                ("val", font.get_size().to_string().as_str()),
            ], true);

            // color
            write_color(&mut writer, &font.get_color(), "color");

            // name
            write_start_tag(&mut writer, "name", vec![
                ("val", font.get_name()),
            ], true);

            // family
            write_start_tag(&mut writer, "family", vec![
                ("val", font.get_family().to_string().as_str()),
            ], true);

            // charset
            let zero:usize = 0;
            if font.get_charset() > &zero {
                write_start_tag(&mut writer, "charset", vec![
                    ("val", font.get_charset().to_string().as_str()),
                ], true);
            }
        
            // scheme
            if font.get_scheme() != "" {
                write_start_tag(&mut writer, "scheme", vec![
                    ("val",  font.get_scheme()),
                ], true);
            }

            write_end_tag(&mut writer, "font");
        }

        write_end_tag(&mut writer, "fonts");
    }

    let all_fill = spreadsheet.get_all_fill();
    
    // fills
    if all_fill.len() > 0 {
        write_start_tag(&mut writer, "fills", vec![
            ("count", all_fill.len().to_string().as_str()),
        ], false);

        for (_, fill) in &all_fill {
            // fill
            write_start_tag(&mut writer, "fill", vec![], false);

            // patternFill
            let fg_color = fill.get_start_color().clone();
            let bg_color = fill.get_end_color().clone();
            let is_fg_color = match fg_color {
                Some(_) => true,
                None => false
            };
            let is_bg_color = match bg_color {
                Some(_) => true,
                None => false
            };
            let is_color = is_fg_color || is_bg_color;
            write_start_tag(&mut writer, "patternFill", vec![
                ("patternType", fill.get_fill_type()),
            ], !is_color);

            if is_fg_color {
                // fgColor
                write_color(&mut writer, &fg_color.unwrap(), "fgColor");
            }
            if is_bg_color {
                // bgColor
                write_color(&mut writer, &bg_color.unwrap(), "bgColor");
            }
            if is_color {
                write_end_tag(&mut writer, "patternFill");
            }

            write_end_tag(&mut writer, "fill");
        }

        write_end_tag(&mut writer, "fills");
    }

    let all_borders = spreadsheet.get_all_borders();

    // borders
    if all_borders.len() > 0 {
        write_start_tag(&mut writer, "borders", vec![
            ("count", all_borders.len().to_string().as_str()),
        ], false);

        for (_, borders) in &all_borders {
            // border
            let is_diagonal_up: bool;
            let is_diagonal_down: bool;
            if borders.get_diagonal_direction() == &Borders::DIAGONAL_NONE {
                is_diagonal_up  = false;
                is_diagonal_down  = false;
            } else if borders.get_diagonal_direction() == &Borders::DIAGONAL_UP {
                is_diagonal_up  = true;
                is_diagonal_down  = false;
            } else if borders.get_diagonal_direction() == &Borders::DIAGONAL_DOWN {
                is_diagonal_up  = false;
                is_diagonal_down  = true;
            } else {
                is_diagonal_up  = true;
                is_diagonal_down  = true;
            }
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            if is_diagonal_up {
                attributes.push(("diagonalUp", "1"));
            }
            if is_diagonal_down {
                attributes.push(("diagonalDown", "1"));
            }
            write_start_tag(&mut writer, "border", attributes, false);

            if borders.get_left().has_border_style() {
                // left
                write_start_tag(&mut writer, "left", vec![
                    ("style", borders.get_left().get_border_style()),
                ], false);

                // color
                write_color(&mut writer, borders.get_left().get_color(), "color");

                write_end_tag(&mut writer, "left");
            } else {
                // left
                write_start_tag(&mut writer, "left", vec![], true);
            }

            if borders.get_right().has_border_style() {
                // right
                write_start_tag(&mut writer, "right", vec![
                    ("style", borders.get_right().get_border_style()),
                ], false);

                // color
                write_color(&mut writer, borders.get_right().get_color(), "color");

                write_end_tag(&mut writer, "right");
            } else {
                // right
                write_start_tag(&mut writer, "right", vec![], true);
            }

            if borders.get_top().has_border_style(){
                // top
                write_start_tag(&mut writer, "top", vec![
                    ("style", borders.get_top().get_border_style()),
                ], false);

                // color
                write_color(&mut writer, borders.get_top().get_color(), "color");
            
                write_end_tag(&mut writer, "top");
            } else {
                // top
                write_start_tag(&mut writer, "top", vec![], true);
            }

            if borders.get_bottom().has_border_style(){
                // bottom
                write_start_tag(&mut writer, "bottom", vec![
                    ("style", borders.get_bottom().get_border_style()),
                ], false);

                // color
                write_color(&mut writer, borders.get_bottom().get_color(), "color");

                write_end_tag(&mut writer, "bottom");
            } else {
                // bottom
                write_start_tag(&mut writer, "bottom", vec![], true);
            }

            if borders.get_diagonal().has_border_style(){
                // diagonal
                write_start_tag(&mut writer, "diagonal", vec![
                    ("style", borders.get_diagonal().get_border_style()),
                ], false);

                // color
                write_color(&mut writer, borders.get_diagonal().get_color(), "color");

                write_end_tag(&mut writer, "diagonal");
            } else {
                // diagonal
                write_start_tag(&mut writer, "diagonal", vec![], true);
            }

            write_end_tag(&mut writer, "border");
        }

        write_end_tag(&mut writer, "borders");
    }

    //cellStyleXfs
    write_start_tag(&mut writer, "cellStyleXfs", vec![
        ("count", "1"),
    ], false);
    write_start_tag(&mut writer, "xf", vec![
        ("numFmtId", "0"),
        ("fontId", "0"),
        ("fillId", "0"),
        ("borderId", "0"),
    ], true);
    write_end_tag(&mut writer, "cellStyleXfs");
    
    // cellXfs
    let all_cell_xf = spreadsheet.get_all_cell_style();
    if all_cell_xf.len() > 0 {
        write_start_tag(&mut writer, "cellXfs", vec![
            ("count", all_cell_xf.len().to_string().as_str()),
        ], false);

        for (_, cell_xf) in all_cell_xf {
            let mut nmfmt_id:usize = 0;
            match cell_xf.get_number_format() {
                Some(v) => {
                    match v.get_built_in_format_code(){
                        Some(code) => {
                            nmfmt_id = code.clone();
                        },
                        None => {
                            for (hash_code, _) in &all_nmfmt {
                                if cell_xf.get_number_format().as_ref().unwrap().get_hash_code().as_str() == hash_code {
                                    nmfmt_id += 177;
                                    break;
                                }
                                nmfmt_id += 1;
                            }
                        }
                    }
                },
                None => {}
            }
            let mut font_id:usize = 0;
            for (hash_code, _) in &all_font {
                match cell_xf.get_font() {
                    Some(v) => {
                        if v.get_hash_code().as_str() == hash_code {
                            break;
                        }
                        font_id += 1;
                    },
                    None => {}
                }
            }
            let mut fill_id:usize = 0;
            for (hash_code, _) in &all_fill {
                match cell_xf.get_fill() {
                    Some(v) => {
                        if v.get_hash_code().as_str() == hash_code {
                            break;
                        }
                        fill_id += 1;
                    },
                    None => {}
                }
            }
            let mut borders_id:usize = 0;
            for (hash_code, _) in &all_borders {
                match cell_xf.get_borders() {
                    Some(v) => {
                        if v.get_hash_code().as_str() == hash_code {
                            break;
                        }
                        borders_id += 1;
                    },
                    None => {}
                }
            }
            // xf
            let use_cell_style = cell_xf.get_xf_id() != &0;
            let is_align_empty = match cell_xf.get_alignment() {
                Some(v) => {v.is_empty()},
                None => true
            } || use_cell_style;
            let nmfmt_id_str:&str = &nmfmt_id.to_string();
            let font_id_str:&str = &font_id.to_string();
            let fill_id_str:&str = &fill_id.to_string();
            let borders_id_str:&str = &borders_id.to_string();
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            attributes.push(("numFmtId", nmfmt_id_str));
            attributes.push(("fontId", font_id_str));
            attributes.push(("fillId", fill_id_str));
            attributes.push(("borderId", borders_id_str));
            attributes.push(("xfId", "0"));
            match cell_xf.get_number_format() {
                Some(_) => {
                    attributes.push(("applyNumberFormat", "1"));
                },
                None => {}
            }
            match cell_xf.get_font() {
                Some(_) => {
                    if use_cell_style == false {
                        attributes.push(("applyFont", "1"));
                    }
                },
                None => {}
            }
            match cell_xf.get_borders() {
                Some(_) => {
                    attributes.push(("applyBorder", "1"));
                },
                None => {}
            }
            match cell_xf.get_alignment() {
                Some(_) => {
                    attributes.push(("applyAlignment", "1"));
                },
                None => {}
            }
            write_start_tag(&mut writer, "xf", attributes, is_align_empty);

            // alignment
            if is_align_empty == false {
                match cell_xf.get_alignment(){
                    Some(v) => {
                        let mut attributes: Vec<(&str, &str)> = Vec::new();
                        if v.get_horizontal() != Alignment::HORIZONTAL_GENERAL {
                            attributes.push(("horizontal", v.get_horizontal()));
                        }
                        if v.get_vertical() != "" {
                            attributes.push(("vertical", v.get_vertical()));
                        }
                        if v.get_wrap_text() == &true {
                            attributes.push(("wrapText", "1"));
                        }
                        write_start_tag(&mut writer, "alignment", attributes, true);

                        write_end_tag(&mut writer, "xf");
                    },
                    None => {}
                }
            }
        }

        write_end_tag(&mut writer, "cellXfs");
    }

    // cellStyles
    write_start_tag(&mut writer, "cellStyles", vec![
        ("count", "1"),
    ], false);
    write_start_tag(&mut writer, "cellStyle", vec![
        ("name", "normal"),
        ("xfId", "0"),
        ("builtinId", "0"),
    ], true);
    write_end_tag(&mut writer, "cellStyles");

    let all_conditional_style = spreadsheet.get_all_conditional_style_list();
    match all_conditional_style.len() > 0 {
        true => {
            // dxfs
            write_start_tag(&mut writer, "dxfs", vec![
                ("count", all_conditional_style.len().to_string().as_str()),
            ], false);

            for (_, style) in &all_conditional_style {
                // dxf
                write_start_tag(&mut writer, "dxf", vec![], false);

                // font
                match style.get_font() {
                    Some(v) => {
                        write_start_tag(&mut writer, "font", vec![], false);
                        
                        // color
                        write_color(&mut writer, v.get_color(), "color");

                        write_end_tag(&mut writer, "font");
                    }
                    None => {}
                }

                // fill
                match style.get_fill() {
                    Some(v) => {
                        write_start_tag(&mut writer, "fill", vec![], false);
                        
                        // patternFill
                        let fg_color = v.get_start_color().clone();
                        let bg_color = v.get_end_color().clone();
                        let is_fg_color = match fg_color {
                            Some(_) => true,
                            None => false
                        };
                        let is_bg_color = match bg_color {
                            Some(_) => true,
                            None => false
                        };
                        let is_color = is_fg_color || is_bg_color;
                        write_start_tag(&mut writer, "patternFill", vec![], !is_color);
            
                        if is_fg_color {
                            // fgColor
                            write_color(&mut writer, &fg_color.unwrap(), "fgColor");
                        }
                        if is_bg_color {
                            // bgColor
                            write_color(&mut writer, &bg_color.unwrap(), "bgColor");
                        }
                        if is_color {
                            write_end_tag(&mut writer, "patternFill");
                        }

                        write_end_tag(&mut writer, "fill");
                    }
                    None => {}
                }

                write_end_tag(&mut writer, "dxf");

                write_end_tag(&mut writer, "dxfs");
            }
        },
        false => {
            // dxfs
            write_start_tag(&mut writer, "dxfs", vec![
                ("count", "0"),
            ], true);
        }
    }

    // tableStyles
    write_start_tag(&mut writer, "tableStyles", vec![
        ("count", "0"),
        ("defaultTableStyle", "TableStyleMedium2"),
        ("defaultPivotStyle", "PivotStyleMedium9"),
    ], true);

    // extLst
    write_start_tag(&mut writer, "extLst", vec![], false);

    // ext
    write_start_tag(&mut writer, "ext", vec![
        ("uri", "{EB79DEF2-80B8-43e5-95BD-54CBDDF9020C}"),
        ("xmlns:x14", "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main"),
    ], false);

    // x14:slicerStyles
    write_start_tag(&mut writer, "x14:slicerStyles", vec![
        ("defaultSlicerStyle", "SlicerStyleLight1"),
    ], true);

    write_end_tag(&mut writer, "ext");

    write_end_tag(&mut writer, "extLst");

    write_end_tag(&mut writer, "styleSheet");

    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, FILE_NAME).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}
