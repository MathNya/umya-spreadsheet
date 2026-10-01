use std::io::{
    self,
    BufRead,
    Read,
};

use quick_xml::{
    NsReader,
    Writer,
    events::{
        BytesEnd,
        Event,
    },
    name::{
        Namespace,
        ResolveResult,
    },
};

use crate::helper::const_str::SHEET_MAIN_NS;

/// Supplies the legacy worksheet sub-readers with their unqualified element
/// names after resolving `SpreadsheetML` namespaces. Buffering one event keeps
/// lazy worksheet reads streaming, including for large sheetData sections.
/// Foreign default namespaces receive an internal prefix so the legacy parser
/// cannot confuse their local names with worksheet elements. This view is only
/// used for parsing; the stored source XML is unchanged.
pub(super) struct WorksheetNamespaceReader<R> {
    reader:   NsReader<R>,
    input:    Vec<u8>,
    output:   Vec<u8>,
    position: usize,
    finished: bool,
}

impl<R: BufRead> WorksheetNamespaceReader<R> {
    pub(super) fn new(source: R) -> Self {
        Self {
            reader:   NsReader::from_reader(source),
            input:    Vec::new(),
            output:   Vec::new(),
            position: 0,
            finished: false,
        }
    }

    fn next_event(&mut self) -> io::Result<()> {
        self.input.clear();
        self.output.clear();
        self.position = 0;
        let (namespace, event) = self
            .reader
            .read_resolved_event_into(&mut self.input)
            .map_err(io::Error::other)?;
        let is_spreadsheet = namespace == ResolveResult::Bound(Namespace(SHEET_MAIN_NS.as_bytes()));
        let event = match event {
            Event::Start(mut element) if is_spreadsheet && element.name().prefix().is_some() => {
                let name = element.local_name().as_ref().to_vec();
                element.set_name(&name);
                Event::Start(element)
            }
            Event::Empty(mut element) if is_spreadsheet && element.name().prefix().is_some() => {
                let name = element.local_name().as_ref().to_vec();
                element.set_name(&name);
                Event::Empty(element)
            }
            Event::End(element) if is_spreadsheet && element.name().prefix().is_some() => {
                let name = String::from_utf8(element.local_name().as_ref().to_vec())
                    .map_err(io::Error::other)?;
                Event::End(BytesEnd::new(name))
            }
            Event::Start(mut element)
                if !is_spreadsheet
                    && matches!(namespace, ResolveResult::Bound(_))
                    && element.name().prefix().is_none() =>
            {
                let mut name = b"__umya_foreign:".to_vec();
                name.extend_from_slice(element.local_name().as_ref());
                element.set_name(&name);
                if let ResolveResult::Bound(uri) = namespace {
                    element.push_attribute((b"xmlns:__umya_foreign".as_slice(), uri.as_ref()));
                }
                Event::Start(element)
            }
            Event::Empty(mut element)
                if !is_spreadsheet
                    && matches!(namespace, ResolveResult::Bound(_))
                    && element.name().prefix().is_none() =>
            {
                let mut name = b"__umya_foreign:".to_vec();
                name.extend_from_slice(element.local_name().as_ref());
                element.set_name(&name);
                if let ResolveResult::Bound(uri) = namespace {
                    element.push_attribute((b"xmlns:__umya_foreign".as_slice(), uri.as_ref()));
                }
                Event::Empty(element)
            }
            Event::End(element)
                if !is_spreadsheet
                    && matches!(namespace, ResolveResult::Bound(_))
                    && element.name().prefix().is_none() =>
            {
                let local = std::str::from_utf8(element.local_name().as_ref())
                    .map_err(io::Error::other)?
                    .to_owned();
                Event::End(BytesEnd::new(format!("__umya_foreign:{local}")))
            }
            Event::Eof => {
                self.finished = true;
                return Ok(());
            }
            other => other,
        };
        Writer::new(&mut self.output).write_event(event)?;
        Ok(())
    }
}

impl<R: BufRead> BufRead for WorksheetNamespaceReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        while self.position == self.output.len() && !self.finished {
            self.next_event()?;
        }
        Ok(&self.output[self.position..])
    }

    fn consume(&mut self, amount: usize) {
        self.position = (self.position + amount).min(self.output.len());
    }
}

impl<R: BufRead> Read for WorksheetNamespaceReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let available = self.fill_buf()?;
        let count = available.len().min(buffer.len());
        buffer[..count].copy_from_slice(&available[..count]);
        self.consume(count);
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_prefix_rebinding_preserves_foreign_names_and_text() {
        let xml = format!(
            r#"<s:worksheet xmlns:s="{SHEET_MAIN_NS}"><s:sheetData><s:row r="1"><s:c r="A1"><s:v>7</s:v></s:c></s:row><s:row xmlns:s="urn:foreign" r="2"><s:c/></s:row><alias:row xmlns:alias="{SHEET_MAIN_NS}" r="3"/></s:sheetData><s:headerFooter><s:oddHeader>&amp;CResearch &amp; Development<![CDATA[<literal>]]></s:oddHeader></s:headerFooter></s:worksheet>"#
        );
        let mut reader = WorksheetNamespaceReader::new(xml.as_bytes());
        let mut normalized = String::new();
        reader.read_to_string(&mut normalized).unwrap();
        assert!(normalized.contains("<row r=\"1\"><c r=\"A1\"><v>7</v></c></row>"));
        assert!(normalized.contains("<s:row xmlns:s=\"urn:foreign\" r=\"2\"><s:c/></s:row>"));
        assert!(normalized.contains(&format!("<row xmlns:alias=\"{SHEET_MAIN_NS}\" r=\"3\"/>")));
        assert!(normalized.contains("&amp;CResearch &amp; Development<![CDATA[<literal>]]>"));
        assert!(normalized.ends_with("</oddHeader></headerFooter></worksheet>"));
    }

    #[test]
    fn foreign_default_namespace_does_not_become_a_spreadsheet_row() {
        let xml = format!(
            r#"<s:worksheet xmlns:s="{SHEET_MAIN_NS}"><s:sheetData><s:row r="1"/><row xmlns="urn:foreign" r="2"/><s:row r="3"/></s:sheetData></s:worksheet>"#
        );
        let mut reader =
            quick_xml::Reader::from_reader(WorksheetNamespaceReader::new(xml.as_bytes()));
        let mut buffer = Vec::new();
        let mut rows = Vec::new();
        loop {
            match reader.read_event_into(&mut buffer).unwrap() {
                Event::Empty(element) if element.name().as_ref() == b"row" => {
                    rows.push(
                        element
                            .try_get_attribute("r")
                            .unwrap()
                            .unwrap()
                            .value
                            .into_owned(),
                    );
                }
                Event::Eof => break,
                _ => {}
            }
            buffer.clear();
        }
        assert_eq!(rows, vec![b"1".to_vec(), b"3".to_vec()]);
    }

    #[test]
    fn first_event_does_not_buffer_the_complete_worksheet() {
        let xml = format!(
            "<s:worksheet xmlns:s=\"{SHEET_MAIN_NS}\"><s:sheetData>{}</s:sheetData></s:worksheet>",
            "<s:row r=\"1\"><s:c r=\"A1\"><s:v>1</s:v></s:c></s:row>".repeat(20_000),
        );
        let mut reader = WorksheetNamespaceReader::new(io::Cursor::new(xml.as_bytes()));
        assert!(reader.fill_buf().unwrap().starts_with(b"<worksheet "));
        assert!(reader.reader.get_ref().position() < 256);
        assert!(reader.input.len() < 256);
        assert!(reader.output.len() < 256);
    }
}
