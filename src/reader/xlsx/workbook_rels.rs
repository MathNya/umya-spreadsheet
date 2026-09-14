use std::io;

use quick_xml::{
    NsReader,
    events::Event,
    name::{
        Namespace,
        ResolveResult,
    },
};

use super::{
    XlsxError,
    driver::{
        get_attribute,
        xml_read_loop,
    },
};
use crate::{
    helper::const_str::{
        PIVOT_CACHE_DEF_NS,
        PKG_WORKBOOK_RELS,
        REL_NS,
    },
    structs::Workbook,
};

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    wb: &mut Workbook,
) -> Result<Vec<(String, String, String)>, XlsxError> {
    let r = io::BufReader::new(super::driver::zip_by_name(arv, PKG_WORKBOOK_RELS)?);
    let mut reader = NsReader::from_reader(r);
    reader.config_mut().trim_text(true);

    let mut result: Vec<(String, String, String)> = Vec::new();

    xml_read_loop!(
        reader,
        Event::Empty(ref e) => {
            let (namespace, local_name) = reader.resolver().resolve_element(e.name());
            if namespace == ResolveResult::Bound(Namespace(REL_NS.as_bytes()))
                && local_name.as_ref() == b"Relationship"
            {
                let id_value = get_attribute(e, b"Id").unwrap();
                let type_value = get_attribute(e, b"Type").unwrap();
                let target_value = get_attribute(e, b"Target").unwrap();
                let target_value = target_value
                    .strip_prefix("/xl/")
                    .map(ToOwned::to_owned)
                    .unwrap_or(target_value);
                if type_value == PIVOT_CACHE_DEF_NS {
                    wb.update_pivot_caches(id_value.as_str(), target_value.as_str());
                }
                result.push((id_value, type_value, target_value));
            }
        },
        Event::Eof => break,
    );

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::io::{
        Cursor,
        Write,
    };

    use rstest::rstest;
    use zip::{
        ZipArchive,
        ZipWriter,
        write::SimpleFileOptions,
    };

    use super::*;

    fn relationships(xml: &str) -> Vec<(String, String, String)> {
        let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
        zip.start_file(PKG_WORKBOOK_RELS, SimpleFileOptions::default())
            .unwrap();
        zip.write_all(xml.as_bytes()).unwrap();
        let mut archive = ZipArchive::new(zip.finish().unwrap()).unwrap();
        read(&mut archive, &mut Workbook::default()).unwrap()
    }

    #[rstest]
    #[case("")]
    #[case("ns0:")]
    #[case("pkg:")]
    fn relationship_namespace_accepts_equivalent_spellings(#[case] prefix: &str) {
        let declaration = if prefix.is_empty() {
            "xmlns".to_string()
        } else {
            format!("xmlns:{}", prefix.trim_end_matches(':'))
        };
        let xml = format!(
            "<{prefix}Relationships {declaration}=\"{REL_NS}\"><{prefix}Relationship Id=\"rId1\" \
             Type=\"worksheet\" Target=\"/xl/worksheets/sheet1.xml\"/></{prefix}Relationships>"
        );
        assert_eq!(
            relationships(&xml),
            vec![(
                "rId1".to_string(),
                "worksheet".to_string(),
                "worksheets/sheet1.xml".to_string(),
            )]
        );
    }

    #[rstest]
    #[case(r#"<Relationship xmlns="urn:unrelated"/>"#)]
    #[case(r#"<other:Relationship xmlns:other="urn:unrelated"/>"#)]
    #[case(r#"<Relationship xmlns=""/>"#)]
    #[case("<Other/>")]
    fn unrelated_relationship_names_are_not_interpreted(#[case] child: &str) {
        let xml = format!(r#"<Relationships xmlns="{REL_NS}">{child}</Relationships>"#);
        assert!(relationships(&xml).is_empty());
    }

    #[test]
    fn child_namespace_declarations_are_resolved_in_scope() {
        let xml = format!(
            "<Relationships xmlns=\"{REL_NS}\" \
             xmlns:p=\"urn:unrelated\"><p:Relationship/><p:Relationship xmlns:p=\"{REL_NS}\" \
             Id=\"rId1\" Type=\"worksheet\" \
             Target=\"worksheets/sheet1.xml\"/><p:Relationship/></Relationships>"
        );
        assert_eq!(relationships(&xml).len(), 1);
    }
}
