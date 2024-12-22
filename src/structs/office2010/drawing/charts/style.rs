// c14:style
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use crate::helper::const_str::{DRAWING_CHART_NS, MC_NS};
use crate::reader::driver::xml_read_loop;
use crate::writer::driver::{write_end_tag, write_start_tag};

#[derive(Clone, Default, Debug)]
pub struct Style {}

impl Style {
    pub(crate) fn set_attributes<R: std::io::BufRead>(reader: &mut Reader<R>, _e: &BytesStart) {
        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"mc:AlternateContent" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "mc:AlternateContent")
        );
    }

    pub(crate) fn write_to(writer: &mut Writer<Cursor<Vec<u8>>>) {
        // mc:AlternateContent
        write_start_tag(writer, "mc:AlternateContent", vec![("xmlns:mc", MC_NS)], false);

        // mc:Choice
        write_start_tag(
            writer,
            "mc:Choice",
            vec![("Requires", "c14"), ("xmlns:c14", DRAWING_CHART_NS)],
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
