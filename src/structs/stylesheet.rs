// styleSheet
use super::BordersCrate;
use super::CellFormat;
use super::CellFormats;
use super::CellStyleFormats;
use super::CellStyles;
use super::Colors;
use super::DifferentialFormats;
use super::Fills;
use super::Fonts;
use super::NumberingFormats;
use super::Style;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct Stylesheet {
    numbering_formats: NumberingFormats,
    fonts: Fonts,
    fills: Fills,
    borders: BordersCrate,
    cell_style_formats: CellStyleFormats,
    cell_formats: CellFormats,
    cell_styles: CellStyles,
    differential_formats: DifferentialFormats,
    colors: Colors,
    maked_style_list: Vec<Style>,
}
impl Stylesheet {
    pub(crate) fn _get_numbering_formats(&self) -> &NumberingFormats {
        &self.numbering_formats
    }

    pub(crate) fn _get_numbering_formats_mut(&mut self) -> &mut NumberingFormats {
        &mut self.numbering_formats
    }

    pub(crate) fn _set_numbering_formats(&mut self, value: NumberingFormats) -> &mut Self {
        self.numbering_formats = value;
        self
    }

    pub(crate) fn _get_fonts(&self) -> &Fonts {
        &self.fonts
    }

    pub(crate) fn get_fonts_mut(&mut self) -> &mut Fonts {
        &mut self.fonts
    }

    pub(crate) fn _set_fonts(&mut self, value: Fonts) -> &mut Self {
        self.fonts = value;
        self
    }

    pub(crate) fn _get_fills(&self) -> &Fills {
        &self.fills
    }

    pub(crate) fn get_fills_mut(&mut self) -> &mut Fills {
        &mut self.fills
    }

    pub(crate) fn _set_fills(&mut self, value: Fills) -> &mut Self {
        self.fills = value;
        self
    }

    pub(crate) fn _get_borders(&self) -> &BordersCrate {
        &self.borders
    }

    pub(crate) fn get_borders_mut(&mut self) -> &mut BordersCrate {
        &mut self.borders
    }

    pub(crate) fn _set_borders(&mut self, value: BordersCrate) -> &mut Self {
        self.borders = value;
        self
    }

    pub(crate) fn _get_cell_style_formats(&self) -> &CellStyleFormats {
        &self.cell_style_formats
    }

    pub(crate) fn _get_cell_style_formats_mut(&mut self) -> &mut CellStyleFormats {
        &mut self.cell_style_formats
    }

    pub(crate) fn _set_cell_style_formats(&mut self, value: CellStyleFormats) -> &mut Self {
        self.cell_style_formats = value;
        self
    }

    pub(crate) fn _get_cell_formats(&self) -> &CellFormats {
        &self.cell_formats
    }

    pub(crate) fn _get_cell_formats_mut(&mut self) -> &mut CellFormats {
        &mut self.cell_formats
    }

    pub(crate) fn _set_cell_formats(&mut self, value: CellFormats) -> &mut Self {
        self.cell_formats = value;
        self
    }

    pub(crate) fn _get_cell_styles(&self) -> &CellStyles {
        &self.cell_styles
    }

    pub(crate) fn _get_cell_styles_mut(&mut self) -> &mut CellStyles {
        &mut self.cell_styles
    }

    pub(crate) fn _set_cell_styles(&mut self, value: CellStyles) -> &mut Self {
        self.cell_styles = value;
        self
    }

    pub(crate) fn get_differential_formats(&self) -> &DifferentialFormats {
        &self.differential_formats
    }

    pub(crate) fn get_differential_formats_mut(&mut self) -> &mut DifferentialFormats {
        &mut self.differential_formats
    }

    pub(crate) fn _set_differential_formats(&mut self, value: DifferentialFormats) -> &mut Self {
        self.differential_formats = value;
        self
    }

    pub(crate) fn _get_colors(&self) -> &Colors {
        &self.colors
    }

    pub(crate) fn _get_colors_mut(&mut self) -> &mut Colors {
        &mut self.colors
    }

