// Copyright 2021-2022 Mark van de Vyver
//
// SPDX-License-Identifier: Apache-2.0 OR MIT
#![deny(unsafe_code)]
#[macro_use]
extern crate approx;
extern crate flate2;
extern crate static_assertions;
extern crate tracing;

// Enumerated errors
mod errors;

// Utility traits and structs for testing and building Histograms.
mod utilities;

// Histogram bucket traits and structs.
mod bins;

// BinSketch value estimators
mod values;

// Histogram bucket layouts
mod layouts;

// Histogram (de)serialization
mod seriate;

// Histogram quantiles
mod quantiles;

// Sketches
pub mod sketches;

// Histograms
pub mod histograms;

pub use crate::histograms::histogram::Histogram;
