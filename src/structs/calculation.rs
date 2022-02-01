#[derive(Clone, Default, Debug)]
pub(crate) struct Calculation {
    pub(crate) calculation_cache: Vec<String>,
    pub(crate) calculation_cache_enabled: bool,
    pub(crate) branch_store_key_counter: i32,
    pub(crate) branch_pruning_enabled: bool,
}
