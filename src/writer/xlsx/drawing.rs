use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;

use super::super::structs::drawing::preset_geometry::PresetGeometry;
use super::super::structs::drawing::solid_fill::SolidFill;
use super::super::structs::drawing::transform2d::Transform2D;
use super::super::structs::drawing::spreadsheet::shape_style::ShapeStyle;
use super::super::structs::drawing::scheme_color::SchemeColor;
use super::super::structs::worksheet::Worksheet;
use super::super::structs::anchor::Anchor;
use super::super::structs::drawing::spreadsheet::run_properties::RunProperties;
use super::super::structs::drawing::spreadsheet::shape_properties::ShapeProperties;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl/drawings";

pub(crate) fn write(
    worksheet: &Worksheet,
    p_worksheet_id: &str,
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

    let mut r_id = 1;
    for chart in charts {
        // xdr:twoCellAnchor
        write_start_tag(&mut writer, "xdr:twoCellAnchor", vec![], false);

        // xdr:from
        // xdr:to
        write_anchor(&mut writer, chart.get_anchor());

        // xdr:graphicFrame
        write_start_tag(&mut writer, "xdr:graphicFrame", vec![
            ("macro", ""),
        ], false);
        
        // xdr:nvGraphicFramePr
        write_start_tag(&mut writer, "xdr:nvGraphicFramePr", vec![], false);

        // xdr:cNvPr
        let obj = chart.get_non_visual_drawing_properties();
        write_start_tag(&mut writer, "xdr:cNvPr", vec![
            ("id", obj.get_id()),
            ("name", obj.get_name()),
        ], true);

        // xdr:cNvGraphicFramePr
        write_start_tag(&mut writer, "xdr:cNvGraphicFramePr", vec![], true);

        write_end_tag(&mut writer, "xdr:nvGraphicFramePr");

        // xdr:xfrm
        write_start_tag(&mut writer, "xdr:xfrm", vec![], false);

        // a:off
        let obj = chart.get_transform();
        write_start_tag(&mut writer, "a:off", vec![
            ("x", obj.get_x().to_string().as_str()),
            ("y", obj.get_y().to_string().as_str()),
        ], true);
        
        // a:ext
        write_start_tag(&mut writer, "a:ext", vec![
            ("cx", obj.get_width().to_string().as_str()),
            ("cy", obj.get_height().to_string().as_str()),
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
            ("r:id", format!("rId{}", r_id).as_str()),
        ], true);

        write_end_tag(&mut writer, "a:graphicData");

        write_end_tag(&mut writer, "a:graphic");

        write_end_tag(&mut writer, "xdr:graphicFrame");

        // xdr:clientData
        write_start_tag(&mut writer, "xdr:clientData", vec![], true);

        write_end_tag(&mut writer, "xdr:twoCellAnchor");

        r_id += 1;
    }

    let shapes = worksheet.get_shape_collection();
    for shape in shapes {
        // xdr:twoCellAnchor
        write_start_tag(&mut writer, "xdr:twoCellAnchor", vec![], false);

        // xdr:from
        // xdr:to
        write_anchor(&mut writer, shape.get_anchor());

        // xdr:sp
        write_start_tag(&mut writer, "xdr:sp", vec![
            ("macro", ""),
            ("textlink", ""),
        ], false);

        // xdr:nvSpPr
        write_start_tag(&mut writer, "xdr:nvSpPr", vec![], false);

        // xdr:cNvPr
        let obj = shape.get_non_visual_shape_properties().get_non_visual_drawing_properties();
        write_start_tag(&mut writer, "xdr:cNvPr", vec![
            ("id", obj.get_id()),
            ("name", obj.get_name()),
        ], true);

        // xdr:cNvSpPr
        write_start_tag(&mut writer, "xdr:cNvSpPr", vec![], true);

        write_end_tag(&mut writer, "xdr:nvSpPr");

        // xdr:spPr
        let obj = shape.get_shape_properties();
        write_shape_properties(&mut writer, obj);

        // xdr:style
        let obj = shape.get_shape_style();
        write_shape_style(&mut writer, obj);

        // xdr:txBody
        write_start_tag(&mut writer, "xdr:txBody", vec![], false);

        // a:bodyPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match shape.get_text_body().get_body_properties().get_vert_overflow() {
            Some(v) => attributes.push(("vertOverflow", v)),
            None => {}
        }
        match shape.get_text_body().get_body_properties().get_horz_overflow() {
            Some(v) => attributes.push(("horzOverflow", v)),
            None => {}
        }
        match shape.get_text_body().get_body_properties().get_rtl_col() {
            Some(v) => attributes.push(("rtlCol", v)),
            None => {}
        }
        match shape.get_text_body().get_body_properties().get_anchor() {
            Some(v) => attributes.push(("anchor", v)),
            None => {}
        }
        write_start_tag(&mut writer, "a:bodyPr", attributes, true);

        // a:lstStyle
        write_start_tag(&mut writer, "a:lstStyle", vec![], true);

        for content in shape.get_text_body().get_text_body_contentes() {
            // a:p
            write_start_tag(&mut writer, "a:p", vec![], false);

            // a:pPr
            write_start_tag(&mut writer, "a:pPr", vec![
                ("algn", content.get_algn()),
            ], true);

            for run_propertie in content.get_run_properties() {
                // a:r
                write_start_tag(&mut writer, "a:r", vec![], false);

                // a:rPr
                write_run_properties(&mut writer, "a:rPr", run_propertie);
    
                // a:t
                write_start_tag(&mut writer, "a:t", vec![], false);
                write_text_node(&mut writer, run_propertie.get_text());
                write_end_tag(&mut writer, "a:t");

                write_end_tag(&mut writer, "a:r");
            }

            // a:endParaRPr
            match content.get_end_para_run_properties() {
                Some(v) => write_run_properties(&mut writer, "a:endParaRPr", v),
                None => {}
            }

            write_end_tag(&mut writer, "a:p");
        }

        write_end_tag(&mut writer, "xdr:txBody");
        write_end_tag(&mut writer, "xdr:sp");

        // xdr:clientData
        write_start_tag(&mut writer, "xdr:clientData", vec![], true);

        write_end_tag(&mut writer, "xdr:twoCellAnchor");
    }

    let connection_shapes = worksheet.get_connection_shape_collection();
    for connection_shape in connection_shapes {
        // xdr:twoCellAnchor
        write_start_tag(&mut writer, "xdr:twoCellAnchor", vec![], false);

        // xdr:from
        // xdr:to
        write_anchor(&mut writer, connection_shape.get_anchor());

        // xdr:cxnSp
        write_start_tag(&mut writer, "xdr:cxnSp", vec![
            ("macro", ""),
        ], false);

        // xdr:nvCxnSpPr
        write_start_tag(&mut writer, "xdr:nvCxnSpPr", vec![], false);

        // xdr:cNvPr
        let obj = connection_shape.get_non_visual_connection_shape_properties().get_non_visual_drawing_properties();
        write_start_tag(&mut writer, "xdr:cNvPr", vec![
            ("id", obj.get_id()),
            ("name", obj.get_name()),
        ], true);

        // xdr:cNvCxnSpPr
        let obj = connection_shape.get_non_visual_connection_shape_properties().get_non_visual_connector_shape_drawing_properties();
        if obj.get_start_connection().is_some() || obj.get_end_connection().is_some() {
            write_start_tag(&mut writer, "xdr:cNvCxnSpPr", vec![], false);

            // a:stCxn
            match obj.get_start_connection() {
                Some(v) => {
                    write_start_tag(&mut writer, "a:stCxn", vec![
                        ("id", v.get_id()),
                        ("idx", v.get_index()),
                    ], true);
                },
                None => {}
            }

            // a:endCxn
            match obj.get_end_connection() {
                Some(v) => {
                    write_start_tag(&mut writer, "a:endCxn", vec![
                        ("id", v.get_id()),
                        ("idx", v.get_index()),
                    ], true);
                },
                None => {}
            }

            write_end_tag(&mut writer, "xdr:cNvCxnSpPr");
        } else {
            write_start_tag(&mut writer, "xdr:cNvCxnSpPr", vec![], true);
        }

        write_end_tag(&mut writer, "xdr:nvCxnSpPr");

        // xdr:spPr
        let obj = connection_shape.get_shape_properties();
        write_shape_properties(&mut writer, obj);

        // xdr:style
        let obj = connection_shape.get_shape_style();
        write_shape_style(&mut writer, obj);

        write_end_tag(&mut writer, "xdr:cxnSp");

        // xdr:clientData
        write_start_tag(&mut writer, "xdr:clientData", vec![], true);

        write_end_tag(&mut writer, "xdr:twoCellAnchor");
    }

    write_end_tag(&mut writer, "xdr:wsDr");

    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}

