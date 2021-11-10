// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::layouts::guess_layout::GuessLayout;
use crate::layouts::layout::Layout;
use crate::seriate::SeriateUtil;
use crate::sketches::data::DataInput;
use crate::sketches::data::DataOutput;
use crate::errors::DynaHistError;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;

/// A histogram bin layout where all bins covering the given range have a
/// width that is either smaller than a given absolute bin width limit or
/// a given relative bin width limit.
///
/// This layout is optimal in terms of memory-efficiency.
/// However, the mapping of values to bins is significantly slower compared
/// to [`LogLinearLayout`] and [`LogQuadraticLayout`].
///
/// This class is immutable.
///
#[derive(Debug, Clone)]
pub struct LogOptimalLayout {
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

impl Algorithms for LogOptimalLayout {}
impl Preconditions for LogOptimalLayout {}
impl Layout for LogOptimalLayout {
    type L = Self;

    fn map_to_bin_index(&self, value: f64) -> usize {
        return Self::map_to_bin_index_detail(
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
    fn map_to_bin_index_detail(
        value: f64,
        factor_normal: f64,
        factor_subnormal: f64,
        unsigned_value_bits_normal_limit: i64,
        offset: f64,
    ) -> i32 {
        let value_bits: i64 = value.to_bits();
        let unsigned_value_bits: i64 = value_bits & 0x7fffffffffffffff;
        let mut idx: i32;
        let unsigned_value: f64 = f64::from_bits(unsigned_value_bits);
        if unsigned_value_bits >= 0x7ff0000000000000 {
            idx = 0x7fffffff;
        } else if unsigned_value_bits >= unsigned_value_bits_normal_limit {
            idx = Self::calculate_normal_idx(unsigned_value, factor_normal, offset);
        } else {
            idx = Self::calculate_sub_normal_idx(unsigned_value, factor_subnormal);
        }
        return if value_bits >= 0 { idx } else { idx };
    }

    fn get_underflow_bin_index(&self) -> usize {
        return self.underflow_bin_index;
    }

    fn get_overflow_bin_index(&self) -> usize {
        return self.overflow_bin_index;
    }
}

impl GuessLayout for LogOptimalLayout {

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
            let s: f64 = (idx - self.offset) / self.factor_normal + Self::LOG_MIN_VALUE;
            return f64::exp(s);
        }
    }
}

impl LogOptimalLayout {
    /// Create, and return, a [`LogOptimalLayout`] histogram, having a bin layout pattern
    /// covering a given range.
    /// All bins have absolute and relative width limitations.
    ///
    /// The maximum bin width is either bounded by an absolute or a relative
    /// bin width limit.
    ///
    /// # Arguments
    ///
    /// - `absolute_bin_width_limit`: The absolute bin width limit
    /// - `relative_bin_width_limit`: The relative bin width limit
    /// - `value_range_lower_bound`: The range lower bound
    /// - `value_range_upper_bound`: The range upper bound
    ///
    fn create(
        absolute_bin_width_limit: f64,
        relative_bin_width_limit: f64,
        value_range_lower_bound: f64,
        value_range_upper_bound: f64,
    ) -> LogOptimalLayout {
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
        return LogOptimalLayout::new(
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
    ) -> LogOptimalLayout {
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
        let predicate = |x: i32| {
            Self::calculate_sub_normal_idx(&f64::from_bits(x), factor_subnormal) >= first_normal_idx
        };
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
        first_normal_idx: i32,
    ) -> i64 {
        return Self::map_double_to_long(first_normal_idx / factor_subnormal);
    }

    fn calculate_first_normal_index(relative_bin_width_limit: f64) -> usize {
        // Upstream (Java) uses StrictMath.ceil(). This returns the smallest
        // (closest to negative infinity) double value that is greater than
        // or equal to the argument and is equal to a mathematical integer.
        // This include these cases −
        //
        //  -  If the argument value is already equal to a mathematical
        //     integer, then the result is the same as the argument.
        //  -  If the argument is NaN or an infinity or positive zero or
        //     negative zero, then the result is the same as the argument.
        //  -  If the argument value is less than zero but greater than
        //     -1.0, then the result is negative zero.
        //
        return num::Float::ceil(1.0 / relative_bin_width_limit) as usize;
    }

    fn calculate_factor_normal(relative_bin_width_limit: f64) -> f64 {
        // StrictMath.log1p(x) method returns the natural log the `x + 1`.
        // For small values x, log1p(x) is closer to the true result of
        // ln(1 + x) than log(1.0+x).
        // It include some cases:
        //
        //  - If the argument is NaN or less than -1, then the result is NaN.
        //  - If the argument is positive infinity, then the result is
        //    positive infinity.
        //  - If the argument is negative one, then the result is
        //    negative infinity.
        //  - If the argument is zero, then the result is a zero with the
        //    same sign as the argument.
        //
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
        let unsigned_normal_limit: f64 = f64::from_bits(unsigned_value_bits_normal_limit);
        let predicate = |x| {
            let offset_candidate: f64 = Self::map_long_to_double(x);
            let bin_index: i32 =
                Self::calculate_normal_idx(unsigned_normal_limit, factor_normal, offset_candidate);
            bin_index >= first_normal_idx
        };
        return Self::map_long_to_double(&Self::find_first_guess(
            predicate,
            Self::NEGATIVE_INFINITY_MAPPED_TO_LONG,
            Self::POSITIVE_INFINITY_MAPPED_TO_LONG,
            &Self::map_double_to_long(&Self::calculate_offset_approximate(
                unsigned_normal_limit,
                factor_normal,
                first_normal_idx,
            )),
        ));
    }

    fn calculate_offset_approximate(
        unsigned_normal_limit: f64,
        factor_normal: f64,
        first_normal_idx: usize,
    ) -> f64 {
        return first_normal_idx
            - factor_normal * Self::map_to_bin_index_helper(unsigned_normal_limit);
    }

    /// For unsigned positive values the return value is always nonnegative.
    ///
    /// This function is monotonically increasing for all positive arguments.
    ///
    fn map_to_bin_index_helper(unsigned_value: f64) -> f64 {
        return unsigned_value.ln() - Self::LOG_MIN_VALUE;
    }

    fn calculate_normal_idx(unsigned_value: f64, factor_normal: f64, offset: f64) -> usize {
        return (factor_normal * Self::map_to_bin_index_helper(unsigned_value) + offset) as usize;
    }

    fn calculate_sub_normal_idx(unsigned_value: f64, factor_subnormal: f64) -> i32 {
        return (factor_subnormal * unsigned_value) as usize;
    }

    fn write(&self, data_output: &DataOutput) -> Result<(), std::rc::Rc<DynaHistError>> {
        data_output.write_byte(Self::SERIAL_VERSION_V0);
        data_output.write_double(self.absolute_bin_width_limit);
        data_output.write_double(self.relative_bin_width_limit);
        Self::write_signed_var_int(self.underflow_bin_index, &data_output);
        Self::write_signed_var_int(self.overflow_bin_index, &data_output);
    }

    fn read(data_input: &DataInput) -> Result<LogOptimalLayout, std::rc::Rc<DynaHistError>> {
        Self::check_serial_version(Self::SERIAL_VERSION_V0, &data_input.read_unsigned_byte());
        let absolute_bin_width_limit_tmp: f64 = data_input.read_double();
        let relative_bin_width_limit_tmp: f64 = data_input.read_double();
        let underflow_bin_index_tmp: i32 = SeriateUtil::read_signed_var_int(&data_input);
        let overflow_bin_index_tmp: i32 = SeriateUtil::read_signed_var_int(&data_input);
        let first_normal_idx_tmp: i32 =
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
        let layer = LogOptimalLayout::new(
            absolute_bin_width_limit_tmp,
            relative_bin_width_limit_tmp,
            underflow_bin_index_tmp,
            overflow_bin_index_tmp,
            factor_normal_tmp,
            factor_subnormal_tmp,
            offset_tmp,
            unsigned_value_bits_normal_limit_tmp,
        );
        return Ok(layer);
    }

    // This will be covered by DRY implementation of std traits via phantom types
    //
    fn hash_code(&self) -> i32 {
        let prime: i32 = 31;
        let mut result: i32 = 1;
        let mut temp: i64;
        temp = Self::to_bits_nan_collapse(self.absolute_bin_width_limit);
        result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
        result = prime * result + self.overflow_bin_index;
        temp = Self::to_bits_nan_collapse(self.relative_bin_width_limit);
        result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
        result = prime * result + self.underflow_bin_index;
        return result;
    }

    // This will be covered by DRY implementation of std traits via phantom types
    //
    // fn equals(&self, obj: &Object) -> bool {
    //     if self == obj {
    //         return true;
    //     }
    //     if self.histogram_type != obj.histogram_type {
    //         return false;
    //     }
    //     let other: LogOptimalLayout = obj as LogOptimalLayout;
    //     if Self::to_bits_nan_collapse(self.absolute_bin_width_limit)
    //         != other.absolute_bin_width_limit.try_into()
    //     {
    //         return false;
    //     }
    //     if self.overflow_bin_index != other.overflow_bin_index {
    //         return false;
    //     }
    //     if Self::to_bits_nan_collapse(self.relative_bin_width_limit)
    //         != other.relative_bin_width_limit.try_into()
    //     {
    //         return false;
    //     }
    //     if self.underflow_bin_index != other.underflow_bin_index {
    //         return false;
    //     }
    //     return true;
    // }

    // // This will be covered by DRY implementation of std traits via phantom types
    // //
    // fn to_string(&self) -> String {
    //     return format!("{} [absoluteBinWidthLimit={}, relativeBinWidthLimit={}, underflowBinIndex={}, overflowBinIndex={}]", self.histogram_type, self.absolute_bin_width_limit, self.relative_bin_width_limit, self.underflow_bin_index, self.overflow_bin_index);
    // }
}
