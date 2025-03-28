use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    AutoTitleDeleted,
    BackWall,
    DisplayBlanksAs,
    Floor,
    Formula,
    Legend,
    PlotArea,
    PlotVisibleOnly,
    ShowDataLabelsOverMaximum,
    SideWall,
    Title,
    View3D,
};
use crate::{
    structs::Workbook,
    traits::AdjustmentCoordinateWithSheet,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct Chart {
    title:                         Option<Title>,
    auto_title_deleted:            AutoTitleDeleted,
    view_3d:                       Option<View3D>,
    floor:                         Option<Floor>,
    side_wall:                     Option<SideWall>,
    back_wall:                     Option<BackWall>,
    plot_area:                     PlotArea,
    legend:                        Legend,
    plot_visible_only:             PlotVisibleOnly,
    display_blanks_as:             DisplayBlanksAs,
    show_data_labels_over_maximum: ShowDataLabelsOverMaximum,
}

impl Chart {
    pub const LANG_EN_GB: &'static str = "en_GB";
    pub const LANG_JA_JP: &'static str = "ja-JP";

    #[must_use]
    pub fn title(&self) -> Option<&Title> {
        self.title.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use title()")]
    pub fn get_title(&self) -> Option<&Title> {
        self.title()
    }

    pub fn title_mut(&mut self) -> Option<&mut Title> {
        self.title.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use title_mut()")]
    pub fn get_title_mut(&mut self) -> Option<&mut Title> {
        self.title_mut()
    }

    pub fn set_title(&mut self, value: Title) -> &mut Self {
        self.title = Some(value);
        self
    }

    #[must_use]
    pub fn auto_title_deleted(&self) -> &AutoTitleDeleted {
        &self.auto_title_deleted
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use auto_title_deleted()")]
    pub fn get_auto_title_deleted(&self) -> &AutoTitleDeleted {
        self.auto_title_deleted()
    }

    pub fn auto_title_deleted_mut(&mut self) -> &mut AutoTitleDeleted {
        &mut self.auto_title_deleted
    }

    #[deprecated(since = "3.0.0", note = "Use auto_title_deleted_mut()")]
    pub fn get_auto_title_deleted_mut(&mut self) -> &mut AutoTitleDeleted {
        self.auto_title_deleted_mut()
    }

    pub fn set_auto_title_deleted(&mut self, value: AutoTitleDeleted) -> &mut Self {
        self.auto_title_deleted = value;
        self
    }

    #[must_use]
    pub fn view_3d(&self) -> Option<&View3D> {
        self.view_3d.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use view_3d()")]
    pub fn get_view_3d(&self) -> Option<&View3D> {
        self.view_3d()
    }

    pub fn view_3d_mut(&mut self) -> Option<&mut View3D> {
        self.view_3d.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use view_3d_mut()")]
    pub fn get_view_3d_mut(&mut self) -> Option<&mut View3D> {
        self.view_3d_mut()
    }

    pub fn set_view_3d(&mut self, value: View3D) -> &mut Self {
        self.view_3d = Some(value);
        self
    }

    #[must_use]
    pub fn floor(&self) -> Option<&Floor> {
        self.floor.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use floor()")]
    pub fn get_floor(&self) -> Option<&Floor> {
        self.floor()
    }

    pub fn floor_mut(&mut self) -> Option<&mut Floor> {
        self.floor.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use floor_mut()")]
    pub fn get_floor_mut(&mut self) -> Option<&mut Floor> {
        self.floor_mut()
    }

    pub fn set_floor(&mut self, value: Floor) -> &mut Self {
        self.floor = Some(value);
        self
    }

    #[must_use]
    pub fn side_wall(&self) -> Option<&SideWall> {
        self.side_wall.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use side_wall()")]
    pub fn get_side_wall(&self) -> Option<&SideWall> {
        self.side_wall()
    }

    pub fn side_wall_mut(&mut self) -> Option<&mut SideWall> {
        self.side_wall.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use side_wall_mut()")]
    pub fn get_side_wall_mut(&mut self) -> Option<&mut SideWall> {
        self.side_wall_mut()
    }

    pub fn set_side_wall(&mut self, value: SideWall) -> &mut Self {
        self.side_wall = Some(value);
        self
    }

    #[must_use]
    pub fn back_wall(&self) -> Option<&BackWall> {
        self.back_wall.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use back_wall()")]
    pub fn get_back_wall(&self) -> Option<&BackWall> {
        self.back_wall()
    }

    pub fn back_wall_mut(&mut self) -> Option<&mut BackWall> {
        self.back_wall.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use back_wall_mut()")]
    pub fn get_back_wall_mut(&mut self) -> Option<&mut BackWall> {
        self.back_wall_mut()
    }

    pub fn set_back_wall(&mut self, value: BackWall) -> &mut Self {
        self.back_wall = Some(value);
        self
    }

    #[must_use]
    pub fn plot_area(&self) -> &PlotArea {
        &self.plot_area
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use plot_area()")]
    pub fn get_plot_area(&self) -> &PlotArea {
        self.plot_area()
    }

    pub fn plot_area_mut(&mut self) -> &mut PlotArea {
        &mut self.plot_area
    }

    #[deprecated(since = "3.0.0", note = "Use plot_area_mut()")]
    pub fn get_plot_area_mut(&mut self) -> &mut PlotArea {
        self.plot_area_mut()
    }

    pub fn set_plot_area(&mut self, value: PlotArea) -> &mut Self {
        self.plot_area = value;
        self
    }

    #[must_use]
    pub fn legend(&self) -> &Legend {
        &self.legend
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use legend()")]
    pub fn get_legend(&self) -> &Legend {
        self.legend()
    }

    pub fn legend_mut(&mut self) -> &mut Legend {
        &mut self.legend
    }

    #[deprecated(since = "3.0.0", note = "Use legend_mut()")]
    pub fn get_legend_mut(&mut self) -> &mut Legend {
        self.legend_mut()
    }

    pub fn set_legend(&mut self, value: Legend) -> &mut Self {
        self.legend = value;
        self
    }

    #[must_use]
    pub fn plot_visible_only(&self) -> &PlotVisibleOnly {
        &self.plot_visible_only
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use plot_visible_only()")]
    pub fn get_plot_visible_only(&self) -> &PlotVisibleOnly {
        self.plot_visible_only()
    }

    pub fn plot_visible_only_mut(&mut self) -> &mut PlotVisibleOnly {
        &mut self.plot_visible_only
    }

    #[deprecated(since = "3.0.0", note = "Use plot_visible_only_mut()")]
    pub fn get_plot_visible_only_mut(&mut self) -> &mut PlotVisibleOnly {
        self.plot_visible_only_mut()
    }

    pub fn set_plot_visible_only(&mut self, value: PlotVisibleOnly) -> &mut Self {
        self.plot_visible_only = value;
        self
    }

    #[must_use]
    pub fn display_blanks_as(&self) -> &DisplayBlanksAs {
        &self.display_blanks_as
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use display_blanks_as()")]
    pub fn get_display_blanks_as(&self) -> &DisplayBlanksAs {
        self.display_blanks_as()
    }

    pub fn display_blanks_as_mut(&mut self) -> &mut DisplayBlanksAs {
        &mut self.display_blanks_as
    }

    #[deprecated(since = "3.0.0", note = "Use display_blanks_as_mut()")]
    pub fn get_display_blanks_as_mut(&mut self) -> &mut DisplayBlanksAs {
        self.display_blanks_as_mut()
    }

    pub fn set_display_blanks_as(&mut self, value: DisplayBlanksAs) -> &mut Self {
        self.display_blanks_as = value;
        self
    }

    #[must_use]
    pub fn show_data_labels_over_maximum(&self) -> &ShowDataLabelsOverMaximum {
        &self.show_data_labels_over_maximum
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use show_data_labels_over_maximum()")]
    pub fn get_show_data_labels_over_maximum(&self) -> &ShowDataLabelsOverMaximum {
        self.show_data_labels_over_maximum()
    }

    pub fn show_data_labels_over_maximum_mut(&mut self) -> &mut ShowDataLabelsOverMaximum {
        &mut self.show_data_labels_over_maximum
    }

    #[deprecated(since = "3.0.0", note = "Use show_data_labels_over_maximum_mut()")]
    pub fn get_show_data_labels_over_maximum_mut(&mut self) -> &mut ShowDataLabelsOverMaximum {
        self.show_data_labels_over_maximum_mut()
    }

    pub fn set_show_data_labels_over_maximum(
        &mut self,
        value: ShowDataLabelsOverMaximum,
    ) -> &mut Chart {
        self.show_data_labels_over_maximum = value;
        self
    }

    pub fn formula_mut(&mut self) -> Vec<&mut Formula> {
        self.plot_area_mut().formula_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use formula_mut()")]
    pub fn get_formula_mut(&mut self) -> Vec<&mut Formula> {
        self.formula_mut()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => match e.name().into_inner() {
                b"c:title" => {
                    let mut obj = Title::default();
                    obj.set_attributes(reader, e);
                    self.set_title(obj);
                }
                b"c:view3D" => {
                    let mut obj = View3D::default();
                    obj.set_attributes(reader, e);
                    self.set_view_3d(obj);
                }
                b"c:floor" => {
                    let mut obj = Floor::default();
                    obj.set_attributes(reader, e);
                    self.set_floor(obj);
                }
                b"c:sideWall" => {
                    let mut obj = SideWall::default();
                    obj.set_attributes(reader, e);
                    self.set_side_wall(obj);
                }
                b"c:backWall" => {
                    let mut obj = BackWall::default();
                    obj.set_attributes(reader, e);
                    self.set_back_wall(obj);
                }
                b"c:plotArea" => {
                    self.plot_area.set_attributes(reader, e);
                }
                b"c:legend" => {
                    self.legend.set_attributes(reader, e);
                }
                _ => (),
            },
            Event::Empty(ref e) => match e.name().into_inner() {
                b"c:autoTitleDeleted" => {
                    self.auto_title_deleted.set_attributes(reader, e);
                }
                b"c:plotVisOnly" => {
                    self.plot_visible_only.set_attributes(reader, e);
                }
                b"c:dispBlanksAs" => {
                    self.display_blanks_as.set_attributes(reader, e);
                }
                b"c:showDLblsOverMax" => {
                    self.show_data_labels_over_maximum.set_attributes(reader, e);
                }
                _ => (),
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:chart" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:chart"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, wb: &Workbook) {
        // c:chart
        write_start_tag(writer, "c:chart", vec![], false);

        // c:title
        if let Some(v) = &self.title {
            v.write_to(writer);
        }

        // c:autoTitleDeleted
        self.auto_title_deleted.write_to(writer);

        // c:view3D
        if let Some(v) = &self.view_3d {
            v.write_to(writer);
        }

        // c:floor
        if let Some(v) = &self.floor {
            v.write_to(writer);
        }

        // c:sideWall
        if let Some(v) = &self.side_wall {
            v.write_to(writer);
        }

        // c:backWall
        if let Some(v) = &self.back_wall {
            v.write_to(writer);
        }

        // c:plotArea
        self.plot_area.write_to(writer, wb);

        // c:legend
        self.legend.write_to(writer);

        // c:plotVisOnly
        self.plot_visible_only.write_to(writer);

        // c:dispBlanksAs
        self.display_blanks_as.write_to(writer);

        // c:showDLblsOverMax
        self.show_data_labels_over_maximum.write_to(writer);

        write_end_tag(writer, "c:chart");
    }
}
impl AdjustmentCoordinateWithSheet for Chart {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.plot_area.adjustment_insert_coordinate_with_sheet(
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.plot_area.adjustment_remove_coordinate_with_sheet(
            sheet_name,
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
