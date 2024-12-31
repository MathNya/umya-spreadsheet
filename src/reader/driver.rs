use std::{
    path::{
        Component,
        Path,
        PathBuf,
    },
    string::FromUtf8Error,
};

use quick_xml::events::attributes::Attribute;

#[macro_export]
macro_rules! xml_read_loop {
    ($reader:ident $(,$pat:pat => $result:expr)+ $(,)?) => {
        let mut buf = Vec::new();
        loop {
            let ev = match $reader.read_event_into(&mut buf) {
                Ok(v) => v,
                Err(e) => panic!("Error at position {}: {e:?}", $reader.buffer_position()),
            };

            match ev {
                $($pat => $result,)+
                _ => (),
            }

            buf.clear();
        }
    };
}

pub(crate) use crate::xml_read_loop;

#[macro_export]
macro_rules! set_string_from_xml {
    ($self:ident, $e:ident, $attr:ident, $xml_attr:expr) => {{
        if let Some(v) = get_attribute($e, $xml_attr.as_bytes()) {
            $self.$attr.set_value_string(v);
        }
    }};
}

pub(crate) use crate::set_string_from_xml;

pub(crate) fn normalize_path(path: &str) -> PathBuf {
    let path = Path::new(path);
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().copied() {
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

#[inline]
pub(crate) fn join_paths(base_path: &str, target: &str) -> String {
    match target.split_once('/') {
        Some(("", target)) => normalize_path_to_str(target),
        _ => normalize_path_to_str(&format!("{base_path}/{target}")),
    }
}

#[inline]
pub(crate) fn normalize_path_to_str(path: &str) -> String {
    let ret = normalize_path(path);
    ret.to_str().unwrap_or("").replace('\\', "/")
}

#[inline]
pub(crate) fn get_attribute(e: &quick_xml::events::BytesStart<'_>, key: &[u8]) -> Option<String> {
    e.attributes()
        .with_checks(false)
        .find_map(|attr| match attr {
            Ok(ref attr) if attr.key.into_inner() == key => {
                Some(get_attribute_value(attr).unwrap())
            }
            _ => None,
        })
}

#[inline]
pub(crate) fn get_attribute_value(attr: &Attribute) -> Result<String, FromUtf8Error> {
    String::from_utf8(attr.value.to_vec())
}
