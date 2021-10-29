// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT


 const ENCOUNTERED_UNEXPECTED_DATA_MSG: &'static str = "Encountered unexpected data!";

 const INCOMPATIBLE_SERIAL_VERSION_MSG: &'static str = "Incompatible serial versions! Expected version %d but was %d.";
pub struct SerializationUtil {
}

impl SerializationUtil {

    fn new() -> SerializationUtil {
    }

    pub fn check_serial_version( expected_serial_version: i8,  current_serial_version: i32)  -> /*  throws IOException */Result<Void, Rc<Exception>>   {
        if expected_serial_version != current_serial_version {
            throw IOException::new(&String::format(null as Locale, &INCOMPATIBLE_SERIAL_VERSION_MSG, expected_serial_version, current_serial_version));
        }
    }


   /// Writes a {@code long} to the given {@link DataOutput} using variable-length encoding.
   ///
   /// @param value the {@code long} value
   /// @param dataOutput the {@link DataOutput}
   /// @throws IOException if an I/O error occurs
   ///
    pub fn write_unsigned_var_long( value: i64,  data_output: &DataOutput)  -> /*  throws IOException */Result<Void, Rc<Exception>>   {
        while (value & 0xFFFFFFFFFFFFFF80) != 0 {
            data_output.write_byte((value as i32 & 0x7F) | 0x80);
            value >>= /* >>>= */ 7;
        }
        data_output.write_byte(value as i32 & 0x7F);
    }


   /// Writes an {@code int} to the given {@link DataOutput} using variable-length and zigzag
   /// encoding.
   ///
   /// @param value the {@code int} value
   /// @param dataOutput the {@link DataOutput}
   /// @throws IOException if an I/O error occurs
   ///
    pub fn write_signed_var_int( value: i32,  data_output: &DataOutput)  -> /*  throws IOException */Result<Void, Rc<Exception>>   {
        ::write_unsigned_var_int((value << 1) ^ (value >> 31), &data_output);
    }


   /// Writes an {@code int} to the given {@link DataOutput} using variable-length encoding.
   ///
   /// @param value the {@code int} value
   /// @param dataOutput the {@link DataOutput}
   /// @throws IOException if an I/O error occurs
   ///
    pub fn write_unsigned_var_int( value: i32,  data_output: &DataOutput)  -> /*  throws IOException */Result<Void, Rc<Exception>>   {
        while (value & 0xFFFFFF80) != 0 {
            data_output.write_byte((value & 0x7F) | 0x80);
            value >>= /* >>>= */ 7;
        }
        data_output.write_byte(value & 0x7F);
    }


   /// Reads a variable-length encoded {@code long} from the given {@link DataInput}.
   ///
   /// @param dataInput the {@link DataInput}
   /// @return the read {@code long} value
   /// @throws IOException if an I/O error occurs
   ///
    pub fn read_unsigned_var_long( data_input: &DataInput) -> /*  throws IOException */Result<i64, Rc<Exception>>   {
         let mut value: i64 = 0;
         let mut i: i32 = 0;
         let mut b: i64;
        while ((b = data_input.read_byte()) & 0x80) != 0 {
            value |= (b & 0x7F) << i;
            i += 7;
            if i > 63 {
                throw IOException::new(&ENCOUNTERED_UNEXPECTED_DATA_MSG);
            }
        }
        return Ok(value | (b << i));
    }


   /// Reads a variable-length and zigzag encoded {@code long} from the given {@link DataInput}.
   ///
   /// @param dataInput the {@link DataInput}
   /// @return the read {@code long} value
   /// @throws IOException if an I/O error occurs
   ///
    pub fn read_signed_var_int( data_input: &DataInput) -> /*  throws IOException */Result<i32, Rc<Exception>>   {
         let raw: i32 = ::read_unsigned_var_int(&data_input);
         let temp: i32 = (((raw << 31) >> 31) ^ raw) >> 1;
        return Ok(temp ^ (raw & (1 << 31)));
    }


   /// Reads a variable-length encoded {@code int} from the given {@link DataInput}.
   ///
   /// @param dataInput the {@link DataInput}
   /// @return the read {@code int} value
   /// @throws IOException if an I/O error occurs
   ///
    pub fn read_unsigned_var_int( data_input: &DataInput) -> /*  throws IOException */Result<i32, Rc<Exception>>   {
         let mut value: i32 = 0;
         let mut i: i32 = 0;
         let mut b: i32;
        while ((b = data_input.read_byte()) & 0x80) != 0 {
            value |= (b & 0x7F) << i;
            i += 7;
            if i > 35 {
                throw IOException::new(&ENCOUNTERED_UNEXPECTED_DATA_MSG);
            }
        }
        return Ok(value | (b << i));
    }


