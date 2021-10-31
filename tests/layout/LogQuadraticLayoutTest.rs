// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct LogQuadraticLayoutTest {
    super: AbstractErrorLimitingLayoutTest;
}

impl LogQuadraticLayoutTest {

    #[test]
    pub fn test(&self)   {
        assert_true(4.0 * StrictMath::log1p(f64::MAX) <= 2840.0);
    }

    #[test]
    pub fn test_map_to_bin_index_helper_special_values(&self)   {
        assert_eq!(6144.0, &LogQuadraticLayout::map_to_bin_index_helper(i64::MAX), 0.0);
        assert_eq!(6144.0, &LogQuadraticLayout::map_to_bin_index_helper(0x7fffffffffffffff), 0.0);
        assert_eq!(6142.75, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(f64::NAN)), 0.0);
        assert_eq!(6141.0, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(f64::INFINITY)), 0.0);
        assert_eq!(3.0, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(self.min_normal_f64())), 0.0);
        assert_eq!(0.0, &LogQuadraticLayout::map_to_bin_index_helper(0), 0.0);
        assert_eq!(3063.0, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(0.25)), 0.0);
        assert_eq!(3066.0, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(0.5)), 0.0);
        assert_eq!(3069.0, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(1.0)), 0.0);
        assert_eq!(3072.0, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(2.0)), 0.0);
        assert_eq!(3075.0, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(4.0)), 0.0);
        assert_eq!(3078.0, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(8.0)), 0.0);
        assert_eq!(3081.0, &LogQuadraticLayout::map_to_bin_index_helper(&to_bits_nan_collapse(16.0)), 0.0);
    }

    pub fn create_layout(&self,  absolute_bin_width_limit: f64,  relative_bin_width_limit: f64,  value_range_lower_bound: f64,  value_range_upper_bound: f64) -> GuessLayout  {
        return LogQuadraticLayout::create(absolute_bin_width_limit, relative_bin_width_limit, value_range_lower_bound, value_range_upper_bound);
    }

    #[test]
    pub fn test_overflow_and_underflow_indices(&self)   {
        {
             let layout: LogQuadraticLayout = LogQuadraticLayout::create(1e-7, 1e-6, -1e12, 1e12);
            assert_eq!(33391320, &layout.get_overflow_bin_index());
            assert_eq!(-33391321, &layout.get_underflow_bin_index());
        }
        {
             let layout: LogQuadraticLayout = LogQuadraticLayout::create(1e-7, 1e-6, 1e12, 1e12);
            assert_eq!(33391320, &layout.get_overflow_bin_index());
            assert_eq!(33391318, &layout.get_underflow_bin_index());
        }
    }

    #[test]
    pub fn test_serialization(&self)  -> Result<Void, Rc<DynaHistError>>   {
         let value_range_upper_bound: f64 = 1e7;
         let value_range_lower_bound: f64 = -1e6;
         let relative_bin_width_limit: f64 = 1e-3;
         let absolute_bin_width_limit: f64 = 1e-9;
         let layout: LogQuadraticLayout = LogQuadraticLayout::create(absolute_bin_width_limit, relative_bin_width_limit, value_range_lower_bound, value_range_upper_bound);
         let deserialized_layout: LogQuadraticLayout = SerializationTestUtil::test_serialization(layout, LogQuadraticLayout::write, LogQuadraticLayout::read, "003E112E0BE826D6953F50624DD2F1A9FC8FE303F48904");
        assert_eq!(deserialized_layout, layout);
    }

    #[test]
    pub fn test_to_string(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
        assert_eq!("LogQuadraticLayout [absoluteBinWidthLimit=1.0E-8, relativeBinWidthLimit=0.01, underflowBinIndex=-3107, overflowBinIndex=3106]", &layout.to_string());
    }

    #[test]
    pub fn test_get_width(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_static(layout);
        histogram.add_value(0);
        histogram.add_value(10);
        assert_eq!(9.999999999999999E-9, &histogram.get_first_non_empty_bin().get_width(), 0);
        assert_eq!(0.031135683241927836, &histogram.get_last_non_empty_bin().get_width(), 0);
    }

    #[test]
    pub fn test_equals(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
        assert_false(&layout.equals(null));
        assert_false(&layout.equals(&LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6)));
        assert_false(&layout.equals(&LogQuadraticLayout::create(1e-7, 1e-2, -1e6, 1e6)));
        assert_false(&LogQuadraticLayout::create(1, 0, 1, 10)::equals(&LogQuadraticLayout::create(1, 1e-3, 1, 10)));
        assert_false(&layout.equals(&LogQuadraticLayout::create(1e-8, 1e-2, -1e5, 1e6)));
        assert_false(&layout.equals(&LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e5)));
    }

    #[test]
    pub fn test_initial_guesses(&self)   {
         let absolute_bin_width_limits: vec![Vec<f64>; 10] = vec![1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1e0, 1e1, 1e2, 1e3, ]
        ;
         let relative_bin_width_limits: vec![Vec<f64>; 12] = vec![0.0, 1e-100, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1e0, 1e1, 1e2, 1e3, ]
        ;
        for  let absolute_bin_width_limit: f64 in absolute_bin_width_limits {
            for  let relative_bin_width_limit: f64 in relative_bin_width_limits {
                 let factor_normal: f64 = LogQuadraticLayout::calculate_factor_normal(relative_bin_width_limit);
                 let factor_subnormal: f64 = LogQuadraticLayout::calculate_factor_sub_normal(absolute_bin_width_limit);
                 let first_normal_idx: i32 = LogQuadraticLayout::calculate_first_normal_index(relative_bin_width_limit);
                 let unsigned_value_bits_normal_limit_approximate: i64 = LogQuadraticLayout::calculate_unsigned_value_bits_normal_limit_approximate(factor_subnormal, first_normal_idx);
                 let unsigned_value_bits_normal_limit: i64 = LogQuadraticLayout::calculate_unsigned_value_bits_normal_limit(factor_subnormal, first_normal_idx);
                 let offset_approximate: f64 = LogQuadraticLayout::calculate_offset_approximate(unsigned_value_bits_normal_limit, factor_normal, first_normal_idx);
                 let offset: f64 = LogQuadraticLayout::calculate_offset(unsigned_value_bits_normal_limit, factor_normal, first_normal_idx);
                assert_that(&Algorithms::map_double_to_long(offset_approximate)).is_close_to(&Algorithms::map_double_to_long(offset), &Offset::offset(1));
                assert_that(unsigned_value_bits_normal_limit_approximate).is_close_to(unsigned_value_bits_normal_limit, &Offset::offset(1));
            }
        }
    }

    #[test]
    pub fn test_hash_code(&self)   {
        assert_eq!(-1339415786, &self.create_layout(1e-6, 1e-4, -10, 1000).hash_code());
    }

    fn test_function(&self,  mantissa_plus1: f64) -> f64  {
        return (mantissa_plus1 - 1.0) * (5.0 - mantissa_plus1);
    }

    #[test]
    pub fn test_monotonicity_close_to2(&self)   {
         let mantissa_plus1: f64 = 2;
         {
             let mut l: i64 = 0;
            while l < 10_000_000 {
                {
                     let next_mantissa_plus1: f64 = Math::next_down(mantissa_plus1);
                    assert_that(&self.test_function(next_mantissa_plus1)).is_less_than_or_equal_to(&self.test_function(mantissa_plus1));
                    mantissa_plus1 = next_mantissa_plus1;
                }
                l += 1;
             }
         }

    }

    #[test]
    pub fn test_monotonicity_close_to2_strict(&self)   {
         let mantissa_plus1: f64 = 2;
         {
             let mut l: i64 = 0;
            while l < 10_000_000 {
                {
                     let next_mantissa_plus1: f64 = Math::next_down(mantissa_plus1);
                    assert_that(&self.test_function(next_mantissa_plus1)).is_less_than_or_equal_to(&self.test_function(mantissa_plus1));
                    mantissa_plus1 = next_mantissa_plus1;
                }
                l += 1;
             }
         }

    }

    #[test]
    pub fn test_monotonicity_close_to1(&self)   {
         let mantissa_plus1: f64 = 1;
         {
             let mut l: i64 = 0;
            while l < 10_000_000 {
                {
                     let next_mantissa_plus1: f64 = Math::next_up(mantissa_plus1);
                    assert_that(&self.test_function(next_mantissa_plus1)).is_greater_than_or_equal_to(&self.test_function(mantissa_plus1));
                    mantissa_plus1 = next_mantissa_plus1;
                }
                l += 1;
             }
         }

    }

    #[test]
    pub fn test_monotonicity_close_to1_strict(&self)   {
         let mantissa_plus1: f64 = 1;
         {
             let mut l: i64 = 0;
            while l < 10_000_000 {
                {
                     let next_mantissa_plus1: f64 = Math::next_up(mantissa_plus1);
                    assert_that(&self.test_function(next_mantissa_plus1)).is_greater_than_or_equal_to(&self.test_function(mantissa_plus1));
                    mantissa_plus1 = next_mantissa_plus1;
                }
                l += 1;
             }
         }

    }

    #[test]
    pub fn test_lower_bound_approximation(&self)   {
         let absolute_bin_width_limits: vec![Vec<f64>; 10] = vec![1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1e0, 1e1, 1e2, 1e3, ]
        ;
         let relative_bin_width_limits: vec![Vec<f64>; 12] = vec![0.0, 1e-100, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1e0, 1e1, 1e2, 1e3, ]
        ;
        for  let absolute_bin_width_limit: f64 in absolute_bin_width_limits {
            for  let relative_bin_width_limit: f64 in relative_bin_width_limits {
                 let layout: LogQuadraticLayout = LogQuadraticLayout::create(absolute_bin_width_limit, relative_bin_width_limit, -absolute_bin_width_limit * 1e6, absolute_bin_width_limit * 1e6);
                assert_that(&LayoutTestUtil::max_lower_bound_approximation_offset(layout)).is_less_than_or_equal_to(2000);
            }
        }
    }
}
