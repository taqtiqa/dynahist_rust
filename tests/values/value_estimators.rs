// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

const BIN1: Bin = Self::create_bin(10, 5, 20, 3, 8, 77);

const BIN2: Bin = Self::create_bin(0, 5, 30, 3, 8, 77);

const BIN3: Bin = Self::create_bin(30, 5, 0, 3, 8, 77);

const BIN4: Bin = Self::create_bin(0, 5, 0, 3, 8, 77);
pub struct ValueEstimatorImplsTest {}

impl ValueEstimatorImplsTest {
    fn create_bin(
        less_count: i64,
        bin_count: i64,
        greater_count: i64,
        lower_bound: f64,
        upper_bound: f64,
        bin_index: i32,
    ) -> Bin {
        fn new() {
            fn is_underflow_bin(&self) -> bool {
                return false;
            }

            fn is_overflow_bin(&self) -> bool {
                return false;
            }

            fn get_upper_bound(&self) -> f64 {
                return upper_bound;
            }

            fn get_lower_bound(&self) -> f64 {
                return lower_bound;
            }

            fn get_less_count(&self) -> i64 {
                return less_count;
            }

            fn get_greater_count(&self) -> i64 {
                return greater_count;
            }

            fn get_bin_index(&self) -> i32 {
                return bin_index;
            }

            fn get_bin_count(&self) -> i64 {
                return bin_count;
            }
        };
        return new();
    }

