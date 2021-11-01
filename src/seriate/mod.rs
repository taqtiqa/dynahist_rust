// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod deserialization;
pub(crate) mod serialization;

trait Seriate {
    type H: Histogram; // TODO: rename Histogram trait to Sketch

    const ENCOUNTERED_UNEXPECTED_DATA_MSG: &'static str = "Encountered unexpected data!";
    const INCOMPATIBLE_SERIAL_VERSION_MSG: &'static str =
        "Incompatible serial versions! Expected version %d but was %d.";

    fn check_serial_version(
        expected_serial_version: i8,
        current_serial_version: i32,
    ) -> Result<(), Rc<DynaHistError>> {
        if expected_serial_version != current_serial_version {
            return Err(DynaHistError::IOError::new(&String::format(
                &INCOMPATIBLE_SERIAL_VERSION_MSG,
                expected_serial_version,
                current_serial_version,
            )));
        }
    }

    /// Writes a [`u64`] to the given [`DataOutput`] using variable-length encoding.
    ///
    /// @param value the [`u64`] value
    /// @param data_utput the [`DataOutput`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write_unsigned_var_long(
        value: i64,
        data_output: &DataOutput,
    ) -> Result<Void, Rc<DynaHistError>> {
        while (value & 0xFFFFFFFFFFFFFF80_u64) != 0 {
            data_output.write_byte((value as i32 & 0x7F) | 0x80);
            value >>= /* >>>= */ 7;
        }
        data_output.write_byte(value as i32 & 0x7F);
    }

    /// Writes an [`i32`] to the given [`DataOutput`] using variable-length and zigzag
    /// encoding.
    ///
    /// @param value the [`i32`] value
    /// @param dataOutput the [`DataOutput`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write_signed_var_int(
        value: i32,
        data_output: &DataOutput,
    ) -> Result<Void, Rc<DynaHistError>> {
        ::write_unsigned_var_int((value << 1) ^ (value >> 31), &data_output);
    }

    /// Writes an [`i32`] to the given [`DataOutput`] using variable-length encoding.
    ///
    /// @param value the [`i32`] value
    /// @param dataOutput the [`DataOutput`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write_unsigned_var_int(
        value: i32,
        data_output: &DataOutput,
    ) -> Result<Void, Rc<DynaHistError>> {
        while (value & 0xFFFFFF80) != 0 {
            data_output.write_byte((value & 0x7F) | 0x80);
            value >>= /* >>>= */ 7;
        }
        data_output.write_byte(value & 0x7F);
    }

    /// Reads a variable-length encoded [`u64`] from the given [`DataInput`].
    ///
    /// @param dataInput the [`DataInput`]
    /// @return the read [`u64`] value
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_unsigned_var_long(data_input: impl DataInput) -> Result<i64, Rc<DynaHistError>> {
        let mut value: i64 = 0;
        let mut i: i32 = 0;
        let mut b: i64;
        while ((b = data_input.read_byte()) & 0x80) != 0 {
            value |= (b & 0x7F) << i;
            i += 7;
            if i > 63 {
                return Err(DynaHistError::IOError::new(
                    &ENCOUNTERED_UNEXPECTED_DATA_MSG,
                ));
            }
        }
        return Ok(value | (b << i));
    }

    /// Reads a variable-length and zigzag encoded [`u64`] from the given [`DataInput`].
    ///
    /// @param dataInput the [`DataInput`]
    /// @return the read [`u64`] value
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_signed_var_int(data_input: impl DataInput) -> Result<i32, Rc<DynaHistError>> {
        let raw: i32 = ::read_unsigned_var_int(&data_input);
        let temp: i32 = (((raw << 31) >> 31) ^ raw) >> 1;
        return Ok(temp ^ (raw & (1 << 31)));
    }

    /// Reads a variable-length encoded [`i32`] from the given [`DataInput`].
    ///
    /// @param dataInput the [`DataInput`]
    /// @return the read [`i32`] value
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn read_unsigned_var_int(data_input: impl DataInput) -> Result<i32, Rc<DynaHistError>> {
        let mut value: i32 = 0;
        let mut i: i32 = 0;
        let mut b: i32;
        while ((b = data_input.read_byte()) & 0x80) != 0 {
            value |= (b & 0x7F) << i;
            i += 7;
            if i > 35 {
                return Err(DynaHistError::IOError::new(
                    &ENCOUNTERED_UNEXPECTED_DATA_MSG,
                ));
            }
        }
        return Ok(value | (b << i));
    }

    /// Writes this histogram to a given [`[u8]`].
    ///
    /// The [`Layout`] information will not be written. Therefore, it is necessary to provide
    /// the layout when reading using {@link #readAsDynamic(Layout, byte[])}, {@link
    /// #readAsStatic(Layout, byte[])} or {@link #readAsPreprocessed(Layout, byte[])}.
    ///
    /// @param histogram the [`Histogram`]
    /// @return the [`[u8]`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write(histogram: impl Histogram) -> Result<Vec<i8>, Rc<DynaHistError>> {
        return Ok(Self::to_byte_array(Histogram::write, histogram));
    }

    /// Reads a histogram from a given [`[u8]`].
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
    ) -> Result<Self::H, Rc<DynaHistError>> {
        let data_input: SerializationReader<Self::H> = Self::H::read_as_static(layout, data_input);
        return Ok(Self::from_byte_array(data_input, &serialized_histogram));
    }

    /// Reads a histogram from a given [`[u8]`].
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
    ) -> Result<Self::H, Rc<DynaHistError>> {
        let data_input: SerializationReader<Self::H> = Self::H::read_as_dynamic(layout, data_input);
        return Ok(Self::from_byte_array(data_input, &serialized_histogram));
    }

    /// Reads a histogram from a given [`[u8]`].
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
    ) -> Result<Self::H, Rc<DynaHistError>> {
        let data_input = Self::H::read_as_preprocessed(layout, data_input);
        return Ok(Self::from_byte_array(data_input, &serialized_histogram));
    }

    /// Writes this histogram compressed to a given [`[u8]`].
    ///
    /// The [`Layout`] information will not be written. Therefore, it is necessary to provide
    /// the layout when reading using {@link #readCompressedAsDynamic(Layout, byte[])}, {@link
    /// #readCompressedAsStatic(Layout, byte[])} or {@link #readCompressedAsPreprocessed(Layout,
    /// byte[])}.
    ///
    /// @param histogram the [`Histogram`]
    /// @return the [`[u8]`]
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn write_compressed(histogram: Self::H) -> Result<Vec<i8>, Rc<DynaHistError>> {
        return Ok(Self::compress(&Self::L::write(histogram)));
    }

    /// Reads a histogram from a given compressed [`[u8]`].
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
    ) -> Result<Self::H, Rc<DynaHistError>> {
        return Ok(Self::H::read_as_static(
            layout,
            &Self::H::decompress(&serialized_histogram),
        ));
    }

    /// Reads a histogram from a given compressed [`[u8]`].
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
    ) -> Result<Self::H, Rc<DynaHistError>> {
        return Ok(::read_as_dynamic(
            layout,
            &::decompress(&serialized_histogram),
        ));
    }

    /// Reads a histogram from a given compressed [`[u8]`].
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
    ) -> Result<Self::H, Rc<DynaHistError>> {
        return Ok(::read_as_preprocessed(
            layout,
            &::decompress(&serialized_histogram),
        ));
    }

    fn compress(data: &Vec<i8>) -> Result<Vec<i8>, Rc<DynaHistError>> {
        match ByteArrayOutputStream::new() {
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
        // ( let output_stream: ByteArrayOutputStream = ByteArrayOutputStream::new()) {
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
    fn decompress(data: &Vec<i8>) -> Result<Vec<i8>, Rc<DynaHistError>> {
        match ByteArrayOutputStream::new(data.len()) {
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
        // ( let output_stream: ByteArrayOutputStream = ByteArrayOutputStream::new(data.len())) {
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

    /// Deserializes an object from a given byte array.
    ///
    /// @param <T> the type to be deserialized
    /// @param byteArray the byte array
    /// @param serializationReader the serialization reader
    /// @return the deserialized data
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn from_byte_array(
        serialization_reader: impl SerializationReader<T>,
        byte_array: &Vec<i8>,
    ) -> Result<T, Rc<DynaHistError>> {
        match ByteArrayInputStream::new(&byte_array) {
            Ok(byte_array_input_stream) => match DataInputStream::new(&byte_array_input_stream) {
                Ok(data_input_stream) => return Ok(serialization_reader.read(&data_input_stream)),
                Err(e2) => return Err(e2),
            },
            Err(e1) => return Err(e1),
        };
    }

    /// Serializes a given object to a byte array.
    ///
    /// @param <T> the type to be serialized
    /// @param serializationWriter the serialization writer
    /// @param data the data to be serialized
    /// @return a byte array
    /// @return Err(DynaHist::Error::IOError) if an I/O error occurs
    ///
    fn to_byte_array(
        serialization_writer: impl SerializationWriter<T>,
        data: &T,
    ) -> Result<Vec<i8>, Rc<DynaHistError>> {
        match ByteArrayOutputStream::new() {
            Ok(byte_array_output_stream) => {
                match DataOutputStream::new(&byte_array_output_stream) {
                    Ok(data_output_stream) => {
                        serialization_writer.write(data, &data_output_stream);
                        return Ok(byte_array_output_stream.to_byte_array());
                    }
                    Err(e2) => return Err(e2),
                }
            }
            Err(e1) => return Err(e1),
        }

        // let tryResult1 = 0;
        // 'try1: loop {
        // ( let byte_array_output_stream: ByteArrayOutputStream = ByteArrayOutputStream::new()) {
        //     let tryResult2 = 0;
        //     'try2: loop {
        //     ( let data_output_stream: DataOutputStream = DataOutputStream::new(&byte_array_output_stream)) {
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