fn write_anchor(writer: &mut Writer<Cursor<Vec<u8>>>, anchor: &Anchor) {
    // xdr:from
    write_start_tag(writer, "xdr:from", vec![], false);

    // xdr:col
    write_start_tag(writer, "xdr:col", vec![], false);
    write_text_node(writer, anchor.get_left_column().to_string().as_str());
    write_end_tag(writer, "xdr:col");

    // xdr:colOff
    write_start_tag(writer, "xdr:colOff", vec![], false);
    write_text_node(writer, anchor.get_left_offset().to_string().as_str());
    write_end_tag(writer, "xdr:colOff");
        
    // xdr:row
    write_start_tag(writer, "xdr:row", vec![], false);
    write_text_node(writer, anchor.get_top_row().to_string().as_str());
    write_end_tag(writer, "xdr:row");

    // xdr:rowOff
    write_start_tag(writer, "xdr:rowOff", vec![], false);
    write_text_node(writer, anchor.get_top_offset().to_string().as_str());
    write_end_tag(writer, "xdr:rowOff");

    write_end_tag(writer, "xdr:from");

    // xdr:to
    write_start_tag(writer, "xdr:to", vec![], false);

    // xdr:col
    write_start_tag(writer, "xdr:col", vec![], false);
    write_text_node(writer, anchor.get_right_column().to_string().as_str());
    write_end_tag(writer, "xdr:col");

    // xdr:colOff
    write_start_tag(writer, "xdr:colOff", vec![], false);
    write_text_node(writer, anchor.get_right_offset().to_string().as_str());
    write_end_tag(writer, "xdr:colOff");
        
    // xdr:row
     write_start_tag(writer, "xdr:row", vec![], false);
    write_text_node(writer, anchor.get_bottom_row().to_string().as_str());
    write_end_tag(writer, "xdr:row");

    // xdr:rowOff
    write_start_tag(writer, "xdr:rowOff", vec![], false);
    write_text_node(writer, anchor.get_bottom_offset().to_string().as_str());
    write_end_tag(writer, "xdr:rowOff");

    write_end_tag(writer, "xdr:to");
}

