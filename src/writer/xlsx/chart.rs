use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;

use super::super::structs::font::Font;
use super::super::structs::title::Title;
use super::super::structs::chart::Chart;
use super::super::structs::axis::Axis;
use super::super::structs::data_series::DataSeries;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl/charts";

pub(crate) fn write(
    chart: &Chart,
    p_chart_id: &str,
    dir: &TempDir
) -> Result<(), XlsxError> {
    let file_name = format!("chart{}.xml", p_chart_id);

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // c:chartSpace
    write_start_tag(&mut writer, "c:chartSpace", vec![
        ("xmlns:c", "http://schemas.openxmlformats.org/drawingml/2006/chart"),
        ("xmlns:a", "http://schemas.openxmlformats.org/drawingml/2006/main"),
        ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
    ], false);

    // c:date1904
    write_start_tag(&mut writer, "c:date1904", vec![
        ("val", "0"),
    ], true);
    
    // c:lang
    write_start_tag(&mut writer, "c:lang", vec![
        ("val", chart.get_lang()),
    ], true);

    // c:roundedCorners
    write_start_tag(&mut writer, "c:roundedCorners", vec![
        ("val", "0"),
    ], true);

    // mc:AlternateContent
    write_start_tag(&mut writer, "mc:AlternateContent", vec![
        ("xmlns:mc", "http://schemas.openxmlformats.org/markup-compatibility/2006"),
    ], false);

    // mc:Choice
    write_start_tag(&mut writer, "mc:Choice", vec![
        ("Requires", "c14"),
        ("xmlns:c14", "http://schemas.microsoft.com/office/drawing/2007/8/2/chart"),
    ], false);

    // c14:style
    write_start_tag(&mut writer, "c14:style", vec![
        ("val", "102"),
    ], true);

    write_end_tag(&mut writer, "mc:Choice");

    // mc:Fallback
    write_start_tag(&mut writer, "mc:Fallback", vec![], false);

    // c:style
    write_start_tag(&mut writer, "c:style", vec![
        ("val", "2"),
    ], true);

    write_end_tag(&mut writer, "mc:Fallback");

    write_end_tag(&mut writer, "mc:AlternateContent");

    // c:chart
    write_start_tag(&mut writer, "c:chart", vec![], false);

    // title
    match chart.get_title() {
        Some(v) => {
            write_title(&mut writer, v);
        },
        None => {}
    }

    // c:autoTitleDeleted
    write_start_tag(&mut writer, "c:autoTitleDeleted", vec![
        ("val", "0"),
    ], true);

    // c:plotArea
    write_start_tag(&mut writer, "c:plotArea", vec![], false);

    // c:layout
    write_start_tag(&mut writer, "c:layout", vec![], true);

    let mut plot_type :&str = "";
    for plot_series in chart.get_plot_area().get_plot_series() {
        // c:[plot_type]
        plot_type = plot_series.get_plot_type();
        write_start_tag(&mut writer, format!("c:{}", plot_type), vec![], false);

        // c:grouping
        match plot_series.get_plot_grouping() {
            Some(v) => {
                write_start_tag(&mut writer, "c:grouping", vec![
                    ("val", v),
                ], true);
            },
            None => {}
        };

        // c:varyColors
        write_start_tag(&mut writer, "c:varyColors", vec![
            ("val", "1"),
        ], true);

        let mut is_marker = false;
        for (idx, ser) in plot_series.get_plot_values() {
            // c:ser
            write_start_tag(&mut writer, "c:ser", vec![], false);

            // c:idx
            write_start_tag(&mut writer, "c:idx", vec![
                ("val", idx.to_string().as_str()),
            ], true);

            // c:order
            write_start_tag(&mut writer, "c:order", vec![
                ("val", plot_series.get_plot_order().get(idx).unwrap().to_string().as_str()),
            ], true);

            // c:marker
            match ser.get_point_marker() {
                Some(v) => {
                    is_marker = true;
                    write_start_tag(&mut writer, "c:marker", vec![], false);
                    // c:symbol
                    write_start_tag(&mut writer, "c:symbol", vec![
                        ("val", v.as_str()),
                    ], true);
                    write_end_tag(&mut writer, "c:marker");
                },
                None => {}
            }

            // c:cat
            match plot_series.get_plot_category().get(idx) {
                Some(v) => {
                    // c:cat
                    write_start_tag(&mut writer, "c:cat", vec![], false);

                    // c:strRef
                    write_start_tag(&mut writer, "c:strRef", vec![], false);

                    // c:f
                    write_start_tag(&mut writer, "c:f", vec![], false);
                    write_text_node(&mut writer, v.get_address());
                    write_end_tag(&mut writer, "c:f");

                    // c:numCache
                    write_start_tag(&mut writer, "c:strCache", vec![], false);

                    // c:formatCode
                    if v.get_format_code() != "" {
                        write_start_tag(&mut writer, "c:formatCode", vec![], false);
                        write_text_node(&mut writer, v.get_format_code());
                        write_end_tag(&mut writer, "c:formatCode");
                    }

                    // c:ptCount
                    write_start_tag(&mut writer, "c:ptCount", vec![
                        ("val", v.get_point_count().to_string().as_str()),
                    ], true);

                    for (i, value) in v.get_data_values(){
                        // c:pt
                        write_start_tag(&mut writer, "c:pt", vec![
                            ("idx", i.to_string().as_str()),
                        ], false);

                        // c:v
                        write_start_tag(&mut writer, "c:v", vec![], false);
                        write_text_node(&mut writer, value);
                        write_end_tag(&mut writer, "c:v");

                        write_end_tag(&mut writer, "c:pt");
                    }

                    write_end_tag(&mut writer, "c:strCache");

                    write_end_tag(&mut writer, "c:strRef");

                    write_end_tag(&mut writer, "c:cat");
                },
                None => {}
            }

            // c:val
            write_start_tag(&mut writer, "c:val", vec![], false);

            // c:numRef
            write_start_tag(&mut writer, "c:numRef", vec![], false);

            // c:f
            write_start_tag(&mut writer, "c:f", vec![], false);
            write_text_node(&mut writer, ser.get_address());
            write_end_tag(&mut writer, "c:f");

            // c:numCache
            write_start_tag(&mut writer, "c:numCache", vec![], false);

            // c:formatCode
            if ser.get_format_code() != "" {
                write_start_tag(&mut writer, "c:formatCode", vec![], false);
                write_text_node(&mut writer, ser.get_format_code());
                write_end_tag(&mut writer, "c:formatCode");
            }

            // c:ptCount
            write_start_tag(&mut writer, "c:ptCount", vec![
                ("val", ser.get_point_count().to_string().as_str()),
            ], true);

            for (i, value) in ser.get_data_values(){
                // c:pt
                write_start_tag(&mut writer, "c:pt", vec![
                    ("idx", i.to_string().as_str()),
                ], false);

                // c:v
                write_start_tag(&mut writer, "c:v", vec![], false);
                write_text_node(&mut writer, value);
                write_end_tag(&mut writer, "c:v");

                write_end_tag(&mut writer, "c:pt");
            }

            write_end_tag(&mut writer, "c:numCache");

            write_end_tag(&mut writer, "c:numRef");

            write_end_tag(&mut writer, "c:val");

            match plot_type {
                DataSeries::TYPE_LINECHART => {
                    // c:smooth
                    write_start_tag(&mut writer, "c:smooth", vec![
                        ("val", if plot_series.get_smooth_line() == &true {"1"} else {"0"}),
                    ], true);
                },
                _ => (),
            }

            write_end_tag(&mut writer, "c:ser");
        }

        // c:dLbls
        write_start_tag(&mut writer, "c:dLbls", vec![], false);

        // c:showLegendKey
        write_start_tag(&mut writer, "c:showLegendKey", vec![
            ("val", "0"),
        ], true);

        // c:showVal
        write_start_tag(&mut writer, "c:showVal", vec![
            ("val", "0"),
        ], true);

        // c:showCatName
        write_start_tag(&mut writer, "c:showCatName", vec![
            ("val", "0"),
        ], true);

        // c:showSerName
        write_start_tag(&mut writer, "c:showSerName", vec![
            ("val", "0"),
        ], true);

        // c:showPercent
        write_start_tag(&mut writer, "c:showPercent", vec![
            ("val", "0"),
        ], true);

        // c:showBubbleSize
        write_start_tag(&mut writer, "c:showBubbleSize", vec![
            ("val", "0"),
        ], true);

        write_end_tag(&mut writer, "c:dLbls");

        // c:marker
        if is_marker {
            write_start_tag(&mut writer, "c:marker", vec![
                ("val", "1"),
            ], true);
        }

        // c:smooth
        match plot_type {
            DataSeries::TYPE_LINECHART => {
                // c:smooth
                write_start_tag(&mut writer, "c:smooth", vec![
                    ("val", if plot_series.get_smooth_line() == &true {"1"} else {"0"}),
                ], true);
            },
            _ => (),
        }

        if plot_type != DataSeries::TYPE_PIECHART
            && plot_type != DataSeries::TYPE_PIECHART_3D
            && plot_type != DataSeries::TYPE_DONUTCHART {
            // c:axId
            write_start_tag(&mut writer, "c:axId", vec![
                ("val", chart.get_chart_axis_y().get_id().to_string().as_str()),
            ], true);

            // c:axId
            write_start_tag(&mut writer, "c:axId", vec![
                ("val", chart.get_chart_axis_x().get_id().to_string().as_str()),
            ], true);
        } else {
            // c:firstSliceAng
            write_start_tag(&mut writer, "c:firstSliceAng", vec![
                ("val", "0"),
            ], true);

            if plot_type == DataSeries::TYPE_DONUTCHART {
                // c:holeSize
                write_start_tag(&mut writer, "c:holeSize", vec![
                    ("val", "50"),
                ], true);
            }
        }

        write_end_tag(&mut writer, format!("c:{}", plot_type));
    }

    if plot_type != DataSeries::TYPE_PIECHART
        && plot_type != DataSeries::TYPE_PIECHART_3D
        && plot_type != DataSeries::TYPE_DONUTCHART {
        if plot_type == DataSeries::TYPE_BUBBLECHART {
            // c:valAx
            // Todo
        } else {
            // c:catAx
            write_category_axis(&mut writer, chart.get_chart_axis_y(), chart.get_chart_axis_x().get_id());
        }

        // c:valAx
        write_value_axis(&mut writer, chart.get_chart_axis_x(), chart.get_chart_axis_y().get_id());
    }

    write_end_tag(&mut writer, "c:plotArea");

    // c:legend
    write_start_tag(&mut writer, "c:legend", vec![], false);

    // c:legendPos
    write_start_tag(&mut writer, "c:legendPos", vec![
        ("val", "r"),
    ], true);

    // c:layout
    //write_start_tag(&mut writer, "c:layout", vec![], true);

    // c:overlay
    write_start_tag(&mut writer, "c:overlay", vec![
        ("val", "0"),
    ], true);

    write_end_tag(&mut writer, "c:legend");

    // c:overlay
    write_start_tag(&mut writer, "c:plotVisOnly", vec![
        ("val", "1"),
    ], true);

    // c:dispBlanksAs
    write_start_tag(&mut writer, "c:dispBlanksAs", vec![
        ("val", chart.get_display_blanks_as()),
    ], true);

    // c:showDLblsOverMax
    write_start_tag(&mut writer, "c:showDLblsOverMax", vec![
        ("val", "0"),
    ], true);

    write_end_tag(&mut writer, "c:chart");

    // c:printSettings
    write_start_tag(&mut writer, "c:printSettings", vec![], false);

    // c:headerFooter
    write_start_tag(&mut writer, "c:headerFooter", vec![], true);

    // c:pageMargins
    write_start_tag(&mut writer, "c:pageMargins", vec![
        ("b", "0.75"),
        ("l", "0.7"),
        ("r", "0.7"),
        ("t", "0.75"),
        ("header", "0.3"),
        ("footer", "0.3"),
    ], true);

    // c:pageSetup
    write_start_tag(&mut writer, "c:pageSetup", vec![], true);

    write_end_tag(&mut writer, "c:printSettings");

    write_end_tag(&mut writer, "c:chartSpace");

    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}

