//xdr:cNvPicPr
use super::super::picture_locks::PictureLocks;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct NonVisualPictureDrawingProperties {
    picture_locks: PictureLocks,
}
impl NonVisualPictureDrawingProperties {
    pub fn get_picture_locks(&self) -> &PictureLocks {
        &self.picture_locks
    }

    pub fn get_picture_locks_mut(&mut self) -> &mut PictureLocks {
        &mut self.picture_locks
    }

    pub fn set_picture_locks(&mut self, value:PictureLocks) {
        self.picture_locks = value;
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
    
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:picLocks" => {
                            &mut self.get_picture_locks_mut().set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"xdr:cNvPicPr" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:cNvPicPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:cNvPicPr
        write_start_tag(writer, "xdr:cNvPicPr", vec![], false);

        // a:picLocks
        &self.picture_locks.write_to(writer);

        write_end_tag(writer, "xdr:cNvPicPr");
    }
}
