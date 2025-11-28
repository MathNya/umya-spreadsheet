// pivotTableDefinition
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    helper::const_str::{
        MC_NS,
        SHEET_MAIN_NS,
        SHEET_MS_REVISION_NS,
    }, reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    }, structs::{
        BooleanValue,
        ByteValue,
        ColumnFields,
        ColumnItems,
        DataFields,
        Location,
        PivotFields,
        PivotTableStyle,
        RowItems,
        StringValue,
        UInt32Value,
    }, writer::driver::{
        write_end_tag,
        write_start_tag,
    }
};

#[derive(Clone, Default, Debug)]
pub struct PivotTableDefinition {
    apply_number_formats:       BooleanValue,
    apply_border_formats:       BooleanValue,
    apply_font_formats:         BooleanValue,
    apply_pattern_formats:      BooleanValue,
    apply_alignment_formats:    BooleanValue,
    apply_width_height_formats: BooleanValue,
    use_auto_formatting:        BooleanValue,
    item_print_titles:          BooleanValue,
    outline:                    BooleanValue,
    outline_data:               BooleanValue,
    multiple_field_filters:     BooleanValue,
    name:                       StringValue,
    cache_id:                   UInt32Value,
    indent:                     UInt32Value,
    local_name:                 StringValue,
    data_caption:               StringValue,
    updated_version:            ByteValue,
    min_refreshable_version:    ByteValue,
    created_version:            ByteValue,
    location:                   Location,
    pivot_fields:               PivotFields,
    row_items:                  RowItems,
    column_fields:              ColumnFields,
    column_items:               ColumnItems,
    data_fields:                DataFields,
    pivot_table_style:          PivotTableStyle,
}
impl PivotTableDefinition {
    #[inline]
    #[must_use]
    pub fn apply_number_formats(&self) -> bool {
        self.apply_number_formats.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use apply_number_formats()")]
    pub fn get_apply_number_formats(&self) -> bool {
        self.apply_number_formats()
    }

    #[inline]
    pub fn set_apply_number_formats(&mut self, value: bool) -> &mut Self {
        self.apply_number_formats.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn apply_border_formats(&self) -> bool {
        self.apply_border_formats.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use apply_border_formats()")]
    pub fn get_apply_border_formats(&self) -> bool {
        self.apply_border_formats()
    }

    #[inline]
    pub fn set_apply_border_formats(&mut self, value: bool) -> &mut Self {
        self.apply_border_formats.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn apply_font_formats(&self) -> bool {
        self.apply_font_formats.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use apply_font_formats()")]
    pub fn get_apply_font_formats(&self) -> bool {
        self.apply_font_formats()
    }

    #[inline]
    pub fn set_apply_font_formats(&mut self, value: bool) -> &mut Self {
        self.apply_font_formats.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn apply_pattern_formats(&self) -> bool {
        self.apply_pattern_formats.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use apply_pattern_formats()")]
    pub fn get_apply_pattern_formats(&self) -> bool {
        self.apply_pattern_formats()
    }

    #[inline]
    pub fn set_apply_pattern_formats(&mut self, value: bool) -> &mut Self {
        self.apply_pattern_formats.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn apply_alignment_formats(&self) -> bool {
        self.apply_alignment_formats.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use apply_alignment_formats()")]
    pub fn get_apply_alignment_formats(&self) -> bool {
        self.apply_alignment_formats()
    }

    #[inline]
    pub fn set_apply_alignment_formats(&mut self, value: bool) -> &mut Self {
        self.apply_alignment_formats.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn apply_width_height_formats(&self) -> bool {
        self.apply_width_height_formats.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use apply_width_height_formats()")]
    pub fn get_apply_width_height_formats(&self) -> bool {
        self.apply_width_height_formats()
    }

    #[inline]
    pub fn set_apply_width_height_formats(&mut self, value: bool) -> &mut Self {
        self.apply_width_height_formats.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn use_auto_formatting(&self) -> bool {
        self.use_auto_formatting.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use use_auto_formatting()")]
    pub fn get_use_auto_formatting(&self) -> bool {
        self.use_auto_formatting()
    }
    
    #[inline]
    pub fn set_use_auto_formatting(&mut self, value: bool) -> &mut Self {
        self.use_auto_formatting.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn item_print_titles(&self) -> bool {
        self.item_print_titles.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use item_print_titles()")]
    pub fn get_item_print_titles(&self) -> bool {
        self.item_print_titles()
    }

    #[inline]
    pub fn set_item_print_titles(&mut self, value: bool) -> &mut Self {
        self.item_print_titles.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn outline(&self) -> bool {
        self.outline.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use outline()")]
    pub fn get_outline(&self) -> bool {
        self.outline()
    }
    
    #[inline]
    pub fn set_outline(&mut self, value: bool) -> &mut Self {
        self.outline.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn outline_data(&self) -> bool {
        self.outline_data.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use outline_data()")]
    pub fn get_outline_data(&self) -> bool {
        self.outline_data()
    }

    #[inline]
    pub fn set_outline_data(&mut self, value: bool) -> &mut Self {
        self.outline_data.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn multiple_field_filters(&self) -> bool {
        self.multiple_field_filters.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use multiple_field_filters()")]
    pub fn get_multiple_field_filters(&self) -> bool {
        self.multiple_field_filters()
    }

    #[inline]
    pub fn set_multiple_field_filters(&mut self, value: bool) -> &mut Self {
        self.multiple_field_filters.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use name()")]
    pub fn get_name(&self) -> &str {
        self.name()
    }

    #[inline]
    pub fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn cache_id(&self) -> u32 {
        self.cache_id.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cache_id()")]
    pub fn get_cache_id(&self) -> u32 {
        self.cache_id()
    }

    #[inline]
    pub fn set_cache_id(&mut self, value: u32) -> &mut Self {
        self.cache_id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn indent(&self) -> u32 {
        self.indent.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use indent()")]
    pub fn get_indent(&self) -> u32 {
        self.indent()
    }

    #[inline]
    pub fn set_indent(&mut self, value: u32) -> &mut Self {
        self.indent.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn local_name(&self) -> &str {
        self.local_name.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use local_name()")]
    pub fn get_local_name(&self) -> &str {
        self.local_name()
    }

    #[inline]
    pub fn set_local_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.local_name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn data_caption(&self) -> &str {
        self.data_caption.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use data_caption()")]
    pub fn get_data_caption(&self) -> &str {
        self.data_caption()
    }

    #[inline]
    pub fn set_data_caption<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.data_caption.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn updated_version(&self) -> u8 {
        self.updated_version.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use updated_version()")]
    pub fn get_updated_version(&self) -> u8 {
        self.updated_version()
    }

    #[inline]
    pub fn set_updated_version(&mut self, value: u8) -> &mut Self {
        self.updated_version.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn min_refreshable_version(&self) -> u8 {
        self.min_refreshable_version.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use min_refreshable_version()")]
    pub fn get_min_refreshable_version(&self) -> u8 {
        self.min_refreshable_version()
    }

    #[inline]
    pub fn set_min_refreshable_version(&mut self, value: u8) -> &mut Self {
        self.min_refreshable_version.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn created_version(&self) -> u8 {
        self.created_version.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use created_version()")]
    pub fn get_created_version(&self) -> u8 {
        self.created_version()
    }

    #[inline]
    pub fn set_created_version(&mut self, value: u8) -> &mut Self {
        self.created_version.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn location(&self) -> &Location {
        &self.location
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use location()")]
    pub fn get_location(&self) -> &Location {
        self.location()
    }

    #[inline]
    pub fn location_mut(&mut self) -> &mut Location {
        &mut self.location
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use location_mut()")]
    pub fn get_location_mut(&mut self) -> &mut Location {
        self.location_mut()
    }

    #[inline]
    pub fn set_location(&mut self, value: Location) -> &mut Self {
        self.location = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn pivot_fields(&self) -> &PivotFields {
        &self.pivot_fields
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use pivot_fields()")]
    pub fn get_pivot_fields(&self) -> &PivotFields {
        self.pivot_fields()
    }

    #[inline]
    pub fn pivot_fields_mut(&mut self) -> &mut PivotFields {
        &mut self.pivot_fields
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use pivot_fields_mut()")]
    pub fn get_pivot_fields_mut(&mut self) -> &mut PivotFields {
        self.pivot_fields_mut()
    }

    #[inline]
    pub fn set_pivot_fields(&mut self, value: PivotFields) -> &mut Self {
        self.pivot_fields = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn row_items(&self) -> &RowItems {
        &self.row_items
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use row_items()")]
    pub fn get_row_items(&self) -> &RowItems {
        self.row_items()
    }

    #[inline]
    pub fn row_items_mut(&mut self) -> &mut RowItems {
        &mut self.row_items
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_items_mut()")]
    pub fn get_row_items_mut(&mut self) -> &mut RowItems {
        self.row_items_mut()
    }

    #[inline]
    pub fn set_row_items(&mut self, value: RowItems) -> &mut Self {
        self.row_items = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn column_fields(&self) -> &ColumnFields {
        &self.column_fields
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use column_fields()")]
    pub fn get_column_fields(&self) -> &ColumnFields {
        self.column_fields()
    }

    #[inline]
    pub fn column_fields_mut(&mut self) -> &mut ColumnFields {
        &mut self.column_fields
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use column_fields_mut()")]
    pub fn get_column_fields_mut(&mut self) -> &mut ColumnFields {
        self.column_fields_mut()
    }

    #[inline]
    pub fn set_column_fields(&mut self, value: ColumnFields) -> &mut Self {
        self.column_fields = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn column_items(&self) -> &ColumnItems {
        &self.column_items
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use column_items()")]
    pub fn get_column_items(&self) -> &ColumnItems {
        self.column_items()
    }

    #[inline]
    pub fn column_items_mut(&mut self) -> &mut ColumnItems {
        &mut self.column_items
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use column_items_mut()")]
    pub fn get_column_items_mut(&mut self) -> &mut ColumnItems {
        self.column_items_mut()
    }

    #[inline]
    pub fn set_column_items(&mut self, value: ColumnItems) -> &mut Self {
        self.column_items = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn data_fields(&self) -> &DataFields {
        &self.data_fields
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use data_fields()")]
    pub fn get_data_fields(&self) -> &DataFields {
        self.data_fields()
    }

    #[inline]
    pub fn data_fields_mut(&mut self) -> &mut DataFields {
        &mut self.data_fields
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use data_fields_mut()")]
    pub fn get_data_fields_mut(&mut self) -> &mut DataFields {
        self.data_fields_mut()
    }

    #[inline]
    pub fn set_data_fields(&mut self, value: DataFields) -> &mut Self {
        self.data_fields = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn pivot_table_style(&self) -> &PivotTableStyle {
        &self.pivot_table_style
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use pivot_table_style()")]
    pub fn get_pivot_table_style(&self) -> &PivotTableStyle {
        self.pivot_table_style()
    }

    #[inline]
    pub fn pivot_table_style_mut(&mut self) -> &mut PivotTableStyle {
        &mut self.pivot_table_style
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use pivot_table_style_mut()")]
    pub fn get_pivot_table_style_mut(&mut self) -> &mut PivotTableStyle {
        self.pivot_table_style_mut()
    }

    #[inline]
    pub fn set_pivot_table_style(&mut self, value: PivotTableStyle) -> &mut Self {
        self.pivot_table_style = value;
        self
    }

    /// Create a new minimal pivot table definition with required fields
    pub fn new_simple(name: impl Into<String>, cache_id: u32, destination: impl Into<String>) -> Self {
        let mut pivot_def = Self::default();

        // Set required fields
        pivot_def.set_name(name);
        pivot_def.set_cache_id(cache_id);

        // Set location
        let mut location = Location::default();
        location.set_reference(destination);
        pivot_def.set_location(location);

        // Set Excel version compatibility (Excel 2007+)
        pivot_def.set_updated_version(3);
        pivot_def.set_min_refreshable_version(3);
        pivot_def.set_created_version(3);

        // Enable auto-formatting by default
        pivot_def.set_use_auto_formatting(true);

        pivot_def
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, name, "name");
        set_string_from_xml!(self, e, cache_id, "cacheId");
        set_string_from_xml!(self, e, apply_number_formats, "applyNumberFormats");
        set_string_from_xml!(self, e, apply_border_formats, "applyBorderFormats");
        set_string_from_xml!(self, e, apply_font_formats, "applyFontFormats");
        set_string_from_xml!(self, e, apply_pattern_formats, "applyPatternFormats");
        set_string_from_xml!(self, e, apply_alignment_formats, "applyAlignmentFormats");
        set_string_from_xml!(
            self,
            e,
            apply_width_height_formats,
            "applyWidthHeightFormats"
        );
        set_string_from_xml!(self, e, data_caption, "dataCaption");
        set_string_from_xml!(self, e, updated_version, "updatedVersion");
        set_string_from_xml!(self, e, min_refreshable_version, "minRefreshableVersion");
        set_string_from_xml!(self, e, use_auto_formatting, "useAutoFormatting");
        set_string_from_xml!(self, e, item_print_titles, "itemPrintTitles");
        set_string_from_xml!(self, e, created_version, "createdVersion");
        set_string_from_xml!(self, e, indent, "indent");
        set_string_from_xml!(self, e, outline, "outline");
        set_string_from_xml!(self, e, outline_data, "outlineData");
        set_string_from_xml!(self, e, multiple_field_filters, "multipleFieldFilters");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"location" {
                    let mut obj = Location::default();
                    obj.set_attributes(reader, e);
                    self.set_location(obj);
                }
                if e.name().into_inner() == b"pivotTableStyleInfo" {
                    let mut obj = PivotTableStyle::default();
                    obj.set_attributes(reader, e);
                    self.set_pivot_table_style(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"pivotFields" {
                    let mut obj = PivotFields::default();
                    obj.set_attributes(reader, e);
                    self.set_pivot_fields(obj);
                }
                if e.name().into_inner() == b"rowItems" {
                    let mut obj = RowItems::default();
                    obj.set_attributes(reader, e);
                    self.set_row_items(obj);
                }
                if e.name().into_inner() == b"colFields" {
                    let mut obj = ColumnFields::default();
                    obj.set_attributes(reader, e);
                    self.set_column_fields(obj);
                }
                if e.name().into_inner() == b"colItems" {
                    let mut obj = ColumnItems::default();
                    obj.set_attributes(reader, e);
                    self.set_column_items(obj);
                }
                if e.name().into_inner() == b"dataFields" {
                    let mut obj = DataFields::default();
                    obj.set_attributes(reader, e);
                    self.set_data_fields(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"pivotTableDefinition" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "pivotTableDefinition")
        );
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pivotTableDefinition
        let mut attributes: crate::structs::AttrCollection = vec![
            ("xmlns", SHEET_MAIN_NS).into(),
            ("xmlns:mc", MC_NS).into(),
            ("mc:Ignorable", "xr").into(),
            ("xmlns:xr", SHEET_MS_REVISION_NS).into(),
        ];

        if self.name.has_value() {
            attributes.push(("name", self.name.value_str()).into());
        }
        let cache_id_str = self.cache_id.value_string();
        if self.cache_id.has_value() {
            attributes.push(("cacheId", &cache_id_str).into());
        }
        if self.apply_number_formats.has_value() {
            attributes.push(
                (
                    "applyNumberFormats",
                    self.apply_number_formats.value_string(),
                )
                    .into(),
            );
        }
        if self.apply_border_formats.has_value() {
            attributes.push(
                (
                    "applyBorderFormats",
                    self.apply_border_formats.value_string(),
                )
                    .into(),
            );
        }
        if self.apply_font_formats.has_value() {
            attributes.push(
                (
                    "applyFontFormats",
                    self.apply_font_formats.value_string(),
                )
                    .into(),
            );
        }
        if self.apply_pattern_formats.has_value() {
            attributes.push(
                (
                    "applyPatternFormats",
                    self.apply_pattern_formats.value_string(),
                )
                    .into(),
            );
        }
        if self.apply_alignment_formats.has_value() {
            attributes.push(
                (
                    "applyAlignmentFormats",
                    self.apply_alignment_formats.value_string(),
                )
                    .into(),
            );
        }
        if self.apply_width_height_formats.has_value() {
            attributes.push(
                (
                    "applyWidthHeightFormats",
                    self.apply_width_height_formats.value_string(),
                )
                    .into(),
            );
        }
        if self.data_caption.has_value() {
            attributes.push(("dataCaption", self.data_caption.value_str()).into());
        }
        let updated_version_str = self.updated_version.value_string();
        if self.updated_version.has_value() {
            attributes.push(("updatedVersion", &updated_version_str).into());
        }
        let min_refreshable_version_str = self.min_refreshable_version.value_string();
        if self.min_refreshable_version.has_value() {
            attributes.push(
                (
                    "minRefreshableVersion",
                    &min_refreshable_version_str,
                )
                    .into(),
            );
        }
        if self.use_auto_formatting.has_value() {
            attributes.push(
                (
                    "useAutoFormatting",
                    self.use_auto_formatting.value_string(),
                )
                    .into(),
            );
        }
        if self.item_print_titles.has_value() {
            attributes.push(("itemPrintTitles", self.item_print_titles.value_string()).into());
        }
        let created_version_str = self.created_version.value_string();
        if self.created_version.has_value() {
            attributes.push(("createdVersion", &created_version_str).into());
        }
        let indent_str = self.indent.value_string();
        if self.indent.has_value() {
            attributes.push(("indent", &indent_str).into());
        }
        if self.outline.has_value() {
            attributes.push(("outline", self.outline.value_string()).into());
        }
        if self.outline_data.has_value() {
            attributes.push(("outlineData", self.outline_data.value_string()).into());
        }
        if self.multiple_field_filters.has_value() {
            attributes.push(
                (
                    "multipleFieldFilters",
                    self.multiple_field_filters.value_string(),
                )
                    .into(),
            );
        }
        write_start_tag(writer, "pivotTableDefinition", attributes, false);

        // location
        self.location.write_to(writer);

        // pivotFields
        self.pivot_fields.write_to(writer);

        // rowItems
        self.row_items.write_to(writer);

        // colFields
        self.column_fields.write_to(writer);

        // colItems
        self.column_items.write_to(writer);

        // dataFields
        self.data_fields.write_to(writer);

        // pivotTableStyleInfo
        self.pivot_table_style.write_to(writer);

        write_end_tag(writer, "pivotTableDefinition");
    }
}
