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
// package com::dynatrace::dynahist::value;


 const BIN1: Bin = ::create_bin(10, 5, 20, 3, 8, 77);

 const BIN2: Bin = ::create_bin(0, 5, 30, 3, 8, 77);

 const BIN3: Bin = ::create_bin(30, 5, 0, 3, 8, 77);

 const BIN4: Bin = ::create_bin(0, 5, 0, 3, 8, 77);
pub struct ValueEstimatorImplsTest {
}

impl ValueEstimatorImplsTest {

    fn  create_bin( less_count: i64,  bin_count: i64,  greater_count: i64,  lower_bound: f64,  upper_bound: f64,  bin_index: i32) -> Bin  {
        return Bin::new() {

            pub fn  is_underflow_bin(&self) -> bool  {
                return false;
            }

            pub fn  is_overflow_bin(&self) -> bool  {
                return false;
            }

            pub fn  get_upper_bound(&self) -> f64  {
                return upper_bound;
            }

            pub fn  get_lower_bound(&self) -> f64  {
                return lower_bound;
            }

            pub fn  get_less_count(&self) -> i64  {
                return less_count;
            }

            pub fn  get_greater_count(&self) -> i64  {
                return greater_count;
            }

            pub fn  get_bin_index(&self) -> i32  {
                return bin_index;
            }

            pub fn  get_bin_count(&self) -> i64  {
                return bin_count;
            }
        };
    }

