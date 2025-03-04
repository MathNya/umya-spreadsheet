use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Spreadsheet;
use crate::structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    has_shared_string_table: bool,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let is_light = *writer_mng.get_is_light();
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // relationships
    let root_tag_name = "Relationships";
    let attributes: Vec<(&str, &str)> = vec![("xmlns", REL_NS)];
    write_start_tag(&mut writer, root_tag_name, attributes, false);

    let mut index = 1;

    // relationships worksheet
    for _ in spreadsheet.get_sheet_collection_no_check() {
        let path_str = format!("worksheets/sheet{}.xml", index);
        write_relationship(&mut writer, &index.to_string(), WORKSHEET_NS, &path_str, "");
        index += 1;
    }

    // relationships pivot_cache_definition
    for (_, _, pivot_cache_definition) in spreadsheet.get_pivot_caches() {
        write_relationship(
            &mut writer,
            &index.to_string(),
            PIVOT_CACHE_DEF_NS,
            &pivot_cache_definition,
            "",
        );
        index += 1;
    }

    // relationship styles.xml
    write_relationship(&mut writer, &index.to_string(), STYLES_NS, "styles.xml", "");
    index += 1;

    // relationship theme/theme1.xml
    write_relationship(
        &mut writer,
        &index.to_string(),
        THEME_NS,
        "theme/theme1.xml",
        "",
    );
    index += 1;

    // relationship sharedStrings.xml
    if has_shared_string_table {
        write_relationship(
            &mut writer,
            &index.to_string(),
            SHARED_STRINGS_NS,
            "sharedStrings.xml",
            "",
        );
        index += 1;
    }

    // relationships for vbaProject if needed
    if spreadsheet.get_has_macros() {
        write_relationship(
            &mut writer,
            &index.to_string(),
            VBA_PROJECT_NS,
            "vbaProject.bin",
            "",
        );
    }

    write_end_tag(&mut writer, root_tag_name);
    make_file_from_writer(
        PKG_WORKBOOK_RELS,
        writer_mng.get_arv_mut(),
        writer,
        None,
        &is_light,
    )?;
    Ok(())
}

fn write_relationship(
    writer: &mut Writer<io::Cursor<Vec<u8>>>,
    p_id: &str,
    p_type: &str,
    p_target: &str,
    p_target_mode: &str,
) {
    let tag_name = "Relationship";
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    let r_id = format!("rId{}", p_id);
    attributes.push(("Id", &r_id));
    attributes.push(("Type", p_type));
    attributes.push(("Target", p_target));
    if !p_target_mode.is_empty() {
        attributes.push(("TargetMode", p_target_mode));
    }
    write_start_tag(writer, tag_name, attributes, true);
}
