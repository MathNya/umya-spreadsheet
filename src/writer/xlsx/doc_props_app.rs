use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use structs::Spreadsheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // Properties
    write_start_tag(
        &mut writer,
        "Properties",
        vec![
            (
                "xmlns",
                "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties",
            ),
            (
                "xmlns:vt",
                "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes",
            ),
        ],
        false,
    );

    // Application
    write_start_tag(&mut writer, "Application", vec![], false);
    write_text_node(&mut writer, "Microsoft Excel");
    write_end_tag(&mut writer, "Application");

    // DocSecurity
    write_start_tag(&mut writer, "DocSecurity", vec![], false);
    write_text_node(&mut writer, "0");
    write_end_tag(&mut writer, "DocSecurity");

    // ScaleCrop
    write_start_tag(&mut writer, "ScaleCrop", vec![], false);
    write_text_node(&mut writer, "false");
    write_end_tag(&mut writer, "ScaleCrop");

    // HeadingPairs
    write_start_tag(&mut writer, "HeadingPairs", vec![], false);

    // vt:vector
    write_start_tag(
        &mut writer,
        "vt:vector",
        vec![("size", "2"), ("baseType", "variant")],
        false,
    );

    // vt:variant
    write_start_tag(&mut writer, "vt:variant", vec![], false);

    // vt:i4
    write_start_tag(&mut writer, "vt:lpstr", vec![], false);
    write_text_node(&mut writer, "Worksheets");
    write_end_tag(&mut writer, "vt:lpstr");

    write_end_tag(&mut writer, "vt:variant");

    // vt:variant
    write_start_tag(&mut writer, "vt:variant", vec![], false);

    // vt:i4
    write_start_tag(&mut writer, "vt:i4", vec![], false);
    write_text_node(
        &mut writer,
        spreadsheet.get_sheet_count().to_string().as_str(),
    );
    write_end_tag(&mut writer, "vt:i4");

    write_end_tag(&mut writer, "vt:variant");

    write_end_tag(&mut writer, "vt:vector");

    write_end_tag(&mut writer, "HeadingPairs");

    // TitlesOfParts
    write_start_tag(&mut writer, "TitlesOfParts", vec![], false);

    // vt:vector
    write_start_tag(
        &mut writer,
        "vt:vector",
        vec![
            ("size", spreadsheet.get_sheet_count().to_string().as_str()),
            ("baseType", "lpstr"),
        ],
        false,
    );

    for workseet in spreadsheet.get_sheet_collection_no_check() {
        // vt:lpstr
        write_start_tag(&mut writer, "vt:lpstr", vec![], false);
        write_text_node(&mut writer, workseet.get_name());
        write_end_tag(&mut writer, "vt:lpstr");
    }

    write_end_tag(&mut writer, "vt:vector");

    write_end_tag(&mut writer, "TitlesOfParts");

    // Manager
    write_start_tag(&mut writer, "Manager", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_manager());
    write_end_tag(&mut writer, "Manager");

    // Company
    write_start_tag(&mut writer, "Company", vec![], false);
    write_text_node(&mut writer, spreadsheet.get_properties().get_company());
    write_end_tag(&mut writer, "Company");

    // LinksUpToDate
    write_start_tag(&mut writer, "LinksUpToDate", vec![], false);
    write_text_node(&mut writer, "false");
    write_end_tag(&mut writer, "LinksUpToDate");

    // SharedDoc
    write_start_tag(&mut writer, "SharedDoc", vec![], false);
    write_text_node(&mut writer, "false");
    write_end_tag(&mut writer, "SharedDoc");

    // HyperlinksChanged
    write_start_tag(&mut writer, "HyperlinksChanged", vec![], false);
    write_text_node(&mut writer, "false");
    write_end_tag(&mut writer, "HyperlinksChanged");

    // AppVersion
    write_start_tag(&mut writer, "AppVersion", vec![], false);
    write_text_node(&mut writer, "14.0300");
    write_end_tag(&mut writer, "AppVersion");

    write_end_tag(&mut writer, "Properties");

    let target = "docProps/app.xml";
    writer_mng.add_writer(target, writer)
}
