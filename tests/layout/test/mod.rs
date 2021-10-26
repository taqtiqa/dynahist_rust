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
// package com::dynatrace::dynahist::layout;

pub struct TestLayout {

     let underflow_index: i32;

     let overflow_index: i32;
}

impl Layout for TestLayout {

    pub fn new( underflow_index: i32,  overflow_index: i32) -> TestLayout {
        check_argument(underflow_index < overflow_index);
        let .underflowIndex = underflow_index;
        let .overflowIndex = overflow_index;
    }

    pub fn  map_to_bin_index(&self,  value: f64) -> i32  {
        if Double::is_na_n(value) {
            return Integer::MAX_VALUE;
        }
        if value >= 0.0 {
            return (value + 0.5) as i32;
        } else {
            return (value - 0.5) as i32;
        }
    }

    pub fn  get_underflow_bin_index(&self) -> i32  {
        return self.underflow_index;
    }

    pub fn  get_overflow_bin_index(&self) -> i32  {
        return self.overflow_index;
    }

    pub fn  to_string(&self) -> String  {
        return format!("{} [underflowIndex={}, overflowIndex={}]", get_class().get_simple_name(), self.underflow_index, self.overflow_index);
    }

    pub fn  hash_code(&self) -> i32  {
         let prime: i32 = 31;
         let mut result: i32 = 1;
        result = prime * result + self.overflow_index;
        result = prime * result + self.underflow_index;
        return result;
    }

    pub fn  equals(&self,  obj: &Object) -> bool  {
        if self == obj {
            return true;
        }
        if obj == null {
            return false;
        }
        if get_class() != obj.get_class() {
            return false;
        }
         let other: TestLayout = obj as TestLayout;
        if self.overflow_index != other.overflowIndex {
            return false;
        }
        if self.underflow_index != other.underflowIndex {
            return false;
        }
        return true;
    }
}
