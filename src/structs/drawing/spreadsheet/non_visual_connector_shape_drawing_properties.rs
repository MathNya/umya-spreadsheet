// xdr:cNvCxnSpPr
use super::super::EndConnection;
use super::super::StartConnection;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NonVisualConnectorShapeDrawingProperties {
    start_connection: Option<StartConnection>,
    end_connection: Option<EndConnection>,
}
impl NonVisualConnectorShapeDrawingProperties {
    pub fn get_start_connection(&self) -> &Option<StartConnection> {
        &self.start_connection
    }

    pub fn set_start_connection(&mut self, value: StartConnection) {
        self.start_connection = Some(value);
    }

    pub fn remove_start_connection(&mut self) {
        self.start_connection = None;
    }

    pub fn get_end_connection(&self) -> &Option<EndConnection> {
        &self.end_connection
    }

    pub fn set_end_connection(&mut self, value: EndConnection) {
        self.end_connection = Some(value);
    }

    pub fn remove_end_connection(&mut self) {
        self.end_connection = None;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:stCxn" => {
                        let mut connection_type = StartConnection::default();
                        connection_type.set_attributes(reader, e);
                        self.set_start_connection(connection_type);
                    }
                    b"a:endCxn" => {
                        let mut connection_type = EndConnection::default();
                        connection_type.set_attributes(reader, e);
                        self.set_end_connection(connection_type);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:cNvCxnSpPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:cNvCxnSpPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:cNvCxnSpPr
        if self.start_connection.is_some() || self.end_connection.is_some() {
            write_start_tag(writer, "xdr:cNvCxnSpPr", vec![], false);

            // a:stCxn
            match &self.start_connection {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            // a:endCxn
            match &self.end_connection {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            write_end_tag(writer, "xdr:cNvCxnSpPr");
        } else {
            write_start_tag(writer, "xdr:cNvCxnSpPr", vec![], true);
        }
    }
}
