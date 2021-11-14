// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::bins::bin::BinSketch;
use crate::bins::bin_iterator::BinIterator;
// use crate::errors::DynaHistError;
// use crate::histograms::dynamic_histogram::DynamicHistogram;
// use crate::histograms::fixed::StaticHistogram;
use crate::histograms::histogram::Histogram;
use crate::histograms::preprocessed_histogram::PreprocessedHistogram;
use crate::layouts::layout::Layout;
use crate::quantiles::quantile_estimation::QuantileEstimation;
use crate::quantiles::quantile_estimators::QuantileEstimator;
// use crate::sketches::data::{DataInput, DataOutput};
// use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::values::value_estimation::ValueEstimation;
use crate::values::value_estimators::ValueEstimatorUniform;

// struct AbstractHistogram {
//     layout: impl Layout,
// }

// When histogram plays the role of a BTreeMap it should be simple to iterate
// over all (quantile, value) or (percentile, value) pairs:
//
// https://www.reddit.com/r/rust/comments/f7uoyo/advice_on_implementing_a_custom_flat_iterator/
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=eea0af2c427fef86b643dad2901bed20

pub trait Probability {}

// Strategies (inter-bin):
// - All
// - NonEmpty
//
// Values (intra-bin):
// - ValueEstimator: Uniform, Lower, Upper, Midpoint
//

/// Quantiles are pseudo-bounded iterators.
/// While limited to the interval `[0,1]` they are points on the distribution
/// function - in  case a 'constrained' or storage-optimized historgram.
/// The maximum number of values are the number of data points.
// https://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step
struct QuantileRange<T>(T, T)
where
    //for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
    T: Probability + PartialOrd,
    T: Clone;

struct Quantiles<T>
where
    //for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
    T: Probability + PartialOrd,
    T: Clone,
{
    current: T, //builder always initializes current=start
    end: T,
    n: usize,
    start: T,
}

impl<T: Probability + Clone + std::cmp::PartialOrd> Probability for Quantiles<T> {}

impl<T: Probability + Clone + std::cmp::PartialOrd> Quantiles<T> {
    fn next_quantile(&self) {
        self.quantiles().next()
    }
}

// impl<T> Iterator for Quantiles<T>
// where
//     for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
//     T: Probability + PartialOrd,
//     T: Clone,
// {
//     type Item = T;

//     #[inline]
//     fn next(&mut self) -> Option<T> {
//         if self.current < self.end {
//             self.current = &self.next_quantile();
//             Some(&self.current)
//         } else {
//             None
//         }
//     }
// }
// Iterator yielding immutable references
impl<'a, T: Probability> IntoIterator for &'a Quantiles<T>
where
    &'a T: std::ops::Add<&'a T, Output = T>,
    T: Probability + PartialOrd,
    T: Clone,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> std::slice::Iter<'a, T> { /* ... */
    }
}

// Iterator yielding mutable references
impl<'a, T> IntoIterator for &'a mut Quantiles<T>
where
    &'a T: std::ops::Add<&'a T, Output = T>,
    T: Probability + PartialOrd,
    T: Clone,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> std::slice::IterMut<'a, T> { /* ... */
    }
}

// Iterator yielding values
impl<T> IntoIterator for Quantiles<T>
where
    T: std::ops::Add<T, Output = T>,
    T: Probability + PartialOrd,
    T: Clone,
    T: std::iter::Iterator,
{
    type Item = T;
    type IntoIter = <T as std::iter::IntoIterator>::IntoIter;

    fn into_iter(mut self) -> <T as std::iter::IntoIterator>::IntoIter { /* ... */
    }
}

// Operator overloading for all structs with a trait.
// https://stackoverflow.com/questions/26518233/overloading-an-operator-for-all-structs-with-a-trait-in-rust

// updated, DRY impl for `Quantiles<T>` and `&Quantiles<T>`
// impl<T> std::ops::Add for T {
//     type Output = <T as Add>::Output;
//     fn add(self, rhs: &T) -> Self::Output {
//         // some calculations here
//     }
// }

// updated, DRY impl for `Quantiles<T>` and `&Quantiles<T>`
// impl<T> std::ops::Add for &T {
//     type Output = <T as Add>::Output;
//     fn add(self, rhs: &T) -> Self::Output {
//         T::add(*self, *rhs)
//     }
// }

/// Percentiles are bounded iterators.  Limited to the interval `[0,1]` with a
/// minimum delta/step-size of f64::EPSILON
// https://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step
struct PercentileRange<T>(T, T, T)
where
    // for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
    T: Probability + PartialOrd,
    T: Clone;

struct Percentiles<T>
where
    // for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
    T: PartialOrd,
    T: Clone,
    T: Probability,
{
    _marker: std::marker::PhantomData<T>

    }

impl<T: Probability + std::cmp::PartialOrd + Clone> Probability for Percentiles<T> {}

    // Note `PercentileRange` is used to configure the `Percentiles` struct.
