use std::collections::HashMap;
use std::io::Cursor;
use std::result;
use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use tempdir::TempDir;
use super::XlsxError;
use onig::*;
use super::super::structs::spreadsheet::Spreadsheet;
use super::super::structs::rich_text::RichText;
use super::super::structs::font::Font;
use super::driver::*;

const SHARED_STRINGS: &'static str = "xl/sharedStrings.xml";

pub fn write(spreadsheet: &Spreadsheet, dir: &TempDir) -> result::Result<HashMap<String, usize>, XlsxError> {
    let mut shared_strings: HashMap<String, (String, Option<RichText>)> = HashMap::new();
    let mut count: usize = 0;
    for sheet in spreadsheet.get_sheet_collection() {
        for cell in sheet.get_cell_collection() {
            if cell.get_data_type() == "s" {
                count = count + 1;
                let value = cell.get_hash_code_by_value();
                shared_strings.entry(value).or_insert((cell.get_value().clone(), cell.get_rich_text().clone()));
            }
        }
    }

    let mut map: HashMap<String, usize> = HashMap::new();
    if shared_strings.len() == 0 {
        return Ok(map);
    }
    
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let _ = writer.write_event(Event::Decl(
        BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_text_node(&mut writer, "\n");
    write_start_tag(&mut writer, "sst", vec![
        ("xmlns", "http://schemas.openxmlformats.org/spreadsheetml/2006/main"),
        ("count", count.to_string().as_str()),
        ("uniqueCount", shared_strings.len().to_string().as_str()),
    ], false);
    let mut index = 0;
    for (hash_code, (value, rich_text)) in shared_strings {
         write_start_tag(&mut writer, "si", vec![], false);
         match rich_text {
             Some(v) => {
                for element in v.get_rich_text_elements() {
                    write_start_tag(&mut writer, "r", vec![], false);
                    match element.get_font() {
                        Some(font) => {
                            write_start_tag(&mut writer, "rPr", vec![], false);
                            // bold
                            if font.get_bold() == &true {
                                write_start_tag(&mut writer, "b", vec![], true);
                             }
 
                             // under line
                             if font.get_underline() != Font::UNDERLINE_NONE {
                                 write_start_tag(&mut writer, "u", vec![
                                     ("val", font.get_underline()),
                                 ], true);
                             }

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
                            write_end_tag(&mut writer, "rPr");
                        },
                        None => {}
                    }

                    // t
                    let mut attributes: Vec<(&str, &str)> = Vec::new();
                    let re = Regex::new(r#"^\r\n.*"#).unwrap();
                    if re.find(element.get_text()).is_some() {
                        attributes.push(("xml:space" , "preserve"));
                    }
                    write_start_tag(&mut writer, "t", attributes, false);
                    write_text_node(&mut writer, element.get_text());
                    write_end_tag(&mut writer, "t");

                    write_end_tag(&mut writer, "r");
                }
             },
             None => {
                // t
                let mut attributes: Vec<(&str, &str)> = Vec::new();
                let re = Regex::new(r#"^\r\n.*"#).unwrap();
                if re.find(&value).is_some() {
                    attributes.push(("xml:space" , "preserve"));
                }
                write_start_tag(&mut writer, "t", attributes, false);
                write_text_node(&mut writer, value);
                write_end_tag(&mut writer, "t");
             }
         }
         write_start_tag(&mut writer, "phoneticPr", vec![
             ("fontId", "1"),
         ], true);
         write_end_tag(&mut writer, "si");
         map.insert(hash_code, index);
         index = index + 1;
    }
    write_end_tag(&mut writer, "sst");
    let _ = make_file_from_writer(SHARED_STRINGS, dir, writer, Some("xl"))?;
    Ok(map)
}

