use super::ConditionalFormattingRule;
use super::DifferentialFormats;
use super::SequenceOfReferences;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug, Clone)]
pub struct ConditionalFormatting {
    sequence_of_references: SequenceOfReferences,
    conditional_collection: Vec<ConditionalFormattingRule>,
}
impl ConditionalFormatting {
    pub fn get_sequence_of_references(&self) -> &SequenceOfReferences {
        &self.sequence_of_references
    }

    pub fn get_sequence_of_references_mut(&mut self) -> &mut SequenceOfReferences {
        &mut self.sequence_of_references
    }

    pub fn set_sequence_of_references(&mut self, value: SequenceOfReferences) -> &mut Self {
        self.sequence_of_references = value;
        self
    }

    pub fn get_conditional_collection(&self) -> &Vec<ConditionalFormattingRule> {
        &self.conditional_collection
    }

    pub fn get_conditional_collection_mut(&mut self) -> &mut Vec<ConditionalFormattingRule> {
        &mut self.conditional_collection
    }

    pub fn set_conditional_collection(
        &mut self,
        value: Vec<ConditionalFormattingRule>,
    ) -> &mut Self {
        self.conditional_collection = value;
        self
    }

    pub fn add_conditional_collection(&mut self, value: ConditionalFormattingRule) -> &mut Self {
        self.conditional_collection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        differential_formats: &DifferentialFormats,
    ) {
        match get_attribute(e, b"sqref") {
            Some(v) => {
                self.sequence_of_references.set_sqref(v);
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"cfRule" => {
                        let mut obj = ConditionalFormattingRule::default();
                        obj.set_attributes(reader, e, differential_formats, true);
                        self.conditional_collection.push(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"cfRule" => {
                        let mut obj = ConditionalFormattingRule::default();
                        obj.set_attributes(reader, e, differential_formats, false);
                        self.conditional_collection.push(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"conditionalFormatting" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "conditionalFormatting"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        differential_formats: &mut DifferentialFormats,
    ) {
        let is_inner = !self.conditional_collection.is_empty();

        // conditionalFormatting
        let mut attributes: Vec<(&str, &str)> = Vec::new();

        let sequence_of_references = &self.sequence_of_references.get_sqref();
        attributes.push(("sqref", sequence_of_references));

        write_start_tag(writer, "conditionalFormatting", attributes, !is_inner);

        if is_inner {
            // cfRule
            for v in &self.conditional_collection {
                v.write_to(writer, differential_formats);
            }

            write_end_tag(writer, "conditionalFormatting");
        }
    }
}
