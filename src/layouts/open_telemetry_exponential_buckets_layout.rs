// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;
use crate::layouts::layout::Layout;
use crate::seriate::deserialization::SeriateRead;
use crate::seriate::serialization::SeriateWrite;
use crate::seriate::Seriate;
use crate::sketches::data::DataInput;
use crate::sketches::data::DataOutput;
use crate::utilities::Algorithms;
use crate::utilities::Preconditions;

/// A tentative histogram bin layout that implements the proposal as discussed
/// in https://github.com/open-telemetry/oteps/pull/149.
///
/// BETA:
/// This Histogram is still subject to incompatible changes, or even
/// removal, in a future release.
///
/// This [`OpenTelemetryLayout`] trait is sealed.
///
pub(crate) trait OpenTelemetryLayout: Layout
where
    Self: Sized,
{
    // currently only used by OpenTelemetry exponential buckets layout
    const BOUNDARY_CONSTANTS: Vec<usize>;
    const MAX_PRECISION: i32 = 10;
    const INSTANCES: std::sync::Arc<std::sync::RwLock<Vec<Self>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![
            Self::new(
                Self::MAX_PRECISION + 1
            );
            1
        ]));

    fn new(precision: i32) -> Self;

    fn get_boundary_constant(idx: i32) -> i64 {
        return Self::BOUNDARY_CONSTANTS[idx];
    }

    fn calculate_boundaries(precision: i32) -> Vec<i64>;

    fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
        v.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }

    fn calculate_indices(boundaries: &Vec<i64>, precision: i32) -> Vec<i32>;
}

// # Port Notes
//
// The `PartialEq` trait implementation encompasses the logic in upstream `equals`.
// Because [`PartialEq`] is implemented we implement the [`Hash`] trait too.
// Like the Upstream `hash` we use `precision`. This port also provides
// `PartialOrd` and `Ord` trait implementations.
//
// We use Rust `derive(eq)` to provide an `Eq` implementation of `Eq`.
// This is an equivalence relation and hence requires the three properties of
//
// 1. transitivity: if x == y and y == z then x == z
// 2. symmetry: if x == y then y == x
// 3. reflexivity: x == x is always true
//
#[derive(Eq, Debug, Clone)]
pub struct OpenTelemetryExponentialBucketsLayout {
    boundaries: Vec<i64>,
    first_normal_value_bits: i64,
    histogram_type: String,
    index_offset: i32,
    indices: Vec<usize>,
    overflow_bin_index: usize,
    precision: i32,
    underflow_bin_index: usize,
}

impl Algorithms for OpenTelemetryExponentialBucketsLayout {}
impl Preconditions for OpenTelemetryExponentialBucketsLayout {}
impl Seriate for OpenTelemetryExponentialBucketsLayout {}

impl Layout for OpenTelemetryExponentialBucketsLayout {
    type L = Self;

    fn map_to_bin_index(&self, value: f64) -> usize {
        let value_bits: i64 = value.to_bits();
        let index: i32 = Self::map_to_bin_index_helper(
            value_bits,
            &self.indices,
            &self.boundaries,
            self.precision,
            self.first_normal_value_bits,
            self.index_offset,
        );
        return if value_bits >= 0 { index } else { -index };
    }

    fn map_to_bin_index_detail(
        &self,
        value: f64,
        factor_normal: f64,
        factor_subnormal: f64,
        unsigned_value_bits_normal_limit: i64,
        offset: f64,
    ) -> usize {
    }

    fn get_underflow_bin_index(&self) -> usize {
        return self.underflow_bin_index;
    }

    fn get_overflow_bin_index(&self) -> usize {
        return self.overflow_bin_index;
    }
}

impl PartialEq for OpenTelemetryExponentialBucketsLayout {
    fn eq(&self, other: &Self) -> bool {
        self.precision == other.precision
    }
}

impl std::hash::Hash for OpenTelemetryExponentialBucketsLayout {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.precision.hash(hasher);
    }
}

impl PartialOrd for OpenTelemetryExponentialBucketsLayout {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.precision.partial_cmp(&other.precision)
    }
}

impl Ord for OpenTelemetryExponentialBucketsLayout {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.precision.cmp(&other.precision)
    }
}

impl SeriateRead for OpenTelemetryExponentialBucketsLayout {
    fn read(data_input: &DataInput) -> Result<Self, std::rc::Rc<DynaHistError>> {
        Self::check_serial_version(Self::SERIAL_VERSION_V0, &data_input.read_unsigned_byte());
        let precision: i32 = data_input.read_unsigned_byte();
        return Ok(OpenTelemetryExponentialBucketsLayout::create(precision));
    }
}

impl SeriateWrite for OpenTelemetryExponentialBucketsLayout {
    fn write(&self, data_output: &DataOutput) -> Result<(), std::rc::Rc<DynaHistError>> {
        data_output.write_byte(Self::SERIAL_VERSION_V0);
        data_output.write_byte(self.precision);
    }
}

impl OpenTelemetryExponentialBucketsLayout {
    /// Create a histogram bin layout with exponential buckets with given precision.
    ///
    /// - `precision`: the precision
    ///
    /// a new [`OpenTelemetryExponentialBucketsLayout`] instance
    ///
    fn create(precision: i32) -> Self {
        Self::check_argument(precision >= 0);
        Self::check_argument(precision <= Self::MAX_PRECISION);
        let update_fn = |x| {
            if x != None {
                return x;
            } else {
                return Self::new(precision);
            };
        };
        // Upstream implementation (AtomicReferenceArray.update_and_get)
        // corresponds to using Arc(Mutex(Vec))
        return Self::INSTANCES::update_and_get(precision, update_fn);
    }

