// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Use the `strum` crate to convert enum variants into String
#[derive(strum_macros::IntoStaticStr)]
pub enum Sketch {
    Custom,
    LogOptimal,
    LogLinear,
    LogQuadratic,
    OpenTelemetryExponentialBuckets,
}

pub(crate) mod custom_layout;
pub(crate) mod guess_layout;
pub(crate) mod layout;
pub(crate) mod layout_serialization;
pub(crate) mod layout_serialization_definition;
pub(crate) mod log_linear_layout;
pub(crate) mod log_optimal_layout;
pub(crate) mod log_quadratic_layout;
pub(crate) mod open_telemetry_exponential_buckets_layout;

// reexport for `use crate::Layout`
//pub(crate) use crate::layouts::layout::Layout;
