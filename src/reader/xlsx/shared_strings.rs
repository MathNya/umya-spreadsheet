use structs::Spreadsheet;
use structs::SharedStringTable;
use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;

const FILE_PATH: &'static str = "xl/sharedStrings.xml";

pub(crate) fn read(dir: &TempDir, spreadsheet: &mut Spreadsheet) -> result::Result<(), XlsxError> {
    let path = dir.path().join(FILE_PATH);
    let mut reader = match Reader::from_file(path){
        Ok(v) => {v},
        Err(_) => {return Ok(());}
    };
    reader.trim_text(false);
    let mut buf = Vec::new();

    let theme = spreadsheet.get_theme().clone();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"sst" => {
                        let mut obj = SharedStringTable::default();
                        obj.set_attributes(&mut reader, e);

                        // set ThemeColor
                        for item in obj.get_shared_string_item_mut() {
                            match item.get_rich_text_mut() {
                                Some(v) => {
                                    for element in  v.get_rich_text_elements_mut() {
                                        match element.get_run_properties_crate() {
                                            Some(r) => {
                                                let color = r.get_color_mut();
                                                color.set_argb_by_theme(&theme);
                                            },
                                            None => {},                           
                                        }
                                    }
                                },
                                None => {},                           
                            }
                        }

                        spreadsheet.set_shared_string_table(obj);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}
