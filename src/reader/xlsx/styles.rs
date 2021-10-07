use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use structs::Spreadsheet;
use structs::Stylesheet;

const FILE_PATH: &'static str = "xl/styles.xml";

pub fn read(dir: &TempDir, spreadsheet:&mut Spreadsheet) -> result::Result<(), XlsxError> {
    let path = dir.path().join(FILE_PATH);
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let theme = spreadsheet.get_theme().clone();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"styleSheet" => {
                        let mut obj = Stylesheet::default();
                        obj.set_attributes(&mut reader, e);

                        // set ThemeColor
                        for font in obj.get_fonts_mut().get_font_mut() {
                            let color = font.get_color_mut();
                            color.set_argb_by_theme(&theme);
                        }
                        for fill in obj.get_fills_mut().get_fill_mut() {
                            match fill.get_pattern_fill_mut().get_foreground_color_mut() {
                                Some(v) => {v.set_argb_by_theme(&theme);},
                                None => {},
                            }
                            match fill.get_pattern_fill_mut().get_background_color_mut() {
                                Some(v) => {v.set_argb_by_theme(&theme);},
                                None => {},
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

                        spreadsheet.set_stylesheet(obj);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}
