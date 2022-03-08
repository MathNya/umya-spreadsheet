// a:picLocks
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PictureLocks {
    no_change_aspect: bool,
}
impl PictureLocks {
    pub fn get_no_change_aspect(&self) -> &bool {
        &self.no_change_aspect
    }

    pub fn set_no_change_aspect(&mut self, value: bool) {
        self.no_change_aspect = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"noChangeAspect") {
            Some(v) => {
                match &*v {
                    "1" => {
                        self.set_no_change_aspect(true);
                    }
                    _ => {}
                };
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:picLocks
        let no_change_aspect = if &self.no_change_aspect == &true {
            "1"
        } else {
            "2"
        };
        write_start_tag(
            writer,
            "a:picLocks",
            vec![("noChangeAspect", no_change_aspect)],
            true,
        );
    }
}
