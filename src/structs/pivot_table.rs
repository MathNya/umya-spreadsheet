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
    #[inline]
    #[must_use]
    pub fn pivot_table_definition(&self) -> &PivotTableDefinition {
        &self.pivot_table_definition
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use pivot_table_definition()")]
    pub fn get_pivot_table_definition(&self) -> &PivotTableDefinition {
        self.pivot_table_definition()
    }

    #[inline]
    pub fn pivot_table_definition_mut(&mut self) -> &mut PivotTableDefinition {
        &mut self.
        pivot_table_definition
    }
    #[inline]
    #[deprecated(since = "3.0.0", note = "Use pivot_table_definition_mut()")]
    pub fn get_pivot_table_definition_mut(&mut self) -> &mut PivotTableDefinition {
        self.pivot_table_definition_mut()
    }

    #[inline]
    pub fn set_pivot_table_definition(&mut self, value: PivotTableDefinition) -> &mut Self {
        self.pivot_table_definition = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn pivot_cache_definition(&self) -> &PivotCacheDefinition {
        &self.pivot_cache_definition
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use pivot_cache_definition()")]
    pub fn get_pivot_cache_definition(&self) -> &PivotCacheDefinition {
        self.pivot_cache_definition()
    }

    #[inline]
    pub fn pivot_cache_definition_mut(&mut self) -> &mut PivotCacheDefinition {
        &mut self.pivot_cache_definition
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use pivot_cache_definition_mut()")]
    pub fn get_pivot_cache_definition_mut(&mut self) -> &mut PivotCacheDefinition {
        self.pivot_cache_definition_mut()
    }

    #[inline]
    pub fn set_pivot_cache_definition(&mut self, value: PivotCacheDefinition) -> &mut Self {
        self.pivot_cache_definition = value;
        self
    }
}
