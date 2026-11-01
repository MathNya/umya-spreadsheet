use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    BooleanValue,
    CalculateModeValues,
    DoubleValue,
    EnumValue,
    ReferenceModeValues,
    UInt32Value,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

/// Calculation properties of a workbook (`calcPr`).
///
/// Attributes that are not set are not written, so Excel uses its defaults
/// for them.
#[derive(Clone, Default, Debug)]
pub struct CalculationProperties {
    calc_mode:               EnumValue<CalculateModeValues>,
    full_calc_on_load:       BooleanValue,
    ref_mode:                EnumValue<ReferenceModeValues>,
    iterate:                 BooleanValue,
    iterate_count:           UInt32Value,
    iterate_delta:           DoubleValue,
    full_precision:          BooleanValue,
    calc_completed:          BooleanValue,
    calc_on_save:            BooleanValue,
    concurrent_calc:         BooleanValue,
    concurrent_manual_count: UInt32Value,
    force_full_calc:         BooleanValue,
}
impl CalculationProperties {
    // Excel compares calcId with the version of its calculation engine and
    // recalculates the whole workbook on load when the file is older. The
    // cached values of formulas are not updated when cells are changed, so
    // this old id is always written to make Excel recalculate them.
    const CALC_ID: &'static str = "122211";

    /// Calculation mode (`calcMode`). Defaults to `Auto`.
    #[inline]
    #[must_use]
    pub fn calc_mode(&self) -> &CalculateModeValues {
        self.calc_mode.value()
    }

    #[inline]
    pub fn set_calc_mode(&mut self, value: CalculateModeValues) -> &mut Self {
        self.calc_mode.set_value(value);
        self
    }

    /// Whether the workbook is fully recalculated when it is opened
    /// (`fullCalcOnLoad`).
    #[inline]
    #[must_use]
    pub fn full_calc_on_load(&self) -> bool {
        self.full_calc_on_load.value()
    }

    #[inline]
    pub fn set_full_calc_on_load(&mut self, value: bool) -> &mut Self {
        self.full_calc_on_load.set_value(value);
        self
    }

    /// Reference style used in formulas (`refMode`). Defaults to `A1`.
    #[inline]
    #[must_use]
    pub fn ref_mode(&self) -> &ReferenceModeValues {
        self.ref_mode.value()
    }

    #[inline]
    pub fn set_ref_mode(&mut self, value: ReferenceModeValues) -> &mut Self {
        self.ref_mode.set_value(value);
        self
    }

    /// Whether iterative calculation is enabled (`iterate`).
    #[inline]
    #[must_use]
    pub fn iterate(&self) -> bool {
        self.iterate.value()
    }

    #[inline]
    pub fn set_iterate(&mut self, value: bool) -> &mut Self {
        self.iterate.set_value(value);
        self
    }

    /// Maximum number of iterations (`iterateCount`). Defaults to 100.
    #[inline]
    #[must_use]
    pub fn iterate_count(&self) -> u32 {
        if self.iterate_count.has_value() {
            self.iterate_count.value()
        } else {
            100
        }
    }

    #[inline]
    pub fn set_iterate_count(&mut self, value: u32) -> &mut Self {
        self.iterate_count.set_value(value);
        self
    }

    /// Maximum change between iterations (`iterateDelta`). Defaults to
    /// 0.001.
    #[inline]
    #[must_use]
    pub fn iterate_delta(&self) -> f64 {
        if self.iterate_delta.has_value() {
            self.iterate_delta.value()
        } else {
            0.001
        }
    }

    #[inline]
    pub fn set_iterate_delta(&mut self, value: f64) -> &mut Self {
        self.iterate_delta.set_value(value);
        self
    }

    /// Whether calculations use full precision (`fullPrecision`). Defaults
    /// to `true`.
    #[inline]
    #[must_use]
    pub fn full_precision(&self) -> bool {
        !self.full_precision.has_value() || self.full_precision.value()
    }

    #[inline]
    pub fn set_full_precision(&mut self, value: bool) -> &mut Self {
        self.full_precision.set_value(value);
        self
    }

