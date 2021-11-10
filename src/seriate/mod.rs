// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod deserialization;
pub(crate) mod serialization;

use crate::utilities::macros::*;

use crate::errors::DynaHistError;
use crate::layouts::layout::Layout;
use crate::seriate::deserialization::SerializationReader;
use crate::seriate::deserialization::SeriateRead;
use crate::seriate::serialization::SerializationWriter;
use crate::seriate::serialization::SeriateWrite;
use crate::sketches::data::DataInput;
use crate::sketches::data::DataOutput;
use crate::Histogram;

use bytes::BufMut;
use bytes::Buf;

const CONST: usize = 0;

pub struct SeriateUtil {}

impl Seriate for SeriateUtil {}

// impl bytes::BufMut for SeriateUtil {}

// impl bytes::Buf for SeriateUtil {}

trait Seriate {
    type H: Histogram; // TODO: rename Histogram trait to Sketch

    const ENCOUNTERED_UNEXPECTED_DATA_MSG: &'static str = "Encountered unexpected data!";

    fn check_serial_version(
        expected_serial_version: i8,
        current_serial_version: i32,
    ) -> Result<(), std::rc::Rc<DynaHistError>> {
        if expected_serial_version != current_serial_version {
            return Err(DynaHistError::IOError.context(&dth_version_clash!(
                expected_serial_version,
                current_serial_version
            )));
        }
    }

