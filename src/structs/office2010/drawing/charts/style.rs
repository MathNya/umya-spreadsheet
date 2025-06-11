// c14:style
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
    StringValue,
    helper::const_str::{
        DRAWING_CHART_NS,
        MC_NS,
    },
    reader::driver::{
        get_attribute,
        xml_read_loop,
    },
    set_string_from_xml,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Style {
    include_alternate_content: bool,
    val:                       StringValue,
}

impl Style {
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        include_alternate_content: bool,
    ) {
        self.include_alternate_content = include_alternate_content;

        if include_alternate_content {
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
        if self.include_alternate_content {
            // mc:AlternateContent
            write_start_tag(
                writer,
                "mc:AlternateContent",
                vec![("xmlns:mc", MC_NS).into()],
                false,
            );

            // mc:Choice
            write_start_tag(
                writer,
                "mc:Choice",
                vec![
                    ("Requires", "c14").into(),
                    ("xmlns:c14", DRAWING_CHART_NS).into(),
                ],
                false,
            );

            // c14:style
            write_start_tag(writer, "c14:style", vec![("val", "102").into()], true);

            write_end_tag(writer, "mc:Choice");

            // mc:Fallback
            write_start_tag(writer, "mc:Fallback", vec![], false);
        }

        // c14:style
        write_start_tag(
            writer,
            "c:style",
            vec![("val", self.val.value_str()).into()],
            true,
        );

        if self.include_alternate_content {
            write_end_tag(writer, "mc:Fallback");

            write_end_tag(writer, "mc:AlternateContent");
        }
    }
}
