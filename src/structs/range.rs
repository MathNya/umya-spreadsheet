use super::coordinate::Coordinate;
use super::super::helper::coordinate::*;

#[derive(Clone, Default, Debug)]
pub struct Range {
    coordinate_start: Coordinate,
    coordinate_end: Option<Coordinate>,
}
impl Range {
    pub(crate) fn set_range<S: Into<String>>(&mut self, value:S) {
        let org_value = value.into().clone();
        let coordinate_collection: Vec<&str> = org_value.split(":").collect();

        if coordinate_collection.len() == 1 {
            let coordinate_str = coordinate_collection[0].to_string();
            let nums = index_from_coordinate(coordinate_str);
            let mut coordinate_start = Coordinate::default();
            coordinate_start.set_col_num(&nums[0]);
            coordinate_start.set_row_num(&nums[1]);
            coordinate_start.set_is_lock_col(if &nums[2] == &1 { &true } else { &false });
            coordinate_start.set_is_lock_row(if &nums[3] == &1 { &true } else { &false });
            self.coordinate_start = coordinate_start;

        } else if coordinate_collection.len() == 2 {
            let coordinate_str = coordinate_collection[0].to_string();
            let nums = index_from_coordinate(coordinate_str);
            let mut coordinate_start = Coordinate::default();
            coordinate_start.set_col_num(&nums[0]);
            coordinate_start.set_row_num(&nums[1]);
            coordinate_start.set_is_lock_col(if &nums[2] == &1 { &true } else { &false });
            coordinate_start.set_is_lock_row(if &nums[3] == &1 { &true } else { &false });
            self.coordinate_start = coordinate_start;

            let coordinate_str = coordinate_collection[1].to_string();
            let nums = index_from_coordinate(coordinate_str);
            let mut coordinate_end = Coordinate::default();
            coordinate_end.set_col_num(&nums[0]);
            coordinate_end.set_row_num(&nums[1]);
            coordinate_end.set_is_lock_col(if &nums[2] == &1 { &true } else { &false });
            coordinate_end.set_is_lock_row(if &nums[3] == &1 { &true } else { &false });
            self.coordinate_end = Some(coordinate_end);

        } else {
            panic!("Non-standard coordinate");
        }
    }

    pub(crate) fn get_range(&self) -> String {
        match &self.coordinate_end {
            Some(v) => {
                format!("{}:{}", &self.coordinate_start.get_coordinate(), &v.get_coordinate())
            },
            None => {
                format!("{}", &self.coordinate_start.get_coordinate())
            }
        }
    }

    pub(crate) fn get_coordinate_start(&self)-> &Coordinate {
        &self.coordinate_start
    }

    pub(crate) fn get_coordinate_start_mut(&mut self)-> &mut Coordinate {
        &mut self.coordinate_start
    }

    pub(crate) fn get_coordinate_end(&self)-> &Option<Coordinate> {
        &self.coordinate_end
    }

    pub(crate) fn get_coordinate_end_mut(&mut self)-> &mut Option<Coordinate> {
        &mut self.coordinate_end
    }

    pub(crate) fn adjustment_insert_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        self.coordinate_start.adjustment_insert_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
        match &mut self.coordinate_end {
            Some(v) => v.adjustment_insert_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num),
            None => {}
        }
    }

    pub(crate) fn adjustment_remove_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        self.coordinate_start.adjustment_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
        match &mut self.coordinate_end {
            Some(v) => v.adjustment_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num),
            None => {}
        }
    }

    pub(crate) fn is_remove(&self, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize)->bool {
        let start_result = self.coordinate_start.is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num);
        let end_result = match &self.coordinate_end {
            Some(v) => v.is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num),
            None => start_result
        };
        start_result && end_result
    }
}