use super::Range;
use super::Conditional;

#[derive(Default, Debug, Clone)]
pub struct ConditionalSet {
    range_collection: Vec<Range>,
    conditional_collection: Vec<Conditional>,
}

impl ConditionalSet {
    pub fn get_range_collection(&self)-> &Vec<Range> {
        &self.range_collection
    }

    pub fn get_range_collection_mut(&mut self)-> &mut Vec<Range> {
        &mut self.range_collection
    }

    pub fn set_range_collection(&mut self, value:Vec<Range>) {
        self.range_collection = value;
    }

    pub fn get_conditional_collection(&self)-> &Vec<Conditional> {
        &self.conditional_collection
    }

    pub fn get_conditional_collection_mut(&mut self)-> &mut Vec<Conditional> {
        &mut self.conditional_collection
    }

    pub fn set_conditional_collection(&mut self, value:Vec<Conditional>) {
        self.conditional_collection = value;
    }

    pub(crate) fn set_sqref<S: Into<String>>(&mut self, value:S) {
        let org_value = value.into().clone();
        let range_collection: Vec<&str> = org_value.split(" ").collect();
        for range_value in range_collection {
            let mut range = Range::default();
            range.set_range(range_value);
            self.range_collection.push(range);
        }
    }

    pub fn get_sqref(&self)-> String {
        let mut result = String::from("");
        for range in &self.range_collection {
            if result != "" {
                result = format!("{} ", result);
            }
            result = format!("{}{}", result, range.get_range());
        }
        result
    }
}
