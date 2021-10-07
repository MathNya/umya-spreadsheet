use quick_xml::events::attributes::Attribute;
use tempdir::TempDir;
use std::fs;
use std::fs::File;
use std::io;
use std::string::FromUtf8Error;

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
