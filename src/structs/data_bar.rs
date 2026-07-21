use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    Color,
    ConditionalFormatValueObject,
};
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct DataBar {
    cfvo_collection:  Vec<ConditionalFormatValueObject>,
    color_collection: Vec<Color>,
}

impl DataBar {
    #[inline]
    #[must_use]
    pub fn cfvo_collection(&self) -> &[ConditionalFormatValueObject] {
        &self.cfvo_collection
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cfvo_collection()")]
    pub fn get_cfvo_collection(&self) -> &[ConditionalFormatValueObject] {
        self.cfvo_collection()
    }

    #[inline]
    pub fn set_cfvo_collection(&mut self, value: Vec<ConditionalFormatValueObject>) -> &mut Self {
        self.cfvo_collection = value;
        self
    }

    #[inline]
    pub fn add_cfvo_collection(&mut self, value: ConditionalFormatValueObject) -> &mut Self {
        self.cfvo_collection.push(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn color_collection(&self) -> &[Color] {
        &self.color_collection
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use color_collection()")]
    pub fn get_color_collection(&self) -> &[Color] {
        self.color_collection()
    }

    #[inline]
    pub fn set_color_collection(&mut self, value: impl Into<Vec<Color>>) -> &mut Self {
        self.color_collection = value.into();
        self
    }

    #[inline]
    pub fn add_color_collection(&mut self, value: Color) -> &mut Self {
        self.color_collection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            ref n @ (Event::Empty(ref e) | Event::Start(ref e)) => {
                let is_empty = matches!(n, Event::Empty(_));
                match e.name().into_inner() {
                    b"cfvo" => {
                        let mut obj = ConditionalFormatValueObject::default();
                        obj.set_attributes(reader, e, is_empty);
                        self.cfvo_collection.push(obj);
                    }
                    b"color" => {
                        let mut obj = Color::default();
                        obj.set_attributes(reader, e, is_empty);
                        self.color_collection.push(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"dataBar" {
                    return
                }
            },
            Event::Eof => return
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dataBar
        write_start_tag(writer, "dataBar", vec![], false);

        // cfvo
        for v in &self.cfvo_collection {
            v.write_to(writer);
        }

        // color
        for v in &self.color_collection {
            v.write_to_color(writer);
        }

        write_end_tag(writer, "dataBar");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_data_bar(xml: &str) -> DataBar {
        let mut reader = Reader::from_reader(std::io::BufReader::new(xml.as_bytes()));
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e))
                    if e.name().into_inner() == b"dataBar" =>
                {
                    let mut obj = DataBar::default();
                    obj.set_attributes(&mut reader, e);
                    return obj;
                }
                Ok(Event::Eof) => panic!("dataBar element not found"),
                _ => (),
            }
            buf.clear();
        }
    }

    #[test]
    fn read_child_elements_with_end_tags() {
        // Writers such as openpyxl emit <cfvo .../> children as
        // <cfvo ...></cfvo> (Start + End events instead of Empty).
        let obj = read_data_bar(
            r#"<dataBar showValue="1" minLength="10" maxLength="90"><cfvo type="num" val="0"></cfvo><cfvo type="num" val="1400"></cfvo><color rgb="FF1E2761"></color></dataBar>"#,
        );
        assert_eq!(obj.get_cfvo_collection().len(), 2);
        assert_eq!(obj.get_color_collection().len(), 1);
        assert_eq!(obj.get_color_collection()[0].get_argb_str(), "FF1E2761");
    }

    #[test]
    fn read_child_elements_self_closing() {
        let obj = read_data_bar(
            r#"<dataBar><cfvo type="min"/><cfvo type="max"/><color rgb="FF638EC6"/></dataBar>"#,
        );
        assert_eq!(obj.get_cfvo_collection().len(), 2);
        assert_eq!(obj.get_color_collection().len(), 1);
        assert_eq!(obj.get_color_collection()[0].get_argb_str(), "FF638EC6");
    }
}
