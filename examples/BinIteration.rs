// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct BinIteration {
}

impl BinIteration {

    #[test]
    fn test_bin_iteration_ascending_order(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-3, 1e-2, 0, 1E6);
         let histogram: Histogram = Histogram::create_dynamic(layout).add_values(7.5, 3).add_value(3.5, 6).add_value(8.5, 11).add_value(9.5,  2);
         let expected_bin_counts : vec![i64; 4] = vec![6, 3, 11, 2, ]
        ;
         let bin_iterator: BinIterator = histogram.get_first_non_empty_bin();
         let mut counter: i32 = 0;
        assert_eq!(expected_bin_counts[counter], &bin_iterator.get_bin_count());
        while !bin_iterator.is_last_non_empty_bin() {
            counter += 1;
            bin_iterator.next();
            bin_iterator.get_bin_count();
            assert_eq!(expected_bin_counts[counter], &bin_iterator.get_bin_count());
        }
    }

    #[test]
    fn test_bin_iteration_descending_order(&self) {
         let layout: Layout = LogQuadraticLayout::create(1e-3, 1e-2, 0, 1E6);
         let histogram: Histogram = Histogram::create_dynamic(layout)::add_values(7.5, 3).add_value(3.5, 6).add_value(8.5, 11).add_value(9.5,  2);
         let expected_bin_counts : vec![i64; 4] = vec![2, 11, 3, 6, ]
        ;
         let bin_iterator: BinIterator = histogram.get_last_non_empty_bin();
         let mut counter: i32 = 0;
        assert_eq!(expected_bin_counts[counter], &bin_iterator.get_bin_count());
        while !bin_iterator.is_first_non_empty_bin() {
            counter += 1;
            bin_iterator.previous();
            bin_iterator.get_bin_count();
            assert_eq!(expected_bin_counts[counter], &bin_iterator.get_bin_count());
        }
    }
}