// xdr:spPr
fn write_shape_properties(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    shape_properties: &ShapeProperties
) {
    write_start_tag(writer, "xdr:spPr", vec![], false);
        
    // a:xfrm
    let obj = shape_properties.get_transform2d();
    write_transform2d(writer, obj);

    // a:prstGeom
    let obj = shape_properties.get_geometry();
    write_preset_geometry(writer, obj);

    // a:solidFill
    let obj = shape_properties.get_solid_fill();
    match obj {
        Some(v) => {
            write_solid_fill(writer, v);
        },
        None => {}
    }

    // a:ln
    let obj = shape_properties.get_outline();
    match obj {
        Some(v) => {
            write_start_tag(writer, "a:ln", vec![], false);
            write_start_tag(writer, "a:tailEnd", vec![
                ("type", v.get_tail_end().get_type()),
            ], true);
            write_end_tag(writer, "a:ln");
        },
        None => {}
    }

    write_end_tag(writer, "xdr:spPr");
}

// a:xfrm
fn write_transform2d(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    transform2d: &Transform2D
) {
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    match transform2d.get_rot() {
        Some(v) => attributes.push(("rot", v)),
        None => {}
    }
    match transform2d.get_flip_h() {
        Some(v) => attributes.push(("flipH", v)),
        None => {}
    }
    match transform2d.get_flip_v() {
        Some(v) => attributes.push(("flipV", v)),
        None => {}
    }
    write_start_tag(writer, "a:xfrm", attributes, false);

    // a:off
    write_start_tag(writer, "a:off", vec![
        ("x", transform2d.get_x().to_string().as_str()),
        ("y", transform2d.get_y().to_string().as_str()),
    ], true);

    // a:ext
    write_start_tag(writer, "a:ext", vec![
        ("cx", transform2d.get_width().to_string().as_str()),
        ("cy", transform2d.get_height().to_string().as_str()),
    ], true);

    write_end_tag(writer, "a:xfrm");
}

