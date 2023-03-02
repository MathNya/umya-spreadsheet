use quick_xml::events::attributes::Attribute;
use std::path::{Component, Path, PathBuf};
use std::string::FromUtf8Error;

pub(crate) fn normalize_path(path: &str) -> PathBuf {
    let path = Path::new(path);
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

pub(crate) fn normalize_path_to_str(path: &str) -> String {
    let ret = normalize_path(path);
    ret.to_str().unwrap_or("").to_string().replace('\\', "/")
}

pub(crate) fn get_attribute(e: &quick_xml::events::BytesStart<'_>, key: &[u8]) -> Option<String> {
    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key.into_inner() == key => {
                return Some(get_attribute_value(attr).unwrap());
            }
            Ok(_) => {}
            Err(_) => {}
        }
    }
    None
}
pub(crate) fn get_attribute_value(attr: &Attribute) -> Result<String, FromUtf8Error> {
    let value = attr.value.clone().into_owned();
    String::from_utf8(value)
}

pub(crate) fn condvert_character_reference(src: &str) -> String {
    src.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}
