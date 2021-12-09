// mc:AlternateContent
use super::AlternateContentChoice;
use super::AlternateContentFallback;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct AlternateContent {
    alternate_content_choice: AlternateContentChoice,
    alternate_content_fallback: AlternateContentFallback,
}
impl AlternateContent {
    
    pub fn get_alternate_content_choice(&self)-> &AlternateContentChoice {
        &self.alternate_content_choice
    }

    pub fn get_alternate_content_choice_mut(&mut self)-> &mut AlternateContentChoice {
        &mut self.alternate_content_choice
    }

    pub fn set_alternate_content_choice(&mut self, value:AlternateContentChoice)-> &mut AlternateContent {
        self.alternate_content_choice = value;
        self
    }

    pub fn get_alternate_content_fallback(&self)-> &AlternateContentFallback {
        &self.alternate_content_fallback
    }

    pub fn get_alternate_content_fallback_mut(&mut self)-> &mut AlternateContentFallback {
        &mut self.alternate_content_fallback
    }

    pub fn set_alternate_content_fallback(&mut self, value:AlternateContentFallback)-> &mut AlternateContent {
        self.alternate_content_fallback = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"mc:Choice" => {
                            self.alternate_content_choice.set_attributes(reader, e);
                        },
                        b"mc:Fallback" => {
                            self.alternate_content_fallback.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"mc:AlternateContent" => return,
                        _ => (),
                    }
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
        write_start_tag(writer, "mc:AlternateContent", vec![
            ("xmlns:mc", "http://schemas.openxmlformats.org/markup-compatibility/2006"),
        ], false);

        // mc:Choice
        &self.alternate_content_choice.write_to(writer);

        // mc:Fallback
        &self.alternate_content_fallback.write_to(writer);

        write_end_tag(writer, "mc:AlternateContent");
    }
}