// a:prstGeom
fn write_preset_geometry(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    preset_geometry: &PresetGeometry
) {
    write_start_tag(writer, "a:prstGeom", vec![
        ("prst", preset_geometry.get_geometry()),
    ], false);

    // a:avLst
    let obj = preset_geometry.get_adjust_value_list().get_shape_guide_collection();
    if obj.len() > 0 {
        write_start_tag(writer, "a:avLst", vec![], false);
        for gd in obj {
            write_start_tag(writer, "a:gd", vec![
                ("name", gd.get_name()),
                ("fmla", gd.get_fmla()),
            ], true);
        }
        write_end_tag(writer, "a:avLst");
    } else {
        write_start_tag(writer, "a:avLst", vec![], true);
    }

    write_end_tag(writer, "a:prstGeom");
}

// a:solidFill
fn write_solid_fill(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    solid_fill: &SolidFill
) {
    write_start_tag(writer, "a:solidFill", vec![], false);
    match solid_fill.get_scheme_color() {
        Some(color) => {
            write_scheme_color(writer, color);
        },
        None => {}
    }
    match solid_fill.get_rgb_color_model_hex() {
        Some(hex) => {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            match hex.get_val() {
                Some(v) => attributes.push(("val", v)),
                None => {}
            }
            match hex.get_r() {
                Some(v) => attributes.push(("r", v)),
                None => {}
            }
            match hex.get_g() {
                Some(v) => attributes.push(("g", v)),
                None => {}
            }
            match hex.get_b() {
                Some(v) => attributes.push(("b", v)),
                None => {}
            }
            write_start_tag(writer, "a:srgbClr", attributes, true);
        },
        None => {}
    }
    write_end_tag(writer, "a:solidFill");
}

