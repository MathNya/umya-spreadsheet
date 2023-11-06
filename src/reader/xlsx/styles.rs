use crate::xml_read_loop;

use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};
use structs::Spreadsheet;
use structs::Stylesheet;

const FILE_PATH: &str = "xl/styles.xml";

pub fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<(), XlsxError> {
    let r = io::BufReader::new(arv.by_name(FILE_PATH)?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);

    let theme = spreadsheet.get_theme().clone();

    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"styleSheet" {
                let mut obj = Stylesheet::default();
                obj.set_attributes(&mut reader, e);

                // set ThemeColor
                for font in obj.get_fonts_mut().get_font_mut() {
                    let color = font.get_color_mut();
                    color.set_argb_by_theme(&theme);
                }

                for fill in obj.get_fills_mut().get_fill_mut() {
                    if fill.get_pattern_fill().is_some() {
                        if fill.get_pattern_fill_mut().get_foreground_color().is_some() {
                            fill.get_pattern_fill_mut()
                                .get_foreground_color_mut()
                                .set_argb_by_theme(&theme);
                        }
                        if fill.get_pattern_fill_mut().get_background_color().is_some() {
                            fill.get_pattern_fill_mut()
                                .get_background_color_mut()
                                .set_argb_by_theme(&theme);
                        }
                    }
                }

                for border in obj.get_borders_mut().get_borders_mut() {
                    let color = border.get_left_border_mut().get_color_mut();
                    color.set_argb_by_theme(&theme);
                    let color = border.get_right_border_mut().get_color_mut();
                    color.set_argb_by_theme(&theme);
                    let color = border.get_top_border_mut().get_color_mut();
                    color.set_argb_by_theme(&theme);
                    let color = border.get_bottom_border_mut().get_color_mut();
                    color.set_argb_by_theme(&theme);
                    let color = border.get_diagonal_border_mut().get_color_mut();
                    color.set_argb_by_theme(&theme);
                    let color = border.get_vertical_border_mut().get_color_mut();
                    color.set_argb_by_theme(&theme);
                    let color = border.get_horizontal_border_mut().get_color_mut();
                    color.set_argb_by_theme(&theme);
                }

                obj.make_style();
                spreadsheet.set_stylesheet(obj);
            }
        },
        Event::Eof => break
    );

    Ok(())
}
