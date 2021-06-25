// a:p
use super::ParagraphProperties;
use super::Run;
use super::EndParagraphRunProperties;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Paragraph {
    paragraph_properties: ParagraphProperties,
    run: Vec<Run>,
    end_para_run_properties: Option<EndParagraphRunProperties>,
}
impl Paragraph {
    pub fn get_paragraph_properties(&self) -> &ParagraphProperties {
        &self.paragraph_properties
    }

    pub fn get_paragraph_properties_mut(&mut self) -> &mut ParagraphProperties {
        &mut self.paragraph_properties
    }

    pub fn set_paragraph_properties(&mut self, value:ParagraphProperties) -> &mut Paragraph {
        self.paragraph_properties = value;
        self
    }

    pub fn get_run(&self) -> &Vec<Run> {
        &self.run
    }

    pub fn add_run(&mut self, value:Run) {
        self.run.push(value);
    }

    pub fn get_end_para_run_properties(&self) -> &Option<EndParagraphRunProperties> {
        &self.end_para_run_properties
    }

    pub fn get_end_para_run_properties_mut(&mut self) -> &mut Option<EndParagraphRunProperties> {
        &mut self.end_para_run_properties
    }

    pub fn set_end_para_run_properties(&mut self, value:EndParagraphRunProperties) -> &mut Paragraph {
        self.end_para_run_properties = Some(value);
        self
    }

    pub fn remove_end_para_run_properties(&mut self) {
        self.end_para_run_properties = None;
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:pPr" => {
                            &mut self.paragraph_properties.set_attributes(reader, e, false);
                        },
                        b"a:r" => {
                            let mut run = Run::default();
                            run.set_attributes(reader, e);
                            &mut self.add_run(run);
                        },
                        b"a:endParaRPr" => {
                            let mut run_properties = EndParagraphRunProperties::default();
                            run_properties.set_attributes(reader, e, false);
                            &mut self.set_end_para_run_properties(run_properties);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:pPr" => {
                            &mut self.paragraph_properties.set_attributes(reader, e, true);
                        },
                        b"a:endParaRPr" => {
                            let mut run_properties = EndParagraphRunProperties::default();
                            run_properties.set_attributes(reader, e, true);
                            &mut self.set_end_para_run_properties(run_properties);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"a:p" => return,
                        _ => (),
                    }
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
        &self.paragraph_properties.write_to(writer);

        // a:r
        for run in &self.run {
            run.write_to(writer);
        }

        // a:endParaRPr
        match &self.end_para_run_properties {
            Some(v) => v.write_to(writer),
            None => {}
        }

        write_end_tag(writer, "a:p");
    }
}