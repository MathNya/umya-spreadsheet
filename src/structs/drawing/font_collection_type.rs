// a:majorFont
// a:minorFont
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::drawing::SupplementalFont;
use writer::driver::*;

const MAJOR_FONTS: &[(&str, &str)] = &[
    ("Jpan", "ＭＳ Ｐゴシック"),
    ("Hang", "맑은 고딕"),
    ("Hans", "宋体"),
    ("Hant", "新細明體"),
    ("Arab", "Times New Roman"),
    ("Hebr", "Times New Roman"),
    ("Thai", "Tahoma"),
    ("Ethi", "Nyala"),
    ("Beng", "Vrinda"),
    ("Gujr", "Shruti"),
    ("Khmr", "MoolBoran"),
    ("Knda", "Tunga"),
    ("Guru", "Raavi"),
    ("Cans", "Euphemia"),
    ("Cher", "Plantagenet Cherokee"),
    ("Yiii", "Microsoft Yi Baiti"),
    ("Tibt", "Microsoft Himalaya"),
    ("Thaa", "MV Boli"),
    ("Deva", "Mangal"),
    ("Telu", "Gautami"),
    ("Taml", "Latha"),
    ("Syrc", "Estrangelo Edessa"),
    ("Orya", "Kalinga"),
    ("Mlym", "Kartika"),
    ("Laoo", "DokChampa"),
    ("Sinh", "Iskoola Pota"),
    ("Mong", "Mongolian Baiti"),
    ("Viet", "Times New Roman"),
    ("Uigh", "Microsoft Uighur"),
    ("Geor", "Sylfaen"),
];

const MINOR_FONTS: &[(&str, &str)] = &[
    ("Jpan", "ＭＳ Ｐゴシック"),
    ("Hang", "맑은 고딕"),
    ("Hans", "宋体"),
    ("Hant", "新細明體"),
    ("Arab", "Arial"),
    ("Hebr", "Arial"),
    ("Thai", "Tahoma"),
    ("Ethi", "Nyala"),
    ("Beng", "Vrinda"),
    ("Gujr", "Shruti"),
    ("Khmr", "DaunPenh"),
    ("Knda", "Tunga"),
    ("Guru", "Raavi"),
    ("Cans", "Euphemia"),
    ("Cher", "Plantagenet Cherokee"),
    ("Yiii", "Microsoft Yi Baiti"),
    ("Tibt", "Microsoft Himalaya"),
    ("Thaa", "MV Boli"),
    ("Deva", "Mangal"),
    ("Telu", "Gautami"),
    ("Taml", "Latha"),
    ("Syrc", "Estrangelo Edessa"),
    ("Orya", "Kalinga"),
    ("Mlym", "Kartika"),
    ("Laoo", "DokChampa"),
    ("Sinh", "Iskoola Pota"),
    ("Mong", "Mongolian Baiti"),
    ("Viet", "Arial"),
    ("Uigh", "Microsoft Uighur"),
    ("Geor", "Sylfaen"),
];

#[derive(Clone, Default, Debug)]
pub struct FontCollectionType {
    supplemental_font_list: Vec<SupplementalFont>,
}
impl FontCollectionType {
    pub fn get_supplemental_font_list(&self) -> &Vec<SupplementalFont> {
        &self.supplemental_font_list
    }

    pub fn get_supplemental_font_list_mut(&mut self) -> &mut Vec<SupplementalFont> {
        &mut self.supplemental_font_list
    }

    pub fn set_supplemental_font_list(&mut self, value: Vec<SupplementalFont>) -> &mut Self {
        self.supplemental_font_list = value;
        self
    }

    pub fn add_supplemental_font_list(&mut self, value: SupplementalFont) -> &mut Self {
        self.supplemental_font_list.push(value);
        self
    }

    pub(crate) fn set_defalut_value_major(&mut self) -> &mut Self {
        for (font_script, typeface) in self::MAJOR_FONTS {
            let mut obj = SupplementalFont::default();
            obj.set_script(*font_script).set_typeface(*typeface);
            self.supplemental_font_list.push(obj);
        }
        self
    }

    pub(crate) fn set_defalut_value_minor(&mut self) -> &mut Self {
        for (font_script, typeface) in self::MINOR_FONTS {
            let mut obj = SupplementalFont::default();
            obj.set_script(*font_script).set_typeface(*typeface);
            self.supplemental_font_list.push(obj);
        }
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"a:font" => {
                        let mut obj = SupplementalFont::default();
                        obj.set_attributes(reader, e);
                        self.add_supplemental_font_list(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"a:majorFont" => {
                        return;
                    }
                    b"a:minorFont" => {
                        return;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => {
                    panic!("Error not find {} end element", "a:majorFont, a:minorFont")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to_major_font(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:majorFont
        write_start_tag(writer, "a:majorFont", vec![], false);

        // a:latin
        write_start_tag(writer, "a:latin", vec![("typeface", "Calibri")], true);

        // a:ea
        write_start_tag(writer, "a:ea", vec![("typeface", "")], true);

        // a:cs
        write_start_tag(writer, "a:cs", vec![("typeface", "")], true);

        // a:font
        for obj in &self.supplemental_font_list {
            obj.write_to(writer);
        }

        write_end_tag(writer, "a:majorFont");
    }

    pub(crate) fn write_to_minor_font(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:minorFont
        write_start_tag(writer, "a:minorFont", vec![], false);

        // a:latin
        write_start_tag(writer, "a:latin", vec![("typeface", "Cambria")], true);

        // a:ea
        write_start_tag(writer, "a:ea", vec![("typeface", "")], true);

        // a:cs
        write_start_tag(writer, "a:cs", vec![("typeface", "")], true);

        // a:font
        for obj in &self.supplemental_font_list {
            obj.write_to(writer);
        }

        write_end_tag(writer, "a:minorFont");
    }
}
