// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::layouts::guess_layout::GuessLayout;
use crate::layouts::layout::Layout;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::seriate::Seriate;
use crate::seriate::SeriateUtil;
use crate::seriate::serialization::SeriateWrite;
use crate::seriate::deserialization::SeriateRead;
use crate::sketches::data::DataInput;
use crate::sketches::data::DataOutput;
use crate::errors::DynaHistError;

/// A histogram bin layout where all bins covering the given range have a width that is either
/// smaller than a given absolute bin width limit or a given relative bin width limit. This layout
/// uses a piecewise-linear function to map values to bin indices.
///
/// This class is immutable.
///
pub struct LogLinearLayout {
    absolute_bin_width_limit: f64,
    factor_normal: f64,
    factor_subnormal: f64,
    histogram_type: String,
    offset: f64,
    overflow_bin_index: usize,
    relative_bin_width_limit: f64,
    underflow_bin_index: usize,
    unsigned_value_bits_normal_limit: i64,
}

impl Algorithms for LogLinearLayout {}
impl Preconditions for LogLinearLayout {}
impl Seriate for LogLinearLayout {

}

impl Layout for LogLinearLayout {
    type L = Self;

    fn map_to_bin_index(&self, value: f64) -> usize {
        return self.map_to_bin_index_detail(&self,
            value,
            self.factor_normal,
            self.factor_subnormal,
            self.unsigned_value_bits_normal_limit,
            self.offset,
        );
    }

    // Upstream (Java) Notes:
    //
    // Unfortunately this mapping is not platform-independent.
    // It would be independent if the `strictfp` keyword was used for this
    // method and all called methods.
    // Due to a performance penalty of `strictfp`, which is hopefully fixed
    // in Java 15, we have omitted `strictfp` here in the meantime.
    //
    // References:
    // - https://bugs.openjdk.java.net/browse/JDK-8136414
    //
    fn map_to_bin_index_detail(&self,
        value: f64,
        factor_normal: f64,
        factor_subnormal: f64,
        unsigned_value_bits_normal_limit: i64,
        offset: f64,
    ) -> usize {
        let value_bits: i64 = value.to_bits();
        let unsigned_value_bits: i64 = value_bits & 0x7fffffffffffffff;
        let mut idx: i32;
        if unsigned_value_bits >= unsigned_value_bits_normal_limit {
            idx = Self::calculate_normal_idx(unsigned_value_bits, factor_normal, offset);
        } else {
            idx = Self::calculate_sub_normal_idx(unsigned_value_bits, factor_subnormal);
        }
        return if value_bits >= 0x0 { idx } else { !idx };
    }

    fn get_underflow_bin_index(&self) -> usize {
        return self.underflow_bin_index;
    }

    fn get_overflow_bin_index(&self) -> usize {
        return self.overflow_bin_index;
    }
}

impl GuessLayout for LogLinearLayout {
    fn get_bin_lower_bound_approximation(&self, bin_index: i32) -> f64 {
        if bin_index >= 0 {
            return self.get_bin_lower_bound_approximation_helper(bin_index);
        } else {
            return -self.get_bin_lower_bound_approximation_helper(-bin_index);
        }
    }

    fn get_bin_lower_bound_approximation_helper(&self, idx: i32) -> f64 {
        let x: f64 = idx * self.absolute_bin_width_limit;
        if x < f64::from_bits(self.unsigned_value_bits_normal_limit) {
            return x;
        } else {
            let s: f64 = (idx - self.offset) / self.factor_normal;
            let exponent: i32 = (num::Float::floor(s) as i32) - 1;
            let mantissa_plus1: f64 = s - exponent;
            // Upstream (Java) uses `Math.scalb` as an efficient `f * 2 ^ scale_factor`
            // `f * Math.pow(2, scale_factor)` claims of 2-fold speed up are around.
            //return Math::scalb(mantissa_plus1, exponent - 1023);
            return mantissa_plus1 * i32::pow(2, exponent - 1023);
        }
    }
}

impl LogLinearLayout {

    fn read(data_input: &DataInput) -> Result<LogLinearLayout, std::rc::Rc<DynaHistError>> {
        Self::check_serial_version(Self::SERIAL_VERSION_V0, &data_input.read_unsigned_byte());
        let absolute_bin_width_limit_tmp: f64 = data_input.read_double();
        let relative_bin_width_limit_tmp: f64 = data_input.read_double();
        let underflow_bin_index_tmp: i32 = SeriateUtil::read_signed_var_int(&data_input);
        let overflow_bin_index_tmp: i32 = SeriateUtil::read_signed_var_int(&data_input);
        let first_normal_idx_tmp: usize =
            Self::calculate_first_normal_index(relative_bin_width_limit_tmp);
        let factor_normal_tmp: f64 = Self::calculate_factor_normal(relative_bin_width_limit_tmp);
        let factor_subnormal_tmp: f64 =
            Self::calculate_factor_sub_normal(absolute_bin_width_limit_tmp);
        let unsigned_value_bits_normal_limit_tmp: i64 =
            Self::calculate_unsigned_value_bits_normal_limit(
                factor_subnormal_tmp,
                first_normal_idx_tmp,
            );
        let offset_tmp: f64 = Self::calculate_offset(
            unsigned_value_bits_normal_limit_tmp,
            factor_normal_tmp,
            first_normal_idx_tmp,
        );
        return Ok(LogLinearLayout::new(
            absolute_bin_width_limit_tmp,
            relative_bin_width_limit_tmp,
            underflow_bin_index_tmp,
            overflow_bin_index_tmp,
            factor_normal_tmp,
            factor_subnormal_tmp,
            offset_tmp,
            unsigned_value_bits_normal_limit_tmp,
        ));
    }

