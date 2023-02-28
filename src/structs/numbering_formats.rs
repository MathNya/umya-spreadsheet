// numFmts
use super::NumberingFormat;
use super::Style;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::collections::HashMap;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct NumberingFormats {
    numbering_format: HashMap<u32, NumberingFormat>,
}
impl NumberingFormats {
    pub(crate) fn get_numbering_format(&self) -> &HashMap<u32, NumberingFormat> {
        &self.numbering_format
    }

    pub(crate) fn _get_numbering_format_mut(&mut self) -> &mut HashMap<u32, NumberingFormat> {
        &mut self.numbering_format
    }

    pub(crate) fn set_numbering_format(&mut self, value: NumberingFormat) -> &mut Self {
        let number_format_id = value.get_number_format_id();
        self.numbering_format.insert(*number_format_id, value);
        self
    }

    pub(crate) fn _init_setup(&mut self) -> &mut Self {
        self.get_build_in_formats();
        self
    }

    pub(crate) fn get_build_in_formats(&mut self) {
        for (index, code) in super::numbering_format::FILL_BUILT_IN_FORMAT_CODES.iter() {
            let mut obj = NumberingFormat::default();
            obj.set_number_format_id_crate(*index)
                .set_format_code_crate(code.clone());
            self.set_numbering_format(obj);
        }
    }

    pub(crate) fn set_style(&mut self, style: &Style) -> u32 {
        match style.get_numbering_format() {
            Some(v) => {
                let number_format_id = v.get_number_format_id();
                if v.get_is_build_in() == &true {
                    return *number_format_id;
                }
                if self.numbering_format.get(number_format_id).is_some() {
                    return *number_format_id;
                }
                let hash_code = v.get_hash_code();
                let mut id = 175;
                for (index, numbering_format) in &self.numbering_format {
                    if numbering_format.get_hash_code() == hash_code {
                        return *index;
                    }
                    if &id < index {
                        id = *index;
                    }
                }
                id += 1;
                let mut num = v.clone();
                num.set_number_format_id_crate(id);
                self.set_numbering_format(num);
                id
            }
            None => 0,
        }
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
                    b"numFmt" => {
                        let mut obj = NumberingFormat::default();
                        obj.set_attributes(reader, e);
                        self.set_numbering_format(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"numFmts" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "numFmts"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let mut has_format = false;
        let mut cnt = 0;
        for (_, numbering_format) in &self.numbering_format {
            if numbering_format.get_is_build_in() == &false {
                has_format = true;
                cnt += 1;
            }
        }

        if has_format {
            // numFmts
            let cnt_str = cnt.to_string();
            write_start_tag(writer, "numFmts", vec![("count", &cnt_str)], false);

            // numFmt
            for (index, numbering_format) in &self.numbering_format {
                if numbering_format.get_is_build_in() == &false {
                    numbering_format.write_to(writer, index);
                }
            }

            write_end_tag(writer, "numFmts");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_style() {
        let mut obj = NumberingFormats::default();

        let mut style = Style::default();
        style
            .get_number_format_mut()
            .set_format_code(NumberingFormat::FORMAT_TEXT);
        assert_eq!(obj.set_style(&style), 49);

        let mut style = Style::default();
        style
            .get_number_format_mut()
            .set_format_code("test-yyyy-mm-dd");
        assert_eq!(obj.set_style(&style), 176);

        let mut style = Style::default();
        style
            .get_number_format_mut()
            .set_format_code("test-yyyy/mm/dd");
        assert_eq!(obj.set_style(&style), 177);

        let mut style = Style::default();
        style
            .get_number_format_mut()
            .set_format_code("test-yyyy-mm-dd");
        assert_eq!(obj.set_style(&style), 176);

        let mut style = Style::default();
        style.get_number_format_mut().set_format_code("m/d/yy");
        assert_eq!(obj.set_style(&style), 30);

        let mut style = Style::default();
        style
            .get_number_format_mut()
            .set_format_code(NumberingFormat::FORMAT_TEXT);
        assert_eq!(obj.set_style(&style), 49);

        let mut style = Style::default();
        style
            .get_number_format_mut()
            .set_format_code(NumberingFormat::FORMAT_DATE_TIME5);
        assert_eq!(obj.set_style(&style), 45);
    }
}
