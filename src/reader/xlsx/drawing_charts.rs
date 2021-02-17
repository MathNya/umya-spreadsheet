use quick_xml::events::{Event};
use super::driver::*;

use super::drawing::*;
use super::super::structs::drawing::charts::chart::Chart;

// xdr:graphicFrame
pub(crate) fn read_chart(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>) -> (String, Chart) {
    let mut buf = Vec::new();

    let mut chart_id = String::from("");
    let mut chart = Chart::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"a:xfrm" => {
                        chart.set_transform(read_transform2d(reader, e));
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"xdr:cNvPr" => {
                        let obj = chart.get_non_visual_drawing_properties_mut();
                        obj.set_id(get_attribute(e, b"id").unwrap());
                        obj.set_name(get_attribute(e, b"name").unwrap());
                    },
                    b"c:chart" => {
                        chart_id = get_attribute(e, b"r:id").unwrap();
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"xdr:graphicFrame" => return (chart_id, chart),
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:graphicFrame"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}