    fn write(&self, data_output: &DataOutput) -> Result<(), std::rc::Rc<DynaHistError>> {
        data_output.write_byte(Self::SERIAL_VERSION_V0);
        data_output.write_double(self.absolute_bin_width_limit);
        data_output.write_double(self.relative_bin_width_limit);
        Self::write_signed_var_int(self.underflow_bin_index, &data_output);
        Self::write_signed_var_int(self.overflow_bin_index, &data_output)
    }
}

impl LogLinearLayout {
    /// Create a histogram bin layout covering a given range and with bins that have absolute and
    /// relative width limitations.
    ///
    /// The maximum bin width is either bounded by an absolute or a relative bin width limit.
    ///
    /// - `absoluteBinWidthLimit`: the absolute bin width limit
    /// - `relativeBinWidthLimit`: the relative bin width limit
    /// - `valueRangeLowerBound`: the range lower bound
    /// - `valueRangeUpperBound`: the range upper bound
    ///
    /// a new [`LogLinearLayout`] instance
    ///
    fn create(
        absolute_bin_width_limit: f64,
        relative_bin_width_limit: f64,
        value_range_lower_bound: f64,
        value_range_upper_bound: f64,
    ) -> LogLinearLayout {
        Self::check_argument(value_range_upper_bound.is_finite());
        Self::check_argument(value_range_lower_bound.is_finite());
        Self::check_argument(value_range_upper_bound >= value_range_lower_bound);
        Self::check_argument(absolute_bin_width_limit >= Self::min_normal_f64());
        Self::check_argument(absolute_bin_width_limit <= f64::MAX);
        Self::check_argument(relative_bin_width_limit >= 0.0);
        Self::check_argument(relative_bin_width_limit <= f64::MAX);
        let first_normal_idx: i32 = Self::calculate_first_normal_index(relative_bin_width_limit);
        // will always be >= 1 because 0 <= relativeBinWidthLimit <= Double.MAX_VALUE
        let factor_normal: f64 = Self::calculate_factor_normal(relative_bin_width_limit);
        let factor_subnormal: f64 = Self::calculate_factor_sub_normal(absolute_bin_width_limit);
        let unsigned_value_bits_normal_limit: i64 =
            Self::calculate_unsigned_value_bits_normal_limit(factor_subnormal, first_normal_idx);
        let offset: f64 = Self::calculate_offset(
            unsigned_value_bits_normal_limit,
            factor_normal,
            first_normal_idx,
        );
        let value_range_lower_bound_bin_index: i32 = Self::map_to_bin_index(
            value_range_lower_bound,
            factor_normal,
            factor_subnormal,
            unsigned_value_bits_normal_limit,
            offset,
        );
        let value_range_upper_bound_bin_index: i32 = Self::map_to_bin_index(
            value_range_upper_bound,
            factor_normal,
            factor_subnormal,
            unsigned_value_bits_normal_limit,
            offset,
        );
        Self::check_argument(value_range_lower_bound_bin_index > i32::MIN);
        Self::check_argument(value_range_upper_bound_bin_index < i32::MAX);
        let underflow_bin_index: i32 = value_range_lower_bound_bin_index - 1;
        let overflow_bin_index: i32 = value_range_upper_bound_bin_index + 1;
        Self::check_argument(
            overflow_bin_index as i64 - underflow_bin_index as i64 - 1 <= i32::MAX as i64,
        );
        return LogLinearLayout::new(
            absolute_bin_width_limit,
            relative_bin_width_limit,
            underflow_bin_index,
            overflow_bin_index,
            factor_normal,
            factor_subnormal,
            offset,
            unsigned_value_bits_normal_limit,
        );
    }

    fn new(
        absolute_bin_width_limit: f64,
        relative_bin_width_limit: f64,
        underflow_bin_index: usize,
        overflow_bin_index: usize,
        factor_normal: f64,
        factor_subnormal: f64,
        offset: f64,
        unsigned_value_bits_normal_limit: i64,
    ) -> LogLinearLayout {
        absolute_bin_width_limit;
        relative_bin_width_limit;
        underflow_bin_index;
        overflow_bin_index;
        factor_normal;
        factor_subnormal;
        offset;
        unsigned_value_bits_normal_limit;
    }

