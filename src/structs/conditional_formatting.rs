use super::ConditionalFormattingRule;
use super::DifferentialFormats;
use super::SequenceOfReferences;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use traits::AdjustmentCoordinate;
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
        if let Some(v) = get_attribute(e, b"sqref") {
            self.sequence_of_references.set_sqref(v);
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"cfRule" {
                    let mut obj = ConditionalFormattingRule::default();
                    obj.set_attributes(reader, e, differential_formats, true);
                    self.conditional_collection.push(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"cfRule" {
                    let mut obj = ConditionalFormattingRule::default();
                    obj.set_attributes(reader, e, differential_formats, false);
                    self.conditional_collection.push(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"conditionalFormatting" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "conditionalFormatting")
        );
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
impl AdjustmentCoordinate for ConditionalFormatting {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.sequence_of_references.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.sequence_of_references.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn is_remove_coordinate(
        &self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        self.sequence_of_references.is_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        )
    }
}
