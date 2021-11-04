// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::bins::bin::BinSketch;
use crate::Histogram;

use num::Float;

pub trait AbstractBin: BinSketch {
    // Self returned is the immediate type the trait is implemented for.
    // This renders the trait no longer object-safe.
    fn get_histogram(&self) -> Self;

    fn to_string(&self) -> String {
        return format!("Bin [bin_index={}, lowerBound={}, upperBound={}, binCount={}, lessCount={}, greaterCount={}, isUnderflowBin={}, isOverflowBin={}]",
            self.get_bin_index(),
            AbstractBin::get_lower_bound(self),
            AbstractBin::get_upper_bound(self),
            self.get_bin_count(),
            self.get_less_count(),
            self.get_greater_count(),
            AbstractBin::is_underflow_bin(self),
            AbstractBin::is_overflow_bin(self));
    }

    fn is_underflow_bin(&self) -> bool {
        return self.get_bin_index() == self.get_histogram().get_layout().get_underflow_bin_index();
    }

    fn is_overflow_bin(&self) -> bool {
        return self.get_bin_index() == self.get_histogram().get_layout().get_overflow_bin_index();
    }

    fn get_lower_bound(&self) -> f64 {
        let histogram = self.get_histogram();
        let h1 = histogram.get_min();
        let h2 = histogram
            .get_layout()
            .get_bin_lower_bound(&self.get_bin_index());
        return h1.min(h2);
    }

    fn get_upper_bound(&self) -> f64 {
        let histogram = self.get_histogram();
        let h1 = histogram.get_max();
        let h2 = histogram
            .get_layout()
            .get_bin_upper_bound(&self.get_bin_index());
        return h1.min(h2);
    }
}
