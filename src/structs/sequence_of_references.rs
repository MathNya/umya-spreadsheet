use super::Range;

#[derive(Default, Debug, Clone)]
pub struct SequenceOfReferences {
    range_collection: Vec<Range>,
}

impl SequenceOfReferences {
    pub fn get_range_collection(&self) -> &Vec<Range> {
        &self.range_collection
    }

    pub fn get_range_collection_mut(&mut self) -> &mut Vec<Range> {
        &mut self.range_collection
    }

    pub fn set_range_collection(&mut self, value: Vec<Range>) {
        self.range_collection = value;
    }

    pub fn add_range_collection(&mut self, value: Range) {
        self.range_collection.push(value);
    }

    pub fn set_sqref<S: Into<String>>(&mut self, value: S) {
        value.into().split(' ').for_each(|range_value| {
            let mut range = Range::default();
            range.set_range(range_value);
            self.range_collection.push(range);
        });
    }

    pub fn get_sqref(&self) -> String {
        self.range_collection
            .iter()
            .map(|range| range.get_range())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
