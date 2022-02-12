// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct HistogramTest {}

#[cfg(feature = "test-traits")]
impl utilities::tests::Histogram for HistogramTest {}

trait Histogram {
    pub fn new() -> impl Histogram {}

    fn check_histogram_data_consistency(histogram_data: &Histogram) -> Map<Integer, Long> {
        let layout: Layout = histogram_data.get_layout();
        let total_count: i64 = histogram_data.get_total_count();
        let overflow_count: i64 = histogram_data.get_overflow_count();
        let underflow_count: i64 = histogram_data.get_underflow_count();
        let min: f64 = histogram_data.get_min();
        let max: f64 = histogram_data.get_max();
        let non_empty_bins_from_get_count: Map<i32, u64> = BTreeMap::new();
        {
            {
                let bin_index: i32 = layout.get_underflow_bin_index();
                while bin_index <= layout.get_overflow_bin_index() {
                    {
                        let count: i64 = histogram_data.get_count(bin_index);
                        if count > 0 {
                            non_empty_bins_from_get_count.put(bin_index, count);
                        }
                    }
                    bin_index += 1;
                }
            }
        }
        let non_empty_bins_from_forward_bin_iteration: BTreeMap<i32, u64> = BTreeMap::new();
        {
            if !histogram_data.is_empty() {
                let bin_iterator: BinIterator = histogram_data.get_first_non_empty_bin();
                while true {
                    non_empty_bins_from_forward_bin_iteration
                        .put(&bin_iterator.get_bin_index(), &bin_iterator.get_bin_count());
                    assert_eq!(
                        total_count,
                        bin_iterator.get_less_count()
                            + bin_iterator.get_bin_count()
                            + bin_iterator.get_greater_count()
                    );
                    //     0d);
                    if bin_iterator.is_last_non_empty_bin() {
                        break;
                    }
                    bin_iterator.next();
                }
            }
        }
        let non_empty_bins_from_forward_bin_iteration2: BTreeMap<i32, u64> = BTreeMap::new();
        {
            if !histogram_data.is_empty() {
                let bin_iterator: BinIterator = histogram_data.get_first_non_empty_bin();
                while true {
                    non_empty_bins_from_forward_bin_iteration2
                        .put(&bin_iterator.get_bin_index(), &bin_iterator.get_bin_count());
                    assert_eq!(
                        total_count,
                        bin_iterator.get_less_count()
                            + bin_iterator.get_bin_count()
                            + bin_iterator.get_greater_count()
                    );
                    assert_eq!(
                        &std::cmp::max(
                            min,
                            &layout.get_bin_lower_bound(&bin_iterator.get_bin_index())
                        ),
                        &bin_iterator.get_lower_bound(),
                        0.0
                    );
                    assert_eq!(
                        &std::cmp::min(
                            max,
                            &layout.get_bin_upper_bound(&bin_iterator.get_bin_index())
                        ),
                        &bin_iterator.get_upper_bound(),
                        0.0
                    );
                    if bin_iterator.is_last_non_empty_bin() {
                        break;
                    }
                    bin_iterator.next();
                    bin_iterator.previous();
                    bin_iterator.next();
                }
            }
        }
        let non_empty_bins_from_backward_bin_iteration: BTreeMap<i32, u64> = BTreeMap::new();
        {
            if !histogram_data.is_empty() {
                let bin_iterator: BinIterator = histogram_data.get_last_non_empty_bin();
                while true {
                    non_empty_bins_from_backward_bin_iteration
                        .put(&bin_iterator.get_bin_index(), &bin_iterator.get_bin_count());
                    assert_eq!(
                        total_count,
                        bin_iterator.get_less_count()
                            + bin_iterator.get_bin_count()
                            + bin_iterator.get_greater_count()
                    );
                    assert_eq!(
                        &std::cmp::max(
                            min,
                            &layout.get_bin_lower_bound(&bin_iterator.get_bin_index())
                        ),
                        &bin_iterator.get_lower_bound(),
                        0.0
                    );
                    assert_eq!(
                        &std::cmp::min(
                            max,
                            &layout.get_bin_upper_bound(&bin_iterator.get_bin_index())
                        ),
                        &bin_iterator.get_upper_bound(),
                        0.0
                    );
                    if bin_iterator.is_first_non_empty_bin() {
                        break;
                    }
                    bin_iterator.previous();
                }
            }
        }
        let non_empty_bins_from_backward_bin_iteration2: BTreeMap<i32, u64> = BTreeMap::new();
        {
            if !histogram_data.is_empty() {
                let bin_iterator: BinIterator = histogram_data.get_last_non_empty_bin();
                while true {
                    non_empty_bins_from_backward_bin_iteration2
                        .put(&bin_iterator.get_bin_index(), &bin_iterator.get_bin_count());
                    assert_eq!(
                        total_count,
                        bin_iterator.get_less_count()
                            + bin_iterator.get_bin_count()
                            + bin_iterator.get_greater_count()
                    );
                    assert_eq!(
                        &std::cmp::max(
                            min,
                            &layout.get_bin_lower_bound(&bin_iterator.get_bin_index())
                        ),
                        &bin_iterator.get_lower_bound(),
                        0.0
                    );
                    assert_eq!(
                        &std::cmp::min(
                            max,
                            &layout.get_bin_upper_bound(&bin_iterator.get_bin_index())
                        ),
                        &bin_iterator.get_upper_bound(),
                        0.0
                    );
                    if bin_iterator.is_first_non_empty_bin() {
                        break;
                    }
                    bin_iterator.previous();
                    bin_iterator.next();
                    bin_iterator.previous();
                }
            }
        }

        /// final Map<Integer, Long> nonEmptyBinsFromOrderAccess = new TreeMap<>(); { if
        /// (!histogramData.isEmpty()) { for (long rank = 0; rank < totalCount;
        /// ++order) { BinIterator bin = histogramData.getBinByRank(rank);
        /// nonEmptyBinsFromOrderAccess.compute(bin.getBinIndex(), (idx, count) -> (count
        /// == null)?1:count+1); } } }
        ///
        let non_empty_bins: Map<Integer, Long> = non_empty_bins_from_get_count;
        assert_eq!(&non_empty_bins, &non_empty_bins_from_get_count);
        assert_eq!(&non_empty_bins, &non_empty_bins_from_forward_bin_iteration);
        assert_eq!(&non_empty_bins, &non_empty_bins_from_backward_bin_iteration);
        assert_eq!(&non_empty_bins, &non_empty_bins_from_forward_bin_iteration2);
        assert_eq!(
            &non_empty_bins,
            &non_empty_bins_from_backward_bin_iteration2
        );
        // assertEquals(nonEmptyBins, nonEmptyBinsFromOrderAccess);
        assert_eq!(
            underflow_count,
            &non_empty_bins
                .get_or_default(&layout.get_underflow_bin_index(), 0)
                .long_value()
        );
        assert_eq!(
            overflow_count,
            &non_empty_bins
                .get_or_default(&layout.get_overflow_bin_index(), 0)
                .long_value()
        );
        assert_eq!(
            total_count,
            &non_empty_bins
                .values()
                .stream()
                .map_to_long(Long::longValue)
                .sum()
        );
        return non_empty_bins;
    }

    fn number_of_non_empty_bins(histogram: impl Histogram) -> i32 {
        if histogram.get_total_count() == 0 {
            return 0;
        }
        let mut count: i32 = 1;
        let iterator: BinIterator = histogram.get_first_non_empty_bin();
        while iterator.get_greater_count() != 0 {
            iterator.next();
            count += 1;
        }
        return count;
    }
}