    #[test]
    pub fn  test_lower_bound_value_estimation_policy(&self)   {
        assert_equals(&BIN1::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 10), 0.0);
        assert_equals(&BIN1::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 11), 0.0);
        assert_equals(&BIN1::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 12), 0.0);
        assert_equals(&BIN1::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 13), 0.0);
        assert_equals(&BIN1::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 14), 0.0);
        assert_equals(&BIN2::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN2, 1), 0.0);
        assert_equals(&BIN2::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN2, 2), 0.0);
        assert_equals(&BIN2::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN2, 3), 0.0);
        assert_equals(&BIN2::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN2, 4), 0.0);
        assert_equals(&BIN3::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN3, 30), 0.0);
        assert_equals(&BIN3::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN3, 31), 0.0);
        assert_equals(&BIN3::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN3, 32), 0.0);
        assert_equals(&BIN3::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN3, 33), 0.0);
        assert_equals(&BIN4::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN4, 1), 0.0);
        assert_equals(&BIN4::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN4, 2), 0.0);
        assert_equals(&BIN4::get_lower_bound(), &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN4, 3), 0.0);
    }

    #[test]
    pub fn  test_upper_bound_value_estimation_policy(&self)   {
        assert_equals(&BIN1::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 10), 0.0);
        assert_equals(&BIN1::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 11), 0.0);
        assert_equals(&BIN1::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 12), 0.0);
        assert_equals(&BIN1::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 13), 0.0);
        assert_equals(&BIN1::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 14), 0.0);
        assert_equals(&BIN2::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN2, 1), 0.0);
        assert_equals(&BIN2::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN2, 2), 0.0);
        assert_equals(&BIN2::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN2, 3), 0.0);
        assert_equals(&BIN2::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN2, 4), 0.0);
        assert_equals(&BIN3::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN3, 30), 0.0);
        assert_equals(&BIN3::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN3, 31), 0.0);
        assert_equals(&BIN3::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN3, 32), 0.0);
        assert_equals(&BIN3::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN3, 33), 0.0);
        assert_equals(&BIN4::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN4, 1), 0.0);
        assert_equals(&BIN4::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN4, 2), 0.0);
        assert_equals(&BIN4::get_upper_bound(), &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN4, 3), 0.0);
    }

    #[test]
    pub fn  test_mid_point_value_estimation_policy(&self)   {
        assert_equals(0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 10), 0.0);
        assert_equals(0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 11), 0.0);
        assert_equals(0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 12), 0.0);
        assert_equals(0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 13), 0.0);
        assert_equals(0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 14), 0.0);
        assert_equals(0.5 * (BIN2::get_lower_bound() + BIN2::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN2, 1), 0.0);
        assert_equals(0.5 * (BIN2::get_lower_bound() + BIN2::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN2, 2), 0.0);
        assert_equals(0.5 * (BIN2::get_lower_bound() + BIN2::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN2, 3), 0.0);
        assert_equals(0.5 * (BIN2::get_lower_bound() + BIN2::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN2, 4), 0.0);
        assert_equals(0.5 * (BIN3::get_lower_bound() + BIN3::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN3, 30), 0.0);
        assert_equals(0.5 * (BIN3::get_lower_bound() + BIN3::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN3, 31), 0.0);
        assert_equals(0.5 * (BIN3::get_lower_bound() + BIN3::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN3, 32), 0.0);
        assert_equals(0.5 * (BIN3::get_lower_bound() + BIN3::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN3, 33), 0.0);
        assert_equals(0.5 * (BIN4::get_lower_bound() + BIN4::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN4, 1), 0.0);
        assert_equals(0.5 * (BIN4::get_lower_bound() + BIN4::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN4, 2), 0.0);
        assert_equals(0.5 * (BIN4::get_lower_bound() + BIN4::get_upper_bound()), &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN4, 3), 0.0);
    }

    #[test]
    pub fn  test_uniform_value_estimation_policy(&self)   {
        assert_equals(3.5, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 10), 0.0);
        assert_equals(4.5, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 11), 0.0);
        assert_equals(5.5, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 12), 0.0);
        assert_equals(6.5, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 13), 0.0);
        assert_equals(7.5, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 14), 0.0);
        assert_equals(3.0 + 2.0 / 9.0 * 5.0, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN2, 1), 0.0);
        assert_equals(3.0 + 4.0 / 9.0 * 5.0, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN2, 2), 0.0);
        assert_equals(3.0 + 6.0 / 9.0 * 5.0, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN2, 3), 1e-9);
        assert_equals(3.0 + 8.0 / 9.0 * 5.0, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN2, 4), 0.0);
        assert_equals(3.0 + 1.0 / 9.0 * 5.0, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN3, 30), 0.0);
        assert_equals(3.0 + 3.0 / 9.0 * 5.0, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN3, 31), 0.0);
        assert_equals(3.0 + 5.0 / 9.0 * 5.0, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN3, 32), 0.0);
        assert_equals(3.0 + 7.0 / 9.0 * 5.0, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN3, 33), 0.0);
        assert_equals(4.25, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN4, 1), 1e-3);
        assert_equals(5.5, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN4, 2), 1e-3);
        assert_equals(6.75, &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN4, 3), 1e-3);
    }

    #[test]
    pub fn  test_negative_zero(&self)   {
        for  let value_estimator: ValueEstimatorImpls in ValueEstimatorImpls::values() {
            Assertions::assert_that(&Double::double_to_long_bits(&value_estimator.get_estimate_from_bin(&::create_bin(0, 2, 0, -0.0, -0.0, 34), 1)))::is_less_than(0);
            Assertions::assert_that(&Double::double_to_long_bits(&value_estimator.get_estimate_from_bin(&::create_bin(0, 2, 0, -2.0, -0.0, 34), 1)))::is_less_than(0);
            Assertions::assert_that(&Double::double_to_long_bits(&value_estimator.get_estimate_from_bin(&::create_bin(0, 1, 0, -2.0, -0.0, 34), 0)))::is_less_than(0);
            Assertions::assert_that(&Double::double_to_long_bits(&value_estimator.get_estimate_from_bin(&::create_bin(0, 1, 0, -0.0, -0.0, 34), 0)))::is_less_than(0);
            Assertions::assert_that(&Double::double_to_long_bits(&value_estimator.get_estimate_from_bin(&::create_bin(0, Long::MAX_VALUE, 0, -3.0, -0.0, 34), Long::MAX_VALUE - 1)))::is_less_than(0);
            Assertions::assert_that(&Double::double_to_long_bits(&value_estimator.get_estimate_from_bin(&::create_bin(0, Long::MAX_VALUE, 0, -0.0, -0.0, 34), Long::MAX_VALUE - 1)))::is_less_than(0);
        }
    }
}