fn write_category_axis(writer: &mut Writer<Cursor<Vec<u8>>>, axis:&Axis, cross_ax_id: &usize)
{
        // c:catAx
        write_start_tag(writer, "c:catAx", vec![], false);

        // c:axId
        write_start_tag(writer, "c:axId", vec![
            ("val", axis.get_id().to_string().as_str()),
        ], true);
    
        // c:scaling
        write_start_tag(writer, "c:scaling", vec![], false);
    
        // c:orientation
        write_start_tag(writer, "c:orientation", vec![
            ("val", axis.get_axis_options().get("orientation").unwrap()),
        ], true);
    
        write_end_tag(writer, "c:scaling");
    
        // c:delete
        write_start_tag(writer, "c:delete", vec![
            ("val", "0"),
        ], true);
    
        // c:axPos
        write_start_tag(writer, "c:axPos", vec![
            ("val", "b"),
        ], true);

        // c:title
        match axis.get_label() {
            Some(v) => {
                write_title(writer, v);
            },
            None => {}
        }
    
        // c:majorTickMark
        write_start_tag(writer, "c:majorTickMark", vec![
            ("val", axis.get_axis_options().get("major_tick_mark").unwrap()),
        ], true);
    
        // c:minorTickMark
        write_start_tag(writer, "c:minorTickMark", vec![
            ("val", axis.get_axis_options().get("minor_tick_mark").unwrap()),
        ], true);
    
        // c:tickLblPos
        write_start_tag(writer, "c:tickLblPos", vec![
            ("val", axis.get_axis_options().get("axis_labels").unwrap()),
        ], true);
    
        // c:crossAx
        write_start_tag(writer, "c:crossAx", vec![
            ("val", cross_ax_id.to_string().as_str()),
        ], true);
    
        // c:crosses
        write_start_tag(writer, "c:crosses", vec![
            ("val", "autoZero"),
        ], true);
    
        // c:auto
        write_start_tag(writer, "c:auto", vec![
            ("val", "1"),
        ], true);
    
        // c:lblAlgn
        write_start_tag(writer, "c:lblAlgn", vec![
            ("val", "ctr"),
        ], true);
    
        // c:lblOffset
        write_start_tag(writer, "c:lblOffset", vec![
            ("val", "100"),
        ], true);
    
        // c:noMultiLvlLbl
        write_start_tag(writer, "c:noMultiLvlLbl", vec![
            ("val", "0"),
        ], true);
    
        write_end_tag(writer, "c:catAx");
}

