use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io;

use ::structs::Spreadsheet;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write<W: io::Seek + io::Write>(spreadsheet: &Spreadsheet, arv: &mut zip::ZipWriter<W>, sub_dir: &str, file_name: &str) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // relationships
    let root_tag_name = "Relationships";
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    attributes.push(("xmlns", "http://schemas.openxmlformats.org/package/2006/relationships"));
    write_start_tag(&mut writer, root_tag_name, attributes, false);

    // relationship styles.xml
    write_relationship(
        &mut writer,
        "1",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
        "styles.xml",
        ""
    );

    // relationship theme/theme1.xml
    write_relationship(
        &mut writer,
        "2",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
        "theme/theme1.xml",
        ""
    );
    
    // relationship sharedStrings.xml
    write_relationship(
        &mut writer,
        "3",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
        "sharedStrings.xml",
        ""
    );

    // relationships with sheets
    let mut index = 0;
    for i in 0..spreadsheet.get_sheet_count() {
        index = i;
        let id = (index+1+3).to_string();
        let p_target = format!("worksheets/sheet{}.xml", (index+1).to_string().as_str());
        write_relationship(
            &mut writer,
            id.as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
            p_target.as_str(),
            ""
        );
    }

    // relationships for vbaProject if needed
    if spreadsheet.get_has_macros() == &true {
        let id = (index+1+3+1).to_string();
        write_relationship(
            &mut writer,
            id.as_str(),
            "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
            "vbaProject.bin",
            ""
        );
    }
    
    write_end_tag(&mut writer, root_tag_name);
    let _ = make_file_from_writer(format!("{}/{}",sub_dir,file_name).as_str(), arv, writer, Some(sub_dir)).unwrap();
    Ok(())
}

fn write_relationship(writer: &mut Writer<io::Cursor<Vec<u8>>>, p_id: &str, p_type: &str, p_target: &str, p_target_mode: &str)
{
    let tag_name = "Relationship";
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    let r_id = format!("rId{}", p_id);
    attributes.push(("Id", r_id.as_str()));
    attributes.push(("Type", p_type));
    attributes.push(("Target", p_target));
    if p_target_mode != "" {
        attributes.push(("TargetMode", p_target_mode));
    }
    write_start_tag(writer, tag_name, attributes, true);
}