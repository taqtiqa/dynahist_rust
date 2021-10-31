// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT


const DEFAULT_QUANTILE_ESTIMATOR: QuantileEstimator = SciPyQuantileEstimator::create();

const DEFAULT_VALUE_ESTIMATOR: impl ValueEstimation = ValueEstimatorUniform.new();

const ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES: i64 = 4;

 const ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES: i64 = 12;
#[derive(Histogram)]
struct AbstractHistogram {
    layout: impl Layout,
}

impl AbstractHistogram {

    pub fn new( layout: impl Layout) -> AbstractHistogram {
        let layout = require_non_null(layout);
    }

    fn format_counts(&self) -> String  {
        if get_total_count() == 0 {
            return "{}";
        }
         let builder: StringBuilder = StringBuilder::new("{");
         let bin: BinIterator = get_first_non_empty_bin();
        while bin.get_greater_count() > 0 {
            builder.append(&bin.get_bin_index()).append(": ").append(&bin.get_bin_count()).append(", ");
            bin.next();
        }
        builder.append(&bin.get_bin_index()).append(": ").append(&bin.get_bin_count()).append("}");
        return builder.to_string();
    }

    pub fn to_string(&self) -> String  {
        return format!("{} [layout={}, underFlowCount={}, overFlowCount={}, totalCount={}, min={}, max={}, counts={}]", get_class().get_simple_name(), self.get_layout(), get_underflow_count(), get_overflow_count(), get_total_count(), get_min(), get_max(), self.format_counts());
    }

    pub fn hash_code(&self) -> i32  {
         let prime: i32 = 31;
         let mut result: i32 = 1;
        result = prime * result + self.get_layout().hash_code();
         let mut temp: i64;
        temp = to_bits_nan_collapse(get_max());
        result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
        temp = to_bits_nan_collapse(get_min());
        result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
        // hash only count values together with bin indices that are larger than 0
        if get_total_count() > 0 {
             let bin_iterator: BinIterator = get_first_non_empty_bin();
            while true {
                temp = bin_iterator.get_bin_count();
                result = prime * result + (temp ^ (temp >> /* >>> */ 32)) as i32;
                result = prime * result + bin_iterator.get_bin_index();
                if bin_iterator.get_greater_count() == 0 {
                    break;
                }
                bin_iterator.next();
            }
        }
        return result;
    }

    pub fn equals(&self,  obj: &Object) -> bool  {
        if self == obj {
            return true;
        }
        if !(obj instanceof Histogram) {
            return false;
        }
         let other: Histogram = obj as Histogram;
        if !self.get_layout().equals(&other.get_layout()) || get_total_count() != other.get_total_count() || get_underflow_count() != other.get_underflow_count() || get_overflow_count() != other.get_overflow_count() || Double::compare(&get_min(), &other.get_min()) != 0 || Double::compare(&get_max(), &other.get_max()) != 0 {
            return false;
        }
        if get_total_count() > 0 {
             let bin_iterator: BinIterator = get_first_non_empty_bin();
             let other_bin_iterator: BinIterator = other.get_first_non_empty_bin();
            while true {
                if bin_iterator.get_bin_index() != other_bin_iterator.get_bin_index() || bin_iterator.get_bin_count() != other_bin_iterator.get_bin_count() {
                    return false;
                }
                if bin_iterator.get_greater_count() == 0 {
                    break;
                }
                bin_iterator.next();
                other_bin_iterator.next();
            }
        }
        return true;
    }

    pub fn get_layout(&self) -> Layout  {
        return self.layout;
    }

    pub fn get_bin_by_rank(&self,  rank: i64) -> BinIterator  {
         let total_count: i64 = get_total_count();
        check_argument(rank >= 0);
        check_argument(rank < total_count);
         let bin_iterator: BinIterator;
        if rank < (total_count >> /* >>> */ 1) {
            bin_iterator = get_first_non_empty_bin();
            while bin_iterator.get_greater_count() >= total_count - rank {
                bin_iterator.next();
            }
        } else {
            bin_iterator = get_last_non_empty_bin();
            while bin_iterator.get_less_count() > rank {
                bin_iterator.previous();
            }
        }
        return bin_iterator;
    }

    pub fn is_empty(&self) -> bool  {
        return get_total_count() == 0;
    }

