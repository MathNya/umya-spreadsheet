// fills
use super::Column;
use super::ColumnSort;
use super::Stylesheet;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub(crate) struct Columns {
    column: Vec<Column>,
}
impl Columns {
    pub(crate) fn get_column_collection(&self)-> &Vec<Column> {
        &self.column
    }

    pub(crate) fn get_column_collection_mut(&mut self)-> &mut Vec<Column> {
        &mut self.column
    }

    pub(crate) fn get_column(&self, value:&u32)-> Option<&Column> {
        for column in &self.column {
            if value == column.get_col_num() {
                return Some(column);
            }
        }
        None
    }

    pub(crate) fn set_column(&mut self, value:Column)-> &mut Self {
        self.column.push(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart,
        stylesheet: &Stylesheet,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"col" => {
                            let mut obj = Column::default();
                            obj.set_attributes(reader, e, stylesheet);
                            let min = get_attribute(e, b"min").unwrap().parse::<u32>().unwrap();
                            let max = get_attribute(e, b"max").unwrap().parse::<u32>().unwrap();
                            for i in min..=max {
                                obj.set_col_num(i);
                                self.set_column(obj.clone());
                            }
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"cols" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "cols"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        stylesheet: &mut Stylesheet,
    ) {
        if self.column.len() > 0 {
            let mut column_index: Vec<ColumnSort> = Vec::new();
            for column in &self.column {
                let mut obj = ColumnSort::default();
                obj.col_num = column.get_col_num().clone();
                obj.hash_code = column.get_hash_code();
                column_index.push(obj);
            }

            // cols
            write_start_tag(writer, "cols", vec![], false);

            // col
            let mut hash_code = column_index[0].hash_code.clone();
            let mut min = column_index[0].col_num.clone();
            let mut max = column_index[0].col_num.clone();
            for index in &column_index {
                if hash_code == index.hash_code {
                    max = index.col_num.clone();
                } else {
                    self.write_to_column(writer, &min, &max, stylesheet);
                    hash_code = index.hash_code.clone();
                    min = index.col_num.clone();
                    max = index.col_num.clone();
                }
            }
            self.write_to_column(writer, &min, &max, stylesheet);

            write_end_tag(writer, "cols");
        }
    }

    pub(crate) fn write_to_column(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        min: &u32,
        max: &u32,
        stylesheet: &mut Stylesheet,
    ) {
        // col
        let obj = self.get_column(min).unwrap();
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let min_str = min.to_string();
        let max_str = max.to_string();
        attributes.push(("min", min_str.as_str()));
        attributes.push(("max", max_str.as_str()));
        attributes.push(("width", obj.width.get_value_string()));
        if obj.best_fit.get_value() == &true {
            attributes.push(("bestFit", obj.best_fit.get_value_string()));
        }
        attributes.push(("customWidth", "1"));
        let xf_index_str:String;
        let xf_index = stylesheet.set_style(obj.get_style());
        if xf_index > 0 {
            xf_index_str = xf_index.to_string();
            attributes.push(("style", &xf_index_str));
        }
        write_start_tag(writer, "col", attributes, true);
    }
}
