// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct LogLinearLayoutTest {
    super: AbstractErrorLimitingLayoutTest;
}

impl LogLinearLayoutTest {

    #[test]
    pub fn test(&self) {
        assert_true(4.0 * StrictMath::log1p(f64::MAX) <= 2840.0);
    }

    #[test]
    pub fn test_map_to_bin_index_helper_special_values(&self) {
        assert_eq!(2049.0, &LogLinearLayout::map_to_bin_index_helper(i64::MAX), 0.0);
        assert_eq!(2049.0, &LogLinearLayout::map_to_bin_index_helper(0x7fffffffffffffff), 0.0);
        assert_eq!(2048.5, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(f64::NAN)), 0.0);
        assert_eq!(2048.0, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(f64::INFINITY)), 0.0);
        assert_eq!(2.0, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(self.min_normal_f64())), 0.0);
        assert_eq!(1.0, &LogLinearLayout::map_to_bin_index_helper(0), 0.0);
        assert_eq!(1022.0, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(0.25)), 0.0);
        assert_eq!(1023.0, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(0.5)), 0.0);
        assert_eq!(1024.0, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(1.0)), 0.0);
        assert_eq!(1025.0, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(2.0)), 0.0);
        assert_eq!(1026.0, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(4.0)), 0.0);
        assert_eq!(1027.0, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(8.0)), 0.0);
        assert_eq!(1028.0, &LogLinearLayout::map_to_bin_index_helper(&to_bits_nan_collapse(16.0)), 0.0);
    }

    pub fn create_layout(&self,  absolute_bin_width_limit: f64,  relative_bin_width_limit: f64,  value_range_lower_bound: f64,  value_range_upper_bound: f64) -> GuessLayout {
        return LogLinearLayout::create(absolute_bin_width_limit, relative_bin_width_limit, value_range_lower_bound, value_range_upper_bound);
    }

    #[test]
    pub fn test_overflow_and_underflow_indices(&self) {
       {
             let layout: LogLinearLayout = LogLinearLayout::create(1e-7, 1e-6, -1e12, 1e12);
            assert_eq!(44219012, &layout.get_overflow_bin_index());
            assert_eq!(-44219013, &layout.get_underflow_bin_index());
        }
       {
             let layout: LogLinearLayout = LogLinearLayout::create(1e-7, 1e-6, 1e12, 1e12);
            assert_eq!(44219012, &layout.get_overflow_bin_index());
            assert_eq!(44219010, &layout.get_underflow_bin_index());
        }
    }

    #[test]
    pub fn test_serialization(&self)  -> Result<Void, Rc<DynaHistError>> {
         let value_range_upper_bound: f64 = 1e7;
         let value_range_lower_bound: f64 = -1e6;
         let relative_bin_width_limit: f64 = 1e-3;
         let absolute_bin_width_limit: f64 = 1e-9;
         let layout: LogLinearLayout = LogLinearLayout::create(absolute_bin_width_limit, relative_bin_width_limit, value_range_lower_bound, value_range_upper_bound);
         let deserialized_layout: LogLinearLayout = SerializationTestUtil::test_serialization(layout, LogLinearLayout::write, LogLinearLayout::read, "003E112E0BE826D6953F50624DD2F1A9FCDFFE048CB205");
        assert_eq!(deserialized_layout, layout);
    }

    #[test]
    pub fn test_to_string(&self) {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
        assert_eq!("LogLinearLayout [absoluteBinWidthLimit=1.0E-8, relativeBinWidthLimit=0.01, underflowBinIndex=-4107, overflowBinIndex=4106]", &layout.to_string());
    }

    #[test]
    pub fn test_get_width(&self) {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = Histogram::create_static(layout);
        histogram.add_value(0);
        histogram.add_value(10);
        assert_eq!(9.999999999999999E-9, &histogram.get_first_non_empty_bin().get_width(), 0);
        assert_eq!(0.057622250121310614, &histogram.get_last_non_empty_bin().get_width(), 0);
    }

    #[test]
    pub fn test_equals(&self) {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
        assert_false(&layout.equals(null));
        assert_false(&layout.equals(&LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6)));
        assert_false(&layout.equals(&LogLinearLayout::create(1e-7, 1e-2, -1e6, 1e6)));
        assert_false(&LogLinearLayout::create(1, 0, 1, 10)::equals(&LogLinearLayout::create(1, 1e-3, 1, 10)));
        assert_false(&layout.equals(&LogLinearLayout::create(1e-8, 1e-2, -1e5, 1e6)));
        assert_false(&layout.equals(&LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e5)));
    }

    #[test]
    pub fn test_initial_guesses(&self) {
         let absolute_bin_width_limits: vec![Vec<f64>; 10] = vec![1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1e0, 1e1, 1e2, 1e3, ]
        ;
         let relative_bin_width_limits: vec![Vec<f64>; 12] = vec![0.0, 1e-100, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1e0, 1e1, 1e2, 1e3, ]
        ;
        for  let absolute_bin_width_limit: f64 in absolute_bin_width_limits {
            for  let relative_bin_width_limit: f64 in relative_bin_width_limits {
                 let factor_normal: f64 = LogLinearLayout::calculate_factor_normal(relative_bin_width_limit);
                 let factor_subnormal: f64 = LogLinearLayout::calculate_factor_sub_normal(absolute_bin_width_limit);
                 let first_normal_idx: i32 = LogLinearLayout::calculate_first_normal_index(relative_bin_width_limit);
                 let unsigned_value_bits_normal_limit_approximate: i64 = LogLinearLayout::calculate_unsigned_value_bits_normal_limit_approximate(factor_subnormal, first_normal_idx);
                 let unsigned_value_bits_normal_limit: i64 = LogLinearLayout::calculate_unsigned_value_bits_normal_limit(factor_subnormal, first_normal_idx);
                 let offset_approximate: f64 = LogLinearLayout::calculate_offset_approximate(unsigned_value_bits_normal_limit, factor_normal, first_normal_idx);
                 let offset: f64 = LogLinearLayout::calculate_offset(unsigned_value_bits_normal_limit, factor_normal, first_normal_idx);
                assert_that(&Algorithms::map_double_to_long(offset_approximate)).is_close_to(&Algorithms::map_double_to_long(offset), &Offset::offset(1));
                assert_that(unsigned_value_bits_normal_limit_approximate).is_close_to(unsigned_value_bits_normal_limit, &Offset::offset(1));
            }
        }
    }

    #[test]
    pub fn test_hash_code(&self) {
        assert_eq!(-1299004750, &self.create_layout(1e-6, 1e-4, -10, 1000).hash_code());
    }
}
