// c14:style
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Style {}
impl Style {
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"mc:AlternateContent" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "mc:AlternateContent"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // mc:AlternateContent
        write_start_tag(
            writer,
            "mc:AlternateContent",
            vec![(
                "xmlns:mc",
                "http://schemas.openxmlformats.org/markup-compatibility/2006",
            )],
            false,
        );

        // mc:Choice
        write_start_tag(
            writer,
            "mc:Choice",
            vec![
                ("Requires", "c14"),
                (
                    "xmlns:c14",
                    "http://schemas.microsoft.com/office/drawing/2007/8/2/chart",
                ),
            ],
            false,
        );

        // c14:style
        write_start_tag(writer, "c14:style", vec![("val", "102")], true);

        write_end_tag(writer, "mc:Choice");

        // mc:Fallback
        write_start_tag(writer, "mc:Fallback", vec![], false);

        // c14:style
        write_start_tag(writer, "c:style", vec![("val", "2")], true);

        write_end_tag(writer, "mc:Fallback");

        write_end_tag(writer, "mc:AlternateContent");
    }
}
