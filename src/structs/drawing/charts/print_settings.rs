// c:printSettings
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use super::HeaderFooter;
use super::PageMargins;
use super::PageSetup;
use crate::reader::driver::xml_read_loop;
use crate::writer::driver::{write_end_tag, write_start_tag};

#[derive(Clone, Default, Debug)]
pub struct PrintSettings {
    header_footer: HeaderFooter,
    page_margins: PageMargins,
    page_setup: PageSetup,
}

impl PrintSettings {
    #[must_use]
    pub fn get_header_footer(&self) -> &HeaderFooter {
        &self.header_footer
    }

    pub fn get_header_footer_mut(&mut self) -> &mut HeaderFooter {
        &mut self.header_footer
    }

    pub fn set_header_footer(&mut self, value: HeaderFooter) -> &mut Self {
        self.header_footer = value;
        self
    }

    #[must_use]
    pub fn get_page_margins(&self) -> &PageMargins {
        &self.page_margins
    }

    pub fn get_page_margins_mut(&mut self) -> &mut PageMargins {
        &mut self.page_margins
    }

    pub fn set_page_margins(&mut self, value: PageMargins) -> &mut Self {
        self.page_margins = value;
        self
    }

    #[must_use]
    pub fn get_page_setup(&self) -> &PageSetup {
        &self.page_setup
    }

    pub fn get_page_setup_mut(&mut self) -> &mut PageSetup {
        &mut self.page_setup
    }

    pub fn set_page_setup(&mut self, value: PageSetup) -> &mut Self {
        self.page_setup = value;
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
                match e.name().0 {
                b"c:headerFooter" => {
                    HeaderFooter::set_attributes(reader, e);
                }
                b"c:pageSetup" => {
                    PageSetup::set_attributes(reader, e);
                }
                _ => (),
                }
            },
            Event::Empty(ref e) => {
                if e.name().0 == b"c:pageMargins" {
                    self.page_margins.set_attributes(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:printSettings" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:printSettings")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:printSettings
        write_start_tag(writer, "c:printSettings", vec![], false);

        // c:headerFooter
        HeaderFooter::write_to(writer);

        // c:pageMargins
        self.page_margins.write_to(writer);

        // c:pageSetup
        PageSetup::write_to(writer);

        write_end_tag(writer, "c:printSettings");
    }
}
