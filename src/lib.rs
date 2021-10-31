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

// Histogram bucket layouts;
mod layouts;

pub mod histogram;
pub use crate::histogram::Histogram;
