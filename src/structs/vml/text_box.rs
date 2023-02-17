use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct TextBox {}
impl TextBox {
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"v:textbox" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "v:textbox"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:textbox
        write_start_tag(
            writer,
            "v:textbox",
            vec![("style", "mso-direction-alt:auto")],
            false,
        );
        write_start_tag(writer, "div", vec![("style", "text-align:left")], false);
        write_end_tag(writer, "div");
        write_end_tag(writer, "v:textbox");
    }
}
