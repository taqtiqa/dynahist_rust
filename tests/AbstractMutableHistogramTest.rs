// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct AbstractMutableHistogramTest {
    super: AbstractHistogramTest;
}

impl AbstractMutableHistogramTest {

    pub fn add_values(&self,  histogram: impl Histogram,  values: f64) -> impl Histogram  {
        if values != null {
            for  let x: f64 in values {
                histogram.add_value(x);
            }
        }
        return histogram;
    }

    #[test]
    pub fn test_count_overflow(&self)   {
         let layout: Layout = TestLayout::new(-100, 100);
         let histogram: Histogram = create(layout);
        histogram.add_value(10.0, i64::MAX);
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
        assert_throws(ArithmeticException.class, () -> histogram.add_value(90.0));
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
    }

    #[test]
    pub fn test_na_n(&self)   {
         let layout: Layout = TestLayout::new(-100, 100);
         let histogram: Histogram = create(layout);
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.add_value(f64::NAN));
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
    }

    #[test]
    pub fn test_positive_infinity(&self)   {
         let layout: Layout = TestLayout::new(-100, 100);
         let histogram: Histogram = create(layout);
        histogram.add_value(f64::INFINITY);
        assert_eq!(1, &histogram.get_total_count());
        assert_eq!(f64::INFINITY, &histogram.get_min(), 0.0);
        assert_eq!(f64::INFINITY, &histogram.get_max(), 0.0);
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
    }

    #[test]
    pub fn test_negative_infinity(&self)   {
         let layout: Layout = TestLayout::new(-100, 100);
         let histogram: Histogram = create(layout);
        histogram.add_value(f64::NEG_INFINITY);
        assert_eq!(1, &histogram.get_total_count());
        assert_eq!(f64::NEG_INFINITY, &histogram.get_min(), 0.0);
        assert_eq!(f64::NEG_INFINITY, &histogram.get_max(), 0.0);
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
    }

    #[test]
    pub fn test_get_bin_by_rank(&self)   {
         const K: i64 = 57;
         const Z: i64 = 5;
         let layout: Layout = LogQuadraticLayout::create(1.0, 0.0, 0.0, K);
         let histogram: Histogram = create(layout);
         {
             let mut k: i64 = 0;
            while k < K {
                {
                    histogram.add_value(k, Z);
                }
                k += 1;
             }
         }

        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
         {
             let mut k: i32 = 0;
            while k < Z * K {
                {
                     let bin: BinIterator = histogram.get_bin_by_rank(k);
                    assert_eq!(Z, &bin.get_bin_count());
                    assert_eq!((k / Z) * Z, &bin.get_less_count());
                    assert_eq!(Z * K - (k / Z) * Z - Z, &bin.get_greater_count());
                }
                k += 1;
             }
         }

    }

    #[test]
    pub fn test_single_value_histogram_normal(&self)   {
         let histogram: Histogram = create(TestLayout::new(-100, 100)).add_value(5);
        assert_eq!(1, &HistogramTestUtil::number_of_non_empty_bins(histogram));
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
    }

    #[test]
    pub fn test_single_value_histogram_underflow(&self)   {
         let histogram: Histogram = create(TestLayout::new(-100, 100)).add_value(1000);
        assert_eq!(1, &HistogramTestUtil::number_of_non_empty_bins(histogram));
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
    }

    #[test]
    pub fn test_single_value_histogram_overflow(&self)   {
         let histogram: Histogram = create(TestLayout::new(-100, 100)).add_value(-1000);
        assert_eq!(1, &HistogramTestUtil::number_of_non_empty_bins(histogram));
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
    }

    #[test]
    pub fn test_two_values_histogram_underflow_and_overflow(&self)   {
         let histogram: Histogram = create(TestLayout::new(-100, 100)).add_value(-1000).add_value(1000);
        assert_eq!(2, &HistogramTestUtil::number_of_non_empty_bins(histogram));
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
    }

    #[test]
    pub fn test_general(&self)   {
         const N: i32 = 10_000;
         let random: Random = Random::new(0);
         {
             let mut i: i32 = 0;
            while i < N {
                {
                     let layout: Layout = TestLayout::new(-1000, 1000);
                     let histogram: Histogram = create(layout);
                     const N_Data: i32 = random.next_int(50);
                     let count_base: i64 = 1 << random.next_int(33);
                     let total_count: i64 = 0;
                     let mut min: f64 = f64::INFINITY;
                     let mut max: f64 = f64::NEG_INFINITY;
                     {
                         let mut j: i32 = 0;
                        while j < N_Data {
                            {
                                 let count: i64 = count_base + random.next_int(10);
                                 let value: f64 = random.next_double() * 200.0 - 100.0;
                                histogram.add_value(value, count);
                                total_count += count;
                                min = std::cmp::min(min, value);
                                max = std::cmp::max(max, value);
                            }
                            j += 1;
                         }
                     }

                    // verify total count, min, and max
                    assert_eq!(total_count, &histogram.get_total_count());
                    assert_eq!(min, &histogram.get_min(), 0.0);
                    assert_eq!(max, &histogram.get_max(), 0.0);
                    if total_count > 0 {
                        assert_eq!(min, &histogram.get_value(0), 0.0);
                        assert_eq!(max, &histogram.get_value(total_count - 1), 0.0);
                    }
                    HistogramTestUtil::check_histogram_data_consistency(histogram);
                    HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
                    test_serialization(layout, histogram);
                }
                i += 1;
             }
         }

    }

    #[test]
    pub fn test_get_preprocessed_data(&self)   {
         let histogram: Histogram = create(TestLayout::new(-100, 100));
        histogram.add_value(-101, 3);
        histogram.add_value(-53, 2);
        histogram.add_value(3, 4);
        histogram.add_value(106, 1);
         let preprocessed_histogram: Histogram = histogram.get_preprocessed_copy();
        assert_eq!(histogram, preprocessed_histogram);
        HistogramTestUtil::check_histogram_data_consistency(preprocessed_histogram);
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        assert_eq!(106, &preprocessed_histogram.get_max(), 0);
        assert_eq!(-101, &preprocessed_histogram.get_min(), 0);
        assert_eq!(10, &preprocessed_histogram.get_total_count());
        assert_eq!(-100, &preprocessed_histogram.get_bin_by_rank(0).get_bin_index());
        assert_eq!(0, &preprocessed_histogram.get_bin_by_rank(0).get_less_count());
        assert_eq!(3, &preprocessed_histogram.get_bin_by_rank(0).get_bin_count());
        assert_eq!(7, &preprocessed_histogram.get_bin_by_rank(0).get_greater_count());
        assert_eq!(-100, &preprocessed_histogram.get_first_non_empty_bin().get_bin_index());
        assert_eq!(0, &preprocessed_histogram.get_first_non_empty_bin().get_less_count());
        assert_eq!(3, &preprocessed_histogram.get_first_non_empty_bin().get_bin_count());
        assert_eq!(7, &preprocessed_histogram.get_first_non_empty_bin().get_greater_count());
        assert_eq!(-100, &preprocessed_histogram.get_bin_by_rank(1).get_bin_index());
        assert_eq!(0, &preprocessed_histogram.get_bin_by_rank(1).get_less_count());
        assert_eq!(3, &preprocessed_histogram.get_bin_by_rank(1).get_bin_count());
        assert_eq!(7, &preprocessed_histogram.get_bin_by_rank(1).get_greater_count());
        assert_eq!(-100, &preprocessed_histogram.get_bin_by_rank(2).get_bin_index());
        assert_eq!(0, &preprocessed_histogram.get_bin_by_rank(2).get_less_count());
        assert_eq!(3, &preprocessed_histogram.get_bin_by_rank(2).get_bin_count());
        assert_eq!(7, &preprocessed_histogram.get_bin_by_rank(2).get_greater_count());
        assert_eq!(-53, &preprocessed_histogram.get_bin_by_rank(3).get_bin_index());
        assert_eq!(3, &preprocessed_histogram.get_bin_by_rank(3).get_less_count());
        assert_eq!(2, &preprocessed_histogram.get_bin_by_rank(3).get_bin_count());
        assert_eq!(5, &preprocessed_histogram.get_bin_by_rank(3).get_greater_count());
        assert_eq!(-53, &preprocessed_histogram.get_bin_by_rank(4).get_bin_index());
        assert_eq!(3, &preprocessed_histogram.get_bin_by_rank(4).get_less_count());
        assert_eq!(2, &preprocessed_histogram.get_bin_by_rank(4).get_bin_count());
        assert_eq!(5, &preprocessed_histogram.get_bin_by_rank(4).get_greater_count());
        assert_eq!(3, &preprocessed_histogram.get_bin_by_rank(5).get_bin_index());
        assert_eq!(5, &preprocessed_histogram.get_bin_by_rank(5).get_less_count());
        assert_eq!(4, &preprocessed_histogram.get_bin_by_rank(5).get_bin_count());
        assert_eq!(1, &preprocessed_histogram.get_bin_by_rank(5).get_greater_count());
        assert_eq!(3, &preprocessed_histogram.get_bin_by_rank(6).get_bin_index());
        assert_eq!(5, &preprocessed_histogram.get_bin_by_rank(6).get_less_count());
        assert_eq!(4, &preprocessed_histogram.get_bin_by_rank(6).get_bin_count());
        assert_eq!(1, &preprocessed_histogram.get_bin_by_rank(6).get_greater_count());
        assert_eq!(3, &preprocessed_histogram.get_bin_by_rank(7).get_bin_index());
        assert_eq!(5, &preprocessed_histogram.get_bin_by_rank(7).get_less_count());
        assert_eq!(4, &preprocessed_histogram.get_bin_by_rank(7).get_bin_count());
        assert_eq!(1, &preprocessed_histogram.get_bin_by_rank(7).get_greater_count());
        assert_eq!(3, &preprocessed_histogram.get_bin_by_rank(8).get_bin_index());
        assert_eq!(5, &preprocessed_histogram.get_bin_by_rank(8).get_less_count());
        assert_eq!(4, &preprocessed_histogram.get_bin_by_rank(8).get_bin_count());
        assert_eq!(1, &preprocessed_histogram.get_bin_by_rank(8).get_greater_count());
        assert_eq!(100, &preprocessed_histogram.get_bin_by_rank(9).get_bin_index());
        assert_eq!(9, &preprocessed_histogram.get_bin_by_rank(9).get_less_count());
        assert_eq!(1, &preprocessed_histogram.get_bin_by_rank(9).get_bin_count());
        assert_eq!(0, &preprocessed_histogram.get_bin_by_rank(9).get_greater_count());
        assert_eq!(100, &preprocessed_histogram.get_last_non_empty_bin().get_bin_index());
        assert_eq!(9, &preprocessed_histogram.get_last_non_empty_bin().get_less_count());
        assert_eq!(1, &preprocessed_histogram.get_last_non_empty_bin().get_bin_count());
        assert_eq!(0, &preprocessed_histogram.get_last_non_empty_bin().get_greater_count());
    }

    #[test]
    pub fn test_add_ascending_sequence(&self)   {
         let layout: TestLayout = TestLayout::new(-5, 5);
         let num_cycles: i32 = 10000;
         let rnd: Random = Random::new(0);
         {
             let mut i: i32 = 0;
            while i < num_cycles {
                {
                     let values: Vec<f64> = rnd.doubles(&rnd.next_int(100)).map( d: & -> d * 12 - 6).to_array();
                    Arrays::sort(&values);
                     let histogram1: Histogram = create(layout);
                     let histogram2: Histogram = create(layout);
                    for  let v: f64 in values {
                        histogram1.add_value(v);
                    }
                    histogram2.add_ascending_sequence( j: & -> values[j as i32], values.len());
                    assert_eq!(histogram1, histogram2);
                    assert_eq!(&histogram1.get_preprocessed_copy(), &histogram2.get_preprocessed_copy());
                }
                i += 1;
             }
         }

    }

    #[test]
    pub fn test_add_ascending_sequence_unique_value(&self)   {
         let layout: TestLayout = TestLayout::new(-5, 5);
         let values: vec![Vec<f64>; 5] = vec![f64::NEG_INFINITY, -5.5, -0.1, 5.3, f64::INFINITY, ]
        ;
        for  let value: f64 in values {
             let histogram1: Histogram = create(layout);
             let histogram2: Histogram = create(layout);
            histogram1.add_value(value, i64::MAX);
            histogram2.add_ascending_sequence( j: & -> value, i64::MAX);
            assert_eq!(histogram1, histogram2);
        }
    }

    #[test]
    pub fn test_add_ascending_sequence_invalid_length(&self)   {
         let layout: TestLayout = TestLayout::new(-5, 5);
         let histogram: Histogram = create(layout);
         let values: vec![Vec<f64>; 5] = vec![f64::NEG_INFINITY, -5.5, -0.1, 5.3, f64::INFINITY, ]
        ;
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.add_ascending_sequence( j: & -> values[j as i32], -1));
        histogram.add_value(1, i64::MAX);
    // assertThrows(
    //     ArithmeticException.class,
    //     () -> histogram.addAscendingSequence(j -> values[(int) j], Long.MAX_VALUE));
    }

    #[test]
    pub fn test_histogram(&self)   {
         let layout: TestLayout = TestLayout::new(-2, 2);
         let histogram: Histogram = create(layout);
        histogram.add_value(f64::NEG_INFINITY);
        histogram.add_value(-2, 3);
        histogram.add_value(-1, 4);
        histogram.add_value(0, 1);
        histogram.add_value(1, 3);
        histogram.add_value(2, 2);
        assert_eq!(14, &histogram.get_total_count());
        assert_eq!(f64::NEG_INFINITY, &histogram.get_value(0), 0.0);
        assert_eq!(f64::NEG_INFINITY, &histogram.get_value(1), 0.0);
        assert_eq!(f64::NEG_INFINITY, &histogram.get_value(2), 0.0);
        assert_eq!(f64::NEG_INFINITY, &histogram.get_value(3), 0.0);
        assert_eq!(-1.3749999999999998, &histogram.get_value(4), 0.0);
        assert_eq!(-1.5 + 3.0 / 8.0, &histogram.get_value(5), 0.0);
        assert_eq!(-0.8749999999999998, &histogram.get_value(6), 0.0);
        assert_eq!(-1.5 + 7.0 / 8.0, &histogram.get_value(7), 0.0);
        assert_eq!(0, &histogram.get_value(8), 0.0);
        assert_eq!(0.6666666666666665, &histogram.get_value(9), 0.0);
        assert_eq!(0.9999999999999998, &histogram.get_value(10), 0.0);
        assert_eq!(4.0 / 3.0, &histogram.get_value(11), 1e-14);
        assert_eq!(1.6666666666666667, &histogram.get_value(12), 0.0);
        assert_eq!(2, &histogram.get_value(13), 0.0);
        test_serialization(layout, histogram);
    }

    #[test]
    pub fn test_add_histogram_equal_layout(&self)   {
         let cycles: i32 = 1000;
         let random: Random = Random::new(0);
         let layout: Layout = TestLayout::new(-100, 100);
         {
             let cycle_counter: i32 = 0;
            while cycle_counter < cycles {
                {
                     let num_values1: i32 = random.next_int(1000);
                     let num_values2: i32 = random.next_int(1000);
                     let histogram1: Histogram = create(layout);
                     let histogram2: Histogram = create(layout);
                     let histogram_total: Histogram = create(layout);
                     {
                         let mut i: i32 = 0;
                        while i < num_values1 {
                            {
                                 let value: f64 = -101.0 + random.next_double() * 202.0;
                                histogram1.add_value(value);
                                histogram_total.add_value(value);
                            }
                            i += 1;
                         }
                     }

                     {
                         let mut i: i32 = 0;
                        while i < num_values2 {
                            {
                                 let value: f64 = -101.0 + random.next_double() * 202.0;
                                histogram2.add_value(value);
                                histogram_total.add_value(value);
                            }
                            i += 1;
                         }
                     }

                     let histogram_merged: Histogram = histogram1.add_histogram(histogram2);
                    assert_eq!(histogram_total, histogram_merged);
                    assert_eq!(&histogram_total.hash_code(), &histogram_merged.hash_code());
                }
                cycle_counter += 1;
             }
         }

    }

    #[test]
    pub fn test_add_histogram_non_equal_layout(&self)   {
         let cycles: i32 = 1000;
         let random: Random = Random::new(0);
         let layout1: Layout = TestLayout::new(-100, 100);
         let layout2: Layout = TestLayout::new(-100, 101);
         let layout_total: Layout = layout1;
         {
             let cycle_counter: i32 = 0;
            while cycle_counter < cycles {
                {
                     let num_values1: i32 = random.next_int(1000);
                     let num_values2: i32 = random.next_int(1000);
                     let histogram1: Histogram = create(layout1);
                     let histogram2: Histogram = create(layout2);
                     let histogram_total: Histogram = create(layout_total);
                     {
                         let mut i: i32 = 0;
                        while i < num_values1 {
                            {
                                 let value: f64 = -101.0 + random.next_double() * 202.0;
                                histogram1.add_value(value);
                                histogram_total.add_value(value);
                            }
                            i += 1;
                         }
                     }

                     {
                         let mut i: i32 = 0;
                        while i < num_values2 {
                            {
                                 let value: f64 = -101.0 + random.next_double() * 202.0;
                                histogram2.add_value(value);
                                histogram_total.add_value(value);
                            }
                            i += 1;
                         }
                     }

                     let histogram_merged: Histogram = histogram1.add_histogram(histogram2);
                    assert_eq!(histogram_total, histogram_merged);
                    assert_eq!(&histogram_total.hash_code(), &histogram_merged.hash_code());
                }
                cycle_counter += 1;
             }
         }

    }

    #[test]
    pub fn test_add_empty_histogram(&self)   {
         let cycles: i32 = 100;
         let random: Random = Random::new(0);
         let layout: Layout = TestLayout::new(-100, 100);
         {
             let cycle_counter: i32 = 0;
            while cycle_counter < cycles {
                {
                     let num_values: i32 = random.next_int(10);
                     let histogram1: Histogram = create(layout);
                     let histogram2: Histogram = create(layout);
                     {
                         let mut i: i32 = 0;
                        while i < num_values {
                            {
                                 let value: f64 = -101.0 + random.next_double() * 202.0;
                                histogram1.add_value(value);
                                histogram2.add_value(value);
                            }
                            i += 1;
                         }
                     }

                    histogram2.add_histogram(&create(layout));
                    assert_eq!(histogram1, histogram2);
                    assert_eq!(&histogram1.hash_code(), &histogram2.hash_code());
                }
                cycle_counter += 1;
             }
         }

    }

    #[test]
    pub fn test_add_negative_count(&self)   {
         let layout: Layout = TestLayout::new(-100, 100);
         let histogram: Histogram = create(layout);
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.add_value(2.4, -1));
    }

    #[test]
    pub fn test_add_zero_count(&self)   {
         let layout: Layout = TestLayout::new(-100, 100);
         let histogram: Histogram = create(layout);
        histogram.add_value(2.4, 0);
        assert_true(&histogram.is_empty());
        test_serialization(layout, histogram);
    }

    #[test]
    pub fn test_quantile_of_single_value(&self)   {
         let value: f64 = 5.4;
         let layout: Layout = TestLayout::new(-100, 100);
         let histogram: Histogram = create(layout);
        histogram.add_value(value);
        assert_eq!(value, &histogram.get_quantile(0), 0.0);
        assert_eq!(value, &histogram.get_quantile(0.2), 0.0);
        assert_eq!(value, &histogram.get_quantile(0.5), 0.0);
        assert_eq!(value, &histogram.get_quantile(0.7), 0.0);
        assert_eq!(value, &histogram.get_quantile(1), 0.0);
        test_serialization(layout, histogram);
    }

    #[test]
    pub fn test_non_empty_bins(&self)   {
         const K: i32 = 1000;
         let layout: Layout = LogQuadraticLayout::create(1.0, 0.0, 0.0, K);
         let num_cycles: i32 = 100;
         let random: Random = Random::new(0);
         {
             let mut i: i32 = 0;
            while i < num_cycles {
                {
                     let histogram: Histogram = create(layout);
                     let mut count: i64 = 0;
                     let non_empty_bins_count: i32 = 0;
                     {
                         let mut k: i32 = 0;
                        while k < K {
                            {
                                if random.next_boolean() {
                                     let n: i32 = random.next_int(1_000_000);
                                    non_empty_bins_count += 1;
                                    count += n;
                                    histogram.add_value(k, n);
                                }
                            }
                            k += 1;
                         }
                     }

                    assert_eq!(count, &histogram.get_total_count());
                    assert_eq!(non_empty_bins_count, &HistogramTestUtil::number_of_non_empty_bins(histogram));
                    test_serialization(layout, histogram);
                }
                i += 1;
             }
         }

    }

    #[test]
    pub fn test_empty_histogram(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-3, 0.0, 0.0, 10.0);
         let histogram: Histogram = create(layout);
        assert_eq!(0, &HistogramTestUtil::number_of_non_empty_bins(histogram));
        test_serialization(layout, histogram);
    }

    #[test]
    pub fn test_serialization1(&self)   {
         let rnd: Random = Random::new(0);
         let num_cycles: i32 = 10000;
         {
             let mut cycle: i32 = 0;
            while cycle < num_cycles {
                {
                     let layout: Layout = TestLayout::new(-10 + rnd.next_int(10), &rnd.next_int(10));
                     let histogram: Histogram = create(layout);
                     let num_values: i32 = rnd.next_int(20);
                     {
                         let mut i: i32 = 0;
                        while i < num_values {
                            {
                                histogram.add_value(-11 + rnd.next_int(24));
                            }
                            i += 1;
                         }
                     }

                    test_serialization(layout, histogram);
                }
                cycle += 1;
             }
         }

    }

    #[test]
    pub fn test_serialization2(&self)   {
         let rnd: Random = Random::new(0);
         let num_cycles: i32 = 10000;
         {
             let mut cycle: i32 = 0;
            while cycle < num_cycles {
                {
                     let layout: Layout = TestLayout::new(-10 + rnd.next_int(10), &rnd.next_int(10));
                     let histogram: Histogram = create(layout);
                     let num_values: i32 = rnd.next_int(20);
                     {
                         let mut i: i32 = 0;
                        while i < num_values {
                            {
                                histogram.add_value(-11 + rnd.next_int(24), 1 << rnd.next_int(34));
                            }
                            i += 1;
                         }
                     }

                    test_serialization(layout, histogram);
                }
                cycle += 1;
             }
         }

    }

    #[test]
    pub fn test_minimal_layout(&self)   {
         let layout: Layout = TestLayout::new(-1, 0);
         let histogram: Histogram = create(layout);
        histogram.add_value(1000);
        histogram.add_value(-1000);
        test_serialization(layout, histogram);
        assert_eq!(-1000, &histogram.get_min(), 0.0);
        assert_eq!(1000, &histogram.get_max(), 0.0);
        assert_eq!(-1000, &histogram.get_value(0), 0.0);
        assert_eq!(1000, &histogram.get_value(1), 0.0);
        assert_eq!(-1000, &histogram.get_quantile(0), 0.0);
        assert_eq!(1000, &histogram.get_quantile(1), 0.0);
        assert_eq!(0, &histogram.get_quantile(0.5), 0.0);
    }

    #[test]
    pub fn test_very_small_effective_bin(&self)   {
         let layout: Layout = TestLayout::new(-100, 100);
         let x_values: vec![Vec<f64>; 4] = vec![-12143.43, -12.0, 34.535, 21314234.0, ]
        ;
         let c1: i64 = 432;
         let c2: i64 = 331;
        for  let min: f64 in x_values {
             let max: f64 = Math::next_up(min);
             let histogram: Histogram = create(layout);
            histogram.add_value(min, c1);
            histogram.add_value(max, c2);
            assert_eq!(c1 + c2, &histogram.get_total_count());
             let mut previous: f64 = histogram.get_value(0);
            assert_that(previous).is_greater_than_or_equal_to(min);
             {
                 let mut i: i64 = 1;
                while i < c1 + c2 {
                    {
                         let current: f64 = histogram.get_value(i);
                        assert_that(previous).is_less_than_or_equal_to(current);
                        previous = current;
                    }
                    i += 1;
                 }
             }

            assert_that(previous).is_less_than_or_equal_to(max);
        }
    }

    #[test]
    pub fn test_negative_zero(&self)   {
         let layout: Layout = TestLayout::new(-1, 1);
        {
             let histogram: Histogram = create(layout);
            histogram.add_value(0.0);
            histogram.add_value(-0.0);
            assert_eq!(&Double::double_to_raw_long_bits(-0.0), &Double::double_to_raw_long_bits(&histogram.get_min()));
            assert_eq!(&Double::double_to_raw_long_bits(0.0), &Double::double_to_raw_long_bits(&histogram.get_max()));
        }
        {
             let histogram: Histogram = create(layout);
            histogram.add_value(-0.0);
            histogram.add_value(0.0);
            assert_eq!(&Double::double_to_raw_long_bits(-0.0), &Double::double_to_raw_long_bits(&histogram.get_min()));
            assert_eq!(&Double::double_to_raw_long_bits(0.0), &Double::double_to_raw_long_bits(&histogram.get_max()));
        }
        {
             let histogram: Histogram = create(layout);
            histogram.add_value(0.0);
            histogram.add_value(0.0);
            assert_eq!(&Double::double_to_raw_long_bits(0.0), &Double::double_to_raw_long_bits(&histogram.get_min()));
            assert_eq!(&Double::double_to_raw_long_bits(0.0), &Double::double_to_raw_long_bits(&histogram.get_max()));
        }
        {
             let histogram: Histogram = create(layout);
            histogram.add_value(-0.0);
            histogram.add_value(-0.0);
            assert_eq!(&Double::double_to_raw_long_bits(-0.0), &Double::double_to_raw_long_bits(&histogram.get_min()));
            assert_eq!(&Double::double_to_raw_long_bits(-0.0), &Double::double_to_raw_long_bits(&histogram.get_max()));
        }
    }

    #[test]
    pub fn test_get_estimated_footprint_in_byte(&self)   {
    }

    #[test]
    pub fn test_add_histogram_first_non_empty_bin_equals_last_non_empty_bin(&self)   {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram1: Histogram = create(layout);
         let histogram2: Histogram = create(layout);
         let total_histogram: Histogram = create(layout);
        histogram1.add_value(5);
        total_histogram.add_value(5);
        total_histogram.add_value(-5);
        total_histogram.add_value(5.5, 5);
        histogram1.add_value(-5);
        histogram2.add_value(5.5, 5);
        assert_eq!(total_histogram, &histogram1.add_histogram(histogram2));
    }

    #[test]
    pub fn test_add_histogram_overflow(&self)   {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram1: Histogram = create(layout);
         let histogram2: Histogram = create(layout);
        histogram1.add_value(5, 1000000);
        histogram2.add_value(5, i64::MAX);
        assert_throws(ArithmeticException.class, () -> histogram1.add_histogram(histogram2));
    }

    #[test]
    pub fn test_deserialize_invalid_serial_version(&self)   {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let data_input_stream: DataInputStream = DataInputStream::new(ByteArrayInputStream::new( : vec![i8; 1] = vec![1, ]
        ));
        assert_throws(IOException.class, () -> impl Histogram::read_as_dynamic(layout, data_input_stream));
    }

    #[test]
    pub fn test_get_value_estimate_invalid_order(&self)   {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = create(layout);
        histogram.add_value(5);
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.get_value(-1));
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.get_value(1));
    }

    #[test]
    pub fn test_get_bin_by_rank_invalid_order(&self)   {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = create(layout);
        histogram.add_value(5);
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.get_bin_by_rank(-1));
        assert_throws(DynaHist::IllegalArgumentError.class, () -> histogram.get_bin_by_rank(1));
    }

    #[test]
    pub fn test_equals(&self)   {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = create(layout);
         let other_histogram: Histogram = create(layout);
        assert_false(&histogram.equals(null));
        assert_not_equals(histogram, &create(&LogLinearLayout::create(1e-8, 1e-2, -1e5, 1e5)));
        histogram.add_value(1e4);
        assert_not_equals(histogram, other_histogram);
        other_histogram.add_value(-1e7 * 2);
        assert_not_equals(histogram, other_histogram);
        histogram.add_value(-1e7);
        other_histogram.add_value(1e7);
        assert_not_equals(histogram, other_histogram);
        histogram.add_value(1e7 * 2.0);
        other_histogram.add_value(1e6);
        assert_not_equals(histogram, other_histogram);
        histogram.add_value(-1e7 * 2);
        other_histogram.add_value(-1e7);
        assert_not_equals(histogram, other_histogram);
        other_histogram.add_value(1e7 * 2.0);
        histogram.add_value(1e2);
        histogram.add_value(1e7);
        other_histogram.add_value(1e4);
        assert_not_equals(histogram, other_histogram);
        histogram.add_value(1e2);
        other_histogram.add_value(1e2);
        assert_not_equals(histogram, other_histogram);
    }

    #[test]
    pub fn test_total_count_overflow(&self)   {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = create(layout);
        histogram.add_value(1, i64::MAX);
        assert_throws(ArithmeticException.class, () -> histogram.add_value(2));
    }

    fn test_add_histogram_helper( histogram_factory1: &Function<Layout, Histogram>,  histogram_factory2: &Function<Layout, Histogram>)   {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let random: SplittableRandom = SplittableRandom::new(0);
         let num_values1: i64 = 1000;
         let num_values2: i64 = 2000;
         let histogram1: Histogram = histogram_factory1.apply(layout);
         let histogram2: Histogram = histogram_factory2.apply(layout);
         let histogram_total: Histogram = histogram_factory1.apply(layout);
        DoubleStream::generate(random::nextDouble)::limit(num_values1)::for_each( x: & -> {
            histogram1.add_value(x);
            histogram_total.add_value(x);
        });
        // DoubleStream.generate(random::nextDouble)
        //     .limit(numValues2)
        //     .forEach(
        //         x -> {
        //           histogram2.addValue(x);
        //           histogramTotal.addValue(x);
        //         });
        histogram1.add_histogram(histogram2);
        assert_eq!(histogram_total, histogram1);
    }

    #[test]
    pub fn test_add_histogram(&self)   {
        ::test_add_histogram_helper(self::create, Histogram::createDynamic);
        ::test_add_histogram_helper(self::create, Histogram::createStatic);
    }

    #[test]
    pub fn test_add_histogram_with_static(&self)   {
         let layout: Layout = LogLinearLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let random: SplittableRandom = SplittableRandom::new(0);
         let num_values1: i64 = 1000;
         let num_values2: i64 = 2000;
         let histogram1: Histogram = create(layout);
         let histogram2: Histogram = Histogram::create_dynamic(layout);
         let histogram_total: Histogram = create(layout);
        DoubleStream::generate(random::nextDouble)::limit(num_values1)::for_each( x: & -> {
            histogram1.add_value(x);
            histogram_total.add_value(x);
        });
        // DoubleStream.generate(random::nextDouble)
        //     .limit(numValues2)
        //     .forEach(
        //         x -> {
        //           histogram2.addValue(x);
        //           histogramTotal.addValue(x);
        //         });
        histogram1.add_histogram(histogram2);
        assert_eq!(histogram_total, histogram1);
    }

    #[test]
    pub fn test_add_histogram_with_under_and_over_flow_counts_only(&self)   {
         let layout: Layout = TestLayout::new(-1, 1);
         let histogram1: Histogram = create(layout);
         let histogram2: Histogram = Histogram::create_dynamic(layout);
         let histogram_total: Histogram = create(layout);
        histogram1.add_value(-2, 1000);
        histogram_total.add_value(-2, 1000);
        histogram1.add_value(2, 2000);
        histogram_total.add_value(2, 2000);
        histogram2.add_value(-2, 4000);
        histogram_total.add_value(-2, 4000);
        histogram2.add_value(2, 8000);
        histogram_total.add_value(2, 8000);
        histogram1.add_histogram(histogram2);
        assert_eq!(histogram_total, histogram1);
    }

    #[test]
    pub fn test_is_mutable(&self)   {
         let layout: Layout = TestLayout::new(-1, 1);
         let histogram: Histogram = create(layout);
        assert_true(&histogram.is_mutable());
    }

    #[test]
    pub fn test_deserialization_using_wrong_layout(&self)  -> Result<Void, Rc<DynaHistError>>   {
         let layouts: List<Layout> = Arrays::as_list(&LogLinearLayout::create(1e-1, 1e-1, -5, 5), &LogQuadraticLayout::create(1e-1, 1e-1, -5, 5), &LogLinearLayout::create(1.1e-1, 1e-1, -5, 5), &LogQuadraticLayout::create(1.1e-1, 1e-1, -5, 5), &LogLinearLayout::create(1e-1, 1.1e-1, -5, 5), &LogQuadraticLayout::create(1e-1, 1.1e-1, -5, 5), &CustomLayout::create(-2, 4, 5), &CustomLayout::create(-2), &CustomLayout::create(1));
         let num_iterations: i64 = 10000;
         let random: SplittableRandom = SplittableRandom::new(0);
         {
             let mut i: i32 = 0;
            while i < num_iterations {
                {
                    for  let write_layout: Layout in layouts {
                        for  let read_layout: Layout in layouts {
                             let histogram: Histogram = create(write_layout);
                             let num_values: i64 = random.next_long(100);
                             {
                                 let mut j: i64 = 0;
                                while j < num_values {
                                    {
                                        histogram.add_value(&random.next_double(-6, 6));
                                    }
                                    j += 1;
                                 }
                             }

                             let deserialized_histogram: Histogram = SerializationTestUtil::test_serialization(histogram, Histogram::write,  in: & -> read(read_layout, in));
                            assert_eq!(&histogram.get_total_count(), &deserialized_histogram.get_total_count());
                            assert_eq!(&histogram.get_min(), &deserialized_histogram.get_min(), 0.0);
                            assert_eq!(&histogram.get_max(), &deserialized_histogram.get_max(), 0.0);
                        }
                    }
                }
                i += 1;
             }
         }

    }

    #[test]
    pub fn test_deserialization_special(&self)  -> Result<Void, Rc<DynaHistError>>   {
         let min: f64 = -100;
         let max: f64 = 120;
         let min_regular_idx: i32 = -30;
         let max_regular_idx: i32 = 40;
         let underflow_count: i64 = 2000;
         let overflow_count: i64 = 1000;
         let total_count: i64 = 0;
        total_count += underflow_count;
        total_count += overflow_count;
         let sb: StringBuilder = StringBuilder::new();
        // serial version
        sb.append("00");
        // info byte
        sb.append("FF");
        // minimum
        sb.append(&byte_array_to_hex_string(&to_byte_array(( v: &,  d: &) -> d.write_double(v), min)));
        // sb.append(byteArrayToHexString(toByteArray((v, d) -> d.writeDouble(v), max))); // maximum
        sb.append(&byte_array_to_hex_string(&to_byte_array(SerializationUtil::writeUnsignedVarLong, // underflow count
        underflow_count)));
        sb.append(&byte_array_to_hex_string(&// overflow count
        to_byte_array(SerializationUtil::writeUnsignedVarLong, overflow_count)));
        sb.append(&byte_array_to_hex_string(&// regular min index
        to_byte_array(SerializationUtil::writeSignedVarInt, min_regular_idx)));
        sb.append(&byte_array_to_hex_string(&// regular max index
        to_byte_array(SerializationUtil::writeSignedVarInt, max_regular_idx)));
         {
             let mut idx: i32 = min_regular_idx;
            while idx <= max_regular_idx {
                {
                    sb.append(&byte_array_to_hex_string(&to_byte_array(( i: &,  d: &) -> d.write_long(i), 1)));
                    total_count += 1;
                }
                idx += 1;
             }
         }

        // for min and max and first and last regular bin
        total_count += 4;
         let layout: Layout = TestLayout::new(-2, 2);
         let histogram: Histogram = SerializationTestUtil::test_reading( in: & -> read(layout, in), &sb.to_string());
        assert_eq!(total_count, &histogram.get_total_count());
        assert_eq!(min, &histogram.get_min(), 0.0);
        assert_eq!(max, &histogram.get_max(), 0.0);
    }
}
