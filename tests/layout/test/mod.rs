// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct TestLayout {
    underflow_index: usize,
    overflow_index: usize,
}

impl Layout for TestLayout {
    fn new(underflow_index: usize, overflow_index: usize) -> Self {
        Self::check_argument(underflow_index < overflow_index);
        Self {
            underflow_index: underflow_index,
            overflow_index: overflow_index,
        }
    }

    fn map_to_bin_index(&self, value: f64) -> usize {
        if value.is_nan() {
            return i32::MAX;
        }
        if value >= 0.0 {
            return (value + 0.5) as usize;
        } else {
            return (value - 0.5) as usize;
        }
    }

    fn get_underflow_bin_index(&self) -> usize {
        return self.underflow_index;
    }

    fn get_overflow_bin_index(&self) -> usize {
        return self.overflow_index;
    }

    fn to_string(&self) -> String {
        return format!(
            "{} [underflowIndex={}, overflowIndex={}]",
            self.histogram_type, self.underflow_index, self.overflow_index
        );
    }

    fn hash_code(&self) -> i32 {
        let prime: i32 = 31;
        let mut result: i32 = 1;
        result = prime * result + self.overflow_index;
        result = prime * result + self.underflow_index;
        return result;
    }

    fn equals(&self, obj: &Object) -> bool {
        if self == obj {
            return true;
        }
        if obj == null {
            return false;
        }
        if self.histogram_type != obj.histogram_type {
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
