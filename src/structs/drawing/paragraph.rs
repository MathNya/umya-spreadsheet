// a:p
use super::run_properties::RunProperties;
use super::run::Run;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Paragraph {
    algn: String,
    run: Vec<Run>,
    end_para_run_properties: Option<RunProperties>,
}
impl Paragraph {
    pub fn get_algn(&self) -> &str {
        &self.algn
    }

    pub fn set_algn<S: Into<String>>(&mut self, value:S) {
        self.algn = value.into();
    }

    pub fn get_run(&self) -> &Vec<Run> {
        &self.run
    }

    pub fn add_run(&mut self, value:Run) {
        self.run.push(value);
    }

    pub fn get_end_para_run_properties(&self) -> &Option<RunProperties> {
        &self.end_para_run_properties
    }

    pub fn set_end_para_run_properties(&mut self, value:RunProperties) {
        self.end_para_run_properties = Some(value);
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
                        b"a:r" => {
                            let mut run = Run::default();
                            run.set_attributes(reader, e);
                            &mut self.add_run(run);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:pPr" => {
                            &mut self.set_algn(get_attribute(e, b"algn").unwrap());
                        },
                        b"a:endParaRPr" => {
                            let mut run_properties = RunProperties::default();
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
        write_start_tag(writer, "a:pPr", vec![
            ("algn", &self.algn),
        ], true);

        // a:r
        for run in &self.run {
            run.write_to(writer);
        }

        // a:endParaRPr
        match &self.end_para_run_properties {
            Some(v) => v.write_to(writer, "a:endParaRPr"),
            None => {}
        }

        write_end_tag(writer, "a:p");
    }
}