    fn map_to_bin_index_helper(
        value_bits: i64,
        indices: &Vec<i32>,
        boundaries: &Vec<i64>,
        precision: i32,
        first_normal_value_bits: i64,
        index_offset: i32,
    ) -> i32 {
        let mut mantissa: i64 = 0xfffffffffffff & value_bits;
        let mut exponent: i32 = ((0x7ff0000000000000 & value_bits) >> 52) as i32;
        if exponent == 0 {
            if mantissa < first_normal_value_bits {
                return mantissa as i32;
            }
            // Upstream uses java.lang.Long.numberOfLeadingZeros() which returns
            // the number of leading zeros before the highest-order set bit.
            // Input:  8 Output: 60
            // Input: 25 Output: 59
            let nlz: i32 = mantissa.leading_zeros() - 12;
            exponent -= nlz;
            mantissa <<= nlz + 1;
            mantissa &= 0x000fffffffffffff;
        }
        let i: i32 = indices[(mantissa >> /* >>> */ (52 - precision)) as i32];
        let k: i32 = i
            + (if mantissa >= boundaries[i] { 1 } else { 0 })
            + (if mantissa >= boundaries[i + 1] { 1 } else { 0 });
        return (exponent << precision) + k + index_offset;
    }

    fn get_bin_lower_bound_approximation_helper(&self, abs_bin_index: usize) -> f64 {
        if abs_bin_index < self.first_normal_value_bits {
            return f64::from_bits(abs_bin_index as i64);
        } else {
            let k: i32 = (abs_bin_index - self.index_offset) & (0xFFFFFFFF << self.precision);
            let mut exponent: i32 = (abs_bin_index - self.index_offset) >> self.precision;
            let mut mantissa: i64 = if k > 0 { self.boundaries[k - 1] } else { 0 };
            if exponent <= 0 {
                let shift: i32 = 1 - exponent;
                mantissa += 0xffffffffffffffff << shift;
                mantissa |= 0x0010000000000000;
                mantissa >>= /* >>>= */ shift;
                exponent = 0;
            }
            return f64::from_bits(mantissa | ((exponent as i64) << 52));
        }
    }

    fn get_bin_lower_bound_approximation(&self, bin_index: usize) -> f64 {
        if bin_index == 0 {
            return -0.0;
        } else if bin_index > 0 {
            return self.get_bin_lower_bound_approximation_helper(bin_index);
        } else {
            let value = -self.get_bin_lower_bound_approximation_helper(-bin_index + 1);
            let result = float_next_after::NextAfter::next_after(value, f64::INFINITY);
            return result;
        }
    }

    fn to_string(&self) -> String {
        return format!(
            "OpenTelemetryExponentialBucketsLayout [precision={}]",
            self.precision
        );
    }

    // Upstream (Java) implements an exlicit equality logic
    // fn equals(&self,  o: &Self) -> bool {
    //     if self == o {
    //         return true;
    //     }

    //     if o == null || self.histogram_type != o.histogram_type {
    //         return false;
    //     }

    //      let that: OpenTelemetryExponentialBucketsLayout = o as OpenTelemetryExponentialBucketsLayout;
    //     return self.precision == that.precision;
    // }

    // fn hash_code(&self) -> i32 {
    //     return 31 * self.precision;
    // }
}

impl OpenTelemetryLayout for OpenTelemetryExponentialBucketsLayout {
    fn calculate_boundaries(precision: i32) -> Vec<i64> {
        let mut length: i32 = 1 << precision;
        //  let mut boundaries= [0; length + 1];
        let mut boundaries = (1..=length + 1)
            .collect::<Vec<i64>>()
            .try_into()
            .expect("wrong size iterator");
        {
            let mut i = 0;
            while i < length - 1 {
                {
                    boundaries[i] =
                        Self::get_boundary_constant((i + 1) << (Self::MAX_PRECISION - precision));
                }
                i += 1;
            }
        }
        let boundaries = Self::to_array(boundaries);
        boundaries[length - 1] = 0x0010000000000000;
        boundaries[length] = 0x0010000000000000;
        return boundaries;
    }

    fn calculate_indices(boundaries: &Vec<i64>, precision: i32) -> Vec<i32> {
        let length: i32 = 1 << precision;
        //  let mut indices: [i32; len] = [0; len];
        let mut indices = (1..=length + 1)
            .collect::<Vec<i32>>()
            .try_into()
            .expect("wrong size iterator");
        let mut c: i32 = 0;
        {
            let mut i: i32 = 0;
            while i < length {
                {
                    let mantissa_lower_bound: i64 = (i as i64) << (52 - precision);
                    while boundaries[c] <= mantissa_lower_bound {
                        c += 1;
                    }
                    indices[i] = c;
                }
                i += 1;
            }
        }
        let indices = Self::to_array(indices);
        return indices;
    }

    fn new(precision: i32) -> Self {
        let histogram_type = "OpenTelemetryExponential".to_string();
        let boundaries = Self::calculate_boundaries(precision);
        let indices = Self::calculate_indices(&boundaries, precision);
        let value_bits: i32 = 0;
        let mut index: i32 = i32::MIN;
        loop {
            let next_value_bits: i32 = value_bits + 1;
            let next_index: i32 = Self::map_to_bin_index_helper(
                next_value_bits,
                &indices,
                &boundaries,
                precision,
                0,
                0,
            );
            if index == next_index {
                break;
            }
            value_bits = next_value_bits;
            index = next_index;
        }
        let first_normal_value_bits = value_bits;
        let index_offset = value_bits - index;
        let overflow_bin_index = Self::map_to_bin_index(f64::MAX) + 1;
        let underflow_bin_index = -overflow_bin_index;
        Self {
            boundaries,
            first_normal_value_bits,
            histogram_type,
            index_offset,
            indices,
            overflow_bin_index,
            precision,
            underflow_bin_index,
        }
    }

