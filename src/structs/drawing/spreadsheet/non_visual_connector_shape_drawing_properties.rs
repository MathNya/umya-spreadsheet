// xdr:cNvCxnSpPr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::super::{
    EndConnection,
    StartConnection,
};
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct NonVisualConnectorShapeDrawingProperties {
    start_connection: Option<Box<StartConnection>>,
    end_connection:   Option<Box<EndConnection>>,
}

impl NonVisualConnectorShapeDrawingProperties {
    #[inline]
    #[must_use]
    pub fn get_start_connection(&self) -> Option<&StartConnection> {
        self.start_connection.as_deref()
    }

    #[inline]
    pub fn set_start_connection(&mut self, value: StartConnection) {
        self.start_connection = Some(Box::new(value));
    }

    #[inline]
    pub fn remove_start_connection(&mut self) {
        self.start_connection = None;
    }

    #[inline]
    #[must_use]
    pub fn get_end_connection(&self) -> Option<&EndConnection> {
        self.end_connection.as_deref()
    }

    #[inline]
    pub fn set_end_connection(&mut self, value: EndConnection) {
        self.end_connection = Some(Box::new(value));
    }

    #[inline]
    pub fn remove_end_connection(&mut self) {
        self.end_connection = None;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
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
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:cNvCxnSpPr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:cNvCxnSpPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:cNvCxnSpPr
        if self.start_connection.is_some() || self.end_connection.is_some() {
            write_start_tag(writer, "xdr:cNvCxnSpPr", vec![], false);

            // a:stCxn
            if let Some(v) = &self.start_connection {
                v.write_to(writer);
            }

            // a:endCxn
            if let Some(v) = &self.end_connection {
                v.write_to(writer);
            }

            write_end_tag(writer, "xdr:cNvCxnSpPr");
        } else {
            write_start_tag(writer, "xdr:cNvCxnSpPr", vec![], true);
        }
    }
}
