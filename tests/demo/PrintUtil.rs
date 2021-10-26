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
// package com::dynatrace::dynahist::demo;

pub struct PrintUtil {
}

impl PrintUtil {

    fn new() -> PrintUtil {
    }

    pub fn  print( histogram: &Histogram) -> String  {
        Preconditions::check_argument(histogram != null);
        Preconditions::check_argument(histogram.get_total_count() != 0);
         let iterator: BinIterator = histogram.get_first_non_empty_bin();
         let result: StringBuilder = StringBuilder::new(&String::format(null as Locale, "%24.17E - %24.17E : %19d\n", &iterator.get_lower_bound(), &iterator.get_upper_bound(), &iterator.get_bin_count()));
        while !iterator.is_last_non_empty_bin() {
            iterator.next();
            result.append(&String::format(null as Locale, "%24.17E - %24.17E : %19d\n", &iterator.get_lower_bound(), &iterator.get_upper_bound(), &iterator.get_bin_count()));
        }
        return result.to_string();
    }

    pub fn  pretty_print( histogram: &Histogram) -> String  {
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

         let result: StringBuilder = StringBuilder::new(&String::format(null as Locale, "%24.17E - %24.17E : %s\n", &iterator.get_lower_bound(), &iterator.get_upper_bound(), &temp));
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

            result.append(&String::format(null as Locale, "%24.17E - %24.17E : %s\n", &iterator.get_lower_bound(), &iterator.get_upper_bound(), &temp));
        }
        return result.to_string();
    }
}

