use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use tempdir::TempDir;
use walkdir::WalkDir;
use zip::write::FileOptions;
use std::borrow::Cow;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Cursor, Read, Write};
use std::path::Path;

pub(crate) fn write_to_file(path: &Path, dir: &TempDir) -> Result<(), io::Error> {
    let file = File::create(&path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);
    let walkdir = WalkDir::new(dir.path());
    let it = walkdir.into_iter();

    for dent in it.filter_map(|e| e.ok()) {
        let path = dent.path();
        let name = path
            .strip_prefix(Path::new(dir.path()))
            .unwrap()
            .to_str()
            .unwrap();

        if path.is_file() {
            //println!("adding {:?} as {:?} ...", path, name);
            let _= zip.start_file(name, options);
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
        }
        /*
         else {
            let mut dir_name = String::from(name);
            dir_name.push('/');
            let _ = zip.add_directory(dir_name.as_str(), FileOptions::default());
        }
        */
    }

    let _= zip.finish();
    Ok(())
}

pub(crate) fn write_start_tag<'a, S>(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    tag_name: S,
    attributes: Vec<(&str, &str)>,
    empty_flag: bool,
) where
    S: Into<Cow<'a, str>>,
{
    let tag_name = tag_name.into();
    let mut elem = BytesStart::owned(tag_name.as_bytes().to_vec(), tag_name.len());
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
    let _ = writer.write_event(Event::End(BytesEnd::borrowed(tag_name.into().as_bytes())));
}

pub(crate) fn write_text_node<'a, S>(writer: &mut Writer<Cursor<Vec<u8>>>, data: S)
where
    S: Into<Cow<'a, str>>,
{
    let _ = writer.write_event(Event::Text(BytesText::from_plain_str(&data.into())));
}

pub(crate) fn write_new_line(writer: &mut Writer<Cursor<Vec<u8>>>)
{
    let _ = write_text_node(writer, "\r\n");
}

pub(crate) fn make_file_from_writer(
    path: &str,
    temp_dir: &TempDir,
    writer: Writer<Cursor<Vec<u8>>>,
    dir: Option<&str>,
) -> Result<(), io::Error> {
    match dir {
        Some(dir) => {
            let dir_path = temp_dir.path().join(dir);
            fs::create_dir_all(dir_path)?;
        }
        None => {}
    }
    let file_path = temp_dir.path().join(path);
    let mut f = File::create(file_path)?;
    f.write_all(writer.into_inner().get_ref())?;
    f.sync_all()?;
    Ok(())
}

pub(crate) fn make_file_from_bin(
    path: &str,
    temp_dir: &TempDir,
    writer: &Vec<u8>,
    dir: Option<&str>,
) -> Result<(), io::Error> {
    match dir {
        Some(dir) => {
            let dir_path = temp_dir.path().join(dir);
            fs::create_dir_all(dir_path)?;
        }
        None => {}
    }
    let file_path = temp_dir.path().join(path);
    let mut f = File::create(file_path)?;
    f.write_all(writer)?;
    f.sync_all()?;
    Ok(())
}
