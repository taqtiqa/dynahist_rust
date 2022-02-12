// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::bins::abstract_bin::AbstractBin;
use crate::bins::bin::BinSketch;
use crate::bins::bin_iterator::BinIterator;
use crate::errors::DynaHistError;
use crate::histograms::abstract_histogram::AbstractHistogram;
use crate::histograms::abstract_histogram::ValueIterator;
use crate::histograms::abstract_histogram::ValueSketch;
// use crate::histograms::fixed::StaticHistogram;
use crate::histograms::histogram::Histogram;
// use crate::histograms::preprocessed_histogram::PreprocessedHistogram;
use crate::layouts::layout::Layout;
use crate::quantiles::quantile_estimation::QuantileEstimation;
// use crate::quantiles::quantile_estimators::QuantileEstimator;
use crate::seriate::SeriateUtil;
use crate::sketches::data::{DataInput, DataOutput};
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::values::value_estimation::ValueEstimation;
// use crate::values::value_estimators::ValueEstimatorUniform;

// pub struct Self {
//     underflow_count: i64,
//     overflow_count: i64,
//     total_count: i64,
//     min: f64,
//     max: f64,
// }

// impl Default for Self {
//     fn default() -> Self {
//         Self {
//     underflow_count: 0,
//     overflow_count: 0,
//     total_count: 0,
//     min: f64::NEG_INFINITY,
//     max: f64::INFINITY,
//         }
//     }
// }

struct BinCopyImpl {
    bin_count: i64,
    less_count: i64,
    greater_count: i64,
    bin_index: i32,
}

impl AbstractBin for BinCopyImpl {}
impl BinSketch for BinCopyImpl {}

impl BinCopyImpl {
    fn new(bin_count: i64, less_count: i64, greater_count: i64, bin_index: i32) -> BinCopyImpl {
        bin_count;
        less_count;
        greater_count;
        bin_index;
    }

    fn get_histogram(&self) -> impl Histogram {
        return self.histogram;
    }

    fn get_bin_count(&self) -> i64 {
        return self.bin_count;
    }

    fn get_less_count(&self) -> i64 {
        return self.less_count;
    }

    fn get_greater_count(&self) -> i64 {
        return self.greater_count;
    }

    fn get_bin_index(&self) -> i32 {
        return self.bin_index;
    }
}

pub struct BinIteratorImpl {
    bin_index: i32,
    less_count: i64,
    greater_count: i64,
    count: i64,
}

impl BinIteratorImpl {}

impl BinIteratorImpl {
    fn new(bin_index: i32, less_count: i64, greater_count: i64, count: i64) -> BinIteratorImpl {
        bin_index;
        less_count;
        greater_count;
        count;
    }

    fn get_bin_count(&self) -> i64 {
        return self.count;
    }

    fn get_less_count(&self) -> i64 {
        return self.less_count;
    }

    fn get_greater_count(&self) -> i64 {
        return self.greater_count;
    }

    fn next(&self) {
        if self.greater_count <= 0 {
            return Err(DynaHistError::NoSuchElementError);
        }
        self.less_count += self.count;
        if self.greater_count != self.get_overflow_count() {
            if self.bin_index == Self::get_layout().get_underflow_bin_index() {
                self.bin_index = self.min_allocated_bin_index_inclusive() - 1;
            }
            loop {
                {
                    self.bin_index += 1;
                    self.count = self.get_allocated_bin_count(self.bin_index);
                }
                if !(self.count == 0) {
                    break;
                }
            }
            self.greater_count -= self.count;
        } else {
            self.bin_index = Self::get_layout().get_overflow_bin_index();
            self.count = self.greater_count;
            self.greater_count = 0;
        }
    }

