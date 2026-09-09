//! Assert referenced OOXML components, independently of the workbook reader.
use std::{
    collections::{
        BTreeMap,
        BTreeSet,
    },
    io::{
        Cursor,
        Read,
        Write,
    },
};

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesEnd,
        Event,
    },
};
use umya_spreadsheet::{
    self as umya,
    Border,
    Color,
    ConditionalFormatValues,
    ConditionalFormatting,
    ConditionalFormattingRule,
    Formula,
    Style,
};

type Attributes = BTreeMap<String, String>;
#[derive(Default, Debug)]
struct Element {
    name:     String,
    attrs:    Attributes,
    children: Vec<Element>,
}
impl Element {
    fn child(&self, name: &str) -> &Self {
        self.children.iter().find(|node| node.name == name).unwrap()
    }

    fn id(&self, name: &str) -> usize {
        self.attrs[name].parse().unwrap()
    }
}
fn xml(source: &str) -> Element {
    let mut reader = Reader::from_str(source);
    let mut stack = vec![Element::default()];
    loop {
        let event = reader.read_event().unwrap();
        let empty = matches!(&event, Event::Empty(_));
        match event {
            Event::Start(e) | Event::Empty(e) => {
                let node = Element {
                    name:     String::from_utf8(e.name().as_ref().to_vec()).unwrap(),
                    attrs:    e
                        .attributes()
                        .map(|a| {
                            let a = a.unwrap();
                            (
                                String::from_utf8(a.key.as_ref().to_vec()).unwrap(),
                                a.decoded_and_normalized_value(
                                    quick_xml::XmlVersion::Implicit1_0,
                                    reader.decoder(),
                                )
                                .unwrap()
                                .into_owned(),
                            )
                        })
                        .collect(),
                    children: vec![],
                };
                if empty {
                    stack.last_mut().unwrap().children.push(node);
                } else {
                    stack.push(node);
                }
            }
            Event::End(e) => {
                let node = stack.pop().unwrap();
                assert_eq!(node.name.as_bytes(), e.name().as_ref());
                stack.last_mut().unwrap().children.push(node);
            }
            Event::Eof => break,
            _ => (),
        }
    }
    assert_eq!(stack.len(), 1);
    stack.pop().unwrap().children.pop().unwrap()
}
fn part(bytes: &[u8], name: &str) -> String {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut text = String::new();
    archive
        .by_name(name)
        .unwrap()
        .read_to_string(&mut text)
        .unwrap();
    text
}
fn save(book: &umya::Workbook) -> Vec<u8> {
    let mut bytes = vec![];
    umya::writer::xlsx::write_writer(book, &mut bytes).unwrap();
    bytes
}
fn expected(key: &str, value: &str, tint: Option<&str>) -> Attributes {
    let mut attrs = Attributes::from([(key.to_owned(), value.to_owned())]);
    if let Some(tint) = tint {
        attrs.insert("tint".into(), tint.into());
    }
    attrs
}
fn style(color: Color) -> Style {
    let mut style = Style::default();
    style.font_mut().set_color(color.clone());
    style
        .fill_mut()
        .pattern_fill_mut()
        .set_foreground_color(color.clone())
        .set_background_color(color.clone());
    style
        .borders_mut()
        .left_mut()
        .set_border_style(Border::BORDER_THIN);
    style.borders_mut().left_mut().set_color(color);
    style
}
fn fixture(colors: &[Color]) -> umya::Workbook {
    assert!(colors.len() <= 26);
    let mut book = umya::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    for (i, color) in colors.iter().enumerate() {
        let i = u32::try_from(i).unwrap();
        let style = style(color.clone());
        sheet.cell_mut((i + 1, 1)).set_style(style.clone());
        sheet.row_dimension_mut(i + 3).set_style(style.clone());
        sheet
            .column_dimension_by_number_mut(i + 1)
            .set_style(style.clone());
        let mut rule = ConditionalFormattingRule::default();
        rule.set_type(ConditionalFormatValues::Expression)
            .set_priority(i32::try_from(i + 1).unwrap())
            .set_style(style);
        let mut formula = Formula::default();
        formula.set_string_value("TRUE()");
        rule.set_formula(formula);
        let mut group = ConditionalFormatting::default();
        group.sequence_of_references_mut().set_sqref("A1:Z1");
        group.add_conditional_collection(rule);
        sheet.add_conditional_formatting_collection(group);
    }
    book
}
fn assert_components(font: &Element, fill: &Element, border: &Element, want: &Attributes) {
    assert_eq!(&font.child("color").attrs, want, "font");
    assert_eq!(
        &fill.child("patternFill").child("fgColor").attrs,
        want,
        "foreground fill"
    );
    assert_eq!(
        &fill.child("patternFill").child("bgColor").attrs,
        want,
        "background fill"
    );
    assert_eq!(
        &border.child("left").child("color").attrs,
        want,
        "left border"
    );
}
fn assert_projection(bytes: &[u8], wants: &[Attributes]) {
    let styles = xml(&part(bytes, "xl/styles.xml"));
    let sheet = xml(&part(bytes, "xl/worksheets/sheet1.xml"));
    let fonts = &styles.child("fonts").children;
    let fills = &styles.child("fills").children;
    let borders = &styles.child("borders").children;
    let xfs = &styles.child("cellXfs").children;
    let dxfs = &styles.child("dxfs").children;
    let mut font_ids = BTreeSet::new();
    let mut fill_ids = BTreeSet::new();
    let mut border_ids = BTreeSet::new();
    let mut dxf_ids = BTreeSet::new();
    for (i, want) in wants.iter().enumerate() {
        let address = format!("{}1", char::from(b'A' + u8::try_from(i).unwrap()));
        let first_row = sheet
            .child("sheetData")
            .children
            .iter()
            .find(|r| r.attrs.get("r").map(String::as_str) == Some("1"))
            .unwrap();
        let cell = first_row
            .children
            .iter()
            .find(|c| c.attrs.get("r") == Some(&address))
            .unwrap();
        let row = sheet
            .child("sheetData")
            .children
            .iter()
            .find(|r| r.id("r") == i + 3)
            .unwrap();
        let col = sheet
            .child("cols")
            .children
            .iter()
            .find(|c| (c.id("min")..=c.id("max")).contains(&(i + 1)))
            .unwrap();
        for style_id in [cell.id("s"), row.id("s"), col.id("style")] {
            let xf = &xfs[style_id];
            assert_components(
                &fonts[xf.id("fontId")],
                &fills[xf.id("fillId")],
                &borders[xf.id("borderId")],
                want,
            );
        }
        let xf = &xfs[cell.id("s")];
        font_ids.insert(xf.id("fontId"));
        fill_ids.insert(xf.id("fillId"));
        border_ids.insert(xf.id("borderId"));
        let rule = sheet
            .children
            .iter()
            .filter(|n| n.name == "conditionalFormatting")
            .flat_map(|n| &n.children)
            .find(|n| n.name == "cfRule" && n.id("priority") == i + 1)
            .unwrap();
        let dxf = &dxfs[rule.id("dxfId")];
        dxf_ids.insert(rule.id("dxfId"));
        assert_components(
            dxf.child("font"),
            dxf.child("fill"),
            dxf.child("border"),
            want,
        );
    }
    for ids in [font_ids, fill_ids, border_ids, dxf_ids] {
        assert_eq!(
            ids.len(),
            wants.len(),
            "distinct selector/tint identities aliased"
        );
    }
}
fn expanded_colors(bytes: &[u8]) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut output = zip::ZipWriter::new(Cursor::new(vec![]));
    let mut expanded = 0;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        let mut data = vec![];
        entry.read_to_end(&mut data).unwrap();
        if entry.name() == "xl/styles.xml" {
            let mut reader = Reader::from_reader(data.as_slice());
            let mut writer = Writer::new(vec![]);
            loop {
                match reader.read_event().unwrap() {
                    Event::Empty(e)
                        if matches!(e.name().as_ref(), b"color" | b"fgColor" | b"bgColor") =>
                    {
                        let end = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                        writer.write_event(Event::Start(e)).unwrap();
                        writer.write_event(Event::End(BytesEnd::new(end))).unwrap();
                        expanded += 1;
                    }
                    Event::Eof => break,
                    event => writer.write_event(event).unwrap(),
                }
            }
            data = writer.into_inner();
        }
        output
            .start_file(entry.name(), zip::write::SimpleFileOptions::default())
            .unwrap();
        output.write_all(&data).unwrap();
    }
    assert!(expanded > 0);
    output.finish().unwrap().into_inner()
}
fn check(colors: Vec<Color>, wants: Vec<Attributes>) {
    let original = save(&fixture(&colors));
    assert_projection(&original, &wants);
    for input in [original.clone(), expanded_colors(&original)] {
        let reopened = umya::reader::xlsx::read_reader(Cursor::new(input), true).unwrap();
        assert_projection(&save(&reopened), &wants);
    }
}
#[test]
fn referenced_selector_and_tint_identities_survive_roundtrip() {
    // Equal tint is essential: theme=1 and indexed=1 previously hashed identically.
    // Themes 1/2 also exposed the border hash's omitted theme identity.
    let mut colors = vec![];
    let mut wants = vec![];
    for index in [1, 2] {
        colors.push(Color::default().set_theme_index(index).to_owned());
        wants.push(expected("theme", &index.to_string(), None));
    }
    colors.push(Color::default().set_indexed(1).to_owned());
    wants.push(expected("indexed", "1", None));
    for (tint, text) in [(0.25, "0.25"), (-0.25, "-0.25")] {
        colors.push(
            Color::default()
                .set_theme_index(1)
                .set_tint(tint)
                .to_owned(),
        );
        wants.push(expected("theme", "1", Some(text)));
    }
    for (tint, text) in [(0.25, "0.25"), (-0.25, "-0.25")] {
        colors.push(Color::default().set_indexed(1).set_tint(tint).to_owned());
        wants.push(expected("indexed", "1", Some(text)));
    }
    colors.push(Color::default().set_argb_str("FF000000").to_owned());
    wants.push(expected("rgb", "FF000000", None));
    colors.push(Color::default().set_indexed(0).to_owned());
    wants.push(expected("indexed", "0", None));
    colors.push(
        Color::default()
            .set_argb(Color::hex_to_argb8("FFFF0000").unwrap())
            .to_owned(),
    );
    wants.push(expected("rgb", "FFFF0000", None));
    check(colors, wants);
}
#[test]
fn same_tint_theme_and_indexed_selectors_do_not_alias() {
    check(
        vec![
            Color::default().set_theme_index(1).to_owned(),
            Color::default().set_indexed(1).to_owned(),
        ],
        vec![expected("theme", "1", None), expected("indexed", "1", None)],
    );
}

#[test]
fn different_themes_do_not_alias_in_borders() {
    check(
        vec![
            Color::default().set_theme_index(1).to_owned(),
            Color::default().set_theme_index(2).to_owned(),
        ],
        vec![expected("theme", "1", None), expected("theme", "2", None)],
    );
}

#[test]
fn palette_rgb_setters_keep_rgb_identity() {
    check(
        vec![
            Color::default().set_argb_str("FF000000").to_owned(),
            Color::default()
                .set_argb(Color::hex_to_argb8("FFFF0000").unwrap())
                .to_owned(),
        ],
        vec![
            expected("rgb", "FF000000", None),
            expected("rgb", "FFFF0000", None),
        ],
    );
}

#[test]
fn literal_border_color_survives_import() {
    check(
        vec![Color::default().set_argb_str("FFC00000").to_owned()],
        vec![expected("rgb", "FFC00000", None)],
    );
}

#[test]
fn automatic_selectors_survive_every_style_consumer() {
    check(
        vec![
            Color::default().set_automatic(true).to_owned(),
            Color::default().set_automatic(false).to_owned(),
        ],
        vec![expected("auto", "1", None), expected("auto", "0", None)],
    );
}
