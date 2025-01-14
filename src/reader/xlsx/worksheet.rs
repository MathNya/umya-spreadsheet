use std::collections::HashMap;

use quick_xml::{events::Event, Reader};

use super::{
    driver::{get_attribute, get_attribute_value, xml_read_loop},
    XlsxError,
};
use crate::{
    helper::formula::FormulaToken,
    structs::{
        office2010::excel::DataValidations as DataValidations2010,
        raw::{RawRelationships, RawWorksheet},
        Cells, Columns, ConditionalFormatting, DataValidations, Hyperlink, OleObjects, Row,
        SharedStringTable, SheetProtection, Stylesheet, Worksheet,
    },
};

pub(crate) fn read(
    worksheet: &mut Worksheet,
    raw_data_of_worksheet: &RawWorksheet,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
) -> Result<(), XlsxError> {
    let data = std::io::Cursor::new(raw_data_of_worksheet.get_worksheet_file().get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(true);
    let mut formula_shared_list: HashMap<u32, (String, Vec<FormulaToken>)> = HashMap::new();
    xml_read_loop!(
        reader,
        Event::Start(ref e) => match e.name().into_inner() {
            b"sheetPr" => {
                for a in e.attributes().with_checks(false) {
                    match a {
                        Ok(ref attr) if attr.key.0 == b"codeName" => {
                            worksheet.set_code_name(get_attribute_value(attr)?);
                        }
                        Ok(_) | Err(_) => {}
                    }
                }
            }
            b"sheetViews" => {
                worksheet
                    .get_sheet_views_mut()
                    .set_attributes(&mut reader, e);
            }
            b"sheetFormatPr" => {
                worksheet
                    .get_sheet_format_properties_mut()
                    .set_attributes(&mut reader, e);
            }
            b"selection" => {
                for a in e.attributes().with_checks(false) {
                    match a {
                        Ok(ref attr) if attr.key.0 == b"activeCell" => {
                            worksheet.set_active_cell(get_attribute_value(attr)?);
                        }
                        Ok(_) | Err(_) => {}
                    }
                }
            }
            b"row" => {
                let mut obj = Row::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    worksheet.get_cells_crate_mut(),
                    shared_string_table,
                    stylesheet,
                    &mut formula_shared_list,
                    false,
                );
                worksheet.set_row_dimension(obj);
            }
            b"autoFilter" => {
                worksheet.set_auto_filter(get_attribute(e, b"ref").unwrap());
            }
            b"cols" => {
                let mut obj = Columns::default();
                obj.set_attributes(&mut reader, e, stylesheet);
                worksheet.set_column_dimensions_crate(obj);
            }
            b"mergeCells" => {
                worksheet
                    .get_merge_cells_crate_mut()
                    .set_attributes(&mut reader, e);
            }
            b"conditionalFormatting" => {
                let mut obj = ConditionalFormatting::default();
                obj.set_attributes(&mut reader, e, stylesheet.get_differential_formats());
                worksheet.add_conditional_formatting_collection(obj);
            }
            b"dataValidations" => {
                let mut obj = DataValidations::default();
                obj.set_attributes(&mut reader, e);
                worksheet.set_data_validations(obj);
            }
            b"x14:dataValidations" => {
                let mut obj = DataValidations2010::default();
                obj.set_attributes(&mut reader, e);
                worksheet.set_data_validations_2010(obj);
            }
            b"oleObjects" => {
                let mut obj = OleObjects::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    raw_data_of_worksheet.get_worksheet_relationships().unwrap(),
                );
                worksheet.set_ole_objects(obj);
            }
            b"headerFooter" => {
                worksheet
                    .get_header_footer_mut()
                    .set_attributes(&mut reader, e);
            }
            b"rowBreaks" => {
                worksheet
                    .get_row_breaks_mut()
                    .set_attributes(&mut reader, e);
            }
            b"colBreaks" => {
                worksheet
                    .get_column_breaks_mut()
                    .set_attributes(&mut reader, e);
            }
            _ => (),
        },
        Event::Empty(ref e) => match e.name().into_inner() {
            b"sheetPr" => {
                for a in e.attributes().with_checks(false) {
                    match a {
                        Ok(ref attr) if attr.key.0 == b"codeName" => {
                            worksheet.set_code_name(get_attribute_value(attr)?);
                        }
                        Ok(_) | Err(_) => {}
                    }
                }
            }
            b"tabColor" => {
                worksheet
                    .get_tab_color_mut()
                    .set_attributes(&mut reader, e, true);
            }
            b"sheetFormatPr" => {
                worksheet
                    .get_sheet_format_properties_mut()
                    .set_attributes(&mut reader, e);
            }
            b"selection" => {
                for a in e.attributes().with_checks(false) {
                    match a {
                        Ok(ref attr) if attr.key.0 == b"activeCell" => {
                            worksheet.set_active_cell(get_attribute_value(attr)?);
                        }
                        Ok(_) | Err(_) => {}
                    }
                }
            }
            b"row" => {
                let mut obj = Row::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    worksheet.get_cells_crate_mut(),
                    shared_string_table,
                    stylesheet,
                    &mut formula_shared_list,
                    true,
                );
                worksheet.set_row_dimension(obj);
            }
            b"autoFilter" => {
                worksheet.set_auto_filter(get_attribute(e, b"ref").unwrap());
            }
            b"pageMargins" => {
                worksheet
                    .get_page_margins_mut()
                    .set_attributes(&mut reader, e);
            }
            b"hyperlink" => {
                let (coor, hyperlink) = get_hyperlink(
                    e,
                    raw_data_of_worksheet.get_worksheet_relationships()
                );
                worksheet.get_cell_mut(coor).set_hyperlink(hyperlink);
            }
            b"printOptions" => {
                worksheet
                    .get_print_options_mut()
                    .set_attributes(&mut reader, e);
            }
            b"pageSetup" => {
                worksheet.get_page_setup_mut().set_attributes(
                    &mut reader,
                    e,
                    raw_data_of_worksheet.get_worksheet_relationships(),
                );
            }
            b"sheetProtection" => {
                let mut obj = SheetProtection::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                );
                worksheet.set_sheet_protection(obj);
            }
            _ => (),
        },
        Event::Eof => break,
    );

    Ok(())
}

