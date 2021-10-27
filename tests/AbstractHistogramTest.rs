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
// package com::dynatrace::dynahist;

pub struct AbstractHistogramTest {
}

impl AbstractHistogramTest {

    pub fn  create(&self,  layout: &Layout) -> Histogram ;

    pub fn  read(&self,  layout: &Layout,  data_input: &DataInput) -> /*  throws IOException */Result<Histogram, Rc<Exception>>  ;

    pub fn  add_values(&self,  histogram: &Histogram,  values: f64) -> Histogram ;

    #[test]
    pub fn  test_to_string(&self)   {
         let layout: Layout = TestLayout::new(-100, 100);
         let mut histogram: Histogram = self.create(layout);
        assert_eq!(format!("{} [layout={}, underFlowCount=0, overFlowCount=0, totalCount=0, min=Infinity, max=-Infinity, counts={}]", histogram.get_class().get_simple_name(), layout), &histogram.to_string());
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
        histogram = self.add_values(histogram, 0);
        assert_eq!(format!("{} [layout={}, underFlowCount=0, overFlowCount=0, totalCount=1, min=0.0, max=0.0, counts={0: 1}]", histogram.get_class().get_simple_name(), layout), &histogram.to_string());
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
        histogram = self.add_values(histogram, 1);
        assert_eq!(format!("{} [layout={}, underFlowCount=0, overFlowCount=0, totalCount=2, min=0.0, max=1.0, counts={0: 1, 1: 1}]", histogram.get_class().get_simple_name(), layout), &histogram.to_string());
        HistogramTestUtil::check_histogram_data_consistency(histogram);
        HistogramTestUtil::check_histogram_data_consistency(&histogram.get_preprocessed_copy());
    }