    pub(crate) fn _set_colors(&mut self, value: Colors) -> &mut Self {
        self.colors = value;
        self
    }

    pub(crate) fn get_style(&self, id: usize) -> Style {
        self.maked_style_list.get(id).unwrap().clone()
    }

    pub(crate) fn make_style(&mut self) -> &mut Self {
        for cell_format in self.cell_formats.get_cell_format() {
            let def_cell_format = self
                .cell_style_formats
                .get_cell_format()
                .get(*cell_format.get_format_id() as usize)
                .cloned()
                .unwrap_or_default();

            let mut style = Style::default();
            self.get_style_by_cell_format(&mut style, &def_cell_format, cell_format);
            self.maked_style_list.push(style);
        }

        self
    }

    pub(crate) fn get_style_by_cell_format(
        &self,
        style: &mut Style,
        def_cell_format: &CellFormat,
        cell_format: &CellFormat,
    ) {
        // number_format
        let mut apply = true;
        if def_cell_format.has_apply_number_format() {
            apply = *def_cell_format.get_apply_number_format();
        }
        if cell_format.has_apply_number_format() {
            apply = *cell_format.get_apply_number_format();
        }
        if apply {
            let id = cell_format.get_number_format_id();
            let obj = self
                .numbering_formats
                .get_numbering_format()
                .get(id)
                .unwrap();
            style.set_numbering_format(obj.clone());
        }

        // font
        let mut apply = true;
        if def_cell_format.has_apply_font() {
            apply = *def_cell_format.get_apply_font();
        }
        if cell_format.has_apply_font() {
            apply = *cell_format.get_apply_font();
        }
        if apply {
            let id = *cell_format.get_font_id() as usize;
            let obj = self.fonts.get_font().get(id).unwrap();
            style.set_font(obj.clone());
        }

        // fill
        let mut apply = true;
        if def_cell_format.has_apply_fill() {
            apply = *def_cell_format.get_apply_fill();
        }
        if cell_format.has_apply_fill() {
            apply = *cell_format.get_apply_fill();
        }
        if apply {
            let id = *cell_format.get_fill_id() as usize;
            let obj = self.fills.get_fill().get(id).unwrap();
            style.set_fill(obj.clone());
        }

        // borders
        let mut apply = true;
        if def_cell_format.has_apply_border() {
            apply = *def_cell_format.get_apply_border();
        }
        if cell_format.has_apply_border() {
            apply = *cell_format.get_apply_border();
        }
        if apply {
            let id = *cell_format.get_border_id() as usize;
            let obj = self.borders.get_borders().get(id).unwrap();
            style.set_borders(obj.clone());
        }

        // format_id
        style.set_format_id(*cell_format.get_format_id());

        // alignment
        let mut apply = true;
        if def_cell_format.has_apply_alignment() {
            apply = *def_cell_format.get_apply_alignment();
        }
        if cell_format.has_apply_alignment() {
            apply = *cell_format.get_apply_alignment();
        }
        if apply {
            match def_cell_format.get_alignment() {
                Some(v) => {
                    style.set_alignment(v.clone());
                }
                None => {}
            }
            match cell_format.get_alignment() {
                Some(v) => {
                    style.set_alignment(v.clone());
                }
                None => {}
            }
        }
    }

