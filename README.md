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

- CustomLayout
- LogOptimalLayout
- LogLinearLayout (approximates log-optimal)
- LogQuadraticLayout (approximates log-optimal)
- OpenTelemetryLayout (exponential base-2 layout)

  - [Specification](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/datamodel.md#histogram)
  - [Discussion](https://github.com/open-telemetry/opentelemetry-specification/issues/1776)
  - [PR](https://github.com/open-telemetry/opentelemetry-proto/pull/322)
  - [Data Model](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/datamodel.md)
  - [Transport Protocol](https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/metrics/v1/metrics.proto)
    - Idea convert OTLP ProtoBuf schema to Cap'nProto Schema

## Usage

```Rust
// Histogram builder:
let bin = Bin::new()
          .data_type(f8)
          .abs_error(10)
          .rel_error(1e5)
          ;

let layout = Layout::new()
             .type(LogOptimal) // LogQuardatic, LogLinear, OTExponential, and Custom
             .bin(bin)
             .max(1e12)
             .min(-1e9)
             .hilo_ratio(1e9)
             .sig_digits(2)
             ;

let histogram = Histogram::new()
           .layout(layout)
           .state(Dynamic) //Static, Preprocessed
           .co_hack(10) // Coordinated omission "correction"
           ;
```

## Iterators

-

## Traits

https://www.philipdaniels.com/blog/2019/rust-equality-and-ordering/

The following standard Rust traits are implemented for each Histogram layout:

- Clone (derive)
- Debug
- Default
- Display
- Eq (derive)
- Hash
- Ord
- PartialEq
- PartialOrd

## CLI Examples

### Recording

Grab event time deltas from the local kerenl `dmesg`. We convert all values
to integers, remove leading zeros and use '0' for values smaller than the kernel
log interval tick size.

```bash
dmesg --show-delta --notime |\
      cut -d"]" -f1| \
      tr -d "[<.> "| \
      sed -e 's/^0*//' -e 's/^$/0/g' | \
      dynahist --log-quadratic --dynamic --output "dmesg-dynahist-${$(date --iso-8601=date)}.dth"
```

### Converting HDR Historgrams to DynaHist Histograms

```bash
masts --hdr mydata.hdr --log-optimal --output "mydata-${$(date --iso-8601=date)}.dth"
```

Note:
If you do not choose appropriate parameters, there maybe some precision
loss when converting from HDR to DynaTrace.
What is appropriate depends on the data and the parameters used to record input
data-sketch.

### Converting OpenTelemetry Historgrams to DynaHist Histograms

```bash
masts --otlp mydata.oth --log-optimal --output "mydata-${$(date --iso-8601=date)}.dth"
```

Note:
If you do not choose appropriate parameters, there maybe some precision
loss when converting from HDR to DynaTrace.
What is appropriate depends on the data and the parameters used to record input
data-sketch.

https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/semantic_conventions/http.md
https://docs.rs/tracing-error/0.2.0/tracing_error/

https://github.com/rust-lang/rustc-hash
https://docs.rs/approx/0.5.0/approx/
https://docs.rs/bytestream/0.4.1/bytestream/
  - https://github.com/nickbabcock/bitter
https://docs.rs/decorum/0.3.1/decorum
https://docs.rs/static_assertions/1.1.0/static_assertions/
https://docs.rs/quickcheck/1.0.3/quickcheck/index.html
https://whileydave.com/2021/10/26/test-driving-the-rust-model-checker-rmc/
https://alastairreid.github.io/automatic-rust-verification-tools-2021/

  - https://github.com/tokio-rs/loom
  - https://crates.io/crates/shuttle
  - KLEE: https://project-oak.github.io/rust-verification-tools/
  - Cargo-klee: https://gitlab.henriktjader.com/pln/cargo-klee
  - SeaHorn: https://seahorn.github.io/

https://docs.capnproto-rust.org/capnpc/
https://rustrepo.com/repo/djkoloski-rust_serialization_benchmark-rust-testing
https://rustrepo.com/repo/japaric-trust-rust-testing
https://github.com/xd009642/tarpaulin

## FYI

### Bit hacks

https://graphics.stanford.edu/~seander/bithacks.html
https://gist.github.com/mfuerstenau/ba870a29e16536fdbaba

### Python to Rust

- https://github.com/konchunas/pyrs
  + MoneyType: https://medium.com/@konchunas/monkeytype-type-inference-for-transpiling-python-to-rust-64fa5a9eb966

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
