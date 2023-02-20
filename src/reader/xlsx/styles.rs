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
    let mut buf = Vec::new();

    let theme = spreadsheet.get_theme().clone();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().into_inner() {
                    b"styleSheet" => {
                        let mut obj = Stylesheet::default();
                        obj.set_attributes(&mut reader, e);

                        // set ThemeColor
                        for font in obj.get_fonts_mut().get_font_mut() {
                            let color = font.get_color_mut();
                            color.set_argb_by_theme(&theme);
                        }
                        for fill in obj.get_fills_mut().get_fill_mut() {
                            match fill.get_pattern_fill() {
                                Some(_) => {
                                    match fill.get_pattern_fill_mut().get_foreground_color() {
                                        Some(_) => {
                                            fill.get_pattern_fill_mut()
                                                .get_foreground_color_mut()
                                                .set_argb_by_theme(&theme);
                                        }
                                        None => {}
                                    }
                                    match fill.get_pattern_fill_mut().get_background_color() {
                                        Some(_) => {
                                            fill.get_pattern_fill_mut()
                                                .get_background_color_mut()
                                                .set_argb_by_theme(&theme);
                                        }
                                        None => {}
                                    }
                                }
                                None => {}
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
                    _ => (),
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}
