// c:printSettings
use super::HeaderFooter;
use super::PageMargins;
use super::PageSetup;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PrintSettings {
    header_footer: HeaderFooter,
    page_margins: PageMargins,
    page_setup: PageSetup,
}
impl PrintSettings {
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().0 {
                    b"c:headerFooter" => {
                        self.header_footer.set_attributes(reader, e);
                    }
                    b"c:pageSetup" => {
                        self.page_setup.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().0 {
                    b"c:pageMargins" => {
                        self.page_margins.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:printSettings" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:printSettings"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:printSettings
        write_start_tag(writer, "c:printSettings", vec![], false);

        // c:headerFooter
        self.header_footer.write_to(writer);

        // c:pageMargins
        self.page_margins.write_to(writer);

        // c:pageSetup
        self.page_setup.write_to(writer);

        write_end_tag(writer, "c:printSettings");
    }
}
