use crate::structs::{
    PivotCacheDefinition,
    PivotTableDefinition,
};

#[derive(Clone, Default, Debug)]
pub struct PivotTable {
    pivot_table_definition: PivotTableDefinition,
    pivot_cache_definition: PivotCacheDefinition,
}

impl PivotTable {
    #[must_use]
    pub fn get_pivot_table_definition(&self) -> &PivotTableDefinition {
        &self.pivot_table_definition
    }

    pub fn get_pivot_table_definition_mut(&mut self) -> &mut PivotTableDefinition {
        &mut self.pivot_table_definition
    }

    pub fn set_pivot_table_definition(&mut self, value: PivotTableDefinition) -> &mut Self {
        self.pivot_table_definition = value;
        self
    }

    #[must_use]
    pub fn get_pivot_cache_definition(&self) -> &PivotCacheDefinition {
        &self.pivot_cache_definition
    }

    pub fn get_pivot_cache_definition_mut(&mut self) -> &mut PivotCacheDefinition {
        &mut self.pivot_cache_definition
    }

    pub fn set_pivot_cache_definition(&mut self, value: PivotCacheDefinition) -> &mut Self {
        self.pivot_cache_definition = value;
        self
    }
}
