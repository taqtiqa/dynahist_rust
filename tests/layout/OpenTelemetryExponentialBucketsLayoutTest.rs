/*
 * Copyright 2020-2021 Dynatrace LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::dynatrace::dynahist::layout;

pub struct OpenTelemetryExponentialBucketsLayoutTest {
}

impl OpenTelemetryExponentialBucketsLayoutTest {

    #[test]
    pub fn  test_consistency(&self)   {
         {
             let mut precision: i32 = 0;
            while precision <= MAX_PRECISION {
                {
                     let layout: Layout = OpenTelemetryExponentialBucketsLayout::create(precision);
                    LayoutTestUtil::assert_consistency(layout);
                }
                precision += 1;
             }
         }

    }

    #[test]
    pub fn  test_mapping0(&self)   {
         let layout: Layout = OpenTelemetryExponentialBucketsLayout::create(0);
        assert_equals(0, &layout.map_to_bin_index(&Double::long_bits_to_double(0)));
        assert_equals(1, &layout.map_to_bin_index(&Double::long_bits_to_double(1)));
        assert_equals(2, &layout.map_to_bin_index(&Double::long_bits_to_double(2)));
        assert_equals(2, &layout.map_to_bin_index(&Double::long_bits_to_double(3)));
        assert_equals(3, &layout.map_to_bin_index(&Double::long_bits_to_double(4)));
        assert_equals(3, &layout.map_to_bin_index(&Double::long_bits_to_double(5)));
        assert_equals(3, &layout.map_to_bin_index(&Double::long_bits_to_double(6)));
        assert_equals(3, &layout.map_to_bin_index(&Double::long_bits_to_double(7)));
        assert_equals(4, &layout.map_to_bin_index(&Double::long_bits_to_double(8)));
        assert_equals(4, &layout.map_to_bin_index(&Double::long_bits_to_double(9)));
        assert_equals(4, &layout.map_to_bin_index(&Double::long_bits_to_double(10)));
        assert_equals(4, &layout.map_to_bin_index(&Double::long_bits_to_double(11)));
        assert_equals(4, &layout.map_to_bin_index(&Double::long_bits_to_double(12)));
        assert_equals(4, &layout.map_to_bin_index(&Double::long_bits_to_double(13)));
        assert_equals(53, &layout.map_to_bin_index(Double::MIN_NORMAL));
        assert_equals(1074, &layout.map_to_bin_index(0.5));
        assert_equals(1075, &layout.map_to_bin_index(1));
        assert_equals(2097, &layout.map_to_bin_index(Double::MAX_VALUE / 2.0));
        assert_equals(2098, &layout.map_to_bin_index(Double::MAX_VALUE));
        assert_equals(2099, &layout.map_to_bin_index(Double::POSITIVE_INFINITY));
        assert_equals(2099, &// "smallest" NaN
        layout.map_to_bin_index(&Double::long_bits_to_double(0x7ff0000000000001)));
        assert_equals(2099, &// standard NaN
        layout.map_to_bin_index(&Double::long_bits_to_double(0x7ff8000000000000)));
        assert_equals(2099, &// "greatest" NaN
        layout.map_to_bin_index(&Double::long_bits_to_double(0x7fffffffffffffff)));
        assert_equals(0, &layout.map_to_bin_index(-Double::long_bits_to_double(0)));
        assert_equals(-1, &layout.map_to_bin_index(-Double::long_bits_to_double(1)));
        assert_equals(-2, &layout.map_to_bin_index(-Double::long_bits_to_double(2)));
        assert_equals(-2, &layout.map_to_bin_index(-Double::long_bits_to_double(3)));
        assert_equals(-3, &layout.map_to_bin_index(-Double::long_bits_to_double(4)));
        assert_equals(-3, &layout.map_to_bin_index(-Double::long_bits_to_double(5)));
        assert_equals(-3, &layout.map_to_bin_index(-Double::long_bits_to_double(6)));
        assert_equals(-3, &layout.map_to_bin_index(-Double::long_bits_to_double(7)));
        assert_equals(-4, &layout.map_to_bin_index(-Double::long_bits_to_double(8)));
        assert_equals(-4, &layout.map_to_bin_index(-Double::long_bits_to_double(9)));
        assert_equals(-4, &layout.map_to_bin_index(-Double::long_bits_to_double(10)));
        assert_equals(-4, &layout.map_to_bin_index(-Double::long_bits_to_double(11)));
        assert_equals(-4, &layout.map_to_bin_index(-Double::long_bits_to_double(12)));
        assert_equals(-4, &layout.map_to_bin_index(-Double::long_bits_to_double(13)));
        assert_equals(-53, &layout.map_to_bin_index(-Double::MIN_NORMAL));
        assert_equals(-1074, &layout.map_to_bin_index(-0.5));
        assert_equals(-1075, &layout.map_to_bin_index(-1));
        assert_equals(-2097, &layout.map_to_bin_index(-Double::MAX_VALUE / 2.0));
        assert_equals(-2098, &layout.map_to_bin_index(-Double::MAX_VALUE));
        assert_equals(-2099, &layout.map_to_bin_index(-Double::POSITIVE_INFINITY));
        assert_equals(-2099, &layout.map_to_bin_index(&Double::long_bits_to_double(0xfff0000000000001)));
        assert_equals(-2099, &layout.map_to_bin_index(&Double::long_bits_to_double(0xfff8000000000000)));
        assert_equals(-2099, &layout.map_to_bin_index(&Double::long_bits_to_double(0xffffffffffffffff)));
    }

    #[test]
    pub fn  test_mapping1(&self)   {
         let layout: Layout = OpenTelemetryExponentialBucketsLayout::create(1);
        assert_equals(0, &layout.map_to_bin_index(&Double::long_bits_to_double(0)));
        assert_equals(1, &layout.map_to_bin_index(&Double::long_bits_to_double(1)));
        assert_equals(2, &layout.map_to_bin_index(&Double::long_bits_to_double(2)));
        assert_equals(3, &layout.map_to_bin_index(&Double::long_bits_to_double(3)));
        assert_equals(4, &layout.map_to_bin_index(&Double::long_bits_to_double(4)));
        assert_equals(4, &layout.map_to_bin_index(&Double::long_bits_to_double(5)));
        assert_equals(5, &layout.map_to_bin_index(&Double::long_bits_to_double(6)));
        assert_equals(5, &layout.map_to_bin_index(&Double::long_bits_to_double(7)));
        assert_equals(6, &layout.map_to_bin_index(&Double::long_bits_to_double(8)));
        assert_equals(6, &layout.map_to_bin_index(&Double::long_bits_to_double(9)));
        assert_equals(6, &layout.map_to_bin_index(&Double::long_bits_to_double(10)));
        assert_equals(6, &layout.map_to_bin_index(&Double::long_bits_to_double(11)));
        assert_equals(7, &layout.map_to_bin_index(&Double::long_bits_to_double(12)));
        assert_equals(7, &layout.map_to_bin_index(&Double::long_bits_to_double(13)));
        assert_equals(104, &layout.map_to_bin_index(Double::MIN_NORMAL));
        assert_equals(2146, &layout.map_to_bin_index(0.5));
        assert_equals(2148, &layout.map_to_bin_index(1));
        assert_equals(4193, &layout.map_to_bin_index(Double::MAX_VALUE / 2.0));
        assert_equals(4195, &layout.map_to_bin_index(Double::MAX_VALUE));
        assert_equals(4196, &layout.map_to_bin_index(Double::POSITIVE_INFINITY));
        assert_equals(4196, &// "smallest" NaN
        layout.map_to_bin_index(&Double::long_bits_to_double(0x7ff0000000000001)));
        assert_equals(4197, &// standard NaN
        layout.map_to_bin_index(&Double::long_bits_to_double(0x7ff8000000000000)));
        assert_equals(4197, &// "greatest" NaN
        layout.map_to_bin_index(&Double::long_bits_to_double(0x7fffffffffffffff)));
        assert_equals(0, &layout.map_to_bin_index(-Double::long_bits_to_double(0)));
        assert_equals(-1, &layout.map_to_bin_index(-Double::long_bits_to_double(1)));
        assert_equals(-2, &layout.map_to_bin_index(-Double::long_bits_to_double(2)));
        assert_equals(-3, &layout.map_to_bin_index(-Double::long_bits_to_double(3)));
        assert_equals(-4, &layout.map_to_bin_index(-Double::long_bits_to_double(4)));
        assert_equals(-4, &layout.map_to_bin_index(-Double::long_bits_to_double(5)));
        assert_equals(-5, &layout.map_to_bin_index(-Double::long_bits_to_double(6)));
        assert_equals(-5, &layout.map_to_bin_index(-Double::long_bits_to_double(7)));
        assert_equals(-6, &layout.map_to_bin_index(-Double::long_bits_to_double(8)));
        assert_equals(-6, &layout.map_to_bin_index(-Double::long_bits_to_double(9)));
        assert_equals(-6, &layout.map_to_bin_index(-Double::long_bits_to_double(10)));
        assert_equals(-6, &layout.map_to_bin_index(-Double::long_bits_to_double(11)));
        assert_equals(-7, &layout.map_to_bin_index(-Double::long_bits_to_double(12)));
        assert_equals(-7, &layout.map_to_bin_index(-Double::long_bits_to_double(13)));
        assert_equals(-104, &layout.map_to_bin_index(-Double::MIN_NORMAL));
        assert_equals(-2146, &layout.map_to_bin_index(-0.5));
        assert_equals(-2148, &layout.map_to_bin_index(-1));
        assert_equals(-4193, &layout.map_to_bin_index(-Double::MAX_VALUE / 2.0));
        assert_equals(-4195, &layout.map_to_bin_index(-Double::MAX_VALUE));
        assert_equals(-4196, &layout.map_to_bin_index(-Double::POSITIVE_INFINITY));
        assert_equals(-4196, &layout.map_to_bin_index(&Double::long_bits_to_double(0xfff0000000000001)));
        assert_equals(-4197, &layout.map_to_bin_index(&Double::long_bits_to_double(0xfff8000000000000)));
        assert_equals(-4197, &layout.map_to_bin_index(&Double::long_bits_to_double(0xffffffffffffffff)));
    }

    #[test]
    pub fn  test_mapping2(&self)   {
         let layout: Layout = OpenTelemetryExponentialBucketsLayout::create(2);
        assert_equals(0, &layout.map_to_bin_index(&Double::long_bits_to_double(0)));
        assert_equals(1, &layout.map_to_bin_index(&Double::long_bits_to_double(1)));
        assert_equals(2, &layout.map_to_bin_index(&Double::long_bits_to_double(2)));
        assert_equals(3, &layout.map_to_bin_index(&Double::long_bits_to_double(3)));
        assert_equals(4, &layout.map_to_bin_index(&Double::long_bits_to_double(4)));
        assert_equals(5, &layout.map_to_bin_index(&Double::long_bits_to_double(5)));
        assert_equals(6, &layout.map_to_bin_index(&Double::long_bits_to_double(6)));
        assert_equals(7, &layout.map_to_bin_index(&Double::long_bits_to_double(7)));
        assert_equals(8, &layout.map_to_bin_index(&Double::long_bits_to_double(8)));
        assert_equals(8, &layout.map_to_bin_index(&Double::long_bits_to_double(9)));
        assert_equals(9, &layout.map_to_bin_index(&Double::long_bits_to_double(10)));
        assert_equals(9, &layout.map_to_bin_index(&Double::long_bits_to_double(11)));
        assert_equals(10, &layout.map_to_bin_index(&Double::long_bits_to_double(12)));
        assert_equals(10, &layout.map_to_bin_index(&Double::long_bits_to_double(13)));
        assert_equals(11, &layout.map_to_bin_index(&Double::long_bits_to_double(14)));
        assert_equals(11, &layout.map_to_bin_index(&Double::long_bits_to_double(15)));
        assert_equals(12, &layout.map_to_bin_index(&Double::long_bits_to_double(16)));
        assert_equals(12, &layout.map_to_bin_index(&Double::long_bits_to_double(17)));
        assert_equals(12, &layout.map_to_bin_index(&Double::long_bits_to_double(18)));
        assert_equals(12, &layout.map_to_bin_index(&Double::long_bits_to_double(19)));
        assert_equals(13, &layout.map_to_bin_index(&Double::long_bits_to_double(20)));
        assert_equals(204, &layout.map_to_bin_index(Double::MIN_NORMAL));
        assert_equals(4288, &layout.map_to_bin_index(0.5));
        assert_equals(4292, &layout.map_to_bin_index(1));
        assert_equals(8383, &layout.map_to_bin_index(Double::MAX_VALUE / 2.0));
        assert_equals(8387, &layout.map_to_bin_index(Double::MAX_VALUE));
        assert_equals(8388, &layout.map_to_bin_index(Double::POSITIVE_INFINITY));
        assert_equals(8388, &// "smallest" NaN
        layout.map_to_bin_index(&Double::long_bits_to_double(0x7ff0000000000001)));
        assert_equals(8390, &// standard NaN
        layout.map_to_bin_index(&Double::long_bits_to_double(0x7ff8000000000000)));
        assert_equals(8391, &// "greatest" NaN
        layout.map_to_bin_index(&Double::long_bits_to_double(0x7fffffffffffffff)));
        assert_equals(0, &layout.map_to_bin_index(-Double::long_bits_to_double(0)));
        assert_equals(-1, &layout.map_to_bin_index(-Double::long_bits_to_double(1)));
        assert_equals(-2, &layout.map_to_bin_index(-Double::long_bits_to_double(2)));
        assert_equals(-3, &layout.map_to_bin_index(-Double::long_bits_to_double(3)));
        assert_equals(-4, &layout.map_to_bin_index(-Double::long_bits_to_double(4)));
        assert_equals(-5, &layout.map_to_bin_index(-Double::long_bits_to_double(5)));
        assert_equals(-6, &layout.map_to_bin_index(-Double::long_bits_to_double(6)));
        assert_equals(-7, &layout.map_to_bin_index(-Double::long_bits_to_double(7)));
        assert_equals(-8, &layout.map_to_bin_index(-Double::long_bits_to_double(8)));
        assert_equals(-8, &layout.map_to_bin_index(-Double::long_bits_to_double(9)));
        assert_equals(-9, &layout.map_to_bin_index(-Double::long_bits_to_double(10)));
        assert_equals(-9, &layout.map_to_bin_index(-Double::long_bits_to_double(11)));
        assert_equals(-10, &layout.map_to_bin_index(-Double::long_bits_to_double(12)));
        assert_equals(-10, &layout.map_to_bin_index(-Double::long_bits_to_double(13)));
        assert_equals(-11, &layout.map_to_bin_index(-Double::long_bits_to_double(14)));
        assert_equals(-11, &layout.map_to_bin_index(-Double::long_bits_to_double(15)));
        assert_equals(-12, &layout.map_to_bin_index(-Double::long_bits_to_double(16)));
        assert_equals(-12, &layout.map_to_bin_index(-Double::long_bits_to_double(17)));
        assert_equals(-12, &layout.map_to_bin_index(-Double::long_bits_to_double(18)));
        assert_equals(-12, &layout.map_to_bin_index(-Double::long_bits_to_double(19)));
        assert_equals(-13, &layout.map_to_bin_index(-Double::long_bits_to_double(20)));
        assert_equals(-204, &layout.map_to_bin_index(-Double::MIN_NORMAL));
        assert_equals(-4288, &layout.map_to_bin_index(-0.5));
        assert_equals(-4292, &layout.map_to_bin_index(-1));
        assert_equals(-8383, &layout.map_to_bin_index(-Double::MAX_VALUE / 2.0));
        assert_equals(-8387, &layout.map_to_bin_index(-Double::MAX_VALUE));
        assert_equals(-8388, &layout.map_to_bin_index(-Double::POSITIVE_INFINITY));
        assert_equals(-8388, &layout.map_to_bin_index(&Double::long_bits_to_double(0xfff0000000000001)));
        assert_equals(-8390, &layout.map_to_bin_index(&Double::long_bits_to_double(0xfff8000000000000)));
        assert_equals(-8391, &layout.map_to_bin_index(&Double::long_bits_to_double(0xffffffffffffffff)));
    }

    #[test]
    pub fn  test_lower_bound_approximation(&self)   {
         {
             let mut precision: i32 = 0;
            while precision <= MAX_PRECISION {
                {
                     let layout: OpenTelemetryExponentialBucketsLayout = OpenTelemetryExponentialBucketsLayout::create(precision);
                    assert_that(&LayoutTestUtil::max_lower_bound_approximation_offset(layout)).is_equal_to(0);
                }
                precision += 1;
             }
         }

    }

    #[test]
    pub fn  test_boundary_consistency(&self)   {
         let tolerance: f64 = 1e-14;
         {
             let mut precision: i32 = 0;
            while precision <= MAX_PRECISION {
                {
                     let relative_error_limit: f64 = Math::pow(2.0, &Math::pow(2.0, -precision)) * (1.0 + tolerance);
                     let len: i32 = 1 << precision;
                     let boundaries: Vec<i64> = OpenTelemetryExponentialBucketsLayout::calculate_boundaries(precision);
                    assert_that(2 * boundaries[0]).is_greater_than_or_equal_to(1 << (52 - precision));
                     {
                         let mut i: i32 = 1;
                        while i < len {
                            {
                                assert_that(boundaries[i - 1]).is_less_than(boundaries[i]);
                            }
                            i += 1;
                         }
                     }

                     {
                         let mut i: i32 = 1;
                        while i < len {
                            {
                                assert_that(boundaries[i - 1] - ( if (i == 1) { 0 } else { boundaries[i - 2] })).is_less_than_or_equal_to(boundaries[i] - boundaries[i - 1]);
                            }
                            i += 1;
                         }
                     }

                     {
                         let mut i: i32 = 0;
                        while i < len {
                            {
                                 let low: f64 =  if (i > 0.0) { Double::long_bits_to_double(0x3ff0000000000000 | boundaries[i - 1]) } else { 1.0 };
                                 let high: f64 =  if (i < len - 1.0) { Double::long_bits_to_double(0x3ff0000000000000 | boundaries[i]) } else { 2.0 };
                                assert_that(low).is_less_than_or_equal_to(high);
                                assert_that(low * relative_error_limit).is_greater_than_or_equal_to(high);
                            }
                            i += 1;
                         }
                     }

                }
                precision += 1;
             }
         }

    }

    #[test]
    pub fn  test_hash_code(&self)   {
         let layout: Layout = OpenTelemetryExponentialBucketsLayout::create(3);
        assert_equals(93, &layout.hash_code());
    }

    #[test]
    pub fn  test_to_string(&self)   {
         let layout: Layout = OpenTelemetryExponentialBucketsLayout::create(3);
        assert_equals("OpenTelemetryExponentialBucketsLayout [precision=3]", &layout.to_string());
    }

    #[test]
    pub fn  test_equals(&self)   {
         let layout3a: Layout = OpenTelemetryExponentialBucketsLayout::create(3);
         let layout3b: Layout = OpenTelemetryExponentialBucketsLayout::create(3);
         let layout3c: Layout = OpenTelemetryExponentialBucketsLayout::new(3);
         let layout4: Layout = OpenTelemetryExponentialBucketsLayout::create(4);
        assert_true(layout3a == layout3b);
        assert_true(layout3a != layout3c);
        assert_true(&layout3a.equals(layout3a));
        assert_true(&layout3a.equals(layout3b));
        assert_true(&layout3a.equals(layout3c));
        assert_false(&layout3a.equals(layout4));
        assert_false(&layout3a.equals(Object::new()));
        assert_false(&layout3a.equals(null));
    }

    #[test]
    pub fn  test_create(&self)   {
        assert_throws(IllegalArgumentException.class, () -> OpenTelemetryExponentialBucketsLayout::create(-1));
        assert_throws(IllegalArgumentException.class, () -> OpenTelemetryExponentialBucketsLayout::create(MAX_PRECISION + 1));
    }

    #[test]
    pub fn  test_accuracy(&self)   {
         let tolerance: f64 = 1e-14;
         {
             let mut precision: i32 = 0;
            while precision <= MAX_PRECISION {
                {
                     let layout: OpenTelemetryExponentialBucketsLayout = OpenTelemetryExponentialBucketsLayout::create(precision);
                     let relative_error_limit: f64 = Math::pow(2.0, &Math::pow(2.0, -precision)) * (1.0 + tolerance);
                     {
                         let mut i: i32 = layout.get_underflow_bin_index() + 1;
                        while i < layout.get_overflow_bin_index() {
                            {
                                 let low: f64 = layout.get_bin_lower_bound(i);
                                 let high: f64 = layout.get_bin_upper_bound(i);
                                if low > 0.0 && high > 0.0 {
                                    assert_that(low).is_less_than_or_equal_to(high);
                                    assert_that(low * relative_error_limit).is_greater_than_or_equal_to(high);
                                } else if low < 0.0 && high < 0.0 {
                                    assert_that(low).is_less_than_or_equal_to(high);
                                    assert_that(high * relative_error_limit).is_less_than_or_equal_to(low);
                                } else {
                                    assert_equals(0.0, low, 0.0);
                                    assert_equals(0.0, high, 0.0);
                                }
                            }
                            i += 1;
                         }
                     }

                }
                precision += 1;
             }
         }

    }

    #[test]
    pub fn  test_inclusiveness(&self)   {
         {
             let mut precision: i32 = 0;
            while precision <= MAX_PRECISION {
                {
                     let layout: OpenTelemetryExponentialBucketsLayout = OpenTelemetryExponentialBucketsLayout::create(precision);
                     {
                         let mut exponent: i32 = -1000;
                        while exponent <= 1000 {
                            {
                                 let x: f64 = Math::pow(2.0, exponent);
                                assert_that(&layout.map_to_bin_index(x)).is_greater_than(&layout.map_to_bin_index(&Math::next_down(x)));
                                assert_that(&layout.map_to_bin_index(x)).is_equal_to(&layout.map_to_bin_index(&Math::next_up(x)));
                                assert_that(&layout.map_to_bin_index(-x)).is_less_than(&layout.map_to_bin_index(&Math::next_up(-x)));
                                assert_that(&layout.map_to_bin_index(-x)).is_equal_to(&layout.map_to_bin_index(&Math::next_down(-x)));
                            }
                            exponent += 1;
                         }
                     }

                }
                precision += 1;
             }
         }

    }

    #[test]
    pub fn  test_sqrt2(&self)   {
         let sqrt2_lower_bound: f64 = Math::next_down(&StrictMath::sqrt(2.0));
         let sqrt2_upper_bound: f64 = Math::next_up(sqrt2_lower_bound);
        assert_that(&Math::pow(sqrt2_lower_bound, 2.0)).is_less_than(2.0);
        assert_that(&Math::pow(sqrt2_upper_bound, 2.0)).is_greater_than(2.0);
         {
             let mut precision: i32 = 1;
            while precision <= MAX_PRECISION {
                {
                     let layout: OpenTelemetryExponentialBucketsLayout = OpenTelemetryExponentialBucketsLayout::create(precision);
                     {
                         let mut exponent: i32 = -100;
                        while exponent <= 100 {
                            {
                                assert_that(&layout.map_to_bin_index(sqrt2_upper_bound)).is_greater_than(&layout.map_to_bin_index(sqrt2_lower_bound));
                                assert_that(&layout.map_to_bin_index(sqrt2_upper_bound)).is_equal_to(&layout.map_to_bin_index(&Math::next_up(sqrt2_upper_bound)));
                                assert_that(&layout.map_to_bin_index(sqrt2_lower_bound)).is_equal_to(&layout.map_to_bin_index(&Math::next_down(sqrt2_lower_bound)));
                                assert_that(&layout.map_to_bin_index(-sqrt2_upper_bound)).is_less_than(&layout.map_to_bin_index(-sqrt2_lower_bound));
                                assert_that(&layout.map_to_bin_index(-sqrt2_upper_bound)).is_equal_to(&layout.map_to_bin_index(&Math::next_down(-sqrt2_upper_bound)));
                                assert_that(&layout.map_to_bin_index(-sqrt2_lower_bound)).is_equal_to(&layout.map_to_bin_index(&Math::next_up(-sqrt2_lower_bound)));
                            }
                            exponent += 1;
                         }
                     }

                }
                precision += 1;
             }
         }

    }

    #[test]
    pub fn  test_num_buckets(&self)   {
         let sb: StringBuilder = StringBuilder::new();
        sb.append(" p    num. positive buckets    relative bucket width\n");
        sb.append("----------------------------------------------------\n");
         {
             let mut precision: i32 = 0;
            while precision <= MAX_PRECISION {
                {
                     let layout: OpenTelemetryExponentialBucketsLayout = OpenTelemetryExponentialBucketsLayout::create(precision);
                     let precision_str: String = Integer::to_string(precision);
                     let bucket_str: String = Integer::to_string(layout.get_overflow_bin_index() - 1);
                     let width_str: String = format!("{} %", String::format("%.3f", ((Math::pow(2.0, &Math::pow(2.0, -precision)) - 1.0) * 100.0)));
                     let padded_precision_str: String = format!("{}{}", "  ".substring(&precision_str.length()), precision_str);
                     let padded_bucket_str: String = format!("{}{}", "                         ".substring(&bucket_str.length()), bucket_str);
                     let padded_width_str: String = format!("{}{}", "                         ".substring(&width_str.length()), width_str);
                    sb.append(&padded_precision_str).append(&padded_bucket_str).append(&padded_width_str).append('\n');
                }
                precision += 1;
             }
         }

        assert_that(&sb.to_string()).is_equal_to(format!(" p    num. positive buckets    relative bucket width\n----------------------------------------------------\n 0                     2098                100.000 %\n 1                     4195                 41.421 %\n 2                     8387                 18.921 %\n 3                    16767                  9.051 %\n 4                    33518                  4.427 %\n 5                    67005                  2.190 %\n 6                   133946                  1.089 %\n 7                   267764                  0.543 %\n 8                   535273                  0.271 %\n 9                  1070035                  0.135 %\n10                  2139047                  0.068 %\n"));
    }

    fn  calculate_expected_boundaries( precision: i32) -> Vec<i64>  {
         let len: i32 = 1 << precision;
         let mut boundaries: [i64; len] = [0; len];
        boundaries[0] = 0;
         {
             let mut i: i32 = 0;
            while i < len {
                {
                    boundaries[i] = ::calculate_boundary_exact(len, i);
                }
                i += 1;
             }
         }

        return boundaries;
    }

    #[test]
    pub fn  test_boundaries(&self)   {
         {
             let mut precision: i32 = 0;
            while precision <= MAX_PRECISION {
                {
                     let len: i32 = 1 << precision;
                     let expected_boundaries: Vec<i64> = ::calculate_expected_boundaries(precision);
                     let actual_boundaries: [i64; len] = [0; len];
                     let layout: OpenTelemetryExponentialBucketsLayout = OpenTelemetryExponentialBucketsLayout::create(precision);
                     let start_index: i32 = layout.map_to_bin_index(1.0);
                     {
                         let mut idx: i32 = 0;
                        while idx < len {
                            {
                                actual_boundaries[idx] = Double::double_to_raw_long_bits(&layout.get_bin_lower_bound(start_index + idx)) & 0xfffffffffffff;
                            }
                            idx += 1;
                         }
                     }

                    assert_that(&actual_boundaries).is_equal_to(&expected_boundaries);
                }
                precision += 1;
             }
         }

    }

    fn  calculate_boundary_approximate( len: i32,  i: i32) -> i64  {
        return 0x000fffffffffffff & Double::double_to_raw_long_bits(&Math::pow(2.0, i / len as f64));
    }

    // This function calculates the mantissa representation m
    // of the smallest double-precision floating-point value
    // x := (1 + m * 2^-52) for which x >= 2^(i/len).
    // This is equivalent to (1 + m * 2^-52) >= 2^(i/len)
    // and further to (2^52 + m)^(len) >= 2^(52 * len + i).
    // This inequality can be evaluated exactly
    // and platform-independent using BigInteger.
    // m can then be found using binary search.
    fn  calculate_boundary_exact( len: i32,  i: i32) -> i64  {
         let expected: BigInteger = BigInteger::value_of(2)::pow(52 * len + i);
         let predicate: LongPredicate =  m: & -> {
             let actual: BigInteger = BigInteger::value_of(m)::add(&BigInteger::value_of(1 << 52))::pow(len);
            return actual.compare_to(&expected) >= 0;
        };
         let initial_guess: i64 = ::calculate_boundary_approximate(len, i);
        return Algorithms::find_first(&predicate, 0x0000000000000000, 0x0010000000000000, initial_guess);
    }

    #[test]
    pub fn  test_precalculated_boundary_constans(&self)   {
         let mut expected: [i64; 1 << MAX_PRECISION] = [0; 1 << MAX_PRECISION];
         let mut actual: [i64; 1 << MAX_PRECISION] = [0; 1 << MAX_PRECISION];
         {
             let mut i: i32 = 0;
            while i < 1 << MAX_PRECISION {
                {
                    expected[i] = ::calculate_boundary_exact(1 << MAX_PRECISION, i);
                    actual[i] = get_boundary_constant(i);
                }
                i += 1;
             }
         }

        // System.out.println(LongStream.of(expected).mapToObj(l -> String.format("0x%013xL",
        // l)).collect(Collectors.joining(",", "{", "}")));
        assert_that(&actual).is_equal_to(&expected);
    }
}

