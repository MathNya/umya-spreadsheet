// c14:style
use crate::helper::const_str::*;
use crate::reader::driver::*;
use crate::writer::driver::*;
use crate::StringValue;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Style {
    include_alternateContent: bool,
    val: StringValue,
}

impl Style {
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        include_alternateContent: bool,
    ) {
        self.include_alternateContent = include_alternateContent;

        if include_alternateContent {
            xml_read_loop!(
                reader,
                Event::Empty(ref e) => {
                    if e.name().into_inner() == b"c:style" {
                        self.val.set_value_string(get_attribute(e, b"val").unwrap());
                    }
                },
                Event::End(ref e) => {
                    if e.name().into_inner() == b"mc:AlternateContent" {
                        return
                    }
                },
                Event::Eof => panic!("Error: Could not find {} end element", "mc:AlternateContent")
            );
        } else {
            set_string_from_xml!(self, e, val, "val");
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.include_alternateContent {
            // mc:AlternateContent
            write_start_tag(
                writer,
                "mc:AlternateContent",
                vec![("xmlns:mc", MC_NS)],
                false,
            );

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
        }

        // c14:style
        write_start_tag(
            writer,
            "c:style",
            vec![("val", self.val.get_value_str())],
            true,
        );

        if self.include_alternateContent {
            write_end_tag(writer, "mc:Fallback");

            write_end_tag(writer, "mc:AlternateContent");
        }
    }
}
