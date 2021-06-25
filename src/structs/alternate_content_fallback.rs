// mc:Fallback
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug, Clone)]
pub struct AlternateContentFallback {

}
impl AlternateContentFallback {

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // mc:Fallback
        write_start_tag(writer, "mc:Fallback", vec![], false);

        // c:style
        write_start_tag(writer, "c:style", vec![
            ("val", "2"),
        ], true);

        write_end_tag(writer, "mc:Fallback");
    }
}
