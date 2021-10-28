// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/**
 * A histogram bin layout where all bins covering the given range have a width that is either
 * smaller than a given absolute bin width limit or a given relative bin width limit. This layout is
 * optimal in terms of memory-efficiency. However, the mapping of values to bins is significantly
 * slower compared to {@link LogLinearLayout} and {@link LogQuadraticLayout}.
 *
 * This class is immutable.
 */

 const SERIAL_VERSION_V0: i8 = 0;

 const LOG_MIN_VALUE: f64 = Math::log(f64::MIN);
pub struct LogOptimalLayout {
    super: GuessLayout;

     let absolute_bin_width_limit: f64;

     let relative_bin_width_limit: f64;

     let underflow_bin_index: i32;

     let overflow_bin_index: i32;

     let factor_normal: f64;

     let factor_subnormal: f64;

     let offset: f64;

     let unsigned_value_bits_normal_limit: i64;
}

impl LogOptimalLayout {


   /// Create a histogram bin layout covering a given range and with bins that have absolute and
   /// relative width limitations.
   ///
   /// The maximum bin width is either bounded by an absolute or a relative bin width limit.
   ///
   /// @param absoluteBinWidthLimit the absolute bin width limit
   /// @param relativeBinWidthLimit the relative bin width limit
   /// @param valueRangeLowerBound the range lower bound
   /// @param valueRangeUpperBound the range upper bound
   /// @return a new {@link LogOptimalLayout} instance
   ///
    pub fn  create( absolute_bin_width_limit: f64,  relative_bin_width_limit: f64,  value_range_lower_bound: f64,  value_range_upper_bound: f64) -> LogOptimalLayout  {
        check_argument(&Double::is_finite(value_range_upper_bound));
        check_argument(&Double::is_finite(value_range_lower_bound));
        check_argument(value_range_upper_bound >= value_range_lower_bound);
        check_argument(absolute_bin_width_limit >= Double::MIN_NORMAL);
        check_argument(absolute_bin_width_limit <= f64::MAX);
        check_argument(relative_bin_width_limit >= 0.0);
        check_argument(relative_bin_width_limit <= f64::MAX);
         let first_normal_idx: i32 = ::calculate_first_normal_index(relative_bin_width_limit);
        // will always be >= 1 because 0 <= relativeBinWidthLimit <= Double.MAX_VALUE
         let factor_normal: f64 = ::calculate_factor_normal(relative_bin_width_limit);
         let factor_subnormal: f64 = ::calculate_factor_sub_normal(absolute_bin_width_limit);
         let unsigned_value_bits_normal_limit: i64 = ::calculate_unsigned_value_bits_normal_limit(factor_subnormal, first_normal_idx);
         let offset: f64 = ::calculate_offset(unsigned_value_bits_normal_limit, factor_normal, first_normal_idx);
         let value_range_lower_bound_bin_index: i32 = ::map_to_bin_index(value_range_lower_bound, factor_normal, factor_subnormal, unsigned_value_bits_normal_limit, offset);
         let value_range_upper_bound_bin_index: i32 = ::map_to_bin_index(value_range_upper_bound, factor_normal, factor_subnormal, unsigned_value_bits_normal_limit, offset);
        check_argument(value_range_lower_bound_bin_index > Integer::MIN_VALUE);
        check_argument(value_range_upper_bound_bin_index < Integer::MAX_VALUE);
         let underflow_bin_index: i32 = value_range_lower_bound_bin_index - 1;
         let overflow_bin_index: i32 = value_range_upper_bound_bin_index + 1;
        check_argument(overflow_bin_index as i64 - underflow_bin_index as i64 - 1 <= Integer::MAX_VALUE as i64);
        return LogOptimalLayout::new(absolute_bin_width_limit, relative_bin_width_limit, underflow_bin_index, overflow_bin_index, factor_normal, factor_subnormal, offset, unsigned_value_bits_normal_limit);
    }

    fn new( absolute_bin_width_limit: f64,  relative_bin_width_limit: f64,  underflow_bin_index: usize,  overflow_bin_index: usize,  factor_normal: f64,  factor_subnormal: f64,  offset: f64,  unsigned_value_bits_normal_limit: i64) -> LogOptimalLayout {
        let .absoluteBinWidthLimit = absolute_bin_width_limit;
        let .relativeBinWidthLimit = relative_bin_width_limit;
        let .underflowBinIndex = underflow_bin_index;
        let .overflowBinIndex = overflow_bin_index;
        let .factorNormal = factor_normal;
        let .factorSubnormal = factor_subnormal;
        let .offset = offset;
        let .unsignedValueBitsNormalLimit = unsigned_value_bits_normal_limit;
    }