    #[test]
    pub fn  test_serialization(&self,  layout: &Layout,  histogram: &Histogram)   {
        let tryResult1 = 0;
        'try1: loop {
        {
             let deserialized_histogram: Histogram = SerializationTestUtil::test_serialization(histogram, Histogram::write,  in: & -> self.read(layout, in));
            assert_eq!(histogram, deserialized_histogram);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &IOException) {
                throw UncheckedIOException::new(&e);
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
             let deserialized_histogram: Histogram = SerializationTestUtil::test_serialization(histogram, Histogram::write,  in: & -> Histogram::read_as_static(layout, in));
            assert_eq!(histogram, deserialized_histogram);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &IOException) {
                throw UncheckedIOException::new(&e);
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
             let deserialized_histogram: Histogram = SerializationTestUtil::test_serialization(histogram, Histogram::write,  in: & -> Histogram::read_as_dynamic(layout, in));
            assert_eq!(histogram, deserialized_histogram);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &IOException) {
                throw UncheckedIOException::new(&e);
            }  0 => break
        }

    }

    #[test]
    pub fn  test_hash_code(&self)   {
         let layout: Layout = TestLayout::new(-10, 10);
        {
             let histogram: Histogram = self.create(layout);
            assert_eq!(2115170828, &histogram.hash_code());
        }
        {
             let mut histogram: Histogram = self.create(layout);
            histogram = self.add_values(histogram, 1);
            assert_eq!(-987848916, &histogram.hash_code());
        }
        {
             let mut histogram: Histogram = self.create(layout);
            histogram = self.add_values(histogram, 1, 2);
            assert_eq!(-2115795891, &histogram.hash_code());
        }
        {
             let mut histogram: Histogram = self.create(layout);
            histogram = self.add_values(histogram, -1, -2, 1, 2);
            assert_eq!(-235792952, &histogram.hash_code());
        }
        {
             let mut histogram: Histogram = self.create(layout);
            histogram = self.add_values(histogram, -3 - 2, -1, -2, 1, 2, 3);
            assert_eq!(-299804540, &histogram.hash_code());
        }
    }

    #[test]
    pub fn  test_serialization_of_empty_histogram(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = self.create(layout);
        self.test_serialization(layout, histogram);
    }

    #[test]
    pub fn  test_same_equals(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let histogram: Histogram = self.create(layout);
        assert_true(&histogram.equals(histogram));
    }

    #[test]
    pub fn  test_get_value(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let mut histogram: Histogram = self.create(layout);
        histogram = self.add_values(histogram, 2, 2, 2, 2, 2);
        assert_eq!(2, &histogram.get_value(0), 0.0);
        assert_eq!(2, &histogram.get_value(0, ValueEstimator::LOWER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_value(0, ValueEstimator::UPPER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_value(0, ValueEstimator::MID_POINT), 0.0);
        assert_eq!(2, &histogram.get_value(0, ValueEstimator::UNIFORM), 0.0);
    }

    #[test]
    pub fn  test_get_quantile(&self)   {
         let layout: Layout = LogQuadraticLayout::create(1e-8, 1e-2, -1e6, 1e6);
         let mut histogram: Histogram = self.create(layout);
        histogram = self.add_values(histogram, 2, 2, 2, 2, 2);
        assert_eq!(2, &histogram.get_quantile(0), 0.0);
        assert_eq!(2, &histogram.get_quantile(0, ValueEstimator::LOWER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(0, ValueEstimator::UPPER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(0, ValueEstimator::MID_POINT), 0.0);
        assert_eq!(2, &histogram.get_quantile(0, ValueEstimator::UNIFORM), 0.0);
        assert_eq!(2, &histogram.get_quantile(0, &SciPyQuantileEstimator::create()), 0.0);
        assert_eq!(2, &histogram.get_quantile(0, &SciPyQuantileEstimator::create(), ValueEstimator::LOWER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(0, &SciPyQuantileEstimator::create(), ValueEstimator::UPPER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(0, &SciPyQuantileEstimator::create(), ValueEstimator::MID_POINT), 0.0);
        assert_eq!(2, &histogram.get_quantile(0, &SciPyQuantileEstimator::create(), ValueEstimator::UNIFORM), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5, ValueEstimator::LOWER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5, ValueEstimator::UPPER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5, ValueEstimator::MID_POINT), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5, ValueEstimator::UNIFORM), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5, &SciPyQuantileEstimator::create()), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5, &SciPyQuantileEstimator::create(), ValueEstimator::LOWER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5, &SciPyQuantileEstimator::create(), ValueEstimator::UPPER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5, &SciPyQuantileEstimator::create(), ValueEstimator::MID_POINT), 0.0);
        assert_eq!(2, &histogram.get_quantile(0.5, &SciPyQuantileEstimator::create(), ValueEstimator::UNIFORM), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0, ValueEstimator::LOWER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0, ValueEstimator::UPPER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0, ValueEstimator::MID_POINT), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0, ValueEstimator::UNIFORM), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0, &SciPyQuantileEstimator::create()), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0, &SciPyQuantileEstimator::create(), ValueEstimator::LOWER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0, &SciPyQuantileEstimator::create(), ValueEstimator::UPPER_BOUND), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0, &SciPyQuantileEstimator::create(), ValueEstimator::MID_POINT), 0.0);
        assert_eq!(2, &histogram.get_quantile(1.0, &SciPyQuantileEstimator::create(), ValueEstimator::UNIFORM), 0.0);
    }

    #[test]
    pub fn  test_non_empty_bins_ascending_iterator_for_non_empty_histogram(&self)   {
         let mut histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
        histogram = self.add_values(histogram, -3, -3, -3, -3, -1, -1, 3, 3, 3, 3, 3);
         let bins: List<Bin> = ArrayList<>::new();
        for  let bin: Bin in histogram.non_empty_bins_ascending() {
            bins.add(bin);
        }
        assert_eq!(3, &bins.size());
        assert_eq!(4, &bins.get(0).get_bin_count());
        assert_eq!(7, &bins.get(0).get_greater_count());
        assert_eq!(0, &bins.get(0).get_less_count());
        assert_eq!(1, &bins.get(0).get_bin_index());
        assert_eq!(-3.0, &bins.get(0).get_lower_bound(), 0.0);
        assert_eq!(&Math::next_down(-2.0), &bins.get(0).get_upper_bound(), 0.0);
        assert_eq!(2, &bins.get(1).get_bin_count());
        assert_eq!(5, &bins.get(1).get_greater_count());
        assert_eq!(4, &bins.get(1).get_less_count());
        assert_eq!(2, &bins.get(1).get_bin_index());
        assert_eq!(-2.0, &bins.get(1).get_lower_bound(), 0.0);
        assert_eq!(-0.0, &bins.get(1).get_upper_bound(), 0.0);
        assert_eq!(5, &bins.get(2).get_bin_count());
        assert_eq!(0, &bins.get(2).get_greater_count());
        assert_eq!(6, &bins.get(2).get_less_count());
        assert_eq!(4, &bins.get(2).get_bin_index());
        assert_eq!(2.0, &bins.get(2).get_lower_bound(), 0.0);
        assert_eq!(3.0, &bins.get(2).get_upper_bound(), 0.0);
    }

    #[test]
    pub fn  test_non_empty_bins_ascending_for_each_for_non_empty_histogram(&self)   {
         let mut histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
        histogram = self.add_values(histogram, -3, -3, -3, -3, -1, -1, 3, 3, 3, 3, 3);
         let bins: List<Bin> = ArrayList<>::new();
        histogram.non_empty_bins_ascending().for_each(bins::add);
        assert_eq!(3, &bins.size());
        assert_eq!(4, &bins.get(0).get_bin_count());
        assert_eq!(7, &bins.get(0).get_greater_count());
        assert_eq!(0, &bins.get(0).get_less_count());
        assert_eq!(1, &bins.get(0).get_bin_index());
        assert_eq!(-3.0, &bins.get(0).get_lower_bound(), 0.0);
        assert_eq!(&Math::next_down(-2.0), &bins.get(0).get_upper_bound(), 0.0);
        assert_eq!(2, &bins.get(1).get_bin_count());
        assert_eq!(5, &bins.get(1).get_greater_count());
        assert_eq!(4, &bins.get(1).get_less_count());
        assert_eq!(2, &bins.get(1).get_bin_index());
        assert_eq!(-2.0, &bins.get(1).get_lower_bound(), 0.0);
        assert_eq!(-0.0, &bins.get(1).get_upper_bound(), 0.0);
        assert_eq!(5, &bins.get(2).get_bin_count());
        assert_eq!(0, &bins.get(2).get_greater_count());
        assert_eq!(6, &bins.get(2).get_less_count());
        assert_eq!(4, &bins.get(2).get_bin_index());
        assert_eq!(2.0, &bins.get(2).get_lower_bound(), 0.0);
        assert_eq!(3.0, &bins.get(2).get_upper_bound(), 0.0);
    }

    #[test]
    pub fn  test_non_empty_bins_descending_iterator_for_non_empty_histogram(&self)   {
         let mut histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
        histogram = self.add_values(histogram, -3, -3, -3, -3, -1, -1, 3, 3, 3, 3, 3);
         let bins: List<Bin> = ArrayList<>::new();
        for  let bin: Bin in histogram.non_empty_bins_descending() {
            bins.add(bin);
        }
        assert_eq!(3, &bins.size());
        assert_eq!(5, &bins.get(0).get_bin_count());
        assert_eq!(0, &bins.get(0).get_greater_count());
        assert_eq!(6, &bins.get(0).get_less_count());
        assert_eq!(4, &bins.get(0).get_bin_index());
        assert_eq!(2.0, &bins.get(0).get_lower_bound(), 0.0);
        assert_eq!(3.0, &bins.get(0).get_upper_bound(), 0.0);
        assert_eq!(2, &bins.get(1).get_bin_count());
        assert_eq!(5, &bins.get(1).get_greater_count());
        assert_eq!(4, &bins.get(1).get_less_count());
        assert_eq!(2, &bins.get(1).get_bin_index());
        assert_eq!(-2.0, &bins.get(1).get_lower_bound(), 0.0);
        assert_eq!(-0.0, &bins.get(1).get_upper_bound(), 0.0);
        assert_eq!(4, &bins.get(2).get_bin_count());
        assert_eq!(7, &bins.get(2).get_greater_count());
        assert_eq!(0, &bins.get(2).get_less_count());
        assert_eq!(1, &bins.get(2).get_bin_index());
        assert_eq!(-3.0, &bins.get(2).get_lower_bound(), 0.0);
        assert_eq!(&Math::next_down(-2.0), &bins.get(2).get_upper_bound(), 0.0);
    }

    #[test]
    pub fn  test_non_empty_bins_descending_for_each_for_non_empty_histogram(&self)   {
         let mut histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
        histogram = self.add_values(histogram, -3, -3, -3, -3, -1, -1, 3, 3, 3, 3, 3);
         let bins: List<Bin> = ArrayList<>::new();
        histogram.non_empty_bins_descending().for_each(bins::add);
        assert_eq!(3, &bins.size());
        assert_eq!(5, &bins.get(0).get_bin_count());
        assert_eq!(0, &bins.get(0).get_greater_count());
        assert_eq!(6, &bins.get(0).get_less_count());
        assert_eq!(4, &bins.get(0).get_bin_index());
        assert_eq!(2.0, &bins.get(0).get_lower_bound(), 0.0);
        assert_eq!(3.0, &bins.get(0).get_upper_bound(), 0.0);
        assert_eq!(2, &bins.get(1).get_bin_count());
        assert_eq!(5, &bins.get(1).get_greater_count());
        assert_eq!(4, &bins.get(1).get_less_count());
        assert_eq!(2, &bins.get(1).get_bin_index());
        assert_eq!(-2.0, &bins.get(1).get_lower_bound(), 0.0);
        assert_eq!(-0.0, &bins.get(1).get_upper_bound(), 0.0);
        assert_eq!(4, &bins.get(2).get_bin_count());
        assert_eq!(7, &bins.get(2).get_greater_count());
        assert_eq!(0, &bins.get(2).get_less_count());
        assert_eq!(1, &bins.get(2).get_bin_index());
        assert_eq!(-3.0, &bins.get(2).get_lower_bound(), 0.0);
        assert_eq!(&Math::next_down(-2.0), &bins.get(2).get_upper_bound(), 0.0);
    }

    #[test]
    pub fn  test_non_empty_bins_ascending_iterator_for_empty_histogram(&self)   {
         let histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
         let bins: List<Bin> = ArrayList<>::new();
        for  let bin: Bin in histogram.non_empty_bins_ascending() {
            bins.add(bin);
        }
        assert_true(&bins.is_empty());
    }

    #[test]
    pub fn  test_non_empty_bins_ascending_for_each_for_empty_histogram(&self)   {
         let histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
         let bins: List<Bin> = ArrayList<>::new();
        histogram.non_empty_bins_ascending().for_each(bins::add);
        assert_true(&bins.is_empty());
    }

    #[test]
    pub fn  test_non_empty_bins_descending_iterator_for_empty_histogram(&self)   {
         let histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
         let bins: List<Bin> = ArrayList<>::new();
        for  let bin: Bin in histogram.non_empty_bins_descending() {
            bins.add(bin);
        }
        assert_true(&bins.is_empty());
    }

    #[test]
    pub fn  test_non_empty_bins_descending_for_each_for_empty_histogram(&self)   {
         let histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
         let bins: List<Bin> = ArrayList<>::new();
        histogram.non_empty_bins_descending().for_each(bins::add);
        assert_true(&bins.is_empty());
    }

    #[test]
    pub fn  test_bin_iterator_next_for_last_bin(&self)   {
         let mut histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
        histogram = self.add_values(histogram, 5);
         let iterator: BinIterator = histogram.get_last_non_empty_bin();
        assert_throws(NoSuchElementException.class, iterator::next);
    }

    #[test]
    pub fn  test_bin_iterator_previous_for_first_bin(&self)   {
         let mut histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
        histogram = self.add_values(histogram, 5);
         let iterator: BinIterator = histogram.get_first_non_empty_bin();
        assert_throws(NoSuchElementException.class, iterator::previous);
    }

    #[test]
    pub fn  test_get_bin_iterator_for_empty_histogram(&self)   {
         let histogram: Histogram = self.create(&CustomLayout::create(-4, -2, 0, 2, 4));
        assert_throws(NoSuchElementException.class, histogram::getFirstNonEmptyBin);
        assert_throws(NoSuchElementException.class, histogram::getLastNonEmptyBin);
    }
}
