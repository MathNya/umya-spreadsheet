use quick_xml::events::{Event};
use super::driver::*;

use super::drawing::*;
use super::super::structs::drawing::spreadsheet::shape::Shape;
use super::super::structs::drawing::spreadsheet::connection_shape::ConnectionShape;
use super::super::structs::drawing::spreadsheet::shape_style::ShapeStyle;
use super::super::structs::drawing::spreadsheet::shape_properties::ShapeProperties;
use super::super::structs::drawing::spreadsheet::text_body::TextBody;
use super::super::structs::drawing::spreadsheet::text_body_content::TextBodyContent;
use super::super::structs::drawing::spreadsheet::run_properties::RunProperties;
use super::super::structs::drawing::spreadsheet::body_properties::BodyProperties;
use super::super::structs::drawing::spreadsheet::connection_type::ConnectionType;
use super::super::structs::drawing::style_matrix_reference_type::StyleMatrixReferenceType;
use super::super::structs::drawing::scheme_color::SchemeColor;
use super::super::structs::drawing::solid_fill::SolidFill;
use super::super::structs::drawing::rgb_color_model_hex::RgbColorModelHex;
use super::super::structs::drawing::outline::Outline;

pub(crate) fn read_shape(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>) -> Shape {
    let mut buf = Vec::new();

    let mut shape = Shape::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"xdr:spPr" => {
                        shape.set_shape_properties(read_shape_sroperties(reader));
                    },
                    b"xdr:style" => {
                        shape.set_shape_style(read_shape_styles(reader));
                    },
                    b"xdr:txBody" => {
                        shape.set_text_body(read_drawing_text(reader));
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"xdr:cNvPr" => {
                        let obj = shape.get_non_visual_shape_properties_mut().get_non_visual_drawing_properties_mut();
                        obj.set_id(get_attribute(e, b"id").unwrap());
                        obj.set_name(get_attribute(e, b"name").unwrap());
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"xdr:sp" => return shape,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:sp"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

pub(crate) fn read_connection_shape(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>) -> ConnectionShape {
    let mut buf = Vec::new();

    let mut connection_shape = ConnectionShape::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"xdr:spPr" => {
                        connection_shape.set_shape_properties(read_shape_sroperties(reader));
                    },
                    b"xdr:style" => {
                        connection_shape.set_shape_style(read_shape_styles(reader));
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"xdr:cNvPr" => {
                        let obj = connection_shape.get_non_visual_connection_shape_properties_mut().get_non_visual_drawing_properties_mut();
                        obj.set_id(get_attribute(e, b"id").unwrap());
                        obj.set_name(get_attribute(e, b"name").unwrap());
                    },
                    b"a:stCxn" => {
                        let mut connection_type = ConnectionType::default();
                        connection_type.set_id(get_attribute(e, b"id").unwrap());
                        connection_type.set_index(get_attribute(e, b"idx").unwrap());
                        let obj = connection_shape.get_non_visual_connection_shape_properties_mut().get_non_visual_connector_shape_drawing_properties_mut();
                        obj.set_start_connection(connection_type);
                    }
                    b"a:endCxn" => {
                        let mut connection_type = ConnectionType::default();
                        connection_type.set_id(get_attribute(e, b"id").unwrap());
                        connection_type.set_index(get_attribute(e, b"idx").unwrap());
                        let obj = connection_shape.get_non_visual_connection_shape_properties_mut().get_non_visual_connector_shape_drawing_properties_mut();
                        obj.set_end_connection(connection_type);
                    }
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"xdr:cxnSp" => return connection_shape,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:cxnSp"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

pub(crate) fn read_picture(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>) {
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"xdr:pic" => return,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:pic"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

// xdr:spPr
fn read_shape_sroperties(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>) -> ShapeProperties {
    let mut buf = Vec::new();
    let mut shape_properties = ShapeProperties::default();
    let mut scheme_color = SchemeColor::default();
    let mut solid_fill = SolidFill::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"a:xfrm" => {
                        shape_properties.set_transform2d(read_transform2d(reader, e));
                    },
                    b"a:prstGeom" => {
                        shape_properties.set_geometry(read_preset_geometry(reader, &get_attribute(e, b"prst").unwrap()));
                    },
                    b"a:schemeClr" => {
                        scheme_color = read_scheme_color(reader, e);
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"a:schemeClr" => {
                        scheme_color.set_val(get_attribute(e, b"val").unwrap());
                    },
                    b"a:tailEnd" => {
                        let mut outline = Outline::default();
                        outline.get_tail_end_mut().set_type(get_attribute(e, b"type").unwrap());
                        shape_properties.set_outline(outline);
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"a:solidFill" => {
                        solid_fill.set_scheme_color(scheme_color);
                        shape_properties.set_solid_fill(solid_fill);
                        scheme_color = SchemeColor::default();
                        solid_fill = SolidFill::default();
                    },
                    b"xdr:spPr" => return shape_properties,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:spPr"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn read_shape_styles(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>) -> ShapeStyle {
    let mut buf = Vec::new();
    let mut shape_style = ShapeStyle::default();
    let mut style_matrix_reference_type = StyleMatrixReferenceType::default();
    let mut scheme_color = SchemeColor::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"a:lnRef" => {
                        style_matrix_reference_type.set_index(get_attribute(e, b"idx").unwrap());
                    },
                    b"a:fillRef" => {
                        style_matrix_reference_type.set_index(get_attribute(e, b"idx").unwrap());
                    },
                    b"a:effectRef" => {
                        style_matrix_reference_type.set_index(get_attribute(e, b"idx").unwrap());
                    },
                    b"a:fontRef" => {
                        style_matrix_reference_type.set_index(get_attribute(e, b"idx").unwrap());
                    },
                    b"a:schemeClr" => {
                        scheme_color = read_scheme_color(reader, e);
                        style_matrix_reference_type.set_scheme_color(scheme_color);
                        scheme_color = SchemeColor::default();
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"a:lnRef" => {
                        style_matrix_reference_type.set_index(get_attribute(e, b"idx").unwrap());
                        shape_style.set_line_reference(style_matrix_reference_type);
                        style_matrix_reference_type = StyleMatrixReferenceType::default();
                    },
                    b"a:fillRef" => {
                        style_matrix_reference_type.set_index(get_attribute(e, b"idx").unwrap());
                        shape_style.set_fill_reference(style_matrix_reference_type);
                        style_matrix_reference_type = StyleMatrixReferenceType::default();
                    },
                    b"a:effectRef" => {
                        style_matrix_reference_type.set_index(get_attribute(e, b"idx").unwrap());
                        shape_style.set_effect_reference(style_matrix_reference_type);
                        style_matrix_reference_type = StyleMatrixReferenceType::default();
                    },
                    b"a:fontRef" => {
                        style_matrix_reference_type.set_index(get_attribute(e, b"idx").unwrap());
                        shape_style.set_font_reference(style_matrix_reference_type);
                        style_matrix_reference_type = StyleMatrixReferenceType::default();
                    },
                    b"a:schemeClr" => {
                        scheme_color.set_val(get_attribute(e, b"val").unwrap());
                        style_matrix_reference_type.set_scheme_color(scheme_color);
                        scheme_color = SchemeColor::default();
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"a:lnRef" => {
                        shape_style.set_line_reference(style_matrix_reference_type);
                        style_matrix_reference_type = StyleMatrixReferenceType::default();
                    },
                    b"a:fillRef" => {
                        shape_style.set_fill_reference(style_matrix_reference_type);
                        style_matrix_reference_type = StyleMatrixReferenceType::default();
                    },
                    b"a:effectRef" => {
                        shape_style.set_effect_reference(style_matrix_reference_type);
                        style_matrix_reference_type = StyleMatrixReferenceType::default();
                    },
                    b"a:fontRef" => {
                        shape_style.set_font_reference(style_matrix_reference_type);
                        style_matrix_reference_type = StyleMatrixReferenceType::default();
                    },
                    b"xdr:style" => {
                        //dbg!(&shape_style);
                        return shape_style
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:style"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

// a:schemeClr
fn read_scheme_color(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    e:&quick_xml::events::BytesStart
)->SchemeColor {
    let mut scheme_color = SchemeColor::default();
    let mut buf = Vec::new();

    scheme_color.set_val(get_attribute(e, b"val").unwrap());

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"a:lumMod" => {
                        scheme_color.set_lum_mod(get_attribute(e, b"val").unwrap());
                    },
                    b"a:lumOff" => {
                        scheme_color.set_lum_off(get_attribute(e, b"val").unwrap());
                    },
                    b"a:shade" => {
                        scheme_color.set_shade(get_attribute(e, b"val").unwrap());
                    },
                    b"a:satMod" => {
                        scheme_color.set_sat_mod(get_attribute(e, b"val").unwrap());
                    },
                    b"a:alpha" => {
                        scheme_color.set_alpha(get_attribute(e, b"val").unwrap());
                    }
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"a:schemeClr" => return scheme_color,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "a:schemeClr"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

// xdr:txBody
fn read_drawing_text(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>) -> TextBody {
    let mut buf = Vec::new();
    let mut text_body = TextBody::default();
    let mut content = TextBodyContent::default();
    let mut run_properties = RunProperties::default();
    let mut rgb_color_model_hex = RgbColorModelHex::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"a:rPr" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"kumimoji" => {
                                    run_properties.set_kumimoji(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"lang" => {
                                    run_properties.set_lang(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"altLang" => {
                                    run_properties.set_alt_lang(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"sz" => {
                                    run_properties.set_sz(get_attribute_value(attr).unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"a:bodyPr" => {
                        let mut body_properties = BodyProperties::default();
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"vertOverflow" => {
                                    body_properties.set_vert_overflow(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"horzOverflow" => {
                                    body_properties.set_horz_overflow(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"rtlCol" => {
                                    body_properties.set_rtl_col(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"anchor" => {
                                    body_properties.set_anchor(get_attribute_value(attr).unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                        text_body.set_body_properties(body_properties);
                    },
                    b"a:pPr" => {
                        content.set_algn(get_attribute(e, b"algn").unwrap());
                    },
                    b"a:rPr" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"kumimoji" => {
                                    run_properties.set_kumimoji(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"lang" => {
                                    run_properties.set_lang(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"altLang" => {
                                    run_properties.set_alt_lang(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"sz" => {
                                    run_properties.set_sz(get_attribute_value(attr).unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"a:endParaRPr" => {
                        let mut run_properties = RunProperties::default();
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"kumimoji" => {
                                    run_properties.set_kumimoji(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"lang" => {
                                    run_properties.set_lang(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"altLang" => {
                                    run_properties.set_alt_lang(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"sz" => {
                                    run_properties.set_sz(get_attribute_value(attr).unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                        content.set_end_para_run_properties(run_properties);
                    },
                    b"a:srgbClr" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"r" => {
                                    rgb_color_model_hex.set_r(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"g" => {
                                    rgb_color_model_hex.set_g(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"b" => {
                                    rgb_color_model_hex.set_b(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"val" => {
                                    rgb_color_model_hex.set_val(get_attribute_value(attr).unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                run_properties.set_text(e.unescape_and_decode(&reader).unwrap());
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"a:solidFill" => {
                        let mut solid_fill = SolidFill::default();
                        solid_fill.set_rgb_color_model_hex(rgb_color_model_hex);
                        run_properties.set_solid_fill(solid_fill);
                        rgb_color_model_hex = RgbColorModelHex::default();
                    },
                    b"a:r" => {
                        content.add_run_properties(run_properties);
                        run_properties = RunProperties::default();
                    },
                    b"a:p" => {
                        text_body.add_text_body_contentes(content);
                        content = TextBodyContent::default();
                    },
                    b"xdr:txBody" => return text_body,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:txBody"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}