// This is done in the Histogram builder helper function `iterator`.
impl<T> Iterator for Percentiles<T>
where
    // for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
    T: PartialOrd,
    T: Clone,
    T: Probability
{
    type Item = T;

    // This isn't quite what we need but it is close. We likely will
    // build a percentile reconstruction function from the quantile function.
    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.0 < self.1 {
            let v = self.0.clone();
            self.0 = &v + &self.2;
            Some(v)
        } else {
            None
        }
    }
}

pub trait ValueIterator {}

pub trait ValueSketch {}

/// This is pseudo-bounded iterators. Limited by number of data values which
/// is finite but only known at run time.
/// https://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step
struct UniformValuesIterator {}

/// These are pseudo-bounded iterators. Limited by number of bins which
/// is finite but only known at run time.
struct AllBinsIterator {}
struct NonEmptyBinsIterator {}
struct LowerValuesIterator {}
struct UpperValuesIterator {}
struct MidpointValuesIterator {}

// trait AbstractNonEmptyBinsIterable {
//     type B: BinIterator + BinSketch + Iterator;
//     type L: Layout;

//     fn get_start(&self) -> Self::B;

//     fn advance_bin_iterator(&self, bin_iterator: &Self::B);

//     fn is_at_end(&self, bin_iterator: &Self::B) -> bool;

//     fn iterator(&self) -> Self::B {
//         impl Iterator for Self {
//             type Item = u32;
//             // let it: Self::B = None;

//             // fn has_next(&self) -> bool {
//             //     return self.it == None || !self.is_at_end(self.it);
//             // }

//             fn next(&self) -> Option<Self::Item> {
//                 if self.it != None {
//                     self.advance_bin_iterator(self.it);
//                 } else {
//                     self.it = self.get_start();
//                 }
//                 return self.it.get_bin_copy();
//             }
//         }
//         // return Self {
//         //     it: Self::B = None,
//         // };
//     }

//     // fn for_each(&self, action: impl BinSketch) {
//     //     let it: Self::B = self.get_start();
//     //     action.accept(&it.get_bin_copy());
//     //     while !self.is_at_end(it) {
//     //         self.advance_bin_iterator(it);
//     //         action.accept(&it.get_bin_copy());
//     //     }
//     // }
// }

pub trait AbstractHistogram: Histogram + Probability {
    type L: Layout;
    type H: Histogram;
    type B: BinIterator + BinSketch + std::iter::Iterator;
    type V: ValueIterator + ValueSketch + std::iter::Iterator;

    // No longer required by revised implementation
    //const DEFAULT_QUANTILE_ESTIMATOR: dyn QuantileEstimation = QuantileEstimator::create();

    // const DEFAULT_VALUE_ESTIMATOR: dyn ValueEstimation = ValueEstimatorUniform::new();

    const ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES: i64 = 4;

    const ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES: i64 = 12;

    const GROW_FACTOR: f64 = 0.25;

    const SERIAL_VERSION_V0: i8 = 0;

    const OVERFLOW_MSG: &'static str = "Overflow occurred!";

    const NAN_VALUE_MSG: &'static str = "Value was not a number (NaN)!";

    const NEGATIVE_COUNT_MSG: &'static str = "Count must be non-negative, but was {}!";