fn write_value_axis(writer: &mut Writer<Cursor<Vec<u8>>>, axis:&Axis, cross_ax_id: &usize) {
    // c:valAx
    write_start_tag(writer, "c:valAx", vec![], false);

    // c:axId
    write_start_tag(writer, "c:axId", vec![
        ("val", axis.get_id().to_string().as_str()),
    ], true);

    // c:scaling
    write_start_tag(writer, "c:scaling", vec![], false);

    // c:orientation
    write_start_tag(writer, "c:orientation", vec![
        ("val", axis.get_axis_options().get("orientation").unwrap()),
    ], true);

    write_end_tag(writer, "c:scaling");

    // c:delete
    write_start_tag(writer, "c:delete", vec![
        ("val", "0"),
    ], true);

    // c:axPos
    write_start_tag(writer, "c:axPos", vec![
        ("val", "l"),
    ], true);

    // c:majorGridlines
    write_start_tag(writer, "c:majorGridlines", vec![], true);

    // c:title
    match axis.get_label() {
        Some(v) => {
            write_title(writer, v);
        },
        None => {}
    }

    // c:numFmt
    write_start_tag(writer, "c:numFmt", vec![
        ("formatCode", "General"),
        ("sourceLinked", "1"),
    ], true);

    // c:majorTickMark
    write_start_tag(writer, "c:majorTickMark", vec![
        ("val", axis.get_axis_options().get("major_tick_mark").unwrap()),
    ], true);

    // c:minorTickMark
    write_start_tag(writer, "c:minorTickMark", vec![
        ("val", axis.get_axis_options().get("minor_tick_mark").unwrap()),
    ], true);

    // c:tickLblPos
    write_start_tag(writer, "c:tickLblPos", vec![
        ("val", axis.get_axis_options().get("axis_labels").unwrap()),
    ], true);

    // c:crossAx
    write_start_tag(writer, "c:crossAx", vec![
        ("val", cross_ax_id.to_string().as_str()),
    ], true);

    // c:crosses
    write_start_tag(writer, "c:crosses", vec![
        ("val", "autoZero"),
    ], true);

    // c:crossBetween
    write_start_tag(writer, "c:crossBetween", vec![
        ("val", "between"),
    ], true);

    write_end_tag(writer, "c:valAx");
}

