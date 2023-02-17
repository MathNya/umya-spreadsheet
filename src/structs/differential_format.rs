// dxf
use super::Alignment;
use super::Borders;
use super::Fill;
use super::Font;
use super::Style;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct DifferentialFormat {
    font: Option<Font>,
    fill: Option<Fill>,
    borders: Option<Borders>,
    alignment: Option<Alignment>,
}
impl DifferentialFormat {
    pub(crate) fn _get_font(&self) -> &Option<Font> {
        &self.font
    }

    pub(crate) fn _get_font_mut(&mut self) -> &mut Option<Font> {
        &mut self.font
    }

    pub(crate) fn set_font(&mut self, value: Font) -> &mut Self {
        self.font = Some(value);
        self
    }

    pub(crate) fn _get_fill(&self) -> &Option<Fill> {
        &self.fill
    }

    pub(crate) fn _get_fill_mut(&mut self) -> &mut Option<Fill> {
        &mut self.fill
    }

    pub(crate) fn set_fill(&mut self, value: Fill) -> &mut Self {
        self.fill = Some(value);
        self
    }

    pub(crate) fn _get_borders(&self) -> &Option<Borders> {
        &self.borders
    }

    pub(crate) fn _get_borders_mut(&mut self) -> &mut Option<Borders> {
        &mut self.borders
    }

    pub(crate) fn set_borders(&mut self, value: Borders) -> &mut Self {
        self.borders = Some(value);
        self
    }

    pub(crate) fn _get_alignment(&self) -> &Option<Alignment> {
        &self.alignment
    }

    pub(crate) fn _get_alignment_mut(&mut self) -> &mut Option<Alignment> {
        &mut self.alignment
    }

    pub(crate) fn set_alignment(&mut self, value: Alignment) -> &mut Self {
        self.alignment = Some(value);
        self
    }

    pub(crate) fn get_style(&self) -> Style {
        let mut style = Style::default();
        style.set_font_crate(self.font.clone());
        style.set_fill_crate(self.fill.clone());
        style.set_borders_crate(self.borders.clone());
        style.set_alignment_crate(self.alignment.clone());
        style
    }

    pub(crate) fn set_style(&mut self, style: &Style) {
        self.font = style.get_font().clone();
        self.fill = style.get_fill().clone();
        self.borders = style.get_borders().clone();
        self.alignment = style.get_alignment().clone();
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}{}",
                match &self.font {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
                match &self.fill {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
                match &self.borders {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
                match &self.alignment {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"font" => {
                        let mut obj = Font::default();
                        obj.set_attributes(reader, e);
                        self.set_font(obj);
                    }
                    b"fill" => {
                        let mut obj = Fill::default();
                        obj.set_attributes(reader, e);
                        self.set_fill(obj);
                    }
                    b"border" => {
                        let mut obj = Borders::default();
                        obj.set_attributes(reader, e);
                        self.set_borders(obj);
                    }
                    b"alignment" => {
                        let mut obj = Alignment::default();
                        obj.set_attributes(reader, e);
                        self.set_alignment(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"dxf" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "dxf"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dxf
        write_start_tag(writer, "dxf", vec![], false);

        // font
        match &self.font {
            Some(v) => {
                v.write_to_font(writer);
            }
            None => {}
        }

        // fill
        match &self.fill {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // border
        match &self.borders {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // alignment
        match &self.alignment {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "dxf");
    }
}
