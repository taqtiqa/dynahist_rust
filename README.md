# DynaHist for Rust

DynaHist: A Dynamic Histogram Library for Rust

Minimum Supported Rust Version: 1.43

DynaHist is a port of Otmar Ertl's and Markus Remplbauer's (DynaTrace) [DynaHist](https://github.com/dynatrace-oss/dynahist) to native Rust.

This Rust crate contains histogram implementations with configurable bin
layouts specifically designed for fast value recording.
All upstream base implementations are ported:

- The static histogram
- The dynamic histogram
- The preprocessed histogram

The crate contains all upstream bin layout implementations:

- LogOptimalLayout
- LogLinearLayout
- LogQuadraticLayout
- OpenTelemetryLayout
- CustomLayout

## What is DynaHist

Users are encouraged to read:

1. The documentation from the original [Java implementation](https://github.com/dynatrace-oss/dynahist).  Most of the usage concepts translate directly to the Rust port.
1.
1. [Why this wasn't merged with HdrHistogram](https://github.com/HdrHistogram/HdrHistogram/issues/54)

## Benchmarks

### Java

- [DynaHist v HdrHistogram](https://github.com/dynatrace-oss/dynahist#benchmarks)

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your option. This file may not be
copied, modified, or distributed except according to those terms.