    /// Write an [`i32`] to the given [`DataOutput`] using variable-length and zigzag
    /// encoding.
    ///
    /// - `value`: the [`i32`] value
    /// - `dataOutput`: the [`DataOutput`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write_signed_var_int(
        value: i32,
        data_output: &DataOutput,
    ) -> Result<(), std::rc::Rc<DynaHistError>> {
        Self::write_unsigned_var_int((value << 1) ^ (value >> 31), &data_output);
    }

    /// Write an [`i32`] to the given [`DataOutput`] using variable-length encoding.
    ///
    /// - `value`: the [`i32`] value
    /// - `dataOutput`: the [`DataOutput`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write_unsigned_var_int(
        value: i32,
        data_output: &DataOutput,
    ) -> Result<(), std::rc::Rc<DynaHistError>> {
        while (value & 0xFFFFFF80) != 0 {
            data_output.write_byte((value & 0x7F) | 0x80);
            value >>= /* >>>= */ 7;
        }
        data_output.write_byte(value & 0x7F);
    }

    /// Read a variable-length encoded [`u64`] from the given [`DataInput`].
    ///
    /// @param [`data_input`] the [`DataInput`]
    ///
    /// the read [`u64`] value
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_unsigned_var_long(data_input: &DataInput) -> Result<i64, std::rc::Rc<DynaHistError>> {
        let mut value: i64 = 0;
        let mut i: i32 = 0;
        let mut b: i64;
        while ((b = data_input.read_byte()) & 0x80) != 0 {
            value |= (b & 0x7F) << i;
            i += 7;
            if i > 63 {
                return Err(DynaHistError::IOError.context(&Self::ENCOUNTERED_UNEXPECTED_DATA_MSG));
            }
        }
        return Ok(value | (b << i));
    }

    /// Read a variable-length and zigzag encoded [`u64`] from the given [`DataInput`].
    ///
    /// @param [`data_input`] the [`DataInput`]
    ///
    /// the read [`u64`] value
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_signed_var_int(data_input: &DataInput) -> Result<i32, std::rc::Rc<DynaHistError>> {
        let raw: i32 = Self::read_unsigned_var_int(&data_input);
        let temp: i32 = (((raw << 31) >> 31) ^ raw) >> 1;
        return Ok(temp ^ (raw & (1 << 31)));
    }

    /// Read a variable-length encoded [`i32`] from the given [`DataInput`].
    ///
    /// @param [`data_input`] the [`DataInput`]
    ///
    /// the read [`i32`] value
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_unsigned_var_int(data_input: &DataInput) -> Result<i32, std::rc::Rc<DynaHistError>> {
        let mut value: i32 = 0;
        let mut i: i32 = 0;
        let mut b: i32;
        while ((b = data_input.read_byte()) & 0x80) != 0 {
            value |= (b & 0x7F) << i;
            i += 7;
            if i > 35 {
                return Err(DynaHistError::IOError.context(&Self::ENCOUNTERED_UNEXPECTED_DATA_MSG));
            }
        }
        return Ok(value | (b << i));
    }

    /// Write a histogram to a given [`[u8]`].
    ///
    /// The [`Layout`] information will not be written. Therefore, it is necessary to provide
    /// the layout when reading using [`#readAsDynamic(Layout, byte[])`], {@link
    /// #readAsStatic(Layout, byte[])} or [`#readAsPreprocessed(Layout, byte[])`].
    ///
    /// - `histogram`: the [`Histogram`]
    ///
    /// the [`[u8]`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write(histogram: impl Histogram) -> Result<Vec<i8>, std::rc::Rc<DynaHistError>> {
        return Ok(Self::to_byte_array(Histogram::write, histogram));
    }

    /// Read a static histogram from a given [`[u8]`].
    ///
    /// The returned histogram will allocate internal arrays for bin counts statically. The behavior
    /// is undefined if the given layout does not match the layout before serialization.
    ///
    /// @param `layout`: Any type which implements the [`Layout`] trait
    /// - `serializedHistogram`: the [`[u8]`]
    ///
    /// the [`Histogram`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_as_static(
        layout: impl Layout,
        serialized_histogram: Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        let serialization_reader =
            |data_input: SerializationReader<Self::H>| Self::H::read_as_static(layout, data_input);
        return Ok(Self::from_byte_array(
            serialization_reader,
            serialized_histogram,
        ));
    }

    /// Read a dynamic histogram from a given [`[u8]`].
    ///
    /// The returned histogram will allocate internal arrays for bin counts dynamically. The
    /// behavior is undefined if the given layout does not match the layout before serialization.
    ///
    /// - `layout`: the [`Layout`]
    /// - `serializedHistogram`: the [`[u8]`]
    ///
    /// the [`Histogram`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_as_dynamic(
        layout: impl Layout,
        serialized_histogram: Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        let serialization_reader =
            |data_input: SerializationReader<Self::H>| Self::H::read_as_dynamic(layout, data_input);
        return Ok(Self::from_byte_array(
            serialization_reader,
            serialized_histogram,
        ));
    }

    /// Read a preprocessed histogram from a given [`[u8]`].
    ///
    /// The returned histogram will be immutable and preprocessed in order to support fast queries.
    /// The behavior is undefined if the given layout does not match the layout before serialization.
    ///
    /// - `layout`: the [`Layout`]
    /// - `serializedHistogram`: the [`[u8]`]
    ///
    /// the [`Histogram`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_as_preprocessed(
        layout: impl Layout,
        serialized_histogram: Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        let serialization_reader = |data_input: SerializationReader<Self::H>| {
            Self::H::read_as_preprocessed(layout, data_input)
        };
        return Ok(Self::from_byte_array(
            serialization_reader,
            serialized_histogram,
        ));
    }

    /// Write this histogram compressed to a given [`[u8]`].
    ///
    /// The [`Layout`] information will not be written. Therefore, it is necessary to provide
    /// the layout when reading using [`#readCompressedAsDynamic(Layout, byte[])`], {@link
    /// #readCompressedAsStatic(Layout, byte[])} or {@link #readCompressedAsPreprocessed(Layout,
    /// byte[])}.
    ///
    /// - `histogram`: the [`Histogram`]
    ///
    /// the [`[u8]`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write_compressed(histogram: Self::H) -> Result<Vec<i8>, std::rc::Rc<DynaHistError>> {
        return Ok(Self::compress(&Self::L::write(histogram)));
    }

    /// Read a histogram from a given compressed [`[u8]`].
    ///
    /// The returned histogram will allocate internal arrays for bin counts statically. The behavior
    /// is undefined if the given layout does not match the layout before serialization.
    ///
    /// - `layout`: the [`Layout`]
    /// - `serializedHistogram`: the [`[u8]`]
    ///
    /// the [`Histogram`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    /// # Errors
    ///
    /// DataFormatError if a data format error occurs
    ///
    fn read_compressed_as_static(
        layout: impl Layout,
        serialized_histogram: &Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        return Ok(Self::H::read_as_static(
            layout,
            &Self::H::decompress(&serialized_histogram),
        ));
    }

    /// Read a histogram from a given compressed [`[u8]`].
    ///
    /// The returned histogram will allocate internal arrays for bin counts dynamically. The
    /// behavior is undefined if the given layout does not match the layout before serialization.
    ///
    /// - `layout`: the [`Layout`]
    /// - `serializedHistogram`: the [`[u8]`]
    ///
    /// the [`Histogram`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    /// # Errors
    ///
    /// DataFormatError if a data format error occurs
    ///
    fn read_compressed_as_dynamic(
        layout: impl Layout,
        serialized_histogram: &Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        return Ok(Self::H::read_as_dynamic(
            layout,
            &Self::decompress(&serialized_histogram),
        ));
    }

    /// Read a histogram from a given compressed [`[u8]`].
    ///
    /// The returned histogram will be immutable and preprocessed in order to support fast queries.
    /// The behavior is undefined if the given layout does not match the layout before serialization.
    ///
    /// - `layout`: the [`Layout`]
    /// - `serializedHistogram`: the [`Histogram`]
    ///
    /// the [`Histogram`]
    ///
    /// Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    /// # Errors
    ///
    /// DataFormatError if a data format error occurs
    ///
    fn read_compressed_as_preprocessed(
        layout: impl Layout,
        serialized_histogram: &Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        return Ok(Self::read_as_preprocessed(
            layout,
            &Self::decompress(&serialized_histogram),
        ));
    }

    fn compress(data: &Vec<i8>) -> Result<Vec<i8>, std::rc::Rc<DynaHistError>> {
        match bytes::BytesMut::with_capacity(1024) {
            Ok(buffer) => {
                let deflater: flate2::write::DeflateEncoder =
                    flate2::write::DeflateEncoder::new(data, flate2::Compression::default());
                let count = deflater.read(&buffer)?;
                let bytes = bytes::Bytes::from(buffer);
                let v: Vec<i8> = bytes.into();
                return Ok(v);
            }
            Err(e1) => return Err(e1),
        }
    }

    /// # Errors
    ///
    /// Return [`DynaHistError::DataFormatError`]
    ///
    fn decompress(data: Vec<i8>) -> Result<Vec<i8>, std::rc::Rc<DynaHistError>> {
        match bytes::Bytes::from(data) {
            Ok(bytes) => match bytes.as_ref() {
                Ok(buffer) => {
                    let mut v: Vec<i8> = Vec::new();
                    let mut inflater = flate2::read::DeflateDecoder::new(v);
                    inflater.write_all(buffer)?;
                    v = inflater.finish()?;
                    return Ok(v);
                }
                Err(e1) => return Err(e1),
            },
            Err(e1) => return Err(e1),
        }
    }

    /// Deserialize, and return, a histogram from a given byte array.
    ///
    /// # Port
    ///
    /// Upstream (Java) passes the byte arrays by reference.  However, all
    /// current use of this function is part part of a return.  Consequently,
    /// we can pass ownership of the byte array for the benefit of
    /// subsequently having infallible read and write operations.
    ///
    /// # Errors
    ///
    /// With infallible reads the only errors should be related to setting
    /// up the bytes data structures and the buffer slices.
    ///
    /// # Port
    ///
    /// Use the `bytes` crate for infallible read and write.
    ///
    /// # Arguments
    ///
    /// - `<H>`: The type of histogram to be deserialized
    /// - `byte_array`: The byte array
    /// - `serialization_reader`: The serialization reader
    ///
    fn from_byte_array(
        serialization_reader: impl SeriateRead,
        byte_array: Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        // Efficient byte buffer structure
        match bytes::Bytes::from(byte_array) {
            Ok(bytes) => match bytes.as_ref() {
                Ok(buffer) => return Ok(serialization_reader.read(buffer)),
                Err(e2) => return Err(e2),
            },
            Err(e1) => return Err(e1),
        };
    }

    /// Serialize a given histogram to the byte array returned.
    ///
    /// # Errors
    ///
    /// Return [`DynaHist::Error::IOError`] if an I/O error occurs.
    ///
    /// - `H`: The histogram type to be serialized
    /// - `serialization_writer`: The serialization writer for `H`
    /// - `data`: The data to be serialized
    ///
    fn to_byte_array(
        serialization_writer: SerializationWriter,
        data: &Self::H,
    ) -> Result<Vec<i8>, std::rc::Rc<DynaHistError>> {
        match bytes::BytesMut::with_capacity(1024) {
            Ok(buffer) => {
                serialization_writer.write(data, &buffer);
                let bytes = bytes::Bytes::from(buffer);
                let v: Vec<i8> = bytes.into();
                return Ok(v);
            }
            Err(e1) => return Err(e1),
        }
    }
}
