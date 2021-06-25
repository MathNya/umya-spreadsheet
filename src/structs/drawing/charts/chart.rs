use super::Title;
use super::AutoTitleDeleted;
use super::PlotArea;
use super::Legend;
use super::PlotVisibleOnly;
use super::DisplayBlanksAs;
use super::ShowDataLabelsOverMaximum;
use super::Formula;
use writer::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Chart {
    title: Option<Title>,
    auto_title_deleted: AutoTitleDeleted,
    plot_area: PlotArea,
    legend: Legend,
    plot_visible_only: PlotVisibleOnly,
    display_blanks_as: DisplayBlanksAs,
    show_data_labels_over_maximum: ShowDataLabelsOverMaximum,
}
impl Chart {
    
    pub const LANG_EN_GB: &'static str = "en_GB";
    pub const LANG_JA_JP: &'static str = "ja-JP";

    pub fn get_title(&self) -> &Option<Title> {
        &self.title
    }

    pub fn get_title_mut(&mut self) -> &mut Option<Title> {
        &mut self.title
    }

    pub fn set_title(&mut self, value:Title)-> &mut Chart {
        self.title = Some(value);
        self
    }

    pub fn get_auto_title_deleted(&self)-> &AutoTitleDeleted {
        &self.auto_title_deleted
    }

    pub fn get_auto_title_deleted_mut(&mut self)-> &mut AutoTitleDeleted {
        &mut self.auto_title_deleted
    }

    pub fn set_auto_title_deleted(&mut self, value:AutoTitleDeleted)-> &mut Chart {
        self.auto_title_deleted = value;
        self
    }

    pub fn get_plot_area(&self)-> &PlotArea {
        &self.plot_area
    }

    pub fn get_plot_area_mut(&mut self)-> &mut PlotArea {
        &mut self.plot_area
    }

    pub fn set_plot_area(&mut self, value:PlotArea)-> &mut Chart {
        self.plot_area = value;
        self
    }

    pub fn get_legend(&self)-> &Legend {
        &self.legend
    }

    pub fn get_legend_mut(&mut self)-> &mut Legend {
        &mut self.legend
    }

    pub fn set_legend(&mut self, value:Legend)-> &mut Chart {
        self.legend = value;
        self
    }

    pub fn get_plot_visible_only(&self)-> &PlotVisibleOnly {
        &self.plot_visible_only
    }

    pub fn get_plot_visible_only_mut(&mut self)-> &mut PlotVisibleOnly {
        &mut self.plot_visible_only
    }

    pub fn set_plot_visible_only(&mut self, value:PlotVisibleOnly)-> &mut Chart {
        self.plot_visible_only = value;
        self
    }

    pub fn get_display_blanks_as(&self)-> &DisplayBlanksAs {
        &self.display_blanks_as
    }

    pub fn get_display_blanks_as_mut(&mut self)-> &mut DisplayBlanksAs {
        &mut self.display_blanks_as
    }

    pub fn set_display_blanks_as(&mut self, value:DisplayBlanksAs)-> &mut Chart {
        self.display_blanks_as = value;
        self
    }

    pub fn get_show_data_labels_over_maximum(&self)-> &ShowDataLabelsOverMaximum {
        &self.show_data_labels_over_maximum
    }

    pub fn get_show_data_labels_over_maximum_mut(&mut self)-> &mut ShowDataLabelsOverMaximum {
        &mut self.show_data_labels_over_maximum
    }

    pub fn set_show_data_labels_over_maximum(&mut self, value:ShowDataLabelsOverMaximum)-> &mut Chart {
        self.show_data_labels_over_maximum = value;
        self
    }

    pub fn get_formula_mut(&mut self)-> Vec<&mut Formula> {
        self.get_plot_area_mut().get_formula_mut()
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
                        b"c:title" => {
                            let mut obj = Title::default();
                            obj.set_attributes(reader, e);
                            &mut self.set_title(obj);
                        },
                        b"c:plotArea" => {
                            &mut self.plot_area.set_attributes(reader, e);
                        },
                        b"c:legend" => {
                            &mut self.legend.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:autoTitleDeleted" => {
                            &mut self.auto_title_deleted.set_attributes(reader, e);
                        },
                        b"c:plotVisOnly" => {
                            &mut self.plot_visible_only.set_attributes(reader, e);
                        },
                        b"c:dispBlanksAs" => {
                            &mut self.display_blanks_as.set_attributes(reader, e);
                        },
                        b"c:showDLblsOverMax" => {
                            &mut self.show_data_labels_over_maximum.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:chart" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:chart"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:chart
        write_start_tag(writer, "c:chart", vec![], false);

        // c:title
        match &self.title {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }
        
        // c:autoTitleDeleted
        &self.auto_title_deleted.write_to(writer);

        // c:plotArea
        &self.plot_area.write_to(writer);
        
        // c:legend
        &self.legend.write_to(writer);

        // c:plotVisOnly
        &self.plot_visible_only.write_to(writer);

        // c:dispBlanksAs
        &self.display_blanks_as.write_to(writer);

        // c:showDLblsOverMax
        &self.show_data_labels_over_maximum.write_to(writer);

        write_end_tag(writer, "c:chart");
    }
}