    fn calculate_unsigned_value_bits_normal_limit(
        factor_subnormal: f64,
        first_normal_idx: usize,
    ) -> i64 {
        let predicate =
            |&x| Self::calculate_sub_normal_idx(x, factor_subnormal) >= first_normal_idx;
        return Self::find_first_guess(
            predicate,
            0,
            &Self::double_to_raw_long_bits(f64::INFINITY),
            &Self::calculate_unsigned_value_bits_normal_limit_approximate(
                factor_subnormal,
                first_normal_idx,
            ),
        );
    }

    fn calculate_unsigned_value_bits_normal_limit_approximate(
        factor_subnormal: f64,
        first_normal_idx: usize,
    ) -> i64 {
        return Self::map_double_to_long(first_normal_idx / factor_subnormal);
    }

    fn calculate_first_normal_index(relative_bin_width_limit: f64) -> usize {
        return num::Float::ceil(1.0 / relative_bin_width_limit) as usize;
    }

    fn calculate_factor_normal(relative_bin_width_limit: f64) -> f64 {
        return 1.0 / num::Float::log_1p(relative_bin_width_limit);
    }

    fn calculate_factor_sub_normal(absolute_bin_width_limit: f64) -> f64 {
        return 1.0 / absolute_bin_width_limit;
    }

    fn calculate_offset(
        unsigned_value_bits_normal_limit: i64,
        factor_normal: f64,
        first_normal_idx: usize,
    ) -> f64 {
        let predicate = |&x| {
            let offset_candidate: f64 = Self::map_long_to_double(x);
            let bin_index: i32 = Self::calculate_normal_idx(
                unsigned_value_bits_normal_limit,
                factor_normal,
                offset_candidate,
            );
            return bin_index >= first_normal_idx;
        };
        return Self::map_long_to_double(&Self::find_first_guess(
            predicate,
            Self::NEGATIVE_INFINITY_MAPPED_TO_LONG,
            Self::POSITIVE_INFINITY_MAPPED_TO_LONG,
            &Self::map_double_to_long(&Self::calculate_offset_approximate(
                unsigned_value_bits_normal_limit,
                factor_normal,
                first_normal_idx,
            )),
        ));
    }

    fn calculate_offset_approximate(
        unsigned_value_bits_normal_limit: i64,
        factor_normal: f64,
        first_normal_idx: usize,
    ) -> f64 {
        return first_normal_idx
            - factor_normal * Self::map_to_bin_index_helper(unsigned_value_bits_normal_limit);
    }

    /// For unsigned values the return value is in the range [0, 2049].
    ///
    /// It can be shown that this function is monotonically increasing for all non-negative
    /// arguments.
    ///
    fn map_to_bin_index_helper(unsigned_value_bits: i64) -> f64 {
        let exponent: i64 = unsigned_value_bits >> /* >>> */ 52;
        let mantissa_plus1: f64 =
            f64::from_bits((unsigned_value_bits & 0x000fffffffffffff) | 0x3ff0000000000000);
        return mantissa_plus1 + exponent;
    }

    fn calculate_normal_idx(unsigned_value_bits: i64, factor_normal: f64, offset: f64) -> usize {
        return (factor_normal * Self::map_to_bin_index_helper(unsigned_value_bits) + offset)
            as usize;
    }

    fn calculate_sub_normal_idx(unsigned_value_bits: i64, factor_subnormal: f64) -> usize {
        return (factor_subnormal * f64::from_bits(unsigned_value_bits)) as usize;
    }

    // fn hash_code(&self) -> i32 {
    //     let prime: i32 = 31;
    //     let mut result: i32 = 1;
    //     let mut temp: i64;
    //     temp = Self::to_bits_nan_collapse(self.absolute_bin_width_limit);
    //     result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
    //     result = prime * result + self.overflow_bin_index;
    //     temp = Self::to_bits_nan_collapse(self.relative_bin_width_limit);
    //     result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
    //     result = prime * result + self.underflow_bin_index;
    //     return result;
    // }

    // fn equals(&self, obj: &Object) -> bool {
    //     if self == obj {
    //         return true;
    //     }
    //     if obj == null {
    //         return false;
    //     }
    //     if self.histogram_type != obj.histogram_type {
    //         return false;
    //     }
    //     let other: LogLinearLayout = obj as LogLinearLayout;
    //     if Self::to_bits_nan_collapse(self.absolute_bin_width_limit)
    //         != Double::double_to_long_bits(other.absoluteBinWidthLimit)
    //     {
    //         return false;
    //     }
    //     if self.overflow_bin_index != other.overflowBinIndex {
    //         return false;
    //     }
    //     if Self::to_bits_nan_collapse(self.relative_bin_width_limit)
    //         != Double::double_to_long_bits(other.relativeBinWidthLimit)
    //     {
    //         return false;
    //     }
    //     if self.underflow_bin_index != other.underflowBinIndex {
    //         return false;
    //     }
    //     return true;
    // }

    // fn to_string(&self) -> String {
    //     return format!("{} [absoluteBinWidthLimit={}, relativeBinWidthLimit={}, underflowBinIndex={}, overflowBinIndex={}]", self.histogram_type, self.absolute_bin_width_limit, self.relative_bin_width_limit, self.underflow_bin_index, self.overflow_bin_index);
    // }
}
