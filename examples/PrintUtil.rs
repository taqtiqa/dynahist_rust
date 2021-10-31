// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct PrintUtil {}

impl PrintUtil {
    fn new() -> PrintUtil {}

    pub fn print(histogram: impl Histogram) -> String {
        Preconditions::check_argument(histogram != null);
        Preconditions::check_argument(histogram.get_total_count() != 0);
        let iterator: BinIterator = histogram.get_first_non_empty_bin();
        let result: StringBuilder = StringBuilder::new(&String::format(
            null as Locale,
            "%24.17E - %24.17E : %19d\n",
            &iterator.get_lower_bound(),
            &iterator.get_upper_bound(),
            &iterator.get_bin_count(),
        ));
        while !iterator.is_last_non_empty_bin() {
            iterator.next();
            result.append(&String::format(
                null as Locale,
                "%24.17E - %24.17E : %19d\n",
                &iterator.get_lower_bound(),
                &iterator.get_upper_bound(),
                &iterator.get_bin_count(),
            ));
        }
        return result.to_string();
    }

    pub fn pretty_print(histogram: impl Histogram) -> String {
        Preconditions::check_argument(histogram != null);
        Preconditions::check_argument(histogram.get_total_count() != 0);
        let iterator: BinIterator = histogram.get_first_non_empty_bin();
        let temp: StringBuilder = StringBuilder::new();
        {
            let mut i: i32 = 0;
            while i < iterator.get_bin_count() {
                {
                    temp.append('*');
                }
                i += 1;
            }
        }

        let result: StringBuilder = StringBuilder::new(&String::format(
            null as Locale,
            "%24.17E - %24.17E : %s\n",
            &iterator.get_lower_bound(),
            &iterator.get_upper_bound(),
            &temp,
        ));
        while !iterator.is_last_non_empty_bin() {
            iterator.next();
            {
                let mut i: i32 = 0;
                while i < iterator.get_bin_count() {
                    {
                        temp.append('*');
                    }
                    i += 1;
                }
            }

            result.append(&String::format(
                null as Locale,
                "%24.17E - %24.17E : %s\n",
                &iterator.get_lower_bound(),
                &iterator.get_upper_bound(),
                &temp,
            ));
        }
        return result.to_string();
    }
}
