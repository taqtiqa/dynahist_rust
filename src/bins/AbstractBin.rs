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
// package com::dynatrace::dynahist::bin;

#[derive(Bin)]
pub struct AbstractBin {}

impl AbstractBin {
    fn get_histogram(&self) -> Histogram;

    pub fn to_string(&self) -> String {
        return format!("Bin [bin_index={}, lowerBound={}, upperBound={}, binCount={}, lessCount={}, greaterCount={}, isUnderflowBin={}, isOverflowBin={}]", get_bin_index(), self.get_lower_bound(), self.get_upper_bound(), get_bin_count(), get_less_count(), get_greater_count(), self.is_underflow_bin(), self.is_overflow_bin());
    }

    pub fn is_underflow_bin(&self) -> bool {
        return get_bin_index() == self.get_histogram().get_layout().get_underflow_bin_index();
    }

    pub fn is_overflow_bin(&self) -> bool {
        return get_bin_index() == self.get_histogram().get_layout().get_overflow_bin_index();
    }

    pub fn get_lower_bound(&self) -> f64 {
        let histogram: Histogram = self.get_histogram();
        return std::cmp::max(
            &histogram.get_min(),
            &histogram.get_layout().get_bin_lower_bound(&get_bin_index()),
        );
    }

    pub fn get_upper_bound(&self) -> f64 {
        let histogram: Histogram = self.get_histogram();
        return std::cmp::min(
            &histogram.get_max(),
            &histogram.get_layout().get_bin_upper_bound(&get_bin_index()),
        );
    }
}