    const INCOMPATIBLE_SERIAL_VERSION_MSG: &'static str = format!(
        "Incompatible serial versions! Expected version {} but was %d.",
        Self::SERIAL_VERSION_V0
    );

    const EMPTY_COUNTS: Vec<i32> = vec![];

    fn new(layout: impl Layout) -> Self {
        Default::default()
    }

    fn default_value_estimator(){
        ValueEstimatorUniform::new()
    }
    // fn format_counts(&self) -> String {
    //     if Self::get_total_count() == 0 {
    //         return "{}";
    //     }
    //      let builder: StringBuilder = StringBuilder::new("{");
    //      let bin: Self::B = Self::get_first_non_empty_bin();
    //     while bin.get_greater_count() > 0 {
    //         builder.append(&bin.get_bin_index()).append(": ").append(&bin.get_bin_count()).append(", ");
    //         bin.next();
    //     }
    //     builder.append(&bin.get_bin_index()).append(": ").append(&bin.get_bin_count()).append("}");
    //     return builder.to_string();
    // }

    // fn to_string(&self) -> String {
    //     return format!("{} [layout={}, underFlowCount={}, overFlowCount={}, totalCount={}, min={}, max={}, counts={}]", self.histogram_type, self.get_layout(), get_underflow_count(), get_overflow_count(), Self::get_total_count(), get_min(), get_max(), self.format_counts());
    // }

    // fn hash_code(&self) -> i32 {
    //      let prime: i32 = 31;
    //      let mut result: i32 = 1;
    //     result = prime * result + self.get_layout().hash_code();
    //      let mut temp: i64;
    //     temp = Self::to_bits_nan_collapse(get_max());
    //     result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
    //     temp = Self::to_bits_nan_collapse(get_min());
    //     result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
    //     // hash only count values together with bin indices that are larger than 0
    //     if Self::get_total_count() > 0 {
    //          let bin_iterator: Self::B = Self::get_first_non_empty_bin();
    //         while true {
    //             temp = bin_iterator.get_bin_count();
    //             result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
    //             result = prime * result + bin_iterator.get_bin_index();
    //             if bin_iterator.get_greater_count() == 0 {
    //                 break;
    //             }
    //             bin_iterator.next();
    //         }
    //     }
    //     return result;
    // }

    // fn equals(&self,  obj: &Object) -> bool {
    //     if self == obj {
    //         return true;
    //     }
    //     if !(obj instanceof Histogram) {
    //         return false;
    //     }
    //      let other: Histogram = obj as Histogram;
    //     if !self.get_layout().equals(&other.get_layout()) || Self::get_total_count() != other.get_total_count() || get_underflow_count() != other.get_underflow_count() || get_overflow_count() != other.get_overflow_count() || Double::compare(&get_min(), &other.get_min()) != 0 || Double::compare(&get_max(), &other.get_max()) != 0 {
    //         return false;
    //     }
    //     if Self::get_total_count() > 0 {
    //          let bin_iterator: Self::B = Self::get_first_non_empty_bin();
    //          let other_bin_iterator: Self::B = other.get_first_non_empty_bin();
    //         while true {
    //             if bin_iterator.get_bin_index() != other_bin_iterator.get_bin_index() || bin_iterator.get_bin_count() != other_bin_iterator.get_bin_count() {
    //                 return false;
    //             }
    //             if bin_iterator.get_greater_count() == 0 {
    //                 break;
    //             }
    //             bin_iterator.next();
    //             other_bin_iterator.next();
    //         }
    //     }
    //     return true;
    // }

    fn get_layout(&self) -> <Self as AbstractHistogram>::L {
        return self.layout;
    }

    fn get_bin_by_rank(&self, rank: i64) -> <Self as AbstractHistogram>::B {
        let total_count: i64 = Self::get_total_count();
        Self::check_argument(rank >= 0);
        Self::check_argument(rank < total_count);
        let bin_iterator: Self::B;
        if rank < (total_count >> /* >>> */ 1) {
            bin_iterator = Self::get_first_non_empty_bin();
            while bin_iterator.get_greater_count() >= total_count - rank {
                bin_iterator.next();
            }
        } else {
            bin_iterator = Self::get_last_non_empty_bin();
            while bin_iterator.get_less_count() > rank {
                bin_iterator.previous();
            }
        }
        return bin_iterator;
    }

    fn is_empty(&self) -> bool {
        return Self::get_total_count() == 0;
    }

    fn get_value_from_estimator(&self, rank: i64, value_estimator: impl ValueEstimation) -> f64 {
        return value_estimator.get_value_estimate(self, rank);
    }

    fn get_value(&self, rank: i64) -> f64 {
        return self.get_value_from_estimator(rank, Self::default_value_estimator());
    }

    fn get_preprocessed_copy(&self) -> Self::H {
        return PreprocessedHistogram::of(self);
    }

    fn get_estimated_footprint_in_bytes(&self) -> i64 {
        return // layout
        Self::ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES + // object header for this object
        Self::ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES;
    }

    fn add_histogram(&self, histogram: Self::H) -> Self::H {
        return self.add_histogram_from_estimator(histogram, Self::default_value_estimator());
    }

    fn non_empty_bins_ascending(&self) -> &<Self as AbstractHistogram>::B {
        if self.is_empty() {
            return vec![];
        }

        // impl AbstractNonEmptyBinsIterable for NonEmptyBinsIterable {
        //     fn get_start(&self) -> Self::B {
        //         return Self::get_first_non_empty_bin();
        //     }

        //     fn advance_bin_iterator(&self, bin_iterator: &Self::B) {
        //         bin_iterator.next();
        //     }

        //     fn is_at_end(&self, bin_iterator: &Self::B) -> bool {
        //         return bin_iterator.is_last_non_empty_bin();
        //     }
        // }
        // return NonEmptyBinsIterable {};
    }

    fn non_empty_bins_descending(&self) -> &Vec<<Self as AbstractHistogram>::B> {
        if self.is_empty() {
            return &vec![Self::B];
        }

        // impl AbstractNonEmptyBinsIterable for NonEmptyBinsIterable {
        //     fn get_start(&self) -> Self::B {
        //         return Self::get_last_non_empty_bin();
        //     }

        //     fn advance_bin_iterator(&self, bin_iterator: &Self::B) {
        //         bin_iterator.previous();
        //     }

        //     fn is_at_end(&self, bin_iterator: &Self::B) -> bool {
        //         return bin_iterator.is_first_non_empty_bin();
        //     }
        // }
        // return NonEmptyBinsIterable {};
    }
}
