use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;

use structs::drawing::Theme;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    theme: &Theme,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        b"1.0",
        Some(b"UTF-8"),
        Some(b"yes"),
    )));
    write_new_line(&mut writer);

    // a:theme
    write_start_tag(
        &mut writer,
        "a:theme",
        vec![
            (
                "xmlns:a",
                "http://schemas.openxmlformats.org/drawingml/2006/main",
            ),
            ("name", theme.get_theme_name()),
        ],
        false,
    );

    // a:themeElements
    write_start_tag(&mut writer, "a:themeElements", vec![], false);

    // a:clrScheme
    write_start_tag(
        &mut writer,
        "a:clrScheme",
        vec![("name", theme.get_color_scheme_name())],
        false,
    );

    // a:dk1
    write_start_tag(&mut writer, "a:dk1", vec![], false);

    // a:sysClr
    write_start_tag(
        &mut writer,
        "a:sysClr",
        vec![
            ("val", "windowText"),
            ("lastClr", theme.get_color_map()[1].as_str()),
        ],
        true,
    );

    write_end_tag(&mut writer, "a:dk1");

    // a:lt1
    write_start_tag(&mut writer, "a:lt1", vec![], false);

    // a:sysClr
    write_start_tag(
        &mut writer,
        "a:sysClr",
        vec![
            ("val", "window"),
            ("lastClr", theme.get_color_map()[0].as_str()),
        ],
        true,
    );

    write_end_tag(&mut writer, "a:lt1");

    // a:dk2
    write_start_tag(&mut writer, "a:dk2", vec![], false);

    // a:srgbClr
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[3].as_str())],
        true,
    );

    write_end_tag(&mut writer, "a:dk2");

    // a:lt2
    write_start_tag(&mut writer, "a:lt2", vec![], false);

    // "a:srgbClr"
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[2].as_str())],
        true,
    );

    write_end_tag(&mut writer, "a:lt2");

    // a:accent1
    write_start_tag(&mut writer, "a:accent1", vec![], false);

    // a:srgbClr
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[4].as_str())],
        true,
    );

    write_end_tag(&mut writer, "a:accent1");

    // a:accent2
    write_start_tag(&mut writer, "a:accent2", vec![], false);

    // a:srgbClr
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[5].as_str())],
        true,
    );

    write_end_tag(&mut writer, "a:accent2");

    // a:accent3
    write_start_tag(&mut writer, "a:accent3", vec![], false);

    // a:srgbClr
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[6].as_str())],
        true,
    );

    write_end_tag(&mut writer, "a:accent3");

    // a:accent4
    write_start_tag(&mut writer, "a:accent4", vec![], false);

    // a:srgbClr
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[7].as_str())],
        true,
    );

    write_end_tag(&mut writer, "a:accent4");

    // a:accent5
    write_start_tag(&mut writer, "a:accent5", vec![], false);

    // a:srgbClr
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[8].as_str())],
        true,
    );

    write_end_tag(&mut writer, "a:accent5");

    // a:accent6
    write_start_tag(&mut writer, "a:accent6", vec![], false);

    // a:srgbClr
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[9].as_str())],
        true,
    );
    write_end_tag(&mut writer, "a:accent6");

    // a:hlink
    write_start_tag(&mut writer, "a:hlink", vec![], false);

    // a:srgbClr
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[10].as_str())],
        true,
    );

    write_end_tag(&mut writer, "a:hlink");

    // a:folHlink
    write_start_tag(&mut writer, "a:folHlink", vec![], false);

    // a:srgbClr
    write_start_tag(
        &mut writer,
        "a:srgbClr",
        vec![("val", theme.get_color_map()[11].as_str())],
        true,
    );

    write_end_tag(&mut writer, "a:folHlink");

    write_end_tag(&mut writer, "a:clrScheme");

    // a:fontScheme
    write_start_tag(&mut writer, "a:fontScheme", vec![("name", "Office")], false);

    // a:majorFont
    theme.get_major_font().write_to_major_font(&mut writer);

    // a:minorFont
    theme.get_minor_font().write_to_minor_font(&mut writer);

    write_end_tag(&mut writer, "a:fontScheme");

    // a:fmtScheme
    write_start_tag(&mut writer, "a:fmtScheme", vec![("name", "Office")], false);

    // a:fillStyleLst
    write_start_tag(&mut writer, "a:fillStyleLst", vec![], false);

    // a:solidFill
    write_start_tag(&mut writer, "a:solidFill", vec![], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], true);

    write_end_tag(&mut writer, "a:solidFill");

    // a:gradFill
    write_start_tag(
        &mut writer,
        "a:gradFill",
        vec![("rotWithShape", "1")],
        false,
    );

    // a:gsLst
    write_start_tag(&mut writer, "a:gsLst", vec![], false);

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "0")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:tint
    write_start_tag(&mut writer, "a:tint", vec![("val", "50000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "300000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "35000")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:tint
    write_start_tag(&mut writer, "a:tint", vec![("val", "37000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "300000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "100000")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:tint
    write_start_tag(&mut writer, "a:tint", vec![("val", "15000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "350000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    write_end_tag(&mut writer, "a:gsLst");

    // a:lin
    write_start_tag(
        &mut writer,
        "a:lin",
        vec![("ang", "16200000"), ("scaled", "1")],
        true,
    );

    write_end_tag(&mut writer, "a:gradFill");

    // a:gradFill
    write_start_tag(
        &mut writer,
        "a:gradFill",
        vec![("rotWithShape", "1")],
        false,
    );

    // a:gsLst
    write_start_tag(&mut writer, "a:gsLst", vec![], false);

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "0")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:shade
    write_start_tag(&mut writer, "a:shade", vec![("val", "51000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "130000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "80000")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:shade
    write_start_tag(&mut writer, "a:shade", vec![("val", "93000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "130000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "100000")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:shade
    write_start_tag(&mut writer, "a:shade", vec![("val", "94000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "135000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    write_end_tag(&mut writer, "a:gsLst");

    // a:lin
    write_start_tag(
        &mut writer,
        "a:lin",
        vec![("ang", "16200000"), ("scaled", "0")],
        true,
    );

    write_end_tag(&mut writer, "a:gradFill");

    write_end_tag(&mut writer, "a:fillStyleLst");

    // a:lnStyleLst
    write_start_tag(&mut writer, "a:lnStyleLst", vec![], false);

    // a:ln
    write_start_tag(
        &mut writer,
        "a:ln",
        vec![
            ("w", "9525"),
            ("cap", "flat"),
            ("cmpd", "sng"),
            ("algn", "ctr"),
        ],
        false,
    );

    // a:solidFill
    write_start_tag(&mut writer, "a:solidFill", vec![], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:shade
    write_start_tag(&mut writer, "a:shade", vec![("val", "95000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "105000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:solidFill");

    // a:prstDash
    write_start_tag(&mut writer, "a:prstDash", vec![("val", "solid")], true);

    write_end_tag(&mut writer, "a:ln");

    // a:ln
    write_start_tag(
        &mut writer,
        "a:ln",
        vec![
            ("w", "25400"),
            ("cap", "flat"),
            ("cmpd", "sng"),
            ("algn", "ctr"),
        ],
        false,
    );

    // a:solidFill
    write_start_tag(&mut writer, "a:solidFill", vec![], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], true);

    write_end_tag(&mut writer, "a:solidFill");

    // a:prstDash
    write_start_tag(&mut writer, "a:prstDash", vec![("val", "solid")], true);

    write_end_tag(&mut writer, "a:ln");

    // a:ln
    write_start_tag(
        &mut writer,
        "a:ln",
        vec![
            ("w", "38100"),
            ("cap", "flat"),
            ("cmpd", "sng"),
            ("algn", "ctr"),
        ],
        false,
    );

    // a:solidFill
    write_start_tag(&mut writer, "a:solidFill", vec![], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], true);

    write_end_tag(&mut writer, "a:solidFill");

    // a:prstDash
    write_start_tag(&mut writer, "a:prstDash", vec![("val", "solid")], true);

    write_end_tag(&mut writer, "a:ln");

    write_end_tag(&mut writer, "a:lnStyleLst");

    // a:a:effectStyleLst
    write_start_tag(&mut writer, "a:effectStyleLst", vec![], false);

    // a:effectStyle
    write_start_tag(&mut writer, "a:effectStyle", vec![], false);

    // a:effectLst
    write_start_tag(&mut writer, "a:effectLst", vec![], false);

    // a:outerShdw
    write_start_tag(
        &mut writer,
        "a:outerShdw",
        vec![
            ("blurRad", "40000"),
            ("dist", "20000"),
            ("dir", "5400000"),
            ("rotWithShape", "0"),
        ],
        false,
    );

    // a:srgbClr
    write_start_tag(&mut writer, "a:srgbClr", vec![("val", "000000")], false);

    // a:alpha
    write_start_tag(&mut writer, "a:alpha", vec![("val", "38000")], true);

    write_end_tag(&mut writer, "a:srgbClr");

    write_end_tag(&mut writer, "a:outerShdw");

    write_end_tag(&mut writer, "a:effectLst");

    write_end_tag(&mut writer, "a:effectStyle");

    // a:effectStyle
    write_start_tag(&mut writer, "a:effectStyle", vec![], false);

    // a:effectLst
    write_start_tag(&mut writer, "a:effectLst", vec![], false);

    // a:outerShdw
    write_start_tag(
        &mut writer,
        "a:outerShdw",
        vec![
            ("blurRad", "40000"),
            ("dist", "23000"),
            ("dir", "5400000"),
            ("rotWithShape", "0"),
        ],
        false,
    );

    // a:srgbClr
    write_start_tag(&mut writer, "a:srgbClr", vec![("val", "000000")], false);

    // a:alpha
    write_start_tag(&mut writer, "a:alpha", vec![("val", "35000")], true);

    write_end_tag(&mut writer, "a:srgbClr");

    write_end_tag(&mut writer, "a:outerShdw");

    write_end_tag(&mut writer, "a:effectLst");

    write_end_tag(&mut writer, "a:effectStyle");

    // a:effectStyle
    write_start_tag(&mut writer, "a:effectStyle", vec![], false);

    // a:effectLst
    write_start_tag(&mut writer, "a:effectLst", vec![], false);

    // a:outerShdw
    write_start_tag(
        &mut writer,
        "a:outerShdw",
        vec![
            ("blurRad", "40000"),
            ("dist", "23000"),
            ("dir", "5400000"),
            ("rotWithShape", "0"),
        ],
        false,
    );

    // a:srgbClr
    write_start_tag(&mut writer, "a:srgbClr", vec![("val", "000000")], false);

    // a:alpha
    write_start_tag(&mut writer, "a:alpha", vec![("val", "35000")], true);

    write_end_tag(&mut writer, "a:srgbClr");

    write_end_tag(&mut writer, "a:outerShdw");

    write_end_tag(&mut writer, "a:effectLst");

    // a:scene3d
    write_start_tag(&mut writer, "a:scene3d", vec![], false);

    // a:camera
    write_start_tag(
        &mut writer,
        "a:camera",
        vec![("prst", "orthographicFront")],
        false,
    );

    // a:rot
    write_start_tag(
        &mut writer,
        "a:rot",
        vec![("lat", "0"), ("lon", "0"), ("rev", "0")],
        true,
    );

    write_end_tag(&mut writer, "a:camera");

    // a:lightRig
    write_start_tag(
        &mut writer,
        "a:lightRig",
        vec![("rig", "threePt"), ("dir", "t")],
        false,
    );

    // a:rot
    write_start_tag(
        &mut writer,
        "a:rot",
        vec![("lat", "0"), ("lon", "0"), ("rev", "1200000")],
        true,
    );

    write_end_tag(&mut writer, "a:lightRig");

    write_end_tag(&mut writer, "a:scene3d");

    // a:sp3d
    write_start_tag(&mut writer, "a:sp3d", vec![], false);

    // a:bevelT
    write_start_tag(
        &mut writer,
        "a:bevelT",
        vec![("w", "63500"), ("h", "25400")],
        true,
    );

    write_end_tag(&mut writer, "a:sp3d");

    write_end_tag(&mut writer, "a:effectStyle");

    write_end_tag(&mut writer, "a:effectStyleLst");

    // a:bgFillStyleLst
    write_start_tag(&mut writer, "a:bgFillStyleLst", vec![], false);

    // a:solidFill
    write_start_tag(&mut writer, "a:solidFill", vec![], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], true);

    write_end_tag(&mut writer, "a:solidFill");

    // a:gradFill
    write_start_tag(
        &mut writer,
        "a:gradFill",
        vec![("rotWithShape", "1")],
        false,
    );

    // a:gsLst
    write_start_tag(&mut writer, "a:gsLst", vec![], false);

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "0")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:tint
    write_start_tag(&mut writer, "a:tint", vec![("val", "40000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "350000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "40000")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:tint
    write_start_tag(&mut writer, "a:tint", vec![("val", "45000")], true);

    // a:shade
    write_start_tag(&mut writer, "a:shade", vec![("val", "99000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "350000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "100000")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:shade
    write_start_tag(&mut writer, "a:shade", vec![("val", "20000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "255000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    write_end_tag(&mut writer, "a:gsLst");

    // a:path
    write_start_tag(&mut writer, "a:path", vec![("path", "circle")], false);

    // a:fillToRect
    write_start_tag(
        &mut writer,
        "a:fillToRect",
        vec![
            ("l", "50000"),
            ("t", "-80000"),
            ("r", "50000"),
            ("b", "180000"),
        ],
        true,
    );

    write_end_tag(&mut writer, "a:path");

    write_end_tag(&mut writer, "a:gradFill");

    // a:gradFill
    write_start_tag(
        &mut writer,
        "a:gradFill",
        vec![("rotWithShape", "1")],
        false,
    );

    // a:gsLst
    write_start_tag(&mut writer, "a:gsLst", vec![], false);

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "0")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:tint
    write_start_tag(&mut writer, "a:tint", vec![("val", "80000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "300000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    // a:gs
    write_start_tag(&mut writer, "a:gs", vec![("pos", "100000")], false);

    // a:schemeClr
    write_start_tag(&mut writer, "a:schemeClr", vec![("val", "phClr")], false);

    // a:shade
    write_start_tag(&mut writer, "a:shade", vec![("val", "30000")], true);

    // a:satMod
    write_start_tag(&mut writer, "a:satMod", vec![("val", "200000")], true);

    write_end_tag(&mut writer, "a:schemeClr");

    write_end_tag(&mut writer, "a:gs");

    write_end_tag(&mut writer, "a:gsLst");

    // a:path
    write_start_tag(&mut writer, "a:path", vec![("path", "circle")], false);

    // a:fillToRect
    write_start_tag(
        &mut writer,
        "a:fillToRect",
        vec![
            ("l", "50000"),
            ("t", "50000"),
            ("r", "50000"),
            ("b", "50000"),
        ],
        true,
    );

    write_end_tag(&mut writer, "a:path");

    write_end_tag(&mut writer, "a:gradFill");

    write_end_tag(&mut writer, "a:bgFillStyleLst");

    write_end_tag(&mut writer, "a:fmtScheme");

    write_end_tag(&mut writer, "a:themeElements");

    // a:objectDefaults
    write_start_tag(&mut writer, "a:objectDefaults", vec![], true);

    // a:extraClrSchemeLst
    write_start_tag(&mut writer, "a:extraClrSchemeLst", vec![], true);

    write_end_tag(&mut writer, "a:theme");

    let target = "xl/theme/theme1.xml";
    writer_mng.add_writer(target, writer)
}
