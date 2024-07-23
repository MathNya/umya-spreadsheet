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
use reader::driver::*;
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
    pub(crate) fn _get_font(&self) -> Option<&Font> {
        self.font.as_ref()
    }

    pub(crate) fn _get_font_mut(&mut self) -> Option<&mut Font> {
        self.font.as_mut()
    }

    pub(crate) fn set_font(&mut self, value: Font) -> &mut Self {
        self.font = Some(value);
        self
    }

    pub(crate) fn _get_fill(&self) -> Option<&Fill> {
        self.fill.as_ref()
    }

    pub(crate) fn _get_fill_mut(&mut self) -> Option<&mut Fill> {
        self.fill.as_mut()
    }

    pub(crate) fn set_fill(&mut self, value: Fill) -> &mut Self {
        self.fill = Some(value);
        self
    }

    pub(crate) fn _get_borders(&self) -> Option<&Borders> {
        self.borders.as_ref()
    }

    pub(crate) fn _get_borders_mut(&mut self) -> Option<&mut Borders> {
        self.borders.as_mut()
    }

    pub(crate) fn set_borders(&mut self, value: Borders) -> &mut Self {
        self.borders = Some(value);
        self
    }

    pub(crate) fn _get_alignment(&self) -> Option<&Alignment> {
        self.alignment.as_ref()
    }

    pub(crate) fn _get_alignment_mut(&mut self) -> Option<&mut Alignment> {
        self.alignment.as_mut()
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
        self.font = style.get_font().cloned();
        self.fill = style.get_fill().cloned();
        self.borders = style.get_borders().cloned();
        self.alignment = style.get_alignment().cloned();
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
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
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
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"dxf" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "dxf")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dxf
        write_start_tag(writer, "dxf", vec![], false);

        // font
        if let Some(v) = &self.font {
            v.write_to_font(writer);
        }

        // fill
        if let Some(v) = &self.fill {
            v.write_to(writer);
        }

        // border
        if let Some(v) = &self.borders {
            v.write_to(writer);
        }

        // alignment
        if let Some(v) = &self.alignment {
            v.write_to(writer);
        }

        write_end_tag(writer, "dxf");
    }
}
