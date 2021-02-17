#[derive(Default, Debug)]
pub struct ConnectionType {
    id: String,
    index: String,
}
impl ConnectionType {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn set_id<S: Into<String>>(&mut self, value:S) {
        self.id = value.into();
    }

    pub fn get_index(&self) -> &str {
        &self.index
    }

    pub fn set_index<S: Into<String>>(&mut self, value:S) {
        self.index = value.into();
    }
}