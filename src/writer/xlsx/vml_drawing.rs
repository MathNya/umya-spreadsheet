use std::io;

use quick_xml::Writer;

use super::{
    driver::{write_end_tag, write_start_tag},
    XlsxError,
};
use crate::{
    helper::const_str::{EXCEL_NS, OFFICE_NS, VML_NS},
    structs::{Worksheet, WriterManager},
};

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(String, Vec<(String, String)>), XlsxError> {
    let mut rel_list: Vec<(String, String)> = Vec::new();
    if !worksheet.has_legacy_drawing() {
        return Ok((String::new(), rel_list));
    }

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // xml
    write_start_tag(
        &mut writer,
        "xml",
        vec![
            ("xmlns:v", VML_NS).into(),
            ("xmlns:o", OFFICE_NS).into(),
            ("xmlns:x", EXCEL_NS).into(),
        ],
        false,
    );

    // o:shapelayout
    write_start_tag(
        &mut writer,
        "o:shapelayout",
        vec![("v:ext", "edit").into()],
        false,
    );

    // o:idmap
    write_start_tag(
        &mut writer,
        "o:idmap",
        vec![("v:ext", "edit").into(), ("data", "1").into()],
        true,
    );

    write_end_tag(&mut writer, "o:shapelayout");

    let mut id = 1000 + 25;

    // ole_object
    if worksheet.has_ole_objects() {
        // v:shapetype
        write_start_tag(
            &mut writer,
            "v:shapetype",
            vec![
                ("id", "_x0000_t75").into(),
                ("coordsize", "21600,21600").into(),
                ("o:spt", "75").into(),
                ("o:preferrelative", "t").into(),
                ("path", "m@4@5l@4@11@9@11@9@5xe").into(),
                ("filled", "f").into(),
                ("stroked", "f").into(),
            ],
            false,
        );

        // v:stroke
        write_start_tag(
            &mut writer,
            "v:stroke",
            vec![("joinstyle", "miter").into()],
            true,
        );

        // v:formulas
        write_start_tag(&mut writer, "v:formulas", vec![], false);
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "if lineDrawn pixelLineWidth 0").into()],
            true,
        );
        write_start_tag(&mut writer, "v:f", vec![("eqn", "sum @0 1 0").into()], true);
        write_start_tag(&mut writer, "v:f", vec![("eqn", "sum 0 0 @1").into()], true);
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @2 1 2").into()],
            true,
        );
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @3 21600 pixelWidth").into()],
            true,
        );
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @3 21600 pixelHeight").into()],
            true,
        );
        write_start_tag(&mut writer, "v:f", vec![("eqn", "sum @0 0 1").into()], true);
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @6 1 2").into()],
            true,
        );
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @7 21600 pixelWidth").into()],
            true,
        );
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "sum @8 21600 0").into()],
            true,
        );
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "prod @7 21600 pixelHeight").into()],
            true,
        );
        write_start_tag(
            &mut writer,
            "v:f",
            vec![("eqn", "sum @10 21600 0").into()],
            true,
        );
        write_end_tag(&mut writer, "v:formulas");

        // v:path
        write_start_tag(
            &mut writer,
            "v:path",
            vec![
                ("o:extrusionok", "f").into(),
                ("gradientshapeok", "t").into(),
                ("o:connecttype", "rect").into(),
            ],
            true,
        );

        // o:lock
        write_start_tag(
            &mut writer,
            "o:lock",
            vec![("v:ext", "edit").into(), ("aspectratio", "t").into()],
            true,
        );

        write_end_tag(&mut writer, "v:shapetype");

        for ole_object in worksheet.get_ole_objects().get_ole_object() {
            // v:shape
            ole_object
                .get_shape()
                .write_to(&mut writer, id, &mut rel_list);
            id += 1;
        }
    }

    // comment
    if worksheet.has_comments() {
        // v:shapetype
        write_start_tag(
            &mut writer,
            "v:shapetype",
            vec![
                ("id", "_x0000_t202").into(),
                ("coordsize", "21600,21600").into(),
                ("o:spt", "202").into(),
                ("path", "m,l,21600r21600,l21600,xe").into(),
            ],
            false,
        );

        // v:stroke
        write_start_tag(
            &mut writer,
            "v:stroke",
            vec![("joinstyle", "miter").into()],
            true,
        );

        // v:path
        write_start_tag(
            &mut writer,
            "v:path",
            vec![
                ("gradientshapeok", "t").into(),
                ("o:connecttype", "rect").into(),
            ],
            true,
        );

        write_end_tag(&mut writer, "v:shapetype");

        for comment in worksheet.get_comments() {
            // v:shape
            comment.get_shape().write_to(&mut writer, id, &mut rel_list);
            id += 1;
        }
    }

    write_end_tag(&mut writer, "xml");

    let file_no = writer_mng.add_file_at_vml_drawing(writer)?;
    Ok((file_no.to_string(), rel_list))
}
