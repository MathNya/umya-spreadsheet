// mc:Choice
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug, Clone)]
pub struct AlternateContentChoice {

}
impl AlternateContentChoice {


    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // mc:Choice
        write_start_tag(writer, "mc:Choice", vec![
            ("Requires", "c14"),
            ("xmlns:c14", "http://schemas.microsoft.com/office/drawing/2007/8/2/chart"),
        ], false);

        // c14:style
        write_start_tag(writer, "c14:style", vec![
            ("val", "102"),
        ], true);

        write_end_tag(writer, "mc:Choice");
    }
}