    const BOUNDARY_CONSTANTS: Vec<usize> = vec![
        0x0000000000000,
        0x002c605e2e8cf,
        0x0058c86da1c0a,
        0x0085382faef84,
        0x00b1afa5abcbf,
        0x00de2ed0ee0f5,
        0x010ab5b2cbd12,
        0x0137444c9b5b5,
        0x0163da9fb3336,
        0x019078ad6a19f,
        0x01bd1e77170b5,
        0x01e9cbfe113ef,
        0x02168143b0281,
        0x02433e494b755,
        0x027003103b10e,
        0x029ccf99d720b,
        0x02c9a3e778061,
        0x02f67ffa765e6,
        0x032363d42b028,
        0x03504f75ef072,
        0x037d42e11bbcd,
        0x03aa3e170aafe,
        0x03d7411915a8b,
        0x04044be896ab7,
        0x04315e86e7f85,
        0x045e78f5640ba,
        0x048b9b35659d9,
        0x04b8c54847a28,
        0x04e5f72f654b2,
        0x051330ec1a040,
        0x0540727fc1762,
        0x056dbbebb786c,
        0x059b0d3158575,
        0x05c866520045b,
        0x05f5c74f0bec3,
        0x06233029d8217,
        0x0650a0e3c1f89,
        0x067e197e26c15,
        0x06ab99fa6407c,
        0x06d92259d794d,
        0x0706b29ddf6de,
        0x07344ac7d9d51,
        0x0761ead925493,
        0x078f92d32085e,
        0x07bd42b72a837,
        0x07eafa86a2772,
        0x0818ba42e7d30,
        0x084681ed5a462,
        0x0874518759bc9,
        0x08a22912465f2,
        0x08d0088f80940,
        0x08fdf00068fe3,
        0x092bdf66607e0,
        0x0959d6c2c830e,
        0x0987d61701716,
        0x09b5dd646dd77,
        0x09e3ecac6f384,
        0x0a1203f067a64,
        0x0a402331b9716,
        0x0a6e4a71c726e,
        0x0a9c79b1f391a,
        0x0acab0f3a1b9c,
        0x0af8f03834e53,
        0x0b27378110974,
        0x0b5586cf98910,
        0x0b83de2530d12,
        0x0bb23d833d93f,
        0x0be0a4eb2353c,
        0x0c0f145e46c86,
        0x0c3d8bde0ce7a,
        0x0c6c0b6bdae53,
        0x0c9a93091632a,
        0x0cc922b7247f8,
        0x0cf7ba776bb95,
        0x0d265a4b520ba,
        0x0d5502343de03,
        0x0d83b23395dec,
        0x0db26a4ac0ed5,
        0x0de12a7b26301,
        0x0e0ff2c62d097,
        0x0e3ec32d3d1a3,
        0x0e6d9bb1be415,
        0x0e9c7c55189c7,
        0x0ecb6518b4875,
        0x0efa55fdfa9c5,
        0x0f294f0653b46,
        0x0f58503328e6d,
        0x0f875985e389c,
        0x0fb66affed31b,
        0x0fe584a2afb22,
        0x1014a66f951cf,
        0x1043d06807c30,
        0x1073028d7233f,
        0x10a23ce13f3e3,
        0x10d17f64d9ef2,
        0x1100ca19ad930,
        0x11301d0125b51,
        0x115f781cae1fb,
        0x118edb6db2dc1,
        0x11be46f5a032d,
        0x11edbab5e2ab6,
        0x121d36afe70ca,
        0x124cbae51a5c8,
        0x127c4756e9e06,
        0x12abdc06c31cc,
        0x12db78f613d5c,
        0x130b1e264a0e9,
        0x133acb98d40a2,
        0x136a814f204ab,
        0x139a3f4a9d923,
        0x13ca058cbae1e,
        0x13f9d416e77af,
        0x1429aaea92de0,
        0x14598a092ccb8,
        0x1489717425438,
        0x14b9612cec861,
        0x14e95934f312e,
        0x1519598da9a9a,
        0x154962388149f,
        0x15797336eb333,
        0x15a98c8a58e52,
        0x15d9ae343c1f3,
        0x1609d83606e12,
        0x163a0a912b6ad,
        0x166a45471c3c3,
        0x169a88594c158,
        0x16cad3c92df74,
        0x16fb279835224,
        0x172b83c7d517b,
        0x175be85981993,
        0x178c554eaea8a,
        0x17bccaa8d0889,
        0x17ed48695bbc1,
        0x181dce91c506a,
        0x184e5d23816c9,
        0x187ef4200632b,
        0x18af9388c8dea,
        0x18e03b5f3f36b,
        0x1910eba4df420,
        0x1941a45b1f488,
        0x1972658375d30,
        0x19a32f1f59ab5,
        0x19d4013041dc2,
        0x1a04dbb7a5b13,
        0x1a35beb6fcb76,
        0x1a66aa2fbebc7,
        0x1a979e2363cf9,
        0x1ac89a936440d,
        0x1af99f8138a1d,
        0x1b2aacee59c53,
        0x1b5bc2dc40bf1,
        0x1b8ce14c66e4d,
        0x1bbe084045cd4,
        0x1bef37b95750c,
        0x1c206fb915890,
        0x1c51b040fad16,
        0x1c82f95281c6c,
        0x1cb44aef2547b,
        0x1ce5a51860746,
        0x1d1707cfaeaed,
        0x1d4873168b9ab,
        0x1d79e6ee731d7,
        0x1dab6358e15e8,
        0x1ddce85752c72,
        0x1e0e75eb44027,
        0x1e400c1631fdc,
        0x1e71aad999e83,
        0x1ea35236f9331,
        0x1ed5022fcd91d,
        0x1f06bac594fa1,
        0x1f387bf9cda39,
        0x1f6a45cdf6086,
        0x1f9c18438ce4d,
        0x1fcdf35c1137a,
        0x1fffd7190241f,
        0x2031c37bdf873,
        0x2063b88628cd7,
        0x2095b6395e1d3,
        0x20c7bc96ffc18,
        0x20f9cba08e484,
        0x212be3578a81a,
        0x215e03bd7580d,
        0x21902cd3d09b9,
        0x21c25e9c1d6aa,
        0x21f49917ddc97,
        0x2226dc4893d64,
        0x2259282fc1f28,
        0x228b7cceeac25,
        0x22bdda27912d2,
        0x22f0403b385d3,
        0x2322af0b63c00,
        0x2355269997062,
        0x2387a6e756239,
        0x23ba2ff6254f4,
        0x23ecc1c78903a,
        0x241f5c5d05fe6,
        0x2451ffb82140b,
        0x2484abda600f0,
        0x24b760c547f16,
        0x24ea1e7a5eb35,
        0x251ce4fb2a640,
        0x254fb44931561,
        0x25828c65fa200,
        0x25b56d530b9bd,
        0x25e85711ece76,
        0x261b49a425645,
        0x264e450b3cb82,
        0x26814948bacc3,
        0x26b4565e27cde,
        0x26e76c4d0c2e6,
        0x271a8b16f0a30,
        0x274db2bd5e254,
        0x2780e341ddf2a,
        0x27b41ca5f98cc,
        0x27e75eeb3ab99,
        0x281aaa132b833,
        0x284dfe1f56381,
        0x28815b11456b1,
        0x28b4c0ea83f36,
        0x28e82fac9ceca,
        0x291ba7591bb70,
        0x294f27f18bf73,
        0x2982b17779966,
        0x29b643ec70c28,
        0x29e9df51fdee2,
        0x2a1d83a9add08,
        0x2a5130f50d65c,
        0x2a84e735a9eec,
        0x2ab8a66d10f13,
        0x2aec6e9cd037c,
        0x2b203fc675d20,
        0x2b5419eb90148,
        0x2b87fd0dad990,
        0x2bbbe92e5d3e4,
        0x2befde4f2e281,
        0x2c23dc71afbf8,
        0x2c57e39771b2f,
        0x2c8bf3c203f60,
        0x2cc00cf2f6c18,
        0x2cf42f2bda93e,
        0x2d285a6e4030c,
        0x2d5c8ebbb8a16,
        0x2d90cc15d5347,
        0x2dc5127e277e3,
        0x2df961f64158a,
        0x2e2dba7fb4e33,
        0x2e621c1c14834,
        0x2e9686ccf2e3b,
        0x2ecafa93e2f57,
        0x2eff777277ef1,
        0x2f33fd6a454d2,
        0x2f688c7cded23,
        0x2f9d24abd886b,
        0x2fd1c5f8c6b93,
        0x300670653dfe5,
        0x303b23f2d330b,
        0x306fe0a31b716,
        0x30a4a677ac277,
        0x30d975721b005,
        0x310e4d93fdefc,
        0x31432edeeb2fe,
        0x3178195479413,
        0x31ad0cf63eeac,
        0x31e209c5d33a0,
        0x32170fc4cd832,
        0x324c1ef4c560b,
        0x3281375752b40,
        0x32b658ee0da54,
        0x32eb83ba8ea32,
        0x3320b7be6e634,
        0x3355f4fb45e21,
        0x338b3b72ae62e,
        0x33c08b2641700,
        0x33f5e41798dab,
        0x342b46484ebb4,
        0x3460b1b9fd712,
        0x3496266e3fa2d,
        0x34cba466b03e1,
        0x35012ba4ea77d,
        0x3536bc2a89cc5,
        0x356c55f929ff1,
        0x35a1f912671b2,
        0x35d7a577dd72c,
        0x360d5b2b299fd,
        0x36431a2de883b,
        0x3678e281b7476,
        0x36aeb428335b5,
        0x36e48f22fa77c,
        0x371a7373aa9cb,
        0x3750611be211d,
        0x3786581d3f669,
        0x37bc587961727,
        0x37f26231e754a,
        0x3828754870747,
        0x385e91be9c812,
        0x3894b7960b71f,
        0x38cae6d05d866,
        0x39011f6f33460,
        0x393761742d809,
        0x396dace0ed4e1,
        0x39a401b7140ef,
        0x39da5ff8436bd,
        0x3a10c7a61d55c,
        0x3a4738c244064,
        0x3a7db34e59ff7,
        0x3ab4374c020be,
        0x3aeac4bcdf3ea,
        0x3b215ba294f3a,
        0x3b57fbfec6cf5,
        0x3b8ea5d318bef,
        0x3bc559212ef89,
        0x3bfc15eaadfb2,
        0x3c32dc313a8e5,
        0x3c69abf679c2e,
        0x3ca0853c10f29,
        0x3cd76803a5c01,
        0x3d0e544ede174,
        0x3d454a1f602d1,
        0x3d7c4976d27fa,
        0x3db35256dbd68,
        0x3dea64c123423,
        0x3e2180b7501cc,
        0x3e58a63b0a09b,
        0x3e8fd54df8f5c,
        0x3ec70df1c5175,
        0x3efe502816ee4,
        0x3f359bf29743f,
        0x3f6cf152ef2b8,
        0x3fa4504ac801c,
        0x3fdbb8dbcb6d2,
        0x40132b07a35df,
        0x404aa6cffa0e6,
        0x40822c367a025,
        0x40b9bb3cce07c,
        0x40f153e4a136a,
        0x4128f62f9ef0f,
        0x4160a21f72e2a,
        0x419857b5c9020,
        0x41d016f44d8f5,
        0x4207dfdcad154,
        0x423fb2709468a,
        0x42778eb1b0a8b,
        0x42af74a1af3f2,
        0x42e764423ddfd,
        0x431f5d950a897,
        0x4357609bc3851,
        0x438f6d5817663,
        0x43c783cbb50b5,
        0x43ffa3f84b9d5,
        0x4437cddf8a8fe,
        0x4470018321a1a,
        0x44a83ee4c0dbe,
        0x44e086061892e,
        0x4518d6e8d965c,
        0x4551318eb43ec,
        0x458995f95a532,
        0x45c2042a7d232,
        0x45fa7c23ce7a5,
        0x4632fde7006f4,
        0x466b8975c563f,
        0x46a41ed1d0058,
        0x46dcbdfcd34c9,
        0x471566f8827d0,
        0x474e19c691266,
        0x4786d668b3237,
        0x47bf9ce09c9ac,
        0x47f86d3001fe6,
        0x48314758980bf,
        0x486a2b5c13cd1,
        0x48a3193c2a96c,
        0x48dc10fa920a2,
        0x4915129900140,
        0x494e1e192aed2,
        0x4987337cc91a5,
        0x49c052c5916c5,
        0x49f97bf53affd,
        0x4a32af0d7d3df,
        0x4a6bec100fdbb,
        0x4aa532feaada6,
        0x4ade83db0687b,
        0x4b17dea6db7d7,
        0x4b514363e2a21,
        0x4b8ab213d5283,
        0x4bc42ab86c8f1,
        0x4bfdad5362a28,
        0x4c3739e6717ab,
        0x4c70d073537cb,
        0x4caa70fbc35a1,
        0x4ce41b817c115,
        0x4d1dd00638ed8,
        0x4d578e8bb586c,
        0x4d915713adc1f,
        0x4dcb299fddd0e,
        0x4e05063202328,
        0x4e3eeccbd7b2b,
        0x4e78dd6f1b6a7,
        0x4eb2d81d8abff,
        0x4eecdcd8e366a,
        0x4f26eba2e35f1,
        0x4f61047d48f74,
        0x4f9b2769d2ca7,
        0x4fd5546a3fc17,
        0x500f8b804f127,
        0x5049ccadc0413,
        0x508417f4531ef,
        0x50be6d55c7caa,
        0x50f8ccd3deb0d,
        0x51333670588c0,
        0x516daa2cf6642,
        0x51a8280b798f5,
        0x51e2b00da3b14,
        0x521d423536bbe,
        0x5257de83f4eef,
        0x529284fba0d85,
        0x52cd359dfd53d,
        0x5307f06ccd8bb,
        0x5342b569d4f82,
        0x537d8496d75fd,
        0x53b85df598d78,
        0x53f34187ddc28,
        0x542e2f4f6ad28,
        0x5469274e05079,
        0x54a4298571b06,
        0x54df35f7766a4,
        0x551a4ca5d920f,
        0x55556d92600f2,
        0x559098bed1be0,
        0x55cbce2cf505b,
        0x56070dde910d2,
        0x564257d56d4a3,
        0x567dac1351819,
        0x56b90a9a05c72,
        0x56f4736b527db,
        0x572fe68900573,
        0x576b63f4d854d,
        0x57a6ebb0a3c6e,
        0x57e27dbe2c4cf,
        0x581e1a1f3bd61,
        0x5859c0d59ca08,
        0x589571e3193a0,
        0x58d12d497c7fe,
        0x590cf30a919ed,
        0x5948c32824135,
        0x59849da3ffa96,
        0x59c0827ff07cc,
        0x59fc71bdc2f8f,
        0x5a386b5f43d93,
        0x5a746f664028b,
        0x5ab07dd48542a,
        0x5aec96abe0d20,
        0x5b28b9ee20d1e,
        0x5b64e79d138d8,
        0x5ba11fba87a03,
        0x5bdd62484bf57,
        0x5c19af482fc8f,
        0x5c5606bc02a6d,
        0x5c9268a5946b8,
        0x5cced506b543b,
        0x5d0b4be135acc,
        0x5d47cd36e6747,
        0x5d84590998b93,
        0x5dc0ef5b1de9f,
        0x5dfd902d47c65,
        0x5e3a3b81e85ed,
        0x5e76f15ad2149,
        0x5eb3b1b9d799a,
        0x5ef07ca0cbf10,
        0x5f2d5211826e8,
        0x5f6a320dceb71,
        0x5fa71c9784c0b,
        0x5fe411b078d27,
        0x6021115a7f849,
        0x605e1b976dc09,
        0x609b306918c14,
        0x60d84fd15612b,
        0x611579d1fb926,
        0x6152ae6cdf6f5,
        0x618feda3d829f,
        0x61cd3778bc945,
        0x620a8bed63d20,
        0x6247eb03a5585,
        0x628554bd58ee6,
        0x62c2c91c56ace,
        0x6300482276fe9,
        0x633dd1d1929fe,
        0x637b662b829f6,
        0x63b90532205d8,
        0x63f6aee7458cd,
        0x6434634ccc320,
        0x647222648ea3e,
        0x64afec30678b7,
        0x64edc0b231e41,
        0x652b9febc8fb7,
        0x656989df08719,
        0x65a77e8dcc390,
        0x65e57df9f096c,
        0x6623882552225,
        0x66619d11cdc5f,
        0x669fbcc140be8,
        0x66dde735889b8,
        0x671c1c70833f6,
        0x675a5c740edf5,
        0x6798a7420a036,
        0x67d6fcdc5386b,
        0x68155d44ca974,
        0x6853c87d4eb62,
        0x68923e87bfb7b,
        0x68d0bf65fdc34,
        0x690f4b19e9539,
        0x694de1a563367,
        0x698c830a4c8d4,
        0x69cb2f4a86ccb,
        0x6a09e667f3bcd,
        0x6a48a86475796,
        0x6a877541ee719,
        0x6ac64d0241683,
        0x6b052fa75173f,
        0x6b441d3301fef,
        0x6b8315a736c75,
        0x6bc21905d3df1,
        0x6c012750bdabf,
        0x6c404089d8e7e,
        0x6c7f64b30aa09,
        0x6cbe93ce38381,
        0x6cfdcddd47646,
        0x6d3d12e21e2fc,
        0x6d7c62dea2f8b,
        0x6dbbbdd4bc721,
        0x6dfb23c651a2f,
        0x6e3a94b549e72,
        0x6e7a10a38cee8,
        0x6eb9979302bde,
        0x6ef9298593ae5,
        0x6f38c67d286dd,
        0x6f786e7ba9fef,
        0x6fb8218301b91,
        0x6ff7df9519484,
        0x7037a8b3daadc,
        0x70777ce1303f6,
        0x70b75c1f04a85,
        0x70f7466f42e88,
        0x71373bd3d6552,
        0x71773c4eaa988,
        0x71b747e1abb25,
        0x71f75e8ec5f74,
        0x72378057e611b,
        0x7277ad3ef9011,
        0x72b7e545ec1a9,
        0x72f8286ead08a,
        0x733876bb29cb8,
        0x7378d02d50b90,
        0x73b934c7107c8,
        0x73f9a48a58174,
        0x743a1f7916e05,
        0x747aa5953c849,
        0x74bb36e0b906d,
        0x74fbd35d7cbfe,
        0x753c7b0d785e9,
        0x757d2df29ce7d,
        0x75bdec0edbb6b,
        0x75feb564267c9,
        0x763f89f46f410,
        0x768069c1a861e,
        0x76c154cdc4938,
        0x77024b1ab6e0a,
        0x77434caa72aa8,
        0x7784597eeba8f,
        0x77c5719a15ea6,
        0x780694fde5d40,
        0x7847c3ac50219,
        0x7888fda749e5e,
        0x78ca42f0c88a5,
        0x790b938ac1cf7,
        0x794cef772bcc9,
        0x798e56b7fcf04,
        0x79cfc94f2c000,
        0x7a11473eb0187,
        0x7a52d08880ada,
        0x7a94652e958aa,
        0x7ad60532e6d20,
        0x7b17b0976cfdb,
        0x7b59675e20df0,
        0x7b9b2988fb9ed,
        0x7bdcf719f6bd8,
        0x7c1ed0130c133,
        0x7c60b47635cf9,
        0x7ca2a4456e7a3,
        0x7ce49f82b0f25,
        0x7d26a62ff86f1,
        0x7d68b84f407f8,
        0x7daad5e2850ac,
        0x7decfeebc24ff,
        0x7e2f336cf4e63,
        0x7e71736819bce,
        0x7eb3bedf2e1ba,
        0x7ef615d42fa25,
        0x7f3878491c491,
        0x7f7ae63ff260a,
        0x7fbd5fbab0920,
        0x7fffe4bb55dec,
        0x80427543e1a12,
        0x80851156538be,
        0x80c7b8f4abaa9,
        0x810a6c20ea617,
        0x814d2add106da,
        0x818ff52b1ee51,
        0x81d2cb0d1736b,
        0x8215ac84fb2a6,
        0x82589994cce13,
        0x829b923e8ed53,
        0x82de968443d9b,
        0x8321a667ef1b3,
        0x8364c1eb941f8,
        0x83a7e91136c5e,
        0x83eb1bdadb46e,
        0x842e5a4a8634a,
        0x8471a4623c7ad,
        0x84b4fa24035eb,
        0x84f85b91e07f2,
        0x853bc8add9d4c,
        0x857f4179f5b21,
        0x85c2c5f83ac36,
        0x8606562ab00ed,
        0x8649f2135cf49,
        0x868d99b4492ed,
        0x86d14d0f7cd1e,
        0x87150c27004c3,
        0x8758d6fcdc666,
        0x879cad931a437,
        0x87e08febc3609,
        0x88247e08e1957,
        0x886877ec7f144,
        0x88ac7d98a669a,
        0x88f08f0f627cc,
        0x8934ac52be8f8,
        0x8978d564c63e7,
        0x89bd0a4785810,
        0x8a014afd08a94,
        0x8a4597875c645,
        0x8a89efe88dba2,
        0x8ace5422aa0dc,
        0x8b12c437bf1d4,
        0x8b574029db01f,
        0x8b9bc7fb0c302,
        0x8be05bad61779,
        0x8c24fb42ea034,
        0x8c69a6bdb5598,
        0x8cae5e1fd35c4,
        0x8cf3216b5448c,
        0x8d37f0a248b80,
        0x8d7ccbc6c19e7,
        0x8dc1b2dad04c4,
        0x8e06a5e0866d9,
        0x8e4ba4d9f60a1,
        0x8e90afc931858,
        0x8ed5c6b04b9f6,
        0x8f1ae99157737,
        0x8f60186e68794,
        0x8fa553499284b,
        0x8fea9a24e9c5c,
        0x902fed0282c8b,
        0x90754be472761,
        0x90bab6ccce12c,
        0x91002dbdab404,
        0x9145b0b91ffc6,
        0x918b3fc142a1a,
        0x91d0dad829e70,
        0x921681ffece05,
        0x925c353aa2fe2,
        0x92a1f48a640dc,
        0x92e7bff148396,
        0x932d977168083,
        0x93737b0cdc5e5,
        0x93b96ac5be7d1,
        0x93ff669e2802c,
        0x94456e9832eae,
        0x948b82b5f98e5,
        0x94d1a2f996a34,
        0x9517cf65253d1,
        0x955e07fac0ccd,
        0x95a44cbc8520f,
        0x95ea9dac8e659,
        0x9630faccf9244,
        0x9677641fe2446,
        0x96bdd9a7670b3,
        0x97045b65a51ba,
        0x974ae95cba769,
        0x9791838ec57ab,
        0x97d829fde4e50,
        0x981edcac37d05,
        0x98659b9bddb5c,
        0x98ac66cef66c8,
        0x98f33e47a22a3,
        0x993a220801829,
        0x9981121235681,
        0x99c80e685f2b5,
        0x9a0f170ca07ba,
        0x9a562c011b66e,
        0x9a9d4d47f2598,
        0x9ae47ae3481ed,
        0x9b2bb4d53fe0d,
        0x9b72fb1ffd286,
        0x9bba4dc5a3dd4,
        0x9c01acc858463,
        0x9c49182a3f091,
        0x9c908fed7d2ab,
        0x9cd81414380f3,
        0x9d1fa4a09579e,
        0x9d674194bb8d5,
        0x9daeeaf2d0cb9,
        0x9df6a0bcfc15f,
        0x9e3e62f564ad5,
        0x9e86319e32324,
        0x9ece0cb98ca4b,
        0x9f15f4499c648,
        0x9f5de8508a312,
        0x9fa5e8d07f29e,
        0x9fedf5cba4ce1,
        0xa0360f4424fcb,
        0xa07e353c29f51,
        0xa0c667b5de565,
        0xa10ea6b36d1fe,
        0xa156f23701b16,
        0xa19f4a42c7ca9,
        0xa1e7aed8eb8bc,
        0xa2301ffb99757,
        0xa2789dacfe68c,
        0xa2c127ef47a75,
        0xa309bec4a2d34,
        0xa352622f3def7,
        0xa39b1231475f8,
        0xa3e3ceccede7c,
        0xa42c980460ad8,
        0xa4756dd9cf36e,
        0xa4be504f696b1,
        0xa5073f675f924,
        0xa5503b23e255d,
        0xa599438722c04,
        0xa5e25893523d5,
        0xa62b7a4aa29a2,
        0xa674a8af46053,
        0xa6bde3c36f0e6,
        0xa7072b8950a73,
        0xa75080031e22b,
        0xa799e1330b359,
        0xa7e34f1b4bf62,
        0xa82cc9be14dcb,
        0xa876511d9ac33,
        0xa8bfe53c12e59,
        0xa909861bb2e1d,
        0xa95333beb0b7e,
        0xa99cee2742c9e,
        0xa9e6b5579fdc0,
        0xaa308951ff14d,
        0xaa7a6a1897fd3,
        0xaac457ada2804,
        0xab0e521356ebb,
        0xab58594bedefb,
        0xaba26d59a09ef,
        0xabec8e3ea86ee,
        0xac36bbfd3f37a,
        0xac80f6979f341,
        0xaccb3e100301e,
        0xad159268a5a1c,
        0xad5ff3a3c2775,
        0xadaa61c395493,
        0xadf4dcca5a414,
        0xae3f64ba4dec6,
        0xae89f995ad3ae,
        0xaed49b5eb5803,
        0xaf1f4a17a4735,
        0xaf6a05c2b82ea,
        0xafb4ce622f2ff,
        0xafffa3f84858d,
        0xb04a868742ee5,
        0xb09576115e994,
        0xb0e07298db666,
        0xb12b7c1ff9c62,
        0xb17692a8fa8ce,
        0xb1c1b6361ef31,
        0xb20ce6c9a8953,
        0xb2582465d973c,
        0xb2a36f0cf3f3a,
        0xb2eec6c13addd,
        0xb33a2b84f15fb,
        0xb3859d5a5b0b1,
        0xb3d11c43bbd62,
        0xb41ca843581bb,
        0xb468415b749b1,
        0xb4b3e78e56786,
        0xb4ff9ade433c6,
        0xb54b5b4d80d4a,
        0xb59728de5593a,
        0xb5e303930830c,
        0xb62eeb6ddfc87,
        0xb67ae07123dc3,
        0xb6c6e29f1c52b,
        0xb712f1fa1177b,
        0xb75f0e844bfc7,
        0xb7ab384014f76,
        0xb7f76f2fb5e47,
        0xb843b35578a52,
        0xb89004b3a7804,
        0xb8dc634c8d229,
        0xb928cf22749e4,
        0xb9754837a96b7,
        0xb9c1ce8e77681,
        0xba0e62292ad7e,
        0xba5b030a1064a,
        0xbaa7b133751e3,
        0xbaf46ca7a67a8,
        0xbb413568f255a,
        0xbb8e0b79a6f1f,
        0xbbdaeedc12f83,
        0xbc27df9285776,
        0xbc74dd9f4de50,
        0xbcc1e904bc1d3,
        0xbd0f01c520628,
        0xbd5c27e2cb5e5,
        0xbda95b600e20b,
        0xbdf69c3f3a207,
        0xbe43ea82a13b6,
        0xbe91462c95b60,
        0xbedeaf3f6a3c3,
        0xbf2c25bd71e09,
        0xbf79a9a9001d2,
        0xbfc73b0468d30,
        0xc014d9d2004aa,
        0xc06286141b33d,
        0xc0b03fcd0ea5d,
        0xc0fe06ff301f5,
        0xc14bdbacd586a,
        0xc199bdd85529d,
        0xc1e7ad8405be6,
        0xc235aab23e61e,
        0xc283b5655699a,
        0xc2d1cd9fa652c,
        0xc31ff36385e29,
        0xc36e26b34e066,
        0xc3bc679157e38,
        0xc40ab5fffd07b,
        0xc45912019768c,
        0xc4a77b9881650,
        0xc4f5f2c715c31,
        0xc544778fafb23,
        0xc59309f4aaca0,
        0xc5e1a9f8630ad,
        0xc630579d34ddd,
        0xc67f12e57d14c,
        0xc6cddbd398ea4,
        0xc71cb269e601f,
        0xc76b96aac2686,
        0xc7ba88988c933,
        0xc8098835a3612,
        0xc8589584661a1,
        0xc8a7b087346f5,
        0xc8f6d9406e7b6,
        0xc9460fb274c23,
        0xc99553dfa8314,
        0xc9e4a5ca6a1f9,
        0xca3405751c4db,
        0xca8372e220e61,
        0xcad2ee13da7cc,
        0xcb22770cac0fa,
        0xcb720dcef906a,
        0xcbc1b25d25338,
        0xcc1164b994d23,
        0xcc6124e6ac88c,
        0xccb0f2e6d1675,
        0xcd00cebc68e88,
        0xcd50b869d8f10,
        0xcda0aff187d02,
        0xcdf0b555dc3fa,
        0xce40c8993d63d,
        0xce90e9be12cba,
        0xcee118c6c470a,
        0xcf3155b5bab74,
        0xcf81a08d5e6ed,
        0xcfd1f95018d17,
        0xd022600053846,
        0xd072d4a07897c,
        0xd0c35732f2871,
        0xd113e7ba2c38d,
        0xd164863890fee,
        0xd1b532b08c969,
        0xd205ed248b287,
        0xd256b596f948c,
        0xd2a78c0a43f73,
        0xd2f87080d89f2,
        0xd34962fd2517b,
        0xd39a638197a3c,
        0xd3eb72109ef22,
        0xd43c8eacaa1d7,
        0xd48db95828ac7,
        0xd4def2158a91f,
        0xd53038e7402ce,
        0xd5818dcfba488,
        0xd5d2f0d16a1c3,
        0xd62461eec14bf,
        0xd675e12a31e80,
        0xd6c76e862e6d4,
        0xd7190a0529c51,
        0xd76ab3a99745b,
        0xd7bc6b75eab1f,
        0xd80e316c98398,
        0xd86005901478f,
        0xd8b1e7e2d479d,
        0xd903d8674db2c,
        0xd955d71ff6076,
        0xd9a7e40f43c8a,
        0xd9f9ff37adb4a,
        0xda4c289baaf6f,
        0xda9e603db3286,
        0xdaf0a6203e4f6,
        0xdb42fa45c4dfe,
        0xdb955cb0bfbb7,
        0xdbe7cd63a8315,
        0xdc3a4c60f7fea,
        0xdc8cd9ab294e5,
        0xdcdf7544b6b92,
        0xdd321f301b461,
        0xdd84d76fd269f,
        0xddd79e065807e,
        0xde2a72f628713,
        0xde7d5641c0658,
        0xded047eb9d12d,
        0xdf2347f63c159,
        0xdf7656641b78c,
        0xdfc97337b9b5f,
        0xe01c9e7395b56,
        0xe06fd81a2ece1,
        0xe0c3202e04c5e,
        0xe11676b197d17,
        0xe169dba76894a,
        0xe1bd4f11f8221,
        0xe210d0f3c7fbb,
        0xe264614f5a129,
        0xe2b8002730c72,
        0xe30bad7dcee91,
        0xe35f6955b7b78,
        0xe3b333b16ee12,
        0xe4070c9378843,
        0xe45af3fe592e8,
        0xe4aee9f495ddd,
        0xe502ee78b3ff7,
        0xe557018d3970b,
        0xe5ab2334ac7ee,
        0xe5ff537193e75,
        0xe653924676d76,
        0xe6a7dfb5dcecb,
        0xe6fc3bc24e351,
        0xe750a66e532eb,
        0xe7a51fbc74c84,
        0xe7f9a7af3c60c,
        0xe84e3e4933c7e,
        0xe8a2e38ce53e0,
        0xe8f7977cdb740,
        0xe94c5a1ba18bd,
        0xe9a12b6bc3182,
        0xe9f60b6fcc1c8,
        0xea4afa2a490da,
        0xea9ff79dc6d14,
        0xeaf503ccd2be6,
        0xeb4a1eb9fa9d1,
        0xeb9f4867cca6f,
        0xebf480d8d786e,
        0xec49c80faa594,
        0xec9f1e0ed4ac2,
        0xecf482d8e67f1,
        0xed49f67070436,
        0xed9f78d802dc2,
        0xedf50a122f9e6,
        0xee4aaa2188511,
        0xeea059089f2d1,
        0xeef616ca06dd7,
        0xef4be368527f7,
        0xefa1bee615a28,
        0xeff7a945e4488,
        0xf04da28a52e5a,
        0xf0a3aab5f6609,
        0xf0f9c1cb6412a,
        0xf14fe7cd31c7c,
        0xf1a61cbdf5be7,
        0xf1fc60a046a84,
        0xf252b376bba98,
        0xf2a91543ec595,
        0xf2ff860a70c22,
        0xf35605cce1614,
        0xf3ac948dd7274,
        0xf403324feb781,
        0xf459df15b82ad,
        0xf4b09ae1d78a2,
        0xf50765b6e4541,
        0xf55e3f9779ba6,
        0xf5b5288633626,
        0xf60c2085ad652,
        0xf6632798844f9,
        0xf6ba3dc155227,
        0xf7116302bd527,
        0xf768975f5ac86,
        0xf7bfdad9cbe14,
        0xf8172d74af6e2,
        0xf86e8f32a4b46,
        0xf8c600164b6dd,
        0xf91d802243c89,
        0xf9750f592e678,
        0xf9ccadbdac61d,
        0xfa245b525f439,
        0xfa7c1819e90d9,
        0xfad3e416ec354,
        0xfb2bbf4c0ba55,
        0xfb83a9bbeabd2,
        0xfbdba3692d514,
        0xfc33ac5677ab9,
        0xfc8bc4866e8ae,
        0xfce3ebfbb7238,
        0xfd3c22b8f71f2,
        0xfd9468c0d49cd,
        0xfdecbe15f6315,
        0xfe4522bb02e6e,
        0xfe9d96b2a23da,
        0xfef619ff7c2b3,
        0xff4eaca4391b6,
        0xffa74ea381efd,
    ];
}