    #[test]
    fn test_lower_bound_value_estimation_policy(&self) {
        assert_eq!(
            &BIN1::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 10),
            0.0
        );
        assert_eq!(
            &BIN1::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 11),
            0.0
        );
        assert_eq!(
            &BIN1::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 12),
            0.0
        );
        assert_eq!(
            &BIN1::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 13),
            0.0
        );
        assert_eq!(
            &BIN1::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN1, 14),
            0.0
        );
        assert_eq!(
            &BIN2::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN2, 1),
            0.0
        );
        assert_eq!(
            &BIN2::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN2, 2),
            0.0
        );
        assert_eq!(
            &BIN2::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN2, 3),
            0.0
        );
        assert_eq!(
            &BIN2::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN2, 4),
            0.0
        );
        assert_eq!(
            &BIN3::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN3, 30),
            0.0
        );
        assert_eq!(
            &BIN3::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN3, 31),
            0.0
        );
        assert_eq!(
            &BIN3::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN3, 32),
            0.0
        );
        assert_eq!(
            &BIN3::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN3, 33),
            0.0
        );
        assert_eq!(
            &BIN4::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN4, 1),
            0.0
        );
        assert_eq!(
            &BIN4::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN4, 2),
            0.0
        );
        assert_eq!(
            &BIN4::get_lower_bound(),
            &ValueEstimatorImpls::LOWER_BOUND::get_estimate_from_bin(BIN4, 3),
            0.0
        );
    }

    #[test]
    fn test_upper_bound_value_estimation_policy(&self) {
        assert_eq!(
            &BIN1::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 10),
            0.0
        );
        assert_eq!(
            &BIN1::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 11),
            0.0
        );
        assert_eq!(
            &BIN1::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 12),
            0.0
        );
        assert_eq!(
            &BIN1::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 13),
            0.0
        );
        assert_eq!(
            &BIN1::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN1, 14),
            0.0
        );
        assert_eq!(
            &BIN2::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN2, 1),
            0.0
        );
        assert_eq!(
            &BIN2::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN2, 2),
            0.0
        );
        assert_eq!(
            &BIN2::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN2, 3),
            0.0
        );
        assert_eq!(
            &BIN2::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN2, 4),
            0.0
        );
        assert_eq!(
            &BIN3::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN3, 30),
            0.0
        );
        assert_eq!(
            &BIN3::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN3, 31),
            0.0
        );
        assert_eq!(
            &BIN3::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN3, 32),
            0.0
        );
        assert_eq!(
            &BIN3::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN3, 33),
            0.0
        );
        assert_eq!(
            &BIN4::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN4, 1),
            0.0
        );
        assert_eq!(
            &BIN4::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN4, 2),
            0.0
        );
        assert_eq!(
            &BIN4::get_upper_bound(),
            &ValueEstimatorImpls::UPPER_BOUND::get_estimate_from_bin(BIN4, 3),
            0.0
        );
    }

    #[test]
    fn test_mid_point_value_estimation_policy(&self) {
        assert_eq!(
            0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 10),
            0.0
        );
        assert_eq!(
            0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 11),
            0.0
        );
        assert_eq!(
            0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 12),
            0.0
        );
        assert_eq!(
            0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 13),
            0.0
        );
        assert_eq!(
            0.5 * (BIN1::get_lower_bound() + BIN1::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN1, 14),
            0.0
        );
        assert_eq!(
            0.5 * (BIN2::get_lower_bound() + BIN2::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN2, 1),
            0.0
        );
        assert_eq!(
            0.5 * (BIN2::get_lower_bound() + BIN2::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN2, 2),
            0.0
        );
        assert_eq!(
            0.5 * (BIN2::get_lower_bound() + BIN2::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN2, 3),
            0.0
        );
        assert_eq!(
            0.5 * (BIN2::get_lower_bound() + BIN2::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN2, 4),
            0.0
        );
        assert_eq!(
            0.5 * (BIN3::get_lower_bound() + BIN3::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN3, 30),
            0.0
        );
        assert_eq!(
            0.5 * (BIN3::get_lower_bound() + BIN3::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN3, 31),
            0.0
        );
        assert_eq!(
            0.5 * (BIN3::get_lower_bound() + BIN3::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN3, 32),
            0.0
        );
        assert_eq!(
            0.5 * (BIN3::get_lower_bound() + BIN3::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN3, 33),
            0.0
        );
        assert_eq!(
            0.5 * (BIN4::get_lower_bound() + BIN4::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN4, 1),
            0.0
        );
        assert_eq!(
            0.5 * (BIN4::get_lower_bound() + BIN4::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN4, 2),
            0.0
        );
        assert_eq!(
            0.5 * (BIN4::get_lower_bound() + BIN4::get_upper_bound()),
            &ValueEstimatorImpls::MID_POINT::get_estimate_from_bin(BIN4, 3),
            0.0
        );
    }

    #[test]
    fn test_uniform_value_estimation_policy(&self) {
        assert_eq!(
            3.5,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 10),
            0.0
        );
        assert_eq!(
            4.5,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 11),
            0.0
        );
        assert_eq!(
            5.5,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 12),
            0.0
        );
        assert_eq!(
            6.5,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 13),
            0.0
        );
        assert_eq!(
            7.5,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN1, 14),
            0.0
        );
        assert_eq!(
            3.0 + 2.0 / 9.0 * 5.0,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN2, 1),
            0.0
        );
        assert_eq!(
            3.0 + 4.0 / 9.0 * 5.0,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN2, 2),
            0.0
        );
        assert_eq!(
            3.0 + 6.0 / 9.0 * 5.0,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN2, 3),
            1e-9
        );
        assert_eq!(
            3.0 + 8.0 / 9.0 * 5.0,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN2, 4),
            0.0
        );
        assert_eq!(
            3.0 + 1.0 / 9.0 * 5.0,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN3, 30),
            0.0
        );
        assert_eq!(
            3.0 + 3.0 / 9.0 * 5.0,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN3, 31),
            0.0
        );
        assert_eq!(
            3.0 + 5.0 / 9.0 * 5.0,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN3, 32),
            0.0
        );
        assert_eq!(
            3.0 + 7.0 / 9.0 * 5.0,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN3, 33),
            0.0
        );
        assert_eq!(
            4.25,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN4, 1),
            1e-3
        );
        assert_eq!(
            5.5,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN4, 2),
            1e-3
        );
        assert_eq!(
            6.75,
            &ValueEstimatorImpls::UNIFORM::get_estimate_from_bin(BIN4, 3),
            1e-3
        );
    }

    // #[test]
    // fn test_negative_zero(&self) {
    //     for let value_estimator: ValueEstimatorImpls in ValueEstimatorImpls::values() {
    //         Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, 2, 0, -0.0, -0.0, 34), 1)))::is_less_than(0);
    //         Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, 2, 0, -2.0, -0.0, 34), 1)))::is_less_than(0);
    //         Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, 1, 0, -2.0, -0.0, 34), 0)))::is_less_than(0);
    //         Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, 1, 0, -0.0, -0.0, 34), 0)))::is_less_than(0);
    //         Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, i64::MAX, 0, -3.0, -0.0, 34), i64::MAX - 1)))::is_less_than(0);
    //         Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, i64::MAX, 0, -0.0, -0.0, 34), i64::MAX - 1)))::is_less_than(0);
    //     }
    // }

    // The upstream Java implementation uses:
    // // Assertions.assertThat(
    //           Double.doubleToLongBits(
    //               valueEstimator.getEstimateFromBin(
    //                   createBin(0, Long.MAX_VALUE, 0, -3., -0., 34), Long.MAX_VALUE - 1)))
    //       .isLessThan(0L)
    //
    // The [documentation](https://assertj.github.io/doc/#assertj-core-assertions-guide)
    // and usage give no indication that "soft assertions" are being used.
    //
    // We'll start with a `>` assertion and see how far we get.
    //
    #[test]
    fn test_negative_zero(&self) {
        //for let value_estimator: ValueEstimatorImpls in ValueEstimatorImpls::values() {
        let value_estimate = &ValueEstimatorUniform::new()
            .get_estimate_from_bin(::create_bin(0, 2, 0, -0.0, -0.0, 34), 1);
        assert!(to_bits_nan_collapse(value_estimate) < 0);
        // assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, 2, 0, -2.0, -0.0, 34), 1)))::is_less_than(0);
        // Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, 1, 0, -2.0, -0.0, 34), 0)))::is_less_than(0);
        // Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, 1, 0, -0.0, -0.0, 34), 0)))::is_less_than(0);
        // Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, i64::MAX, 0, -3.0, -0.0, 34), i64::MAX - 1)))::is_less_than(0);
        // Assertions::assert_that(&to_bits_nan_collapse(value_estimator.get_estimate_from_bin(&Self::create_bin(0, i64::MAX, 0, -0.0, -0.0, 34), i64::MAX - 1)))::is_less_than(0);
        // }
    }
}
