// headerFooter
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
    structs::{
        OddFooter,
        OddHeader,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct HeaderFooter {
    odd_header: OddHeader,
    odd_footer: OddFooter,
}

impl HeaderFooter {
    #[inline]
    #[must_use]
    pub fn get_odd_header(&self) -> &OddHeader {
        &self.odd_header
    }

    #[inline]
    pub fn get_odd_header_mut(&mut self) -> &mut OddHeader {
        &mut self.odd_header
    }

    #[inline]
    pub fn set_odd_header(&mut self, value: OddHeader) -> &mut Self {
        self.odd_header = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_odd_footer(&self) -> &OddFooter {
        &self.odd_footer
    }

    #[inline]
    pub fn get_odd_footer_mut(&mut self) -> &mut OddFooter {
        &mut self.odd_footer
    }

    #[inline]
    pub fn set_odd_footer(&mut self, value: OddFooter) -> &mut Self {
        self.odd_footer = value;
        self
    }

    #[inline]
    pub(crate) fn has_param(&self) -> bool {
        self.odd_header.has_param() || self.odd_footer.has_param()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                b"oddHeader" => {
                    self.odd_header.set_attributes(reader, e);
                }
                b"oddFooter" => {
                    self.odd_footer.set_attributes(reader, e);
                }
                _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"headerFooter" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "headerFooter")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.has_param() {
            // headerFooter
            write_start_tag(writer, "headerFooter", vec![], false);

            // oddHeader
            self.get_odd_header().write_to(writer);

            // oddFooter
            self.get_odd_footer().write_to(writer);

            write_end_tag(writer, "headerFooter");
        }
    }
}