fn write_title(writer: &mut Writer<Cursor<Vec<u8>>>, title: &Title) {
    // c:title
    write_start_tag(writer, "c:title", vec![], false);

    // c:tx
    write_start_tag(writer, "c:tx", vec![], false);

    // c:rich
    write_start_tag(writer, "c:rich", vec![], false);

    // a:bodyPr
    write_start_tag(writer, "a:bodyPr", vec![], true);

    // a:lstStyle
    write_start_tag(writer, "a:lstStyle", vec![], true);

    // a:p
    write_start_tag(writer, "a:p", vec![], false);

    // a:pPr
    write_start_tag(writer, "a:pPr", vec![], false);

    // a:defRPr
    write_start_tag(writer, "a:defRPr", vec![], true);

    write_end_tag(writer, "a:pPr");

    for text_element in title.get_caption().get_rich_text_elements() {
        // a:r
        write_start_tag(writer, "a:r", vec![], false);

        // a:rPr
        match text_element.get_font() {
            Some(v) => {
                let def_font = Font::get_defalut_value();
                let mut attributes: Vec<(&str, &str)> = Vec::new();
                // Size
                let size = (v.get_size() * 10).to_string();
                if v.get_size() != def_font.get_size() {
                    attributes.push(("sz", size.as_str()));
                }

                // Bold
                match v.get_bold() {
                    &true => attributes.push(("b", "1")),
                    &false => {}
                }

                // Italic
                match v.get_italic() {
                    &true => attributes.push(("i", "1")),
                    &false => {}
                };

                // Underline
                let mut underline_type = v.get_underline();
                if underline_type == Font::UNDERLINE_SINGLE {
                    underline_type = "sng";
                }
                if underline_type == Font::UNDERLINE_DOUBLE {
                    underline_type = "dbl";
                }
                if underline_type != Font::UNDERLINE_NONE {
                    attributes.push(("u", underline_type));
                }

                // Strikethrough
                match v.get_strikethrough() {
                    &true => attributes.push(("strike", "sngStrike")),
                    &false => {}
                }

                let empty_flg = 
                v.get_name() == def_font.get_name() &&
                v.get_color().get_argb() == def_font.get_color().get_argb();
                write_start_tag(writer, "a:rPr", attributes, empty_flg);
 
                if v.get_color().get_argb() != def_font.get_color().get_argb() {
                    // Color
                    write_start_tag(writer, "a:solidFill", vec![], false);
                    write_start_tag(writer, "a:srgbClr", vec![
                        ("val", v.get_color().get_argb()),
                    ], true);
                    write_end_tag(writer, "a:solidFill");
                }

                if v.get_name() != def_font.get_name() {
                    // Font
                    write_start_tag(writer, "a:latin", vec![
                        ("typeface", v.get_name()),
                    ], true);
                }

                if empty_flg == false {
                    write_end_tag(writer, "a:rPr");
                }
            },
            None => {
                write_start_tag(writer, "a:rPr", vec![], true);
            }
        }

        // a:t
        write_start_tag(writer, "a:t", vec![], false);
        write_text_node(writer, text_element.get_text());
        write_end_tag(writer, "a:t");

        write_end_tag(writer, "a:r");
    }

    // a:endParaRPr
    //write_start_tag(writer, "a:endParaRPr", vec![
    //    ("lang", "ja-JP"),
    //    ("altLang", "en-US"),
    //], true);

    write_end_tag(writer, "a:p");

    write_end_tag(writer, "c:rich");

    write_end_tag(writer, "c:tx");

    // c:layout
    match title.get_layout() {
        Some(v) => {
            write_start_tag(writer, "c:layout", vec![], false);

            // c:manualLayout
            write_start_tag(writer, "c:manualLayout", vec![], false);
        
            // c:xMode
            write_start_tag(writer, "c:xMode", vec![
                ("val", v.get_x_mode()),
            ], true);
        
            // c:yMode
            write_start_tag(writer, "c:yMode", vec![
                ("val", v.get_y_mode()),
            ], true);
        
            // c:x
            write_start_tag(writer, "c:x", vec![
                ("val", v.get_x_pos().to_string().as_str()),
            ], true);
        
            // c:y
            write_start_tag(writer, "c:y", vec![
                ("val", v.get_y_pos().to_string().as_str()),
            ], true);
        
            write_end_tag(writer, "c:manualLayout");
        
            write_end_tag(writer, "c:layout");
        },
        None => {
            write_start_tag(writer, "c:layout", vec![], true);
        }
    }

    // c:overlay
    write_start_tag(writer, "c:overlay", vec![
        ("val", "0"),
    ], true);

    write_end_tag(writer, "c:title");
}