    /// Whether the last calculation was completed (`calcCompleted`).
    /// Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn calc_completed(&self) -> bool {
        !self.calc_completed.has_value() || self.calc_completed.value()
    }

    #[inline]
    pub fn set_calc_completed(&mut self, value: bool) -> &mut Self {
        self.calc_completed.set_value(value);
        self
    }

    /// Whether the workbook is recalculated before it is saved
    /// (`calcOnSave`). Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn calc_on_save(&self) -> bool {
        !self.calc_on_save.has_value() || self.calc_on_save.value()
    }

    #[inline]
    pub fn set_calc_on_save(&mut self, value: bool) -> &mut Self {
        self.calc_on_save.set_value(value);
        self
    }

    /// Whether calculation uses multiple threads (`concurrentCalc`).
    /// Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn concurrent_calc(&self) -> bool {
        !self.concurrent_calc.has_value() || self.concurrent_calc.value()
    }

    #[inline]
    pub fn set_concurrent_calc(&mut self, value: bool) -> &mut Self {
        self.concurrent_calc.set_value(value);
        self
    }

    /// Number of calculation threads set by the user
    /// (`concurrentManualCount`).
    #[inline]
    #[must_use]
    pub fn concurrent_manual_count(&self) -> Option<u32> {
        self.concurrent_manual_count
            .has_value()
            .then(|| self.concurrent_manual_count.value())
    }

    #[inline]
    pub fn set_concurrent_manual_count(&mut self, value: u32) -> &mut Self {
        self.concurrent_manual_count.set_value(value);
        self
    }

    /// Whether every formula is calculated on each calculation
    /// (`forceFullCalc`).
    #[inline]
    #[must_use]
    pub fn force_full_calc(&self) -> bool {
        self.force_full_calc.value()
    }

    #[inline]
    pub fn set_force_full_calc(&mut self, value: bool) -> &mut Self {
        self.force_full_calc.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, calc_mode, "calcMode");
        set_string_from_xml!(self, e, full_calc_on_load, "fullCalcOnLoad");
        set_string_from_xml!(self, e, ref_mode, "refMode");
        set_string_from_xml!(self, e, iterate, "iterate");
        set_string_from_xml!(self, e, iterate_count, "iterateCount");
        set_string_from_xml!(self, e, iterate_delta, "iterateDelta");
        set_string_from_xml!(self, e, full_precision, "fullPrecision");
        set_string_from_xml!(self, e, calc_completed, "calcCompleted");
        set_string_from_xml!(self, e, calc_on_save, "calcOnSave");
        set_string_from_xml!(self, e, concurrent_calc, "concurrentCalc");
        set_string_from_xml!(
            self,
            e,
            concurrent_manual_count,
            "concurrentManualCount"
        );
        set_string_from_xml!(self, e, force_full_calc, "forceFullCalc");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        attributes.push(("calcId", Self::CALC_ID).into());
        if self.calc_mode.has_value() {
            attributes.push(("calcMode", self.calc_mode.value_string()).into());
        }
        if self.full_calc_on_load.has_value() {
            attributes.push(("fullCalcOnLoad", self.full_calc_on_load.value_string()).into());
        }
        if self.ref_mode.has_value() {
            attributes.push(("refMode", self.ref_mode.value_string()).into());
        }
        if self.iterate.has_value() {
            attributes.push(("iterate", self.iterate.value_string()).into());
        }
        let iterate_count = self.iterate_count.value_string();
        if self.iterate_count.has_value() {
            attributes.push(("iterateCount", &iterate_count).into());
        }
        let iterate_delta = self.iterate_delta.value_string();
        if self.iterate_delta.has_value() {
            attributes.push(("iterateDelta", &iterate_delta).into());
        }
        if self.full_precision.has_value() {
            attributes.push(("fullPrecision", self.full_precision.value_string()).into());
        }
        if self.calc_completed.has_value() {
            attributes.push(("calcCompleted", self.calc_completed.value_string()).into());
        }
        if self.calc_on_save.has_value() {
            attributes.push(("calcOnSave", self.calc_on_save.value_string()).into());
        }
        if self.concurrent_calc.has_value() {
            attributes.push(("concurrentCalc", self.concurrent_calc.value_string()).into());
        }
        let concurrent_manual_count = self.concurrent_manual_count.value_string();
        if self.concurrent_manual_count.has_value() {
            attributes.push(("concurrentManualCount", &concurrent_manual_count).into());
        }
        if self.force_full_calc.has_value() {
            attributes.push(("forceFullCalc", self.force_full_calc.value_string()).into());
        }

        // calcPr
        write_start_tag(writer, "calcPr", attributes, true);
    }
}
