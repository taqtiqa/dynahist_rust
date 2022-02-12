// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::histograms::histogram::Histogram;
use crate::layouts::guess_layout::GuessLayout;
use crate::layouts::Sketch;
use crate::seriate::deserialization::SeriateRead;
use crate::seriate::serialization::SeriateWrite;
use crate::seriate::Seriate;
use crate::sketches::data::DataInput;
use crate::sketches::data::DataOutput;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::{errors::DynaHistError, layouts::layout::Layout};

/// A custom histogram bin layout.
pub struct CustomLayout {
    histogram_type: String,
    sorted_bin_boundaries: Vec<f64>,
}

impl Seriate for CustomLayout {}

impl CustomLayout {
    fn read(&self, data_input: &DataInput) -> Result<CustomLayout, std::rc::Rc<DynaHistError>> {
        Self::check_serial_version(Self::SERIAL_VERSION_V0, &data_input.read_unsigned_byte());
        let len: i32 = Self::read_unsigned_var_int(&data_input);
        let sorted_bin_boundaries = vec![0.0; len];
        {
            let mut i: i32 = 0;
            while i < len {
                {
                    sorted_bin_boundaries[i] = data_input.read_double();
                }
                i += 1;
            }
        }

        return Ok(CustomLayout::new(&sorted_bin_boundaries));
    }

    fn write(&self, data_output: &DataOutput) -> Result<(), std::rc::Rc<DynaHistError>> {
        data_output.write_byte(Self::SERIAL_VERSION_V0);
        Self::write_unsigned_var_int(self.sorted_bin_boundaries.len(), &data_output);
        for boundary in self.sorted_bin_boundaries {
            data_output.write_double(boundary);
        }
        Ok(())
    }
}

impl CustomLayout {
    fn new(sorted_bin_boundaries: Vec<f64>) -> Self {
        Self {
            histogram_type: "CustomLayout".to_string(),
            sorted_bin_boundaries,
        }
    }
}

impl Algorithms for CustomLayout {}
impl Preconditions for CustomLayout {}
//impl Layout for CustomLayout {}

impl CustomLayout {
    fn create(sorted_bin_boundaries: f64) -> CustomLayout {
        Self::check_argument(sorted_bin_boundaries.len() > 0);
        Self::check_argument(sorted_bin_boundaries[0] > f64::NEG_INFINITY);
        {
            let mut i: i32 = 1;
            while i < sorted_bin_boundaries.len() {
                {
                    Self::check_argument(
                        Self::map_double_to_long(sorted_bin_boundaries[i - 1])
                            < Self::map_double_to_long(sorted_bin_boundaries[i]),
                    );
                }
                i += 1;
            }
        }
        // To obtain &mut [T] from Vec<T>:
        // - the slicing notation (&mut vec[..]),
        // - the deref conversion (&mut *vec)
        return CustomLayout::new(Self::vec_to_array(sorted_bin_boundaries));
    }

    // Used to convert Vec<u8> into [u8] primarily for use with `bytes` crate.
    // Primary benefit is that `bytes::Buf` operations are infallible, i.e.
    // none of the [`Read`] functions will return with [`Err`].
    //
    // # Errors
    //
    // This function will panic if it turns out the
    fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
        v.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }
}

impl Layout for CustomLayout {
    type L = Self;

    fn map_to_bin_index(&self, value: f64) -> usize {
        if let Some(value) = self.map_to_bin_index_detail(value, 0_f64, 0_f64, 0_i64, 0_f64) {
            return value;
        }
    }
    // Unused. `map_to_bin_index_custom`
    fn map_to_bin_index_detail(
        &self,
        value: f64,
        factor_normal: f64,
        factor_subnormal: f64,
        unsigned_value_bits_normal_limit: i64,
        offset: f64,
    ) -> usize {
        let mapped_value: i64 = Self::map_double_to_long(value);
        let predicate = |&x: usize| {
            x == self.sorted_bin_boundaries.len()
                || Self::map_double_to_long(self.sorted_bin_boundaries[x]) > mapped_value
        };
        return Self::find_first(predicate, 0, self.sorted_bin_boundaries.len());
    }

    fn get_bin_lower_bound(&self, bin_index: usize) -> f64 {
        if bin_index > 0 {
            return self.sorted_bin_boundaries
                [std::cmp::min(bin_index, self.sorted_bin_boundaries.len()) - 1];
        } else {
            return f64::NEG_INFINITY;
        }
    }

    fn get_bin_upper_bound(&self, bin_index: usize) -> f64 {
        if bin_index < self.sorted_bin_boundaries.len() {
            return Self::map_long_to_double(
                Self::map_double_to_long(self.sorted_bin_boundaries[std::cmp::max(0, bin_index)])
                    - 1,
            );
        } else {
            return f64::INFINITY;
        }
    }

    fn get_underflow_bin_index(&self) -> usize {
        return 0;
    }

    fn get_overflow_bin_index(&self) -> usize {
        return self.sorted_bin_boundaries.len();
    }

    // fn hash_code(&self) -> i32 {
    //     let prime: i32 = 31;
    //     let mut result: i32 = 1;
    //     result = prime * result + Arrays::hash_code(&self.sorted_bin_boundaries);
    //     return result;
    // }

    // fn equals(&self,  obj: &Object) -> bool {
    //     if self == obj {
    //         return true;
    //     }
    //     if obj == null {
    //         return false;
    //     }
    //     if self.histogram_type != obj.histogram_type {
    //         return false;
    //     }
    //      let other: CustomLayout = obj as CustomLayout;
    //     if !Arrays::equals(&self.sorted_bin_boundaries, other.sortedBinBoundaries) {
    //         return false;
    //     }
    //     return true;
    // }

    // fn to_string(&self) -> String {
    //     return format!("{} [sortedBinBoundaries={}]", self.histogram_type, Arrays::to_string(&self.sorted_bin_boundaries));
    // }
}