    fn previous(&self) {
        if self.less_count <= 0 {
            return Err(DynaHistError::NoSuchElementError);
        }
        self.greater_count += self.count;
        if self.less_count != self.get_underflow_count() {
            if self.bin_index == Self::get_layout().get_overflow_bin_index() {
                self.bin_index = self.max_allocated_bin_index_exclusive();
            }
            loop {
                {
                    self.bin_index -= 1;
                    self.count = self.get_allocated_bin_count(self.bin_index);
                }
                if !(self.count == 0) {
                    break;
                }
            }
            self.less_count -= self.count;
        } else {
            self.bin_index = Self::get_layout().get_underflow_bin_index();
            self.count = self.less_count;
            self.less_count = 0;
        }
    }

    fn get_bin_copy(&self) -> dyn BinSketch {
        return BinCopyImpl::new(
            self.count,
            self.less_count,
            self.greater_count,
            self.bin_index,
        );
    }

    fn get_bin_index(&self) -> i32 {
        return self.bin_index;
    }

    fn get_histogram(&self) -> impl Histogram {
        return self.histogram;
    }
}

pub trait AbstractMutableHistogram: AbstractHistogram + Histogram + Preconditions {
    type BIter: BinIterator + BinSketch + Iterator;
    type VIter: ValueIterator + ValueSketch + Iterator;

    fn new(layout: impl Layout) -> Self {
        Default::default();
    }

    // accessor methods for struct fields
    fn increment_underflow_count(&self, count: i64) {
        self.underflow_count += count;
    }

    fn increment_overflow_count(&self, count: i64) {
        self.overflow_count += count;
    }

    fn increment_total_count(&self, count: i64) {
        self.total_count += count;
    }

    fn update_min_max(&self, value: f64) {
        self.updates_min_max(value, value);
    }

    fn updates_min_max(&self, min: f64, max: f64) {
        if min <= self.min && (min < self.min || (min.to_bits() == 0x8000000000000000)) {
            self.min = min;
        }
        if max >= self.max && (max > self.max || (max.to_bits() == 0x0000000000000000)) {
            self.max = max;
        }
    }

    fn add_histogram_from_estimator(
        &self,
        histogram: Self::H,
        value_estimator: impl ValueEstimation,
    ) -> Self::H {
        if histogram.is_empty() {
            return self;
        }
        if histogram.get_total_count() > i64::MAX - self.get_total_count() {
            return Err(DynaHistError::ArithmeticError(&Self::OVERFLOW_MSG));
        }
        let layout: Self::L = histogram.get_layout();
        if Self::get_layout().equals(layout) {
            let first_non_empty_bin: Self::BIter = histogram.get_first_non_empty_bin();
            let last_non_empty_bin: Self::BIter = histogram.get_last_non_empty_bin();
            if first_non_empty_bin.get_bin_index() == last_non_empty_bin.get_bin_index() {
                Self::add_values(&histogram.get_min(), histogram.get_total_count() - 1);
                Self::add_value(&histogram.get_max());
            } else {
                Self::add_values(
                    &last_non_empty_bin.get_upper_bound(),
                    &last_non_empty_bin.get_bin_count(),
                );
                {
                    let bin_iterator: Self::BIter = first_non_empty_bin;
                    while !bin_iterator.is_last_non_empty_bin() {
                        {
                            Self::add_values(
                                &bin_iterator.get_lower_bound(),
                                &bin_iterator.get_bin_count(),
                            );
                        }
                        bin_iterator.next();
                    }
                }
            }
            return self;
        } else {
            // preprocess histogram to get a copy that allows faster random access to
            // approximated values
            let preprocessed_histogram: Self::H = histogram.get_preprocessed_copy();
            let ascending_sequence =
                |&rank| preprocessed_histogram.get_value_from_estimator(rank, value_estimator);
            return self.add_ascending_sequence(
                ascending_sequence,
                preprocessed_histogram.get_total_count(),
            );
        }
    }

    fn get_estimated_footprint_in_bytes(&self) -> i64 {
        return // underFlowCount, overFlowCount, totalCount
        3 * usize::to_be_bytes() + // min, max
        2 * f64::to_be_bytes() + Self::get_estimated_footprint_in_bytes();
    }