pub(crate) fn read_lite(
    raw_data_of_worksheet: &RawWorksheet,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
) -> Cells {
    let data = std::io::Cursor::new(raw_data_of_worksheet.get_worksheet_file().get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(true);

    let mut cells = Cells::default();
    let mut formula_shared_list: HashMap<u32, (String, Vec<FormulaToken>)> = HashMap::new();
    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"row" {
                let mut obj = Row::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    &mut cells,
                    shared_string_table,
                    stylesheet,
                    &mut formula_shared_list,
                    false,
                );
            }
        },
        Event::Empty(ref e) => {
            if e.name().into_inner() == b"row" {
                let mut obj = Row::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    &mut cells,
                    shared_string_table,
                    stylesheet,
                    &mut formula_shared_list,
                    true,
                );
            }
        },
        Event::Eof => break,
    );

    cells
}

fn get_hyperlink(
    e: &quick_xml::events::BytesStart<'_>,
    raw_relationships: Option<&RawRelationships>,
) -> (String, Hyperlink) {
    let mut hyperlink = Hyperlink::default();

    let coordition = get_attribute(e, b"ref").unwrap_or_default();
    if let Some(v) = get_attribute(e, b"location") {
        hyperlink.set_url(v);
        hyperlink.set_location(true);
    }
    if let Some(v) = get_attribute(e, b"r:id") {
        let relationship = raw_relationships.unwrap().get_relationship_by_rid(&v);
        hyperlink.set_url(relationship.get_target());
    }
    (coordition, hyperlink)
}