    pub(crate) fn set_style(&mut self, style: &Style) -> u32 {
        let mut index = 0;
        let def_style = Style::default();
        if style == &def_style {
            return index;
        }
        for maked_style in &self.maked_style_list {
            if style == maked_style {
                return index;
            }
            index += 1;
        }
        let mut cell_format = CellFormat::default();

        let number_format_id = self.numbering_formats.set_style(style);
        let font_id = self.fonts.set_style(style);
        let fill_id = self.fills.set_style(style);
        let border_id = self.borders.set_style(style);
        let format_id = 0;

        cell_format.set_number_format_id(number_format_id);
        cell_format.set_font_id(font_id);
        cell_format.set_fill_id(fill_id);
        cell_format.set_border_id(border_id);
        cell_format.set_format_id(format_id);

        match style.get_numbering_format() {
            Some(_) => {
                cell_format.set_apply_number_format(true);
            }
            None => {}
        }

        match style.get_font() {
            Some(_) => {
                cell_format.set_apply_font(true);
            }
            None => {}
        }

        match style.get_fill() {
            Some(_) => {
                cell_format.set_apply_fill(true);
            }
            None => {}
        }

        match style.get_borders() {
            Some(_) => {
                cell_format.set_apply_border(true);
            }
            None => {}
        }

        match style.get_alignment() {
            Some(v) => {
                cell_format.set_alignment(v.clone());
                cell_format.set_apply_alignment(true);
            }
            None => {}
        }

        self.maked_style_list.push(style.clone());
        self.cell_formats.set_cell_format(cell_format);
        index
    }

    pub(crate) fn set_defalut_value(&mut self) -> &mut Self {
        let style = Style::get_defalut_value();
        self.set_style(&style);
        let style = Style::get_defalut_value_2();
        self.set_style(&style);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        self.numbering_formats.get_build_in_formats();

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"numFmts" => {
                        self.numbering_formats.set_attributes(reader, e);
                    }
                    b"fonts" => {
                        self.fonts.set_attributes(reader, e);
                    }
                    b"fills" => {
                        self.fills.set_attributes(reader, e);
                    }
                    b"borders" => {
                        self.borders.set_attributes(reader, e);
                    }
                    b"cellStyleXfs" => {
                        self.cell_style_formats.set_attributes(reader, e);
                    }
                    b"cellXfs" => {
                        self.cell_formats.set_attributes(reader, e);
                    }
                    b"cellStyles" => {
                        self.cell_styles.set_attributes(reader, e);
                    }
                    b"dxfs" => {
                        self.differential_formats.set_attributes(reader, e);
                    }
                    b"colors" => {
                        self.colors.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"styleSheet" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "styleSheet"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // styleSheet
        write_start_tag(
            writer,
            "styleSheet",
            vec![
                (
                    "xmlns",
                    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                ),
                (
                    "xmlns:mc",
                    "http://schemas.openxmlformats.org/markup-compatibility/2006",
                ),
                ("mc:Ignorable", "x14ac"),
                (
                    "xmlns:x14ac",
                    "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac",
                ),
            ],
            false,
        );

        // numFmts
        let _ = &self.numbering_formats.write_to(writer);

        // fonts
        let _ = &self.fonts.write_to(writer);

        // fills
        let _ = &self.fills.write_to(writer);

        // borders
        let _ = &self.borders.write_to(writer);

        // cellStyleXfs
        let _ = &self.cell_style_formats.write_to(writer);

        // cellXfs
        let _ = &self.cell_formats.write_to(writer);

        // cellStyles
        let _ = &self.cell_styles.write_to(writer);

        // dxfs
        let _ = &self.differential_formats.write_to(writer);

        // colors
        let _ = &self.colors.write_to(writer);

        // tableStyles
        write_start_tag(
            writer,
            "tableStyles",
            vec![
                ("count", "0"),
                ("defaultTableStyle", "TableStyleMedium2"),
                ("defaultPivotStyle", "PivotStyleMedium9"),
            ],
            true,
        );

        // extLst
        write_start_tag(writer, "extLst", vec![], false);

        // ext
        write_start_tag(
            writer,
            "ext",
            vec![
                ("uri", "{EB79DEF2-80B8-43e5-95BD-54CBDDF9020C}"),
                (
                    "xmlns:x14",
                    "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main",
                ),
            ],
            false,
        );

        // x14:slicerStyles
        write_start_tag(
            writer,
            "x14:slicerStyles",
            vec![("defaultSlicerStyle", "SlicerStyleLight1")],
            true,
        );

        write_end_tag(writer, "ext");

        write_end_tag(writer, "extLst");

        write_end_tag(writer, "styleSheet");
    }
}
