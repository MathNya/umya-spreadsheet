// xdr:cNvCxnSpPr
use super::connection_type::ConnectionType;

#[derive(Default, Debug)]
pub struct NonVisualConnectorShapeDrawingProperties {
    start_connection: Option<ConnectionType>,
    end_connection: Option<ConnectionType>,
}
impl NonVisualConnectorShapeDrawingProperties {
    pub fn get_start_connection(&self) -> &Option<ConnectionType> {
        &self.start_connection
    }

    pub fn set_start_connection(&mut self, value:ConnectionType) {
        self.start_connection = Some(value);
    }

    pub fn remove_start_connection(&mut self) {
        self.start_connection = None;
    }

    pub fn get_end_connection(&self) -> &Option<ConnectionType> {
        &self.end_connection
    }

    pub fn set_end_connection(&mut self, value:ConnectionType) {
        self.end_connection = Some(value);
    }

    pub fn remove_end_connection(&mut self) {
        self.end_connection = None;
    }
}
