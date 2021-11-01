// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::layouts::layout::Layout;

/// A custom histogram bin layout.
pub struct CustomLayout {
    sorted_bin_boundaries: Vec<f64>,
}

impl CustomLayout {
        fn new( sorted_bin_boundaries: &Vec<f64>) -> CustomLayout {
        sorted_bin_boundaries;
    }
}

impl Layout for CustomLayout {

    fn create( sorted_bin_boundaries: f64) -> CustomLayout {
        Self::check_argument(sorted_bin_boundaries.len() > 0);
        Self::check_argument(sorted_bin_boundaries[0] > f64::NEG_INFINITY);
        {
             let mut i: i32 = 1;
            while i < sorted_bin_boundaries.len() {
               {
                    Self::check_argument(Self::map_double_to_long(sorted_bin_boundaries[i - 1]) < Self::map_double_to_long(sorted_bin_boundaries[i]));
                }
                i += 1;
             }
         }

        return CustomLayout::new(&Arrays::copy_of(sorted_bin_boundaries, sorted_bin_boundaries.len()));
    }

    fn map_to_bin_index(&self,  value: f64) -> usize {
         let mapped_value: i64 = Self::map_double_to_long(value);
        return Self::find_first( l: & -> l == self.sorted_bin_boundaries.len() || Self::map_double_to_long(self.sorted_bin_boundaries[l as i32]) > mapped_value, 0, self.sorted_bin_boundaries.len()) as i32;
    }

    fn get_bin_lower_bound(&self,  bin_index: usize) -> f64 {
        if bin_index > 0 {
            return self.sorted_bin_boundaries[std::cmp::min(bin_index, self.sorted_bin_boundaries.len()) - 1];
        } else {
            return f64::NEG_INFINITY;
        }
    }

    fn get_bin_upper_bound(&self,  bin_index: usize) -> f64 {
        if bin_index < self.sorted_bin_boundaries.len() {
            return Self::map_long_to_double(Self::map_double_to_long(self.sorted_bin_boundaries[std::cmp::max(0, bin_index)]) - 1);
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

    fn write(&self,  data_output: &DataOutput)  -> Result<(), std::rc::Rc<DynaHistError>> {
        data_output.write_byte(Self::SERIAL_VERSION_V0);
        Self::write_unsigned_var_int(self.sorted_bin_boundaries.len(), &data_output);
        for boundary in self.sorted_bin_boundaries {
            data_output.write_double(boundary);
        }
    }

    fn read( data_input: impl DataInput) -> Result<CustomLayout, std::rc::Rc<DynaHistError>> {
        check_serial_version(Self::SERIAL_VERSION_V0, &data_input.read_unsigned_byte());
         let len: i32 = read_unsigned_var_int(&data_input);
         let sorted_bin_boundaries: [f64; len] = [0.0; len];
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

    fn hash_code(&self) -> i32 {
         let prime: i32 = 31;
         let mut result: i32 = 1;
        result = prime * result + Arrays::hash_code(&self.sorted_bin_boundaries);
        return result;
    }

    fn equals(&self,  obj: &Object) -> bool {
        if self == obj {
            return true;
        }

        if obj == null {
            return false;
        }

        if get_class() != obj.get_class() {
            return false;
        }

         let other: CustomLayout = obj as CustomLayout;
        if !Arrays::equals(&self.sorted_bin_boundaries, other.sortedBinBoundaries) {
            return false;
        }

        return true;
    }

    fn to_string(&self) -> String {
        return format!("{} [sortedBinBoundaries={}]", get_class().get_simple_name(), Arrays::to_string(&self.sorted_bin_boundaries));
    }
}