   /// Writes this histogram to a given {@code byte[]}.
   ///
   /// The [`Layout`] information will not be written. Therefore, it is necessary to provide
   /// the layout when reading using {@link #readAsDynamic(Layout, byte[])}, {@link
   /// #readAsStatic(Layout, byte[])} or {@link #readAsPreprocessed(Layout, byte[])}.
   ///
   /// @param histogram the {@link Histogram}
   /// @return the {@code byte[]}
   /// @throws IOException if an I/O error occurs
   ///
    pub fn write( histogram: &Histogram) -> /*  throws IOException */Result<Vec<i8>, Rc<Exception>>   {
        return Ok(::to_byte_array(Histogram::write, histogram));
    }


   /// Reads a histogram from a given {@code byte[]}.
   ///
   /// The returned histogram will allocate internal arrays for bin counts statically. The behavior
   /// is undefined if the given layout does not match the layout before serialization.
   ///
   /// @param layout the [`Layout`]
   /// @param serializedHistogram the {@code byte[]}
   /// @return the {@link Histogram}
   /// @throws IOException if an I/O error occurs
   ///
    pub fn read_as_static( layout: &Layout,  serialized_histogram: &Vec<i8>) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(::from_byte_array( data_input: & -> Histogram::read_as_static(layout, data_input), &serialized_histogram));
    }


   /// Reads a histogram from a given {@code byte[]}.
   ///
   /// The returned histogram will allocate internal arrays for bin counts dynamically. The
   /// behavior is undefined if the given layout does not match the layout before serialization.
   ///
   /// @param layout the [`Layout`]
   /// @param serializedHistogram the {@code byte[]}
   /// @return the {@link Histogram}
   /// @throws IOException if an I/O error occurs
   ///
    pub fn read_as_dynamic( layout: &Layout,  serialized_histogram: &Vec<i8>) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(::from_byte_array( data_input: & -> Histogram::read_as_dynamic(layout, data_input), &serialized_histogram));
    }


   /// Reads a histogram from a given {@code byte[]}.
   ///
   /// The returned histogram will be immutable and preprocessed in order to support fast queries.
   /// The behavior is undefined if the given layout does not match the layout before serialization.
   ///
   /// @param layout the [`Layout`]
   /// @param serializedHistogram the {@code byte[]}
   /// @return the {@link Histogram}
   /// @throws IOException if an I/O error occurs
   ///
    pub fn read_as_preprocessed( layout: &Layout,  serialized_histogram: &Vec<i8>) -> /*  throws IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(::from_byte_array( data_input: & -> Histogram::read_as_preprocessed(layout, data_input), &serialized_histogram));
    }


   /// Writes this histogram compressed to a given {@code byte[]}.
   ///
   /// The [`Layout`] information will not be written. Therefore, it is necessary to provide
   /// the layout when reading using {@link #readCompressedAsDynamic(Layout, byte[])}, {@link
   /// #readCompressedAsStatic(Layout, byte[])} or {@link #readCompressedAsPreprocessed(Layout,
   /// byte[])}.
   ///
   /// @param histogram the {@link Histogram}
   /// @return the {@code byte[]}
   /// @throws IOException if an I/O error occurs
   ///
    pub fn write_compressed( histogram: &Histogram) -> /*  throws IOException */Result<Vec<i8>, Rc<Exception>>   {
        return Ok(::compress(&::write(histogram)));
    }


   /// Reads a histogram from a given compressed {@code byte[]}.
   ///
   /// The returned histogram will allocate internal arrays for bin counts statically. The behavior
   /// is undefined if the given layout does not match the layout before serialization.
   ///
   /// @param layout the [`Layout`]
   /// @param serializedHistogram the {@code byte[]}
   /// @return the {@link Histogram}
   /// @throws IOException if an I/O error occurs
   /// @throws DataFormatException if a data format error occurs
   ///
    pub fn read_compressed_as_static( layout: &Layout,  serialized_histogram: &Vec<i8>) -> /*  throws DataFormatException, IOException */Result<Histogram, Rc<Exception>>   {
        return Ok(::read_as_static(layout, &::decompress(&serialized_histogram)));
    }


   /// Reads a histogram from a given compressed {@code byte[]}.
   ///
   /// The returned histogram will allocate internal arrays for bin counts dynamically. The
   /// behavior is undefined if the given layout does not match the layout before serialization.
   ///
   /// @param layout the [`Layout`]
   /// @param serializedHistogram the {@code byte[]}
   /// @return the {@link Histogram}
   /// @throws IOException if an I/O error occurs
   /// @throws DataFormatException if a data format error occurs
   ///
    pub fn read_compressed_as_dynamic( layout: &Layout,  serialized_histogram: &Vec<i8>) -> /*  throws IOException, DataFormatException */Result<Histogram, Rc<Exception>>   {
        return Ok(::read_as_dynamic(layout, &::decompress(&serialized_histogram)));
    }


   /// Reads a histogram from a given compressed {@code byte[]}.
   ///
   /// The returned histogram will be immutable and preprocessed in order to support fast queries.
   /// The behavior is undefined if the given layout does not match the layout before serialization.
   ///
   /// @param layout the [`Layout`]
   /// @param serializedHistogram the {@link Histogram}
   /// @return the {@link Histogram}
   /// @throws IOException if an I/O error occurs
   /// @throws DataFormatException if a data format error occurs
   ///
    pub fn read_compressed_as_preprocessed( layout: &Layout,  serialized_histogram: &Vec<i8>) -> /*  throws IOException, DataFormatException */Result<Histogram, Rc<Exception>>   {
        return Ok(::read_as_preprocessed(layout, &::decompress(&serialized_histogram)));
    }

    fn compress( data: &Vec<i8>) -> /*  throws IOException */Result<Vec<i8>, Rc<Exception>>   {
        let tryResult1 = 0;
        'try1: loop {
        ( let output_stream: ByteArrayOutputStream = ByteArrayOutputStream::new()) {
             let deflater: Deflater = Deflater::new();
            deflater.set_input(&data);
            deflater.finish();
             let buffer: [i8; 1024] = [0; 1024];
            while !deflater.finished() {
                output_stream.write(&buffer, 0, &deflater.deflate(&buffer));
            }
            return Ok(output_stream.to_byte_array());
        }
        break 'try1
        }
        match tryResult1 {
              0 => break
        }

    }

    fn decompress( data: &Vec<i8>) -> /*  throws DataFormatException, IOException */Result<Vec<i8>, Rc<Exception>>   {
        let tryResult1 = 0;
        'try1: loop {
        ( let output_stream: ByteArrayOutputStream = ByteArrayOutputStream::new(data.len())) {
             let inflater: Inflater = Inflater::new();
            inflater.set_input(&data);
             let buffer: [i8; 1024] = [0; 1024];
            while !inflater.finished() {
                output_stream.write(&buffer, 0, &inflater.inflate(&buffer));
            }
            return Ok(output_stream.to_byte_array());
        }
        break 'try1
        }
        match tryResult1 {
              0 => break
        }

    }


   /// Deserializes an object from a given byte array.
   ///
   /// @param <T> the type to be deserialized
   /// @param byteArray the byte array
   /// @param serializationReader the serialization reader
   /// @return the deserialized data
   /// @throws IOException if an I/O error occurs
   ///
    pub fn <T>  from_byte_array( serialization_reader: &SerializationReader<T>,  byte_array: &Vec<i8>) -> /*  throws IOException */Result<T, Rc<Exception>>   {
        require_non_null(serialization_reader);
        require_non_null(&byte_array);
        let tryResult1 = 0;
        'try1: loop {
        ( let byte_array_input_stream: ByteArrayInputStream = ByteArrayInputStream::new(&byte_array)) {
            let tryResult2 = 0;
            'try2: loop {
            ( let data_input_stream: DataInputStream = DataInputStream::new(&byte_array_input_stream)) {
                return Ok(serialization_reader.read(&data_input_stream));
            }
            break 'try2
            }
            match tryResult2 {
                  0 => break
            }

        }
        break 'try1
        }
        match tryResult1 {
              0 => break
        }

    }


   /// Serializes a given object to a byte array.
   ///
   /// @param <T> the type to be serialized
   /// @param serializationWriter the serialization writer
   /// @param data the data to be serialized
   /// @return a byte array
   /// @throws IOException if an I/O error occurs
   ///
    pub fn <T>  to_byte_array( serialization_writer: &SerializationWriter<T>,  data: &T) -> /*  throws IOException */Result<Vec<i8>, Rc<Exception>>   {
        require_non_null(serialization_writer);
        let tryResult1 = 0;
        'try1: loop {
        ( let byte_array_output_stream: ByteArrayOutputStream = ByteArrayOutputStream::new()) {
            let tryResult2 = 0;
            'try2: loop {
            ( let data_output_stream: DataOutputStream = DataOutputStream::new(&byte_array_output_stream)) {
                serialization_writer.write(data, &data_output_stream);
                return Ok(byte_array_output_stream.to_byte_array());
            }
            break 'try2
            }
            match tryResult2 {
                  0 => break
            }

        }
        break 'try1
        }
        match tryResult1 {
              0 => break
        }

    }
}
