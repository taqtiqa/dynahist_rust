// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::serialization::seriate::Seriate;

trait SeriateTest: Seriate {
    const HEX_UPPER_CASE_CHARACTERS: Vec<char> = "0123456789ABCDEF".to_char_array();
}

pub struct SerializationTestUtil {}

impl SeriateTest for SerializationTestUtil {

    fn new() -> Self {
    }

    fn test_serialization( data: &T,  writer: &SerializationWriter<T>,  reader: &SerializationReader<T>) -> Result<T, Rc<DynaHistError>> {
         let bytes: Vec<i8> = ::to_byte_array(writer, data);
        return Ok(::from_byte_array(reader, &bytes));
    }

    fn test_serialization( data: &T,  writer: &SerializationWriter<T>,  reader: &SerializationReader<T>,  expected_hex_serialization: &String) -> Result<T, Rc<DynaHistError>> {
         let hex_serialization: String = ::byte_array_to_hex_string(&::to_byte_array(writer, data));
        assert_that(&hex_serialization).is_equal_to(&expected_hex_serialization);
        return Ok(::from_byte_array(reader, &::hex_string_to_byte_array(&hex_serialization)));
    }

    fn test_reading( reader: &SerializationReader<T>,  hex_serialization: &String) -> Result<T, Rc<DynaHistError>> {
        return Ok(::from_byte_array(reader, &::hex_string_to_byte_array(&hex_serialization)));
    }

    fn to_byte_array( writer: &SerializationWriter<T>,  data: &T) -> Result<Vec<i8>, Rc<DynaHistError>> {
         let bytes: Vec<i8> = ::to_byte_array_helper(writer, data);
        // repeat serialization multiple times to see if output is the same
         let repetitions: i32 = 5;
         {
             let mut i: i32 = 0;
            while i < repetitions {
                {
                    assert_that(&::to_byte_array_helper(writer, data)).is_equal_to(&bytes);
                }
                i += 1;
             }
         }

        return Ok(bytes);
    }

    fn to_byte_array_helper( writer: &SerializationWriter<T>,  data: &T) -> Result<Vec<i8>, Rc<DynaHistError>> {
         let bos: ByteArrayOutputStream = ByteArrayOutputStream::new();
         let dos: DataOutputStream = DataOutputStream::new(&bos);
        writer.write(data, &dos);
        return Ok(bos.to_byte_array());
    }

    fn from_byte_array( reader: &SerializationReader<T>,  bytes: &Vec<i8>) -> Result<T, Rc<DynaHistError>> {
         let bis: ByteArrayInputStream = ByteArrayInputStream::new(&bytes);
         let dis: DataInputStream = DataInputStream::new(&bis);
         let deserialized_data: T = reader.read(&dis);
        assert_throws(EOFException.class, dis::readByte);
        return Ok(deserialized_data);
    }

    fn hex_string_to_byte_array( s: &String) -> Vec<i8>  {
         let len: i32 = s.length();
        check_argument(len % 2 == 0);
         let mut data: [i8; len / 2] = [0; len / 2];
         {
             let mut i: i32 = 0;
            while i < len {
                {
                     let char1: i32 = Character::digit(&s.char_at(i + 0), 16);
                     let char2: i32 = Character::digit(&s.char_at(i + 1), 16);
                    data[i / 2] = ((char1 << 4) + char2) as i8;
                }
                i += 2;
             }
         }

        return data;
    }

    fn byte_array_to_hex_string( bytes: &Vec<i8>) -> String  {
        check_argument(bytes.len() <= i32::MAX >> /* >>> */ 1);
         let hex_chars: [Option<char>; bytes.len() << 1] = [None; bytes.len() << 1];
         {
             let mut j: i32 = 0;
            while j < bytes.len() {
                {
                     let v: i32 = bytes[j] & 0xFF;
                    hex_chars[(j << 1) + 0] = HEX_UPPER_CASE_CHARACTERS[v >> /* >>> */ 4];
                    hex_chars[(j << 1) + 1] = HEX_UPPER_CASE_CHARACTERS[v & 0x0F];
                }
                j += 1;
             }
         }

        return String::new(&hex_chars);
    }
}
