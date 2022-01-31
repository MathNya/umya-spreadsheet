use super::Address;

#[derive(Clone, Default, Debug)]
pub struct DefinedName {
    name: String,
    address: Address,
    is_local_only: bool,
}
impl DefinedName {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn set_name<S: Into<String>>(&mut self, value: S) {
        self.name = value.into();
    }

    pub fn get_address(&self) -> String {
        self.address.get_address()
    }

    pub(crate) fn get_address_obj(&self) -> &Address {
        &self.address
    }

    pub(crate) fn get_address_obj_mut(&mut self) -> &mut Address {
        &mut self.address
    }

    pub(crate) fn set_address<S: Into<String>>(&mut self, value: S) {
        let mut address = Address::default();
        address.set_address(value);
        self.address = address;
    }

    pub fn get_is_local_only(&self) -> &bool {
        &self.is_local_only
    }

    pub(crate) fn set_is_local_only(&mut self, value: bool) {
        self.is_local_only = value;
    }
}
