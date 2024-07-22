// headerFooter
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::OddFooter;
use structs::OddHeader;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct HeaderFooter {
    odd_header: OddHeader,
    odd_footer: OddFooter,
}

impl HeaderFooter {
    pub fn get_odd_header(&self) -> &OddHeader {
        &self.odd_header
    }

    pub fn get_odd_header_mut(&mut self) -> &mut OddHeader {
        &mut self.odd_header
    }

    pub fn set_odd_header(&mut self, value: OddHeader) -> &mut Self {
        self.odd_header = value;
        self
    }

    pub fn get_odd_footer(&self) -> &OddFooter {
        &self.odd_footer
    }

    pub fn get_odd_footer_mut(&mut self) -> &mut OddFooter {
        &mut self.odd_footer
    }

    pub fn set_odd_footer(&mut self, value: OddFooter) -> &mut Self {
        self.odd_footer = value;
        self
    }

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
