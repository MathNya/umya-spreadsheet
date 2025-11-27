// a:tileRect
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    reader::driver::xml_read_loop,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct TileRectangle {}
impl TileRectangle {
    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        reader: &mut Reader<R>,
        _e: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:tileRect" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:tileRect")
        );
    }

    #[inline]
    pub(crate) fn write_to(writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:tileRect
        write_start_tag(writer, "a:tileRect", vec![], true);
    }
}
