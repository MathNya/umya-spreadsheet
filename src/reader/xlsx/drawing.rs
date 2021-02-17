use std::result;
use std::collections::HashMap; 
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

use super::drawing_spreadsheet::*;
use super::drawing_charts::*;
use super::super::structs::anchor::Anchor;
use super::super::structs::drawing::charts::chart::Chart;
use super::super::structs::worksheet::Worksheet;
use super::super::structs::drawing::transform2d::Transform2D;
use super::super::structs::drawing::preset_geometry::PresetGeometry;
use super::super::structs::drawing::adjust_value_list::AdjustValueList;
use super::super::structs::drawing::shape_guide::ShapeGuide;

pub(crate) fn read(
    dir: &TempDir,
    target: &str,
    worksheet: &mut Worksheet
) -> result::Result<HashMap<String, Chart>, XlsxError> {
    let path = dir.path().join(format!("xl/drawings/{}", target));
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut result:HashMap<String, Chart> = HashMap::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"xdr:twoCellAnchor" => {
                        match two_cell_anchor(&mut reader, worksheet) {
                            Some((chart_id, chart)) => {result.insert(chart_id, chart);},
                            None => {},
                        }
                    },
                    b"mc:AlternateContent" => {
                        read_alternate_content(&mut reader);
                    }
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(result)
}

fn two_cell_anchor(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    worksheet: &mut Worksheet
)->Option<(String, Chart)> {
    let mut buf = Vec::new();
    let mut anchor = Anchor::default();
    let mut result:Option<(String, Chart)> = None;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"xdr:from" => {
                        let from = from_and_to(reader);
                        anchor.set_left_column(from.get("col").unwrap().clone());
                        anchor.set_left_offset(from.get("colOff").unwrap().clone());
                        anchor.set_top_row(from.get("row").unwrap().clone());
                        anchor.set_top_offset(from.get("rowOff").unwrap().clone());
                    },
                    b"xdr:to" => {
                        let to = from_and_to(reader);
                        anchor.set_right_column(to.get("col").unwrap().clone());
                        anchor.set_right_offset(to.get("colOff").unwrap().clone());
                        anchor.set_bottom_row(to.get("row").unwrap().clone());
                        anchor.set_bottom_offset(to.get("rowOff").unwrap().clone());
                    },
                    b"xdr:graphicFrame" => {
                        let (chart_id, mut chart) = read_chart(reader);
                        chart.set_anchor(anchor);
                        anchor = Anchor::default();
                        result = Some((chart_id, chart));
                    },
                    b"xdr:sp" => {
                        let mut shape = read_shape(reader);
                        shape.set_anchor(anchor);
                        worksheet.add_shape(shape);
                        anchor = Anchor::default();
                    },
                    b"xdr:cxnSp" => {
                        let mut connection_shape = read_connection_shape(reader);
                        connection_shape.set_anchor(anchor);
                        worksheet.add_connection_shape(connection_shape);
                        anchor = Anchor::default();
                    }
                    b"xdr:pic" => {
                        read_picture(reader);
                        anchor = Anchor::default();
                    }
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"xdr:twoCellAnchor" => return result,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:twoCellAnchor"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

// mc:AlternateContent
fn read_alternate_content(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>) {
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"mc:AlternateContent" => return,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "mc:AlternateContent"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn from_and_to(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>)->HashMap<String, usize> {
    let mut buf = Vec::new();
    let mut result: HashMap<String, usize> = HashMap::new();

    let mut string_value:String = String::from("");

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"xdr:col" => {
                        result.insert("col".to_string(), string_value.parse::<usize>().unwrap());
                    },
                    b"xdr:colOff" => {
                        result.insert("colOff".to_string(), string_value.parse::<usize>().unwrap());
                    },
                    b"xdr:row" => {
                        result.insert("row".to_string(), string_value.parse::<usize>().unwrap());
                    },
                    b"xdr:rowOff" => {
                        result.insert("rowOff".to_string(), string_value.parse::<usize>().unwrap());
                    },
                    b"xdr:from" => return result,
                    b"xdr:to" => return result,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "c:from or c:to"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

// a:xfrm
pub(crate) fn read_transform2d(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    e:&quick_xml::events::BytesStart
)->Transform2D {
    let mut buf = Vec::new();
    let mut result = Transform2D::default();

    match get_attribute(e, b"rot") {
        Some(v) => {result.set_rot(v);},
        None => {}
    }

    match get_attribute(e, b"flipH") {
        Some(v) => {result.set_flip_h(v);},
        None => {}
    }

    match get_attribute(e, b"flipV") {
        Some(v) => {result.set_flip_v(v);},
        None => {}
    }

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"a:off" => {
                        result.set_x(get_attribute(e, b"x").unwrap().parse::<usize>().unwrap());
                        result.set_y(get_attribute(e, b"y").unwrap().parse::<usize>().unwrap());
                    },
                    b"a:ext" => {
                        result.set_width(get_attribute(e, b"cx").unwrap().parse::<usize>().unwrap());
                        result.set_height(get_attribute(e, b"cy").unwrap().parse::<usize>().unwrap());
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"a:xfrm" => {
                        return result;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "a:xfrm"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

// a:prstGeom
pub(crate) fn read_preset_geometry(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    prst: &str
)->PresetGeometry {
    let mut buf = Vec::new();
    let mut result = PresetGeometry::default();

    result.set_geometry(prst);
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"a:avLst" => {
                        result.set_adjust_value_list(read_addjust_value_list(reader));
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"a:prstGeom" => {
                        return result;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "a:prstGeom"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

// a:avLst
pub(crate) fn read_addjust_value_list(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>
)->AdjustValueList {
    let mut buf = Vec::new();
    let mut result = AdjustValueList::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"a:gd" => {
                        let mut shape_guide = ShapeGuide::default();
                        shape_guide.set_name(get_attribute(e, b"name").unwrap());
                        shape_guide.set_fmla(get_attribute(e, b"fmla").unwrap());
                        result.add_shape_guide_collection(shape_guide);
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"a:avLst" => {
                        return result;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "a:avLst"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}
