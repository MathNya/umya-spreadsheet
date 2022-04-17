use hashbrown::HashMap;
use structs::Row;

#[derive(Clone, Default, Debug)]
pub(crate) struct Rows {
    rows: HashMap<u32, Row>,
}
impl Rows {
    pub(crate) fn has_sheet_data(&self) -> bool {
        !self.rows.is_empty()
    }

    /// Get Row Dimension List.
    pub(crate) fn get_row_dimensions(&self) -> Vec<&Row> {
        self.rows.values().collect()
    }

    /// Get Row Dimension List in mutable.
    pub(crate) fn get_row_dimensions_mut(&mut self) -> Vec<&mut Row> {
        self.rows.values_mut().collect()
    }

    /// Get Row Dimension convert Hashmap.
    pub(crate) fn get_row_dimensions_to_hashmap(&self) -> &HashMap<u32, Row> {
        &self.rows
    }

    pub(crate) fn get_row_dimensions_to_hashmap_mut(&mut self) -> &mut HashMap<u32, Row> {
        &mut self.rows
    }

    /// Get Row Dimension.
    pub(crate) fn get_row_dimension(&self, row: &u32) -> Option<&Row> {
        self.rows.get(row)
    }

    /// Get Row Dimension in mutable.
    pub(crate) fn get_row_dimension_mut(&mut self, row: &u32) -> &mut Row {
        self.rows.entry(row.to_owned()).or_insert_with(|| {
            let mut obj = Row::default();
            obj.set_row_num(*row);
            obj
        })
    }

    /// (This method is crate only.)
    /// Set Row Dimension.
    pub(crate) fn set_row_dimension(&mut self, value: Row) -> &mut Self {
        let row = value.get_row_num();
        self.rows.insert(row.to_owned(), value);
        self
    }

    /// (This method is crate only.)
    /// Adjustment Insert Coordinate
    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for row_dimension in self.get_row_dimensions_mut() {
            row_dimension.adjustment_insert_coordinate(root_row_num, offset_row_num);
        }
        self.rebuild_map();
    }

    /// (This method is crate only.)
    /// Adjustment Remove Coordinate
    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.get_row_dimensions_mut().retain(|k| {
            !(k.get_row_num() > root_row_num && k.get_row_num() < &(root_row_num + offset_row_num))
        });
        for row_dimension in self.get_row_dimensions_mut() {
            row_dimension.adjustment_remove_coordinate(root_row_num, offset_row_num);
        }
        self.rebuild_map();
    }

    pub(crate) fn rebuild_map(&mut self) {
        let mut rebuild: HashMap<u32, Row> = HashMap::new();
        for (_, row) in self.get_row_dimensions_to_hashmap_mut() {
            let row_num = row.get_row_num();
            rebuild.insert(row_num.to_owned(), row.clone());
        }
        self.rows = rebuild;
    }
}