fn write_shape_style(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    shape_style: &ShapeStyle
) {
    // xdr:style
    write_start_tag(writer, "xdr:style", vec![], false);

    // a:lnRef
    match shape_style.get_line_reference() {
        Some(style) => {
            match style.get_scheme_color() {
                Some(color) => {
                    write_start_tag(writer, "a:lnRef", vec![
                        ("idx", style.get_index()),
                    ], false);
                    write_scheme_color(writer, color);
                    write_end_tag(writer, "a:lnRef");
                },
                None => {
                    write_start_tag(writer, "a:lnRef", vec![
                        ("idx", style.get_index()),
                    ], true);
                }
            }
        },
        None => {}
    }

    // a:fillRef
    match shape_style.get_fill_reference() {
        Some(style) => {
            match style.get_scheme_color() {
                Some(color) => {
                    write_start_tag(writer, "a:fillRef", vec![
                        ("idx", style.get_index()),
                    ], false);
                    write_scheme_color(writer, color);
                    write_end_tag(writer, "a:fillRef");
                },
                None => {
                    write_start_tag(writer, "a:fillRef", vec![
                        ("idx", style.get_index()),
                    ], true);
                }
            }
        },
        None => {}
    }

    // a:effectRef
    match shape_style.get_effect_reference() {
        Some(style) => {
            match style.get_scheme_color() {
                Some(color) => {
                    write_start_tag(writer, "a:effectRef", vec![
                        ("idx", style.get_index()),
                    ], false);
                    write_scheme_color(writer, color);
                    write_end_tag(writer, "a:effectRef");
                },
                None => {
                    write_start_tag(writer, "a:effectRef", vec![
                        ("idx", style.get_index()),
                    ], true);
                }
            }
        },
        None => {}
    }

    // a:fontRef
    match shape_style.get_font_reference() {
        Some(style) => {
            match style.get_scheme_color() {
                Some(color) => {
                    write_start_tag(writer, "a:fontRef", vec![
                        ("idx", style.get_index()),
                    ], false);
                    write_scheme_color(writer, color);
                    write_end_tag(writer, "a:fontRef");
                },
                None => {
                    write_start_tag(writer, "a:fontRef", vec![
                        ("idx", style.get_index()),
                    ], true);
                }
            }
        },
        None => {}
    }

    write_end_tag(writer, "xdr:style");
}

fn write_scheme_color(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    scheme_color: &SchemeColor
) {
    // a:schemeClr
    if scheme_color.with_inner_params() {
        write_start_tag(writer, "a:schemeClr", vec![
            ("val", scheme_color.get_val()),
        ], false);

        // a:lumMod
        match scheme_color.get_lum_mod() {
            Some(v) => {
                write_start_tag(writer, "a:lumMod", vec![
                    ("val", v.to_string().as_str()),
                ], true);
            },
            None => {}
        }

        // a:lumOff
        match scheme_color.get_lum_off() {
            Some(v) => {
                write_start_tag(writer, "a:lumOff", vec![
                    ("val", v.to_string().as_str()),
                ], true);
            },
            None => {}
        }

        // a:shade
        match scheme_color.get_shade() {
            Some(v) => {
                write_start_tag(writer, "a:shade", vec![
                    ("val", v.to_string().as_str()),
                ], true);
            },
            None => {}
        }

        // a:satMod
        match scheme_color.get_sat_mod() {
            Some(v) => {
                write_start_tag(writer, "a:satMod", vec![
                    ("val", v.to_string().as_str()),
                ], true);
            },
            None => {}
        }

        // a:alpha
        match scheme_color.get_alpha() {
            Some(v) => {
                write_start_tag(writer, "a:alpha", vec![
                    ("val", v.to_string().as_str()),
                ], true);
            },
            None => {}
        }

        write_end_tag(writer, "a:schemeClr");
    } else {
        write_start_tag(writer, "a:schemeClr", vec![
            ("val", scheme_color.get_val()),
        ], true);
    }
}

fn write_run_properties(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    title: &str,
    run_properties: &RunProperties,
) {
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    match run_properties.get_kumimoji() {
        Some(v) => attributes.push(("kumimoji", v)),
        None => {}
    }
    match run_properties.get_lang() {
        Some(v) => attributes.push(("lang", v)),
        None => {}
    }
    match run_properties.get_alt_lang() {
        Some(v) => attributes.push(("altLang", v)),
        None => {}
    }
    match run_properties.get_sz() {
        Some(v) => attributes.push(("sz", v)),
        None => {}
    }
    match run_properties.get_solid_fill() {
        Some(v) => {
            write_start_tag(writer, title, attributes, false);
            write_solid_fill(writer, v);
            write_end_tag(writer, title);
        },
        None => {
            write_start_tag(writer, title, attributes, true);
        }
    }
}