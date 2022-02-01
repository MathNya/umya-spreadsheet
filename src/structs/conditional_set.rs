use super::Conditional;
use super::SequenceOfReferences;

#[derive(Default, Debug, Clone)]
pub struct ConditionalSet {
    sequence_of_references: SequenceOfReferences,
    conditional_collection: Vec<Conditional>,
}
impl ConditionalSet {
    pub fn get_sequence_of_references(&self) -> &SequenceOfReferences {
        &self.sequence_of_references
    }

    pub fn get_sequence_of_references_mut(&mut self) -> &mut SequenceOfReferences {
        &mut self.sequence_of_references
    }

    pub fn set_sequence_of_references(&mut self, value: SequenceOfReferences) {
        self.sequence_of_references = value;
    }

    pub fn get_conditional_collection(&self) -> &Vec<Conditional> {
        &self.conditional_collection
    }

    pub fn get_conditional_collection_mut(&mut self) -> &mut Vec<Conditional> {
        &mut self.conditional_collection
    }

    pub fn set_conditional_collection(&mut self, value: Vec<Conditional>) {
        self.conditional_collection = value;
    }
}