    pub fn get_value_from_estimator(&self,   rank: i64,   value_estimator: &ValueEstimator) -> f64  {
        require_non_null(value_estimator);
        return value_estimator.get_value_estimate(self, rank);
    }

    pub fn get_value(&self,  rank: i64) -> f64  {
        return self.get_value_from_estimator(rank, DEFAULT_VALUE_ESTIMATOR);
    }

    pub fn get_preprocessed_copy(&self) -> impl Histogram  {
        return PreprocessedHistogram::of(self);
    }

    pub fn get_quantile(&self,  p: f64,  quantile_estimator: &QuantileEstimator,  value_estimator: &ValueEstimator) -> f64  {
        return quantile_estimator.estimate_quantile(p,  rank: & -> self.get_value_from_estimator(rank,  value_estimator),  &get_total_count());
    }

    pub fn get_quantile(&self,  p: f64,  value_estimator: &ValueEstimator) -> f64  {
        return self.get_quantile(p, DEFAULT_QUANTILE_ESTIMATOR, value_estimator);
    }

    pub fn get_quantile(&self,  p: f64,  quantile_estimator: &QuantileEstimator) -> f64  {
        return self.get_quantile(p, quantile_estimator, DEFAULT_VALUE_ESTIMATOR);
    }

    pub fn get_quantile(&self,  p: f64) -> f64  {
        return self.get_quantile(p, DEFAULT_QUANTILE_ESTIMATOR);
    }

    pub fn get_estimated_footprint_in_bytes(&self) -> i64  {
        return // layout
        ESTIMATED_REFERENCE_FOOTPRINT_IN_BYTES + // object header for this object
        ESTIMATED_OBJECT_HEADER_FOOTPRINT_IN_BYTES;
    }

    pub fn add_histogram(&self,  histogram: impl Histogram) -> impl Histogram  {
        return self.add_histogram(histogram, DEFAULT_VALUE_ESTIMATOR);
    }

    #[derive(Iterable<Bin>)]
    struct AbstractNonEmptyBinsIterable {
    }

    impl AbstractNonEmptyBinsIterable {

        pub fn get_start(&self) -> BinIterator ;

        pub fn advance_bin_iterator(&self,  bin_iterator: &BinIterator)  ;

        pub fn is_at_end(&self,  bin_iterator: &BinIterator) -> bool ;

        pub fn iterator(&self) -> Iterator<Bin>  {
            return Iterator<>::new() {

                 let mut it: BinIterator = null;

                pub fn has_next(&self) -> bool  {
                    return self.it == null || !self.is_at_end(self.it);
                }

                pub fn next(&self) -> Bin  {
                    if self.it != null {
                        self.advance_bin_iterator(self.it);
                    } else {
                        self.it = self.get_start();
                    }
                    return self.it.get_bin_copy();
                }
            };
        }

        pub fn for_each(&self,  action: &Consumer<? super Bin>)   {
             let it: BinIterator = self.get_start();
            action.accept(&it.get_bin_copy());
            while !self.is_at_end(it) {
                self.advance_bin_iterator(it);
                action.accept(&it.get_bin_copy());
            }
        }
    }


    pub fn non_empty_bins_ascending(&self) -> Iterable<Bin>  {
        if self.is_empty() {
            return Collections::empty_list();
        }

        return AbstractNonEmptyBinsIterable::new() {

            pub fn get_start(&self) -> BinIterator  {
                return get_first_non_empty_bin();
            }

            pub fn advance_bin_iterator(&self,  bin_iterator: &BinIterator)   {
                bin_iterator.next();
            }

            pub fn is_at_end(&self,  bin_iterator: &BinIterator) -> bool  {
                return bin_iterator.is_last_non_empty_bin();
            }
        };
    }

    pub fn non_empty_bins_descending(&self) -> Iterable<Bin>  {
        if self.is_empty() {
            return Collections::empty_list();
        }

        return AbstractNonEmptyBinsIterable::new() {

            pub fn get_start(&self) -> BinIterator  {
                return get_last_non_empty_bin();
            }

            pub fn advance_bin_iterator(&self,  bin_iterator: &BinIterator)   {
                bin_iterator.previous();
            }

            pub fn is_at_end(&self,  bin_iterator: &BinIterator) -> bool  {
                return bin_iterator.is_first_non_empty_bin();
            }
        };
    }
}