    /// Return value must be greater than or equal to [`#maxAllocatedBinIndexExclusive()`] if
    /// histogram is empty.
    ///
    fn min_allocated_bin_index_inclusive(&self) -> i32;

    /// Return value must be less than or equal to [`#minAllocatedBinIndexInclusive()`] if
    /// histogram is empty.
    ///
    fn max_allocated_bin_index_exclusive(&self) -> i32;

    fn get_allocated_bin_count(&self, bin_index: i32) -> i64;

    fn get_count(&self, bin_index: i32) -> i64 {
        if bin_index <= Self::get_layout().get_underflow_bin_index() {
            return self.get_underflow_count();
        } else if bin_index < self.min_allocated_bin_index_inclusive() {
            return 0;
        } else if bin_index < self.max_allocated_bin_index_exclusive() {
            return self.get_allocated_bin_count(bin_index);
        } else if bin_index < Self::get_layout().get_overflow_bin_index() {
            return 0;
        } else {
            return self.get_overflow_count();
        }
    }

    fn determine_required_mode(value: i64) -> i8 {
        if value > 0xFFFFFFFF {
            return 6;
        } else if value > 0xFFFF {
            return 5;
        } else if value > 0xFF {
            return 4;
        } else if value > 0xF {
            return 3;
        } else if value > 0x3 {
            return 2;
        } else if value > 0x1 {
            return 1;
        } else {
            return 0;
        }
    }

