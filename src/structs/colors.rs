// colors
use super::MruColors;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct Colors {
    mru_colors: MruColors,
}

impl Colors {
    pub(crate) fn _get_mru_colors(&self) -> &MruColors {
        &self.mru_colors
    }

    pub(crate) fn _get_mru_colors_mut(&mut self) -> &mut MruColors {
        &mut self.mru_colors
    }

    pub(crate) fn _set_mru_colors(&mut self, value: MruColors) -> &mut Self {
        self.mru_colors = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"mruColors" {
                    self.mru_colors.set_attributes(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"colors" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "colors")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.mru_colors.get_color().is_empty() {
            return;
        }
        // colors
        write_start_tag(writer, "colors", vec![], false);

        // mruColors
        self.mru_colors.write_to(writer);

        write_end_tag(writer, "colors");
    }
}
