use quick_xml::events::attributes::Attribute;
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
use std::string::FromUtf8Error;

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
            (zip.start_file(name, options))?;
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

    zip.finish()?;
    Ok(())
}

pub(crate) fn unzip(zip_file: &File, dir: &TempDir) -> Result<(), zip::result::ZipError> {
    let mut zip = zip::ZipArchive::new(zip_file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let path = dir.path().join(file.name());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?
        }
        if (&*file.name()).ends_with("/") {
            fs::create_dir_all(path)?
        } else {
            let mut archived_file = File::create(path)?;
            let _ = io::copy(&mut file, &mut archived_file);
        }
    }
    Ok(())
}

pub(crate) fn make_static_file(temp_dir: &TempDir, path: &str, data: &str, dir: Option<&str>,) -> Result<(), io::Error> 
{
    match dir {
        Some(dir) => {
            let dir_path = temp_dir.path().join(dir);
            fs::create_dir_all(dir_path)?;
        }
        None => {}
    }
    let file_path = temp_dir.path().join(path);
    let mut f = File::create(file_path)?;
    f.write_all(data.as_bytes())?;
    f.sync_all()?;
    Ok(())
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

pub(crate) fn get_attribute(
    e:&quick_xml::events::BytesStart<'_>,
    key:&[u8]
) -> Option<String>
{
    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key == key => {
                return Some(get_attribute_value(attr).unwrap());
            },
            Ok(_) => {},
            Err(_) => {},
        }
    }
    None
}
pub(crate) fn get_attribute_value(attr: &Attribute) -> Result<String, FromUtf8Error>
{
    let value = (&attr.value).clone().into_owned();
    String::from_utf8(value)
}

pub(crate) fn condvert_character_reference(src: &str) -> String
{
    src.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