    fn write(&self, data_output: &DataOutput) -> Result<(), std::rc::Rc<DynaHistError>> {
        // 0. write serial version and mode
        data_output.write_byte(Self::SERIAL_VERSION_V0);
        if self.total_count <= 1 {
            // special mode
            if self.is_empty() {
                let info_byte: i32 = 0x00;
                data_output.write_byte(info_byte);
            } else {
                let info_byte: i32 = 0x08;
                data_output.write_byte(info_byte);
                data_output.write_double(self.min);
            }
            return;
        }
        let layout: Self::L = Self::get_layout();
        // since the minimum and maximum values are explicitly serialized, we can drop
        // them from the corresponding bins, which reduces
        // the corresponding counts, the "effective" bin counts represent the bin counts
        // after removing the minimum and the maximum
        let effective_under_flow_count: i64 = self.underflow_count
            - (if self.underflow_count > 0 { 1 } else { 0 })
            - (if self.underflow_count == self.total_count {
                1
            } else {
                0
            });
        let effective_over_flow_count: i64 = self.overflow_count
            - (if self.overflow_count > 0 { 1 } else { 0 })
            - (if self.overflow_count == self.total_count {
                1
            } else {
                0
            });
        let effective_total_count: i64 = self.total_count - 2;
        let effective_regular_total_count: i64 = effective_total_count - effective_under_flow_count - // effective count in normal range
        effective_over_flow_count;
        let mode: i8 = self.get_mode();
        // Upstream (Java) Double::compare(self.min, self.max) < 0
        // This method returns the
        // - value 0 if min is numerically equal to max;
        // - a value less than 0 if min is numerically less than max;
        // - value greater than 0 if min is numerically greater than max.
        let is_min_smaller_than_max: bool = self.min < self.max;
        // 1. write info byte
        let info_byte: i32 = 0;
        info_byte = mode + 1;
        if is_min_smaller_than_max {
            // bit 4
            info_byte |= 0x08;
        }
        // bit 5 and 6
        info_byte |= (std::cmp::min(effective_regular_total_count, 3) as i32) << 4;
        if effective_under_flow_count > 0 {
            // bit 7
            info_byte |= 0x40;
        }
        if effective_over_flow_count > 0 {
            // bit 8
            info_byte |= 0x80;
        }
        data_output.write_byte(info_byte);
        // 2. write minimum and maximum, if necessary
        data_output.write_double(self.min);
        if is_min_smaller_than_max {
            data_output.write_double(self.max);
        }
        // 3. write effective under and over flow counts, if necessary
        if effective_under_flow_count >= 1 {
            Self::write_unsigned_var_long(effective_under_flow_count - 1, &data_output);
        }
        if effective_over_flow_count >= 1 {
            Self::write_unsigned_var_long(effective_over_flow_count - 1, &data_output);
        }
        if effective_regular_total_count >= 1 {
            let min_bin_index: i32 = layout.map_to_bin_index(self.min);
            let max_bin_index: i32 = layout.map_to_bin_index(self.max);
            // 4. write first regular effectively non-zero bin index
            let first_regular_effectively_non_zero_bin_index: i32 =
                i32::max(&self.min_allocated_bin_index_inclusive(), min_bin_index);
            while self.get_allocated_bin_count(first_regular_effectively_non_zero_bin_index)
                - (if min_bin_index == first_regular_effectively_non_zero_bin_index {
                    1
                } else {
                    0
                })
                - (if max_bin_index == first_regular_effectively_non_zero_bin_index {
                    1
                } else {
                    0
                })
                == 0
            {
                first_regular_effectively_non_zero_bin_index += 1;
            }
            Self::write_signed_var_int(first_regular_effectively_non_zero_bin_index, &data_output);
            if effective_regular_total_count >= 2 {
                // 5. write first regular effectively non-zero bin index
                let last_regular_effectively_non_zero_bin_index: i32 =
                    std::cmp::min(self.max_allocated_bin_index_exclusive() - 1, max_bin_index);
                while self.get_allocated_bin_count(last_regular_effectively_non_zero_bin_index)
                    - (if min_bin_index == last_regular_effectively_non_zero_bin_index {
                        1
                    } else {
                        0
                    })
                    - (if max_bin_index == last_regular_effectively_non_zero_bin_index {
                        1
                    } else {
                        0
                    })
                    == 0
                {
                    last_regular_effectively_non_zero_bin_index -= 1;
                }
                Self::write_signed_var_int(
                    last_regular_effectively_non_zero_bin_index,
                    &data_output,
                );
                if effective_regular_total_count >= 3 {
                    // lastRegularEffectivelyNonZeroBinIndex
                    if mode <= 2 {
                        let counts_per_byte: i32 = 1 << (3 - mode);
                        let bits_per_count: i32 = 1 << mode;
                        let bit_mask: i32 = (1 << bits_per_count) - 1;
                        let bin_index: i32 = first_regular_effectively_non_zero_bin_index;
                        while bin_index <= last_regular_effectively_non_zero_bin_index {
                            let mut b: i32 = 0;
                            {
                                let mut i: i32 = 0;
                                while i < counts_per_byte {
                                    {
                                        b <<= bits_per_count;
                                        if bin_index <= last_regular_effectively_non_zero_bin_index
                                        {
                                            let bin_count: i64 = self
                                                .get_allocated_bin_count(bin_index)
                                                - (if min_bin_index == bin_index { 1 } else { 0 })
                                                - (if max_bin_index == bin_index { 1 } else { 0 });
                                            bin_index += 1;
                                            b |= (bin_count as i32) & bit_mask;
                                        }
                                    }
                                    i += 1;
                                }
                            }

                            data_output.write_byte(b);
                        }
                    } else {
                        let byte_per_count: i32 = 1 << (mode - 3);
                        let bin_index: i32 = first_regular_effectively_non_zero_bin_index;
                        while bin_index <= last_regular_effectively_non_zero_bin_index {
                            let bin_count: i64 = self.get_allocated_bin_count(bin_index)
                                - (if min_bin_index == bin_index { 1 } else { 0 })
                                - (if max_bin_index == bin_index { 1 } else { 0 });
                            bin_index += 1;
                            {
                                let mut i: i32 = byte_per_count - 1;
                                while i >= 0 {
                                    {
                                        let b: i32 = (0xff & (bin_count >> (i << 3))) as i32;
                                        data_output.write_byte(b);
                                    }
                                    i -= 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Write a [`u64`] to the given [`DataOutput`] using variable-length
    /// encoding.
    ///
    /// # Errors
    ///
    /// Return [`DynaHistError::IOError`] if an I/O error occurs.
    ///
    fn write_unsigned_var_long(
        value: u64,
        data_output: &DataOutput,
    ) -> Result<(), std::rc::Rc<DynaHistError>> {
        while (value & 0xFFFFFFFFFFFFFF80_u64) != 0 {
            data_output.write_byte((value as i32 & 0x7F) | 0x80);
            value >>= /* >>>= */ 7;
        }
        data_output.write_byte(value as i32 & 0x7F);
        Ok(())
    }

    fn ensure_count_array(
        &self,
        min_non_empty_bin_index: i32,
        max_non_empty_bin_index: i32,
        mode: i8,
    );

    fn deserialize<T>(
        histogram: &T,
        data_input: &DataInput,
    ) -> Result<(), std::rc::Rc<DynaHistError>>
    where
        T: Histogram,
    {
        Self::check_argument(&histogram.is_empty());
        let layout: Self::L = histogram.get_layout();
        // 0. write serial version and mode
        SeriateUtil::check_serial_version(
            Self::SERIAL_VERSION_V0,
            &data_input.read_unsigned_byte(),
        );
        // 1. read info byte
        let info_byte: i32 = data_input.read_unsigned_byte();
        if (info_byte & 0x07) == 0 {
            // special mode
            if (info_byte & 0x08) > 0 {
                histogram.add_value(&data_input.read_double());
            }
            return;
        }
        let mode: i8 = ((info_byte & 0x07) - 1) as i8;
        let is_min_smaller_than_max: bool = (info_byte & 0x08) > 0;
        let effective_regular_total_count: i64 = (info_byte >> /* >>> */ 4) & 0x03;
        let effective_under_flow_count: i64 = (info_byte >> /* >>> */ 6) & 0x01;
        let effective_over_flow_count: i64 = (info_byte >> /* >>> */ 7) & 0x01;
        // 2. read minimum and maximum, if necessary
        let min: f64 = data_input.read_double();
        let min_bin_index: i32 = layout.map_to_bin_index(min);
        let mut max: f64;
        let max_bin_index: i32;
        if is_min_smaller_than_max {
            max = data_input.read_double();
            max_bin_index = layout.map_to_bin_index(max);
        } else {
            max = min;
            max_bin_index = min_bin_index;
        }
        // 3. read effective under and over flow counts, if necessary
        if effective_under_flow_count == 1 {
            effective_under_flow_count += SeriateUtil::read_unsigned_var_long(&data_input);
        }
        if effective_over_flow_count == 1 {
            effective_over_flow_count += SeriateUtil::read_unsigned_var_long(&data_input);
        }
        let total_count: i64 = 2 + effective_over_flow_count + effective_under_flow_count;
        if effective_regular_total_count >= 1 {
            // 4. read first regular effectively non-zero bin index
            let first_regular_effectively_non_zero_bin_index: i32 =
                SeriateUtil::read_signed_var_int(&data_input);
            let last_regular_effectively_non_zero_bin_index: i32;
            if effective_regular_total_count >= 2 {
                // 5. read first regular effectively non-zero bin index
                last_regular_effectively_non_zero_bin_index =
                    SeriateUtil::read_signed_var_int(&data_input);
            } else {
                last_regular_effectively_non_zero_bin_index =
                    first_regular_effectively_non_zero_bin_index;
            }
            if layout.get_underflow_bin_index() + 1 < layout.get_overflow_bin_index() {
                let min_allocated_bin_index_unclipped: i32;
                if min_bin_index <= layout.get_underflow_bin_index() {
                    min_allocated_bin_index_unclipped =
                        first_regular_effectively_non_zero_bin_index;
                } else {
                    min_allocated_bin_index_unclipped =
                        std::cmp::min(min_bin_index, first_regular_effectively_non_zero_bin_index);
                }
                let max_allocated_bin_index_unclipped: i32;
                if max_bin_index >= layout.get_overflow_bin_index() {
                    max_allocated_bin_index_unclipped = last_regular_effectively_non_zero_bin_index;
                } else {
                    max_allocated_bin_index_unclipped =
                        std::cmp::max(max_bin_index, last_regular_effectively_non_zero_bin_index);
                }
                let min_allocated_bin_index: i32 = Self::clip(
                    min_allocated_bin_index_unclipped,
                    layout.get_underflow_bin_index() + 1,
                    layout.get_overflow_bin_index() - 1,
                );
                let max_allocated_bin_index: i32 = Self::clip(
                    max_allocated_bin_index_unclipped,
                    layout.get_underflow_bin_index() + 1,
                    layout.get_overflow_bin_index() - 1,
                );
                histogram.ensure_count_array(
                    min_allocated_bin_index,
                    max_allocated_bin_index,
                    mode,
                );
            }
            if effective_regular_total_count >= 3 {
                if mode <= 2 {
                    let bits_per_count: i32 = 1 << mode;
                    let bit_mask: i32 = (1 << bits_per_count) - 1;
                    let available_bit_count: i32 = 0;
                    let read_bits: i32 = 0;
                    {
                        let bin_index: i32 = first_regular_effectively_non_zero_bin_index;
                        while bin_index <= last_regular_effectively_non_zero_bin_index {
                            {
                                if available_bit_count == 0 {
                                    read_bits = data_input.read_unsigned_byte();
                                    available_bit_count = 8;
                                }
                                available_bit_count -= bits_per_count;
                                let bin_count: i64 =
                                    (read_bits >> /* >>> */ available_bit_count) & bit_mask;
                                histogram.increase_count(bin_index, bin_count);
                                total_count += bin_count;
                            }
                            bin_index += 1;
                        }
                    }
                } else {
                    let byte_per_count: i32 = 1 << (mode - 3);
                    {
                        let bin_index: i32 = first_regular_effectively_non_zero_bin_index;
                        while bin_index <= last_regular_effectively_non_zero_bin_index {
                            {
                                let bin_count: i64 = 0;
                                {
                                    let mut i: i32 = 0;
                                    while i < byte_per_count {
                                        {
                                            bin_count <<= 8;
                                            bin_count += data_input.read_unsigned_byte();
                                        }
                                        i += 1;
                                    }
                                }

                                histogram.increase_count(bin_index, bin_count);
                                total_count += bin_count;
                            }
                            bin_index += 1;
                        }
                    }
                }
            } else {
                histogram.increase_count(first_regular_effectively_non_zero_bin_index, 1);
                total_count += 1;
                if effective_regular_total_count == 2 {
                    histogram.increase_count(last_regular_effectively_non_zero_bin_index, 1);
                    total_count += 1;
                }
            }
        }
        histogram.updates_min_max(min, max);
        histogram.increase_count(min_bin_index, 1);
        histogram.increase_count(max_bin_index, 1);
        histogram.increment_underflow_count(effective_under_flow_count);
        histogram.increment_overflow_count(effective_over_flow_count);
        histogram.increment_total_count(total_count);
    }

    fn increase_count(&self, absolute_index: i32, count: i64);

    fn get_first_non_empty_bin(&self) -> Self::BIter {
        if self.is_empty() {
            return Err(DynaHistError::NoSuchElementError);
        }
        let absolute_index: i32;
        let less_count: i64 = 0;
        let greater_count: i64;
        let mut count: i64;
        if self.get_underflow_count() > 0 {
            absolute_index = Self::get_layout().get_underflow_bin_index();
            count = self.get_underflow_count();
            greater_count = self.get_total_count() - self.get_underflow_count();
        } else if self.get_overflow_count() == self.get_total_count() {
            absolute_index = Self::get_layout().get_overflow_bin_index();
            count = self.get_overflow_count();
            greater_count = 0;
        } else {
            let mut c: i64;
            let mut idx: i32 = self.min_allocated_bin_index_inclusive() - 1;
            loop {
                {
                    idx += 1;
                    c = self.get_allocated_bin_count(idx);
                }
                if !(c == 0) {
                    break;
                }
            }
            absolute_index = idx;
            count = c;
            greater_count = self.get_total_count() - c;
        }
        return BinIteratorImpl::new(absolute_index, less_count, greater_count, count);
    }

    fn get_last_non_empty_bin(&self) -> Self::BIter {
        if self.is_empty() {
            return Err(DynaHistError::NoSuchElementError);
        }
        let absolute_index: i32;
        let less_count: i64;
        let greater_count: i64 = 0;
        let mut count: i64;
        if self.get_overflow_count() > 0 {
            absolute_index = Self::get_layout().get_overflow_bin_index();
            count = self.get_overflow_count();
            less_count = self.get_total_count() - self.get_overflow_count();
        } else if self.get_total_count() == self.get_underflow_count() {
            absolute_index = Self::get_layout().get_underflow_bin_index();
            count = self.get_underflow_count();
            less_count = 0;
        } else {
            let mut c: i64;
            let mut idx: i32 = self.max_allocated_bin_index_exclusive();
            loop {
                {
                    idx -= 1;
                    c = self.get_allocated_bin_count(idx);
                }
                if !(c == 0) {
                    break;
                }
            }
            absolute_index = idx;
            count = c;
            less_count = self.get_total_count() - c;
        }
        return BinIteratorImpl::new(absolute_index, less_count, greater_count, count);
    }

    fn get_underflow_count(&self) -> i64 {
        return self.underflow_count;
    }

    fn get_overflow_count(&self) -> i64 {
        return self.overflow_count;
    }

    fn get_total_count(&self) -> i64 {
        return self.total_count;
    }

    fn get_min(&self) -> f64 {
        return self.min;
    }

    fn get_max(&self) -> f64 {
        return self.max;
    }

    fn map_to_bin_index(&self, value: f64) -> i32 {
        let layout: Self::L = Self::get_layout();
        let idx: i32 = layout.map_to_bin_index(value);
        let under_flow_bin_index: i32 = layout.get_underflow_bin_index();
        if idx <= under_flow_bin_index {
            return under_flow_bin_index;
        }
        let over_flow_bin_index: i32 = layout.get_overflow_bin_index();
        if idx >= over_flow_bin_index {
            return over_flow_bin_index;
        }
        return idx;
    }

    fn add_ascending_sequence<F: Fn(i64) -> f64>(
        &self,
        ascending_sequence: &F,
        length: i64,
    ) -> Self::H {
        if length == 0 {
            return self;
        }
        Self::check_argument(length >= 0);
        if length > i64::MAX - self.get_total_count() {
            return Err(DynaHistError::ArithmeticError(&Self::OVERFLOW_MSG));
        }
        // add last value to update maximum
        let last_value: f64 = ascending_sequence.apply_as_double(length - 1);
        Self::add_value(last_value);
        // add remaining values in ascending order
        let length_without_last: i64 = length - 1;
        let val_index: i64 = 0;
        while val_index != length_without_last {
            let value: f64 = ascending_sequence.apply_as_double(val_index);
            let bin_index: i32 = self.map_to_bin_index(value);
            let next_val_index: i64 = Self::find_first(
                |&i| {
                    i == length_without_last
                        || self.map_to_bin_index(&ascending_sequence.apply_as_double(i)) > bin_index
                },
                val_index + 1,
                length_without_last,
                val_index + 1,
            );
            Self::add_values(value, next_val_index - val_index);
            val_index = next_val_index;
        }
        return self;
    }

    fn get_mode(&self) -> i8;

    fn is_mutable(&self) -> bool {
        return true;
    }
}
