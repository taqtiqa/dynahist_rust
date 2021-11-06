// // Histogram builders: The code we wish we had.
// Typestates: http://cliffle.com/blog/rust-typestate/#what-are-typestates
// Sketch = BinSketch + Layout
// Histogram = Sketch + Measure
// let bin = BinSketch::new()
//           .data_type(f8) // BinDataType -> BinRange: impl abs_error, rel_error
//           .value_estimator(Uniform) // Uniform, LowerBound, UpperBound, Midpoint
//           .abs_error(10)
//           .rel_error(1e5)
//           ;
//
// let sketch = Layout::new() // impl bin, max, min,
//              .type(LogOptimal) // [type(..) -> LayoutType impl ] Unit (impl size()), LogQuardatic, LogLinear, OTExponential, and Custom
//              .bin(bin) // [bin(..) -> LayoutBin: impl hilo_range, sig_digits]
//              .max(1e12)
//              .min(-1e9) // use typestate patterns to enforce order or relations
//              .hilo_ratio(1e9)
//              .sig_digits(2)
//              ;
//
// let histogram = Histogram::new()
//            .sketch(sketch) //
//            .measure(EmpiricalDensity) // EmpiricalFrequency (equal bin widths, custom layout), Guasssian, LogNormal etc.
//            .state(Dynamic) // Static, Preprocessed
//            .co_hack(10) // Coordinated omission "correction"
//            .quantile_estimator(HerrellDavis) // R1-R9(order statistics), Hill, Pot(threshold), see NIST reference: https://www.itl.nist.gov/div898/software/dataplot/refman2/auxillar/quantile.htm
//            .percentile_estimator(R8) // R1-R9, see NIST reference: https://www.itl.nist.gov/div898/handbook/prc/section2/prc262.htm
//            .iterator_strategy(NonEmpty) // All
//            .iterator(PercentileRange(Prob(0.95),Prob(1.0),Prob(0.0001))) // QuantileRange(Prob(0.0), Prob(1.0)),
//            .init()
//            ;
// Builder uses:
// - Typestate (state type parameter variation): Enforce run-time order of operations at compile-tim
// - Sealed traits
// -
//
// http://cliffle.com/blog/rust-typestate/#what-are-typestates:
// State type parameters enable:
//  - Add operations that are valid in all states, or a subset of states.
//  - Added operations are documented on the same generated rustdoc page,
//    but under headings, one per impl block.
//  - Add operations valid in more than one, but not all states: use a
//    trait to identify the states, and a constrained impl block to define
//    the operations.
//  - Add state data inside the state type used as a parameter
//
// Common use cases for typestate:
//
//     1. Enforce order of function calls
//     2. Forbid a function to be called twice
//     3. Mutually exclusive function calls
//     4. Require a function to be always called

// S is the state parameter. We require it to impl
// our {Bin,Layout,Histogram}State trait (below) to prevent users
// from trying weird types like HttpResponse<u8>.
struct BinSketch<S: BinState> {
    // This is the same field as in the previous example.
    state: Box<SomeActualState>,
    // Assure the compiler the parameter S gets used (elsewhere).
    _marker: std::marker::PhantomData<S>,
}

// BinState type options.
enum NumberType {} // expecting incoming data type
enum ErrorLimits {} // expecting error limits

trait BinState {}
impl BinState for NumberType {}
impl BinState for ErrorLimits {}

/// Operations that are valid only when setting error limit state.
impl BinSketch<NumberType> {
    fn new() -> Self {
        // ...
    }

    fn number_type(self, data: impl u8, message: &str) -> BinSketch<ErrorLimits> {
        // ...
    }
}

/// Operations that are valid only in ErrorLimits state.
impl BinSketch<ErrorLimits> {
    fn abs_limit(&mut self, error: &str) {
        // ...
    }

    fn rel_limit(self, error: &str) {
        // ...
    }
}

/// These operations are available in any state.
impl<S> BinSketch<S> {
    fn bytes_so_far(&self) -> usize { /* ... */
    }
}

/// Trait implemented by states that are setting up sketch layout.
trait LayoutState {}
impl LayoutState for Headers {}
// other states could go here

/// These operations are only available in states that
/// impl SendingState.
impl<S> HttpResponse<S>
where
    S: SendingState,
{
    fn spam_spam_spam(&mut self);
}
