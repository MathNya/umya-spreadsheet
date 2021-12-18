// oleObjects
use super::AlternateContent;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct OleObjects {
    alternate_content: Vec<AlternateContent>,
}
impl OleObjects {
    pub fn get_alternate_content(&self)-> &Vec<AlternateContent> {
        &self.alternate_content
    }

    pub fn get_alternate_content_mut(&mut self)-> &mut Vec<AlternateContent> {
        &mut self.alternate_content
    }

    pub fn set_alternate_content(&mut self, value:AlternateContent)-> &mut Self {
        self.alternate_content.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead, A: std::io::Read + std::io::Seek>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart,
        arv: &mut zip::read::ZipArchive<A>,
        sheet_name: Option<&str>,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"mc:AlternateContent" => {
                            let mut obj = AlternateContent::default();
                            obj.set_attributes(reader, e, arv, sheet_name);
                            self.set_alternate_content(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"oleObjects" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "oleObjects"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        r_id: &usize,
    ) {
        if self.alternate_content.len() > 0 {
            // oleObjects
            write_start_tag(writer, "oleObjects", vec![], false);

            // mc:AlternateContent
            let mut r = r_id.clone();
            for obj in &self.alternate_content {
                obj.write_to(writer, Some(&r));
                r += 2;
            }

            write_end_tag(writer, "oleObjects");
        }
    }
}
