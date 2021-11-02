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
use crate::utilities::data::DataInput;
use crate::utilities::data::DataOutput;
use crate::Histogram;

const CONST: usize = 0;

trait Seriate: bytes::Buf {
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

    /// Write a [`u64`] to the given [`DataOutput`] using variable-length
    /// encoding.
    ///
    /// # Errors
    ///
    /// Return [`DynaHistError::IOError`] if an I/O error occurs.
    ///
    fn write_unsigned_var_long(
        value: u64,
        data_output: &DataOutput,
    ) -> Result<(), std::rc::Rc<DynaHistError>> {
        while (value & 0xFFFFFFFFFFFFFF80_u64) != 0 {
            data_output.write_byte((value as i32 & 0x7F) | 0x80);
            value >>= /* >>>= */ 7;
        }
        data_output.write_byte(value as i32 & 0x7F);
        Ok(())
    }

    /// Write an [`i32`] to the given [`DataOutput`] using variable-length and zigzag
    /// encoding.
    ///
    /// @param value the [`i32`] value
    /// @param dataOutput the [`DataOutput`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write_signed_var_int(
        value: i32,
        data_output: &DataOutput,
    ) -> Result<(), std::rc::Rc<DynaHistError>> {
        Self::write_unsigned_var_int((value << 1) ^ (value >> 31), &data_output);
    }

    /// Write an [`i32`] to the given [`DataOutput`] using variable-length encoding.
    ///
    /// @param value the [`i32`] value
    /// @param dataOutput the [`DataOutput`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
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
    /// @return the read [`u64`] value
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
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
    /// @return the read [`u64`] value
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_signed_var_int(data_input: &DataInput) -> Result<i32, std::rc::Rc<DynaHistError>> {
        let raw: i32 = Self::read_unsigned_var_int(&data_input);
        let temp: i32 = (((raw << 31) >> 31) ^ raw) >> 1;
        return Ok(temp ^ (raw & (1 << 31)));
    }

    /// Read a variable-length encoded [`i32`] from the given [`DataInput`].
    ///
    /// @param [`data_input`] the [`DataInput`]
    /// @return the read [`i32`] value
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
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
    /// @param histogram the [`Histogram`]
    /// @return the [`[u8]`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
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
    /// @param serializedHistogram the [`[u8]`]
    /// @return the [`Histogram`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_as_static(
        layout: impl Layout,
        serialized_histogram: &Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        let serialization_reader = |data_input| Self::H::read_as_static(layout, data_input);
        return Ok(Self::from_byte_array(
            serialization_reader,
            &serialized_histogram,
        ));
    }

    /// Read a dynamic histogram from a given [`[u8]`].
    ///
    /// The returned histogram will allocate internal arrays for bin counts dynamically. The
    /// behavior is undefined if the given layout does not match the layout before serialization.
    ///
    /// @param layout the [`Layout`]
    /// @param serializedHistogram the [`[u8]`]
    /// @return the [`Histogram`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_as_dynamic(
        layout: impl Layout,
        serialized_histogram: &Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        let data_input: SerializationReader<Self::H> = Self::H::read_as_dynamic(layout, data_input);
        return Ok(Self::from_byte_array(data_input, &serialized_histogram));
    }

    /// Read a preprocessed histogram from a given [`[u8]`].
    ///
    /// The returned histogram will be immutable and preprocessed in order to support fast queries.
    /// The behavior is undefined if the given layout does not match the layout before serialization.
    ///
    /// @param layout the [`Layout`]
    /// @param serializedHistogram the [`[u8]`]
    /// @return the [`Histogram`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_as_preprocessed(
        layout: impl Layout,
        serialized_histogram: &Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        let data_input = Self::H::read_as_preprocessed(layout, data_input);
        return Ok(Self::from_byte_array(data_input, &serialized_histogram));
    }

    /// Write this histogram compressed to a given [`[u8]`].
    ///
    /// The [`Layout`] information will not be written. Therefore, it is necessary to provide
    /// the layout when reading using [`#readCompressedAsDynamic(Layout, byte[])`], {@link
    /// #readCompressedAsStatic(Layout, byte[])} or {@link #readCompressedAsPreprocessed(Layout,
    /// byte[])}.
    ///
    /// @param histogram the [`Histogram`]
    /// @return the [`[u8]`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write_compressed(histogram: Self::H) -> Result<Vec<i8>, std::rc::Rc<DynaHistError>> {
        return Ok(Self::compress(&Self::L::write(histogram)));
    }

    /// Read a histogram from a given compressed [`[u8]`].
    ///
    /// The returned histogram will allocate internal arrays for bin counts statically. The behavior
    /// is undefined if the given layout does not match the layout before serialization.
    ///
    /// @param layout the [`Layout`]
    /// @param serializedHistogram the [`[u8]`]
    /// @return the [`Histogram`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    /// @throws DataFormatException if a data format error occurs
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
    /// @param layout the [`Layout`]
    /// @param serializedHistogram the [`[u8]`]
    /// @return the [`Histogram`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    /// @throws DataFormatException if a data format error occurs
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
    /// @param layout the [`Layout`]
    /// @param serializedHistogram the [`Histogram`]
    /// @return the [`Histogram`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    /// @throws DataFormatException if a data format error occurs
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
        match ByteArrayOutput::new() {
            Ok(output_stream) => {
                let deflater: Deflater = Deflater::new();
                deflater.set_input(&data);
                deflater.finish();
                let buffer: [i8; 1024] = [0; 1024];
                while !deflater.finished() {
                    output_stream.write(&buffer, 0, &deflater.deflate(&buffer));
                }
                return Ok(output_stream.to_byte_array());
            }
            Err(e1) => return Err(e1),
        }
        // let tryResult1 = 0;
        // 'try1: loop {
        // ( let output_stream: ByteArrayOutput = ByteArrayOutput::new()) {
        //      let deflater: Deflater = Deflater::new();
        //     deflater.set_input(&data);
        //     deflater.finish();
        //      let buffer: [i8; 1024] = [0; 1024];
        //     while !deflater.finished() {
        //         output_stream.write(&buffer, 0, &deflater.deflate(&buffer));
        //     }
        //     return Ok(output_stream.to_byte_array());
        // }
        // break 'try1
        // }
        // match tryResult1 {
        //       0 => break
        // }
    }

    /// # Errors
    ///
    /// Return [`DynaHistError::DataFormatError`]
    fn decompress(data: &Vec<i8>) -> Result<Vec<i8>, std::rc::Rc<DynaHistError>> {
        match std::io::BufferReader::new(data.len()) {
            Ok(output_stream) => {
                let inflater: Inflater = Inflater::new();
                inflater.set_input(&data);
                let buffer: [i8; 1024] = [0; 1024];
                while !inflater.finished() {
                    output_stream.write(&buffer, 0, &inflater.inflate(&buffer));
                }
                return Ok(output_stream.to_byte_array());
            }
            Err(e1) => return Err(e1),
        }
        // let tryResult1 = 0;
        // 'try1: loop {
        // ( let output_stream: ByteArrayOutput = ByteArrayOutput::new(data.len())) {
        //      let inflater: Inflater = Inflater::new();
        //     inflater.set_input(&data);
        //      let buffer: [i8; 1024] = [0; 1024];
        //     while !inflater.finished() {
        //         output_stream.write(&buffer, 0, &inflater.inflate(&buffer));
        //     }
        //     return Ok(output_stream.to_byte_array());
        // }
        // break 'try1
        // }
        // match tryResult1 {
        //       0 => break
        // }
    }

    /// Deserialize a histogram from a given byte array.
    ///
    /// @param <T> the type to be deserialized
    /// @param byteArray the byte array
    /// @param serializationReader the serialization reader
    /// @return the deserialized data as a Histogram
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn from_byte_array(
        serialization_reader: impl SeriateRead,
        byte_array: &Vec<i8>,
    ) -> Result<Self::H, std::rc::Rc<DynaHistError>> {
        // Efficient byte buffer structure
        match bytes::Bytes::from(*byte_array) {
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
        serialization_writer: SerializationWriter<Self::H>,
        data: &Self::H,
    ) -> Result<Vec<i8>, std::rc::Rc<DynaHistError>> {
        match ByteArrayOutput::new() {
            Ok(byte_array_output_stream) => match DataOutput::new(&byte_array_output_stream) {
                Ok(data_output_stream) => {
                    serialization_writer.write(data, &data_output_stream);
                    return Ok(byte_array_output_stream.to_byte_array());
                }
                Err(e2) => return Err(e2),
            },
            Err(e1) => return Err(e1),
        }

        // let tryResult1 = 0;
        // 'try1: loop {
        // ( let byte_array_output_stream: ByteArrayOutput = ByteArrayOutput::new()) {
        //     let tryResult2 = 0;
        //     'try2: loop {
        //     ( let data_output_stream: DataOutput = DataOutput::new(&byte_array_output_stream)) {
        //         serialization_writer.write(data, &data_output_stream);
        //         return Ok(byte_array_output_stream.to_byte_array());
        //     }
        //     break 'try2
        //     }
        //     match tryResult2 {
        //           0 => break
        //     }

        // }
        // break 'try1
        // }
        // match tryResult1 {
        //       0 => break
        // }
    }
}