    fn  calculate_unsigned_value_bits_normal_limit( factor_subnormal: f64,  first_normal_idx: usize) -> i64  {
        return Algorithms::find_first_guess( l: & -> ::calculate_sub_normal_idx(&f64::from_bits(l), factor_subnormal) >= first_normal_idx, 0, &Double::double_to_raw_long_bits(f64::INFINITY), &::calculate_unsigned_value_bits_normal_limit_approximate(factor_subnormal, first_normal_idx));
    }

    fn  calculate_unsigned_value_bits_normal_limit_approximate( factor_subnormal: f64,  first_normal_idx: i32) -> i64  {
        return Algorithms::map_double_to_long(first_normal_idx / factor_subnormal);
    }

    fn  calculate_first_normal_index( relative_bin_width_limit: f64) -> usize  {
        return StrictMath::ceil(1.0 / relative_bin_width_limit) as usize;
    }

    fn  calculate_factor_normal( relative_bin_width_limit: f64) -> f64  {
        return 1.0 / StrictMath::log1p(relative_bin_width_limit);
    }

    fn  calculate_factor_sub_normal( absolute_bin_width_limit: f64) -> f64  {
        return 1.0 / absolute_bin_width_limit;
    }

    fn  calculate_offset( unsigned_value_bits_normal_limit: i64,  factor_normal: f64,  first_normal_idx: usize) -> f64  {
         let unsigned_normal_limit: f64 = f64::from_bits(unsigned_value_bits_normal_limit);
        return Algorithms::map_long_to_double(&Algorithms::find_first_guess( l: & -> {
             let offset_candidate: f64 = Algorithms::map_long_to_double(l);
             let bin_index: i32 = ::calculate_normal_idx(unsigned_normal_limit, factor_normal, offset_candidate);
            return bin_index >= first_normal_idx;
        }, Algorithms::NEGATIVE_INFINITY_MAPPED_TO_LONG, Algorithms::POSITIVE_INFINITY_MAPPED_TO_LONG, &Algorithms::map_double_to_long(&::calculate_offset_approximate(unsigned_normal_limit, factor_normal, first_normal_idx))));
    }

    fn  calculate_offset_approximate( unsigned_normal_limit: f64,  factor_normal: f64,  first_normal_idx: usize) -> f64  {
        return first_normal_idx - factor_normal * ::map_to_bin_index_helper(unsigned_normal_limit);
    }


   /// For unsigned positive values the return value is always nonnegative.
   ///
   /// This function is monotonically increasing for all positive arguments.
   ///
    fn  map_to_bin_index_helper( unsigned_value: f64) -> f64  {
        return Math::log(unsigned_value) - LOG_MIN_VALUE;
    }

    fn  calculate_normal_idx( unsigned_value: f64,  factor_normal: f64,  offset: f64) -> usize  {
        return (factor_normal * ::map_to_bin_index_helper(unsigned_value) + offset) as usize;
    }

    fn  calculate_sub_normal_idx( unsigned_value: f64,  factor_subnormal: f64) -> i32  {
        return (factor_subnormal * unsigned_value) as usize;
    }

    // Unfortunately this mapping is not platform-independent. It would be independent if the strictfp
    // keyword was used for this method and all called methods. Due to a performance penalty (see
    // https://bugs.openjdk.java.net/browse/JDK-8136414) of strictfp, which is hopefully fixed in Java
    // 15, we have omitted strictfp here in the meantime.
    fn  map_to_bin_index( value: f64,  factor_normal: f64,  factor_subnormal: f64,  unsigned_value_bits_normal_limit: i64,  offset: f64) -> i32  {
         let value_bits: i64 = value.to_bits();
         let unsigned_value_bits: i64 = value_bits & 0x7fffffffffffffff;
         let mut idx: i32;
         let unsigned_value: f64 = f64::from_bits(unsigned_value_bits);
        if unsigned_value_bits >= 0x7ff0000000000000 {
            idx = 0x7fffffff;
        } else if unsigned_value_bits >= unsigned_value_bits_normal_limit {
            idx = ::calculate_normal_idx(unsigned_value, factor_normal, offset);
        } else {
            idx = ::calculate_sub_normal_idx(unsigned_value, factor_subnormal);
        }
        return  if (value_bits >= 0) { idx } else { ~idx };
    }

    pub fn  map_to_bin_index(&self,  value: f64) -> usize  {
        return ::map_to_bin_index(value, self.factor_normal, self.factor_subnormal, self.unsigned_value_bits_normal_limit, self.offset);
    }

