use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::borrow::Cow;
use std::io;
use std::io::{Cursor, Write};

pub(crate) fn write_start_tag<'a, S>(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    tag_name: S,
    attributes: Vec<(&str, &str)>,
    empty_flag: bool,
) where
    S: Into<Cow<'a, str>>,
{
    let tag_name = tag_name.into();
    let len = tag_name.len();
    let mut elem = BytesStart::from_content(tag_name, len);
    for attribute in &attributes {
        elem.push_attribute((attribute.0, attribute.1));
    }
    if empty_flag {
        let _ = writer.write_event(Event::Empty(elem));
    } else {
        let _ = writer.write_event(Event::Start(elem));
    }
}

pub(crate) fn write_end_tag<'a, S>(writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: S)
where
    S: Into<Cow<'a, str>>,
{
    let _ = writer.write_event(Event::End(BytesEnd::new(tag_name.into())));
}

pub(crate) fn write_text_node<'a, S>(writer: &mut Writer<Cursor<Vec<u8>>>, data: S)
where
    S: Into<Cow<'a, str>>,
{
    let _ = writer.write_event(Event::Text(BytesText::new(&data.into())));
}

pub(crate) fn write_text_node_no_escape<'a, S>(writer: &mut Writer<Cursor<Vec<u8>>>, data: S)
where
    S: Into<Cow<'a, str>>,
{
    let _ = writer.inner().write(data.into().as_bytes());
}

pub(crate) fn write_new_line(writer: &mut Writer<Cursor<Vec<u8>>>) {
    write_text_node(writer, "\r\n");
}

pub(crate) fn make_file_from_writer<W: io::Seek + io::Write>(
    path: &str,
    arv: &mut zip::ZipWriter<W>,
    writer: Writer<Cursor<Vec<u8>>>,
    dir: Option<&str>,
    is_light: &bool,
) -> Result<(), io::Error> {
    make_file_from_bin(path, arv, &writer.into_inner().into_inner(), dir, is_light)
}

pub(crate) fn make_file_from_bin<W: io::Seek + io::Write>(
    path: &str,
    arv: &mut zip::ZipWriter<W>,
    writer: &Vec<u8>,
    dir: Option<&str>,
    is_light: &bool,
) -> Result<(), io::Error> {
    let zip_opt =
        match is_light {
            &false => zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::DEFLATE),
            &true => zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored),
        };
    arv.start_file(&to_path(path, dir), zip_opt)?;
    arv.write_all(writer)
}

pub(crate) fn to_path(path: &str, dir: Option<&str>) -> String {
    match dir {
        Some(dir) => format!("{}/{}", dir, path),
        None => path.to_owned(),
    }
}
