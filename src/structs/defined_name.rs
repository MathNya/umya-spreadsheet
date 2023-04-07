use super::Address;
use super::StringValue;
use helper::address::*;

#[derive(Clone, Default, Debug)]
pub struct DefinedName {
    name: String,
    address: Address,
    string_value: StringValue,
    is_local_only: bool,
}
impl DefinedName {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name = value.into();
        self
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

    pub(crate) fn set_address<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value = value.into();
        if is_address(&value) {
            self.address.set_address(value);
        } else {
            self.set_string_value(value);
        }
        self
    }

    pub fn get_address_str(&self) -> String {
        if self.string_value.has_value() {
            return self.string_value.get_value_string().to_string();
        }
        self.address.get_address()
    }

    pub fn set_string_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.address = Address::default();
        self.string_value.set_value(value);
        self
    }

    pub fn get_is_local_only(&self) -> &bool {
        &self.is_local_only
    }

    pub(crate) fn set_is_local_only(&mut self, value: bool) {
        self.is_local_only = value;
    }
}
