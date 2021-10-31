// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::values::value_estimation::ValueEstimation;

/// Value estimators.
/// Estimator types for the recorded values from a histogram.
///
/// The default is set in the abstract histogram.
///
/// const DEFAULT_VALUE_ESTIMATOR: impl ValueEstimation = ValueEstimatorUniform.new();
///
pub(crate) struct ValueEstimatorUniform {}
pub(crate) struct ValueEstimatorLowerBound {}
pub(crate) struct ValueEstimatorUpperBound {}
pub(crate) struct ValueEstimatorMidPoint {}

// fn new() -> Self {
//     Self {}
// }

// Implement value estimation for each estimator type.
impl ValueEstimation for ValueEstimatorUniform {
    fn new() -> Self {
        Self {}
    }
}

// Implement value estimation for each estimator type.
impl ValueEstimation for ValueEstimatorLowerBound {
    fn new() -> Self {
        Self {}
    }
}

// Implement value estimation for each estimator type.
impl ValueEstimation for ValueEstimatorUpperBound {
    fn new() -> Self {
        Self {}
    }
}

// Implement value estimation for each estimator type.
impl ValueEstimation for ValueEstimatorMidPoint {
    fn new() -> Self {
        Self {}
    }
}
