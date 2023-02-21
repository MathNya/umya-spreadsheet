// a:majorFont
// a:minorFont
use super::SupplementalFont;
use super::TextFontType;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct FontCollectionType {
    latin_font: TextFontType,
    east_asian_font: TextFontType,
    complex_script_font: TextFontType,
    supplemental_font_list: Vec<SupplementalFont>,
}
impl FontCollectionType {
    pub fn get_latin_font(&self) -> &TextFontType {
        &self.latin_font
    }

    pub fn get_latin_font_mut(&mut self) -> &mut TextFontType {
        &mut self.latin_font
    }

    pub fn set_latin_font(&mut self, value: TextFontType) -> &mut Self {
        self.latin_font = value;
        self
    }

    pub fn get_east_asian_font(&self) -> &TextFontType {
        &self.east_asian_font
    }

    pub fn get_east_asian_font_mut(&mut self) -> &mut TextFontType {
        &mut self.east_asian_font
    }

    pub fn set_east_asian_font(&mut self, value: TextFontType) -> &mut Self {
        self.east_asian_font = value;
        self
    }

    pub fn get_complex_script_font(&self) -> &TextFontType {
        &self.complex_script_font
    }

    pub fn get_complex_script_font_mut(&mut self) -> &mut TextFontType {
        &mut self.complex_script_font
    }

    pub fn set_complex_script_font(&mut self, value: TextFontType) -> &mut Self {
        self.complex_script_font = value;
        self
    }

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
        self.latin_font.set_typeface("Calibri Light");
        self.latin_font.set_panose("020F0302020204030204");
        self.east_asian_font.set_typeface("");
        self.complex_script_font.set_typeface("");
        for (font_script, typeface) in self::MAJOR_FONTS {
            let mut obj = SupplementalFont::default();
            obj.set_script(*font_script).set_typeface(*typeface);
            self.supplemental_font_list.push(obj);
        }
        self
    }

    pub(crate) fn set_defalut_value_minor(&mut self) -> &mut Self {
        self.latin_font.set_typeface("Calibri");
        self.latin_font.set_panose("020F0502020204030204");
        self.east_asian_font.set_typeface("");
        self.complex_script_font.set_typeface("");
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
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:latin" => {
                        self.latin_font.set_attributes(reader, e);
                    }
                    b"a:ea" => {
                        self.east_asian_font.set_attributes(reader, e);
                    }
                    b"a:cs" => {
                        self.complex_script_font.set_attributes(reader, e);
                    }
                    b"a:font" => {
                        let mut obj = SupplementalFont::default();
                        obj.set_attributes(reader, e);
                        self.add_supplemental_font_list(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
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
        self.write_to(writer, "a:majorFont");
    }

    pub(crate) fn write_to_minor_font(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:minorFont
        self.write_to(writer, "a:minorFont");
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tab_name: &str) {
        write_start_tag(writer, tab_name, vec![], false);

        // a:latin
        self.latin_font.write_to_latin(writer);

        // a:ea
        self.east_asian_font.write_to_ea(writer);

        // a:cs
        self.complex_script_font.write_to_cs(writer);

        // a:font
        for obj in &self.supplemental_font_list {
            obj.write_to(writer);
        }

        write_end_tag(writer, tab_name);
    }
}

const MAJOR_FONTS: &[(&str, &str)] = &[
    ("Jpan", "游ゴシック Light"),
    ("Hang", "맑은 고딕"),
    ("Hans", "等线 Light"),
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
    ("Jpan", "游ゴシック"),
    ("Hang", "맑은 고딕"),
    ("Hans", "等线"),
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
