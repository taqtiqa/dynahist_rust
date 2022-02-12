// Copyright 2021-2022 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::utilities::Algorithms;
use crate::utilities::Preconditions;
use crate::values::value_estimation::ValueEstimation;

// Sealing a trait stops other crates from implementing any traits that use it.
mod private {
    pub trait Sealed {}
}

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
impl Algorithms for ValueEstimatorUniform {}
impl Preconditions for ValueEstimatorUniform {}
impl private::Sealed for ValueEstimatorUniform {}
impl ValueEstimation for ValueEstimatorUniform {
    fn new() -> Self {
        Self {}
    }
}

// Implement value estimation for each estimator type.
impl Algorithms for ValueEstimatorLowerBound {}
impl Preconditions for ValueEstimatorLowerBound {}
impl ValueEstimation for ValueEstimatorLowerBound {
    fn new() -> Self {
        Self {}
    }
}

// Implement value estimation for each estimator type.
impl Algorithms for ValueEstimatorUpperBound {}
impl Preconditions for ValueEstimatorUpperBound {}
impl ValueEstimation for ValueEstimatorUpperBound {
    fn new() -> Self {
        Self {}
    }
}

// Implement value estimation for each estimator type.
impl Algorithms for ValueEstimatorMidPoint {}
impl Preconditions for ValueEstimatorMidPoint {}
impl ValueEstimation for ValueEstimatorMidPoint {
    fn new() -> Self {
        Self {}
    }
}
