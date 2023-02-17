// a:p
use super::ParagraphProperties;
use super::Run;
use super::RunProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Paragraph {
    paragraph_properties: ParagraphProperties,
    run: Vec<Run>,
    end_para_run_properties: Option<RunProperties>,
}
impl Paragraph {
    pub fn get_paragraph_properties(&self) -> &ParagraphProperties {
        &self.paragraph_properties
    }

    pub fn get_paragraph_properties_mut(&mut self) -> &mut ParagraphProperties {
        &mut self.paragraph_properties
    }

    pub fn set_paragraph_properties(&mut self, value: ParagraphProperties) -> &mut Paragraph {
        self.paragraph_properties = value;
        self
    }

    pub fn get_run(&self) -> &Vec<Run> {
        &self.run
    }

    pub fn add_run(&mut self, value: Run) {
        self.run.push(value);
    }

    pub fn get_end_para_run_properties(&self) -> &Option<RunProperties> {
        &self.end_para_run_properties
    }

    pub fn get_end_para_run_properties_mut(&mut self) -> &mut Option<RunProperties> {
        &mut self.end_para_run_properties
    }

    pub fn set_end_para_run_properties(&mut self, value: RunProperties) -> &mut Paragraph {
        self.end_para_run_properties = Some(value);
        self
    }

    pub fn remove_end_para_run_properties(&mut self) {
        self.end_para_run_properties = None;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:pPr" => {
                        self.paragraph_properties.set_attributes(reader, e, false);
                    }
                    b"a:r" => {
                        let mut run = Run::default();
                        run.set_attributes(reader, e);
                        self.add_run(run);
                    }
                    b"a:endParaRPr" => {
                        let mut run_properties = RunProperties::default();
                        run_properties.set_attributes(reader, e, false);
                        self.set_end_para_run_properties(run_properties);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:pPr" => {
                        self.paragraph_properties.set_attributes(reader, e, true);
                    }
                    b"a:endParaRPr" => {
                        let mut run_properties = RunProperties::default();
                        run_properties.set_attributes(reader, e, true);
                        self.set_end_para_run_properties(run_properties);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:p" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:p"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:p
        write_start_tag(writer, "a:p", vec![], false);

        // a:pPr
        let _ = &self.paragraph_properties.write_to(writer);

        // a:r
        for run in &self.run {
            run.write_to(writer);
        }

        // a:endParaRPr
        match &self.end_para_run_properties {
            Some(v) => v.write_to_end_para_rpr(writer),
            None => {}
        }

        write_end_tag(writer, "a:p");
    }
}