    pub fn  get_underflow_bin_index(&self) -> usize  {
        return self.underflow_bin_index;
    }

    pub fn  get_overflow_bin_index(&self) -> usize  {
        return self.overflow_bin_index;
    }

    pub fn  write(&self,  data_output: &DataOutput)  -> /*  throws IOException */Result<Void, Rc<Exception>>   {
        data_output.write_byte(SERIAL_VERSION_V0);
        data_output.write_double(self.absolute_bin_width_limit);
        data_output.write_double(self.relative_bin_width_limit);
        write_signed_var_int(self.underflow_bin_index, &data_output);
        write_signed_var_int(self.overflow_bin_index, &data_output);
    }

    pub fn  read( data_input: &DataInput) -> /*  throws IOException */Result<LogOptimalLayout, Rc<Exception>>   {
        check_serial_version(SERIAL_VERSION_V0, &data_input.read_unsigned_byte());
         let absolute_bin_width_limit_tmp: f64 = data_input.read_double();
         let relative_bin_width_limit_tmp: f64 = data_input.read_double();
         let underflow_bin_index_tmp: i32 = SerializationUtil::read_signed_var_int(&data_input);
         let overflow_bin_index_tmp: i32 = SerializationUtil::read_signed_var_int(&data_input);
         let first_normal_idx_tmp: i32 = ::calculate_first_normal_index(relative_bin_width_limit_tmp);
         let factor_normal_tmp: f64 = ::calculate_factor_normal(relative_bin_width_limit_tmp);
         let factor_subnormal_tmp: f64 = ::calculate_factor_sub_normal(absolute_bin_width_limit_tmp);
         let unsigned_value_bits_normal_limit_tmp: i64 = ::calculate_unsigned_value_bits_normal_limit(factor_subnormal_tmp, first_normal_idx_tmp);
         let offset_tmp: f64 = ::calculate_offset(unsigned_value_bits_normal_limit_tmp, factor_normal_tmp, first_normal_idx_tmp);
        return Ok(LogOptimalLayout::new(absolute_bin_width_limit_tmp, relative_bin_width_limit_tmp, underflow_bin_index_tmp, overflow_bin_index_tmp, factor_normal_tmp, factor_subnormal_tmp, offset_tmp, unsigned_value_bits_normal_limit_tmp));
    }

    pub fn  hash_code(&self) -> i32  {
         let prime: i32 = 31;
         let mut result: i32 = 1;
         let mut temp: i64;
        temp = Double::double_to_long_bits(self.absolute_bin_width_limit);
        result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
        result = prime * result + self.overflow_bin_index;
        temp = Double::double_to_long_bits(self.relative_bin_width_limit);
        result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
        result = prime * result + self.underflow_bin_index;
        return result;
    }

    pub fn  equals(&self,  obj: &Object) -> bool  {
        if self == obj {
            return true;
        }
        if obj == null {
            return false;
        }
        if get_class() != obj.get_class() {
            return false;
        }
         let other: LogOptimalLayout = obj as LogOptimalLayout;
        if Double::double_to_long_bits(self.absolute_bin_width_limit) != Double::double_to_long_bits(other.absoluteBinWidthLimit) {
            return false;
        }
        if self.overflow_bin_index != other.overflowBinIndex {
            return false;
        }
        if Double::double_to_long_bits(self.relative_bin_width_limit) != Double::double_to_long_bits(other.relativeBinWidthLimit) {
            return false;
        }
        if self.underflow_bin_index != other.underflowBinIndex {
            return false;
        }
        return true;
    }

    pub fn  get_bin_lower_bound_approximation(&self,  bin_index: i32) -> f64  {
        if bin_index >= 0 {
            return self.get_bin_lower_bound_approximation_helper(bin_index);
        } else {
            return -self.get_bin_lower_bound_approximation_helper(-bin_index);
        }
    }

    fn  get_bin_lower_bound_approximation_helper(&self,  idx: i32) -> f64  {
         let x: f64 = idx * self.absolute_bin_width_limit;
        if x < f64::from_bits(self.unsigned_value_bits_normal_limit) {
            return x;
        } else {
             let s: f64 = (idx - self.offset) / self.factor_normal + LOG_MIN_VALUE;
            return Math::exp(s);
        }
    }

    pub fn  to_string(&self) -> String  {
        return format!("{} [absoluteBinWidthLimit={}, relativeBinWidthLimit={}, underflowBinIndex={}, overflowBinIndex={}]", get_class().get_simple_name(), self.absolute_bin_width_limit, self.relative_bin_width_limit, self.underflow_bin_index, self.overflow_bin_index);
    }
}
