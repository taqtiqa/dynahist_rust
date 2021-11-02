// Copyright 2021 Mark van de Vyver
//
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![deny(unsafe_code)]
#[macro_use]
extern crate approx;
extern crate static_assertions;
extern crate tracing;

// Enumerated errors
mod errors;

// Utility traits and structs for testing and building Histograms.
mod utilities;

// Histogram bucket traits and structs.
mod bins;

// Bin value estimators
mod values;

// Histogram bucket layouts
mod layouts;

// Histogram (de)serialization
mod seriate;

// Histogram quantiles
mod quantiles;

// Raw Data input and Histogram data output
mod data;

pub mod histogram;
pub mod DynamicHistogram;
pub use crate::histogram::Histogram;
