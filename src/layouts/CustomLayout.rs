// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

/** A custom histogram bin layout. */

 const SERIAL_VERSION_V0: i8 = 0;
#[derive(Layout)]
pub struct CustomLayout {

     let sorted_bin_boundaries: Vec<f64>;
}

impl CustomLayout {

    fn new( sorted_bin_boundaries: &Vec<f64>) -> CustomLayout {
        require_non_null(&sorted_bin_boundaries);
        let .sortedBinBoundaries = sorted_bin_boundaries;
    }

    pub fn create( sorted_bin_boundaries: f64) -> CustomLayout  {
        require_non_null(sorted_bin_boundaries);
        check_argument(sorted_bin_boundaries.len() > 0);
        check_argument(sorted_bin_boundaries[0] > f64::NEG_INFINITY);
         {
             let mut i: i32 = 1;
            while i < sorted_bin_boundaries.len() {
                {
                    check_argument(map_double_to_long(sorted_bin_boundaries[i - 1]) < map_double_to_long(sorted_bin_boundaries[i]));
                }
                i += 1;
             }
         }

        return CustomLayout::new(&Arrays::copy_of(sorted_bin_boundaries, sorted_bin_boundaries.len()));
    }

    pub fn map_to_bin_index(&self,  value: f64) -> usize  {
         let mapped_value: i64 = Algorithms::map_double_to_long(value);
        return Algorithms::find_first( l: & -> l == self.sorted_bin_boundaries.len() || Algorithms::map_double_to_long(self.sorted_bin_boundaries[l as i32]) > mapped_value, 0, self.sorted_bin_boundaries.len()) as i32;
    }

    pub fn get_bin_lower_bound(&self,  bin_index: usize) -> f64  {
        if bin_index > 0 {
            return self.sorted_bin_boundaries[std::cmp::min(bin_index, self.sorted_bin_boundaries.len()) - 1];
        } else {
            return f64::NEG_INFINITY;
        }
    }

    pub fn get_bin_upper_bound(&self,  bin_index: usize) -> f64  {
        if bin_index < self.sorted_bin_boundaries.len() {
            return Algorithms::map_long_to_double(Algorithms::map_double_to_long(self.sorted_bin_boundaries[std::cmp::max(0, bin_index)]) - 1);
        } else {
            return f64::INFINITY;
        }
    }

    pub fn get_underflow_bin_index(&self) -> usize  {
        return 0;
    }

    pub fn get_overflow_bin_index(&self) -> usize  {
        return self.sorted_bin_boundaries.len();
    }

    pub fn write(&self,  data_output: &DataOutput)  -> /*  throws IOException */Result<Void, Rc<Exception>>   {
        data_output.write_byte(SERIAL_VERSION_V0);
        write_unsigned_var_int(self.sorted_bin_boundaries.len(), &data_output);
        for  let boundary: f64 in self.sorted_bin_boundaries {
            data_output.write_double(boundary);
        }
    }

    pub fn read( data_input: &DataInput) -> /*  throws IOException */Result<CustomLayout, Rc<Exception>>   {
        check_serial_version(SERIAL_VERSION_V0, &data_input.read_unsigned_byte());
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

    pub fn hash_code(&self) -> i32  {
         let prime: i32 = 31;
         let mut result: i32 = 1;
        result = prime * result + Arrays::hash_code(&self.sorted_bin_boundaries);
        return result;
    }

    pub fn equals(&self,  obj: &Object) -> bool  {
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

    pub fn to_string(&self) -> String  {
        return format!("{} [sortedBinBoundaries={}]", get_class().get_simple_name(), Arrays::to_string(&self.sorted_bin_boundaries));
    }
}
