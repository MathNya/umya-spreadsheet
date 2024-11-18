use super::Range;
use thin_vec::ThinVec;
use traits::AdjustmentCoordinate;

#[derive(Default, Debug, Clone)]
pub struct SequenceOfReferences {
    range_collection: ThinVec<Range>,
}

impl SequenceOfReferences {
    pub fn get_range_collection(&self) -> &[Range] {
        &self.range_collection
    }

    pub fn get_range_collection_mut(&mut self) -> &mut ThinVec<Range> {
        &mut self.range_collection
    }

    pub fn set_range_collection(&mut self, value: impl Into<ThinVec<Range>>) -> &mut Self {
        self.range_collection = value.into();
        self
    }

    pub fn add_range_collection(&mut self, value: Range) -> &mut Self {
        self.range_collection.push(value);
        self
    }

    pub fn remove_range_collection(&mut self) -> &mut Self {
        self.range_collection.clear();
        self
    }

    pub fn set_sqref<S: Into<String>>(&mut self, value: S) -> &mut Self {
        value.into().split(' ').for_each(|range_value| {
            let mut range = Range::default();
            range.set_range(range_value);
            self.range_collection.push(range);
        });
        self
    }

    pub fn get_sqref(&self) -> String {
        self.range_collection
            .iter()
            .map(|range| range.get_range())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
impl AdjustmentCoordinate for SequenceOfReferences {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for range in &mut self.range_collection {
            range.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for range in &mut self.range_collection {
            range.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn is_remove_coordinate(
        &self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        for range in &self.range_collection {
            if range.is_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            ) {
                return true;
            }
        }
        false
    }
}
