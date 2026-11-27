use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::{
        BooleanValue,
        CellCommentsValues,
        EnumValue,
        OrientationValues,
        PageOrderValues,
        PrintErrorValues,
        UInt32Value,
        raw::RawRelationships,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct PageSetup {
    paper_size:            UInt32Value,
    orientation:           EnumValue<OrientationValues>,
    scale:                 UInt32Value,
    first_page_number:     UInt32Value,
    use_first_page_number: BooleanValue,
    fit_to_height:         UInt32Value,
    fit_to_width:          UInt32Value,
    fit_to_page:           BooleanValue,
    page_order:            EnumValue<PageOrderValues>,
    black_and_white:       BooleanValue,
    draft:                 BooleanValue,
    cell_comments:         EnumValue<CellCommentsValues>,
    errors:                EnumValue<PrintErrorValues>,
    horizontal_dpi:        UInt32Value,
    vertical_dpi:          UInt32Value,
    copies:                UInt32Value,
    object_data:           Option<Vec<u8>>,
}

impl PageSetup {
    #[inline]
    #[must_use]
    pub fn paper_size(&self) -> u32 {
        self.paper_size.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use paper_size()")]
    pub fn get_paper_size(&self) -> u32 {
        self.paper_size()
    }

    #[inline]
    pub fn set_paper_size(&mut self, value: u32) -> &mut Self {
        self.paper_size.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn orientation(&self) -> &OrientationValues {
        self.orientation.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use orientation()")]
    pub fn get_orientation(&self) -> &OrientationValues {
        self.orientation()
    }

    #[inline]
    pub fn set_orientation(&mut self, value: OrientationValues) -> &mut Self {
        self.orientation.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn scale(&self) -> u32 {
        self.scale.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use scale()")]
    pub fn get_scale(&self) -> u32 {
        self.scale()
    }

    #[inline]
    pub fn set_scale(&mut self, value: u32) -> &mut Self {
        self.scale.set_value(value);
        self
    }

    /// The page number of the first printed page (`firstPageNumber`).
    /// Only used when `use_first_page_number` is `true`. Defaults to `1`.
    #[inline]
    #[must_use]
    pub fn first_page_number(&self) -> u32 {
        if self.first_page_number.has_value() {
            self.first_page_number.value()
        } else {
            1
        }
    }

    #[inline]
    pub fn set_first_page_number(&mut self, value: u32) -> &mut Self {
        self.first_page_number.set_value(value);
        self
    }

    /// Whether `first_page_number` is used instead of starting at page 1
    /// (`useFirstPageNumber`).
    #[inline]
    #[must_use]
    pub fn use_first_page_number(&self) -> bool {
        self.use_first_page_number.value()
    }

    #[inline]
    pub fn set_use_first_page_number(&mut self, value: bool) -> &mut Self {
        self.use_first_page_number.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn fit_to_height(&self) -> u32 {
        self.fit_to_height.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use fit_to_height()")]
    pub fn get_fit_to_height(&self) -> u32 {
        self.fit_to_height()
    }

    #[inline]
    pub fn set_fit_to_height(&mut self, value: u32) -> &mut Self {
        self.fit_to_height.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn fit_to_width(&self) -> u32 {
        self.fit_to_width.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use fit_to_width()")]
    pub fn get_fit_to_width(&self) -> u32 {
        self.fit_to_width()
    }

    #[inline]
    pub fn set_fit_to_width(&mut self, value: u32) -> &mut Self {
        self.fit_to_width.set_value(value);
        self
    }

    /// Whether the sheet is scaled to `fit_to_width` x `fit_to_height` pages
    /// instead of using `scale`. Stored as `<pageSetUpPr fitToPage="1"/>`
    /// in the sheet properties.
    #[inline]
    #[must_use]
    pub fn fit_to_page(&self) -> bool {
        self.fit_to_page.value()
    }

    #[inline]
    pub fn set_fit_to_page(&mut self, value: bool) -> &mut Self {
        self.fit_to_page.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_fit_to_page(&self) -> bool {
        self.fit_to_page.has_value()
    }

    #[inline]
    pub(crate) fn fit_to_page_str(&self) -> &str {
        self.fit_to_page.value_string()
    }

    #[inline]
    pub(crate) fn set_fit_to_page_str(&mut self, value: &str) -> &mut Self {
        self.fit_to_page.set_value_string(value);
        self
    }

    /// The order in which the pages are printed (`pageOrder`).
    #[inline]
    #[must_use]
    pub fn page_order(&self) -> &PageOrderValues {
        self.page_order.value()
    }

    #[inline]
    pub fn set_page_order(&mut self, value: PageOrderValues) -> &mut Self {
        self.page_order.set_value(value);
        self
    }

    /// Whether the sheet is printed in black and white (`blackAndWhite`).
    #[inline]
    #[must_use]
    pub fn black_and_white(&self) -> bool {
        self.black_and_white.value()
    }

    #[inline]
    pub fn set_black_and_white(&mut self, value: bool) -> &mut Self {
        self.black_and_white.set_value(value);
        self
    }

    /// Whether the sheet is printed in draft quality (`draft`).
    #[inline]
    #[must_use]
    pub fn draft(&self) -> bool {
        self.draft.value()
    }

    #[inline]
    pub fn set_draft(&mut self, value: bool) -> &mut Self {
        self.draft.set_value(value);
        self
    }

    /// How the cell comments are printed (`cellComments`).
    #[inline]
    #[must_use]
    pub fn cell_comments(&self) -> &CellCommentsValues {
        self.cell_comments.value()
    }

    #[inline]
    pub fn set_cell_comments(&mut self, value: CellCommentsValues) -> &mut Self {
        self.cell_comments.set_value(value);
        self
    }

    /// How cells with errors are printed (`errors`).
    #[inline]
    #[must_use]
    pub fn errors(&self) -> &PrintErrorValues {
        self.errors.value()
    }

    #[inline]
    pub fn set_errors(&mut self, value: PrintErrorValues) -> &mut Self {
        self.errors.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn horizontal_dpi(&self) -> u32 {
        self.horizontal_dpi.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use horizontal_dpi()")]
    pub fn get_horizontal_dpi(&self) -> u32 {
        self.horizontal_dpi()
    }

    #[inline]
    pub fn set_horizontal_dpi(&mut self, value: u32) -> &mut Self {
        self.horizontal_dpi.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn vertical_dpi(&self) -> u32 {
        self.vertical_dpi.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use vertical_dpi()")]
    pub fn get_vertical_dpi(&self) -> u32 {
        self.vertical_dpi()
    }

    #[inline]
    pub fn set_vertical_dpi(&mut self, value: u32) -> &mut Self {
        self.vertical_dpi.set_value(value);
        self
    }

    /// The number of copies to print (`copies`). Defaults to `1`.
    #[inline]
    #[must_use]
    pub fn copies(&self) -> u32 {
        if self.copies.has_value() {
            self.copies.value()
        } else {
            1
        }
    }

    #[inline]
    pub fn set_copies(&mut self, value: u32) -> &mut Self {
        self.copies.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn object_data(&self) -> Option<&[u8]> {
        self.object_data.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use object_data()")]
    pub fn get_object_data(&self) -> Option<&[u8]> {
        self.object_data()
    }

    #[inline]
    pub fn object_data_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.object_data.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use object_data_mut()")]
    pub fn get_object_data_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.object_data_mut()
    }

    #[inline]
    pub fn set_object_data(&mut self, value: impl Into<Vec<u8>>) -> &mut Self {
        self.object_data = Some(value.into());
        self
    }

    #[inline]
    pub(crate) fn has_param(&self) -> bool {
        self.paper_size.has_value()
            || self.orientation.has_value()
            || self.scale.has_value()
            || self.first_page_number.has_value()
            || self.use_first_page_number.has_value()
            || self.fit_to_height.has_value()
            || self.fit_to_width.has_value()
            || self.page_order.has_value()
            || self.black_and_white.has_value()
            || self.draft.has_value()
            || self.cell_comments.has_value()
            || self.errors.has_value()
            || self.horizontal_dpi.has_value()
            || self.vertical_dpi.has_value()
            || self.copies.has_value()
            || self.object_data.is_some()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        relationships: Option<&RawRelationships>,
    ) {
        set_string_from_xml!(self, e, paper_size, "paperSize");
        set_string_from_xml!(self, e, orientation, "orientation");
        set_string_from_xml!(self, e, scale, "scale");
        set_string_from_xml!(self, e, first_page_number, "firstPageNumber");
        set_string_from_xml!(self, e, fit_to_height, "fitToHeight");
        set_string_from_xml!(self, e, fit_to_width, "fitToWidth");
        set_string_from_xml!(self, e, page_order, "pageOrder");
        set_string_from_xml!(self, e, black_and_white, "blackAndWhite");
        set_string_from_xml!(self, e, draft, "draft");
        set_string_from_xml!(self, e, cell_comments, "cellComments");
        set_string_from_xml!(self, e, use_first_page_number, "useFirstPageNumber");
        set_string_from_xml!(self, e, errors, "errors");
        set_string_from_xml!(self, e, horizontal_dpi, "horizontalDpi");
        set_string_from_xml!(self, e, vertical_dpi, "verticalDpi");
        set_string_from_xml!(self, e, copies, "copies");

        if let Some(r_id) = get_attribute(e, b"r:id") {
            let attached_file = relationships.unwrap().relationship_by_rid(&r_id).raw_file();
            self.set_object_data(attached_file.file_data());
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &mut usize) {
        if self.has_param() {
            // pageSetup
            let r_id_str = format!("rId{r_id}");
            let mut attributes: crate::structs::AttrCollection = Vec::new();
            let paper_size = self.paper_size.value_string();
            if self.paper_size.has_value() {
                attributes.push(("paperSize", &paper_size).into());
            }
            let scale = self.scale.value_string();
            if self.scale.has_value() {
                attributes.push(("scale", &scale).into());
            }
            let first_page_number = self.first_page_number.value_string();
            if self.first_page_number.has_value() {
                attributes.push(("firstPageNumber", &first_page_number).into());
            }
            let orientation = self.orientation.value_string();
            if self.orientation.has_value() {
                attributes.push(("orientation", orientation).into());
            }
            let fit_to_height = self.fit_to_height.value_string();
            if self.fit_to_height.has_value() {
                attributes.push(("fitToHeight", &fit_to_height).into());
            }
            let fit_to_width = self.fit_to_width.value_string();
            if self.fit_to_width.has_value() {
                attributes.push(("fitToWidth", &fit_to_width).into());
            }
            if self.page_order.has_value() {
                attributes.push(("pageOrder", self.page_order.value_string()).into());
            }
            if self.black_and_white.has_value() {
                attributes.push(("blackAndWhite", self.black_and_white.value_string()).into());
            }
            if self.draft.has_value() {
                attributes.push(("draft", self.draft.value_string()).into());
            }
            if self.cell_comments.has_value() {
                attributes.push(("cellComments", self.cell_comments.value_string()).into());
            }
            if self.use_first_page_number.has_value() {
                attributes.push(
                    (
                        "useFirstPageNumber",
                        self.use_first_page_number.value_string(),
                    )
                        .into(),
                );
            }
            if self.errors.has_value() {
                attributes.push(("errors", self.errors.value_string()).into());
            }
            let horizontal_dpi = self.horizontal_dpi.value_string();
            if self.horizontal_dpi.has_value() {
                attributes.push(("horizontalDpi", &horizontal_dpi).into());
            }
            let vertical_dpi = self.vertical_dpi.value_string();
            if self.vertical_dpi.has_value() {
                attributes.push(("verticalDpi", &vertical_dpi).into());
            }
            let copies = self.copies.value_string();
            if self.copies.has_value() {
                attributes.push(("copies", &copies).into());
            }
            if self.object_data.is_some() {
                attributes.push(("r:id", &r_id_str).into());
                *r_id += 1;
            }
            write_start_tag(writer, "pageSetup", attributes, true);
        }
    }
}
