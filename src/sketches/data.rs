// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::convert::TryInto;

// Used to convert Vec<u8> into [u8] primarily for use with `bytes` crate.
// Primary benefit is that `bytes::Buf` operations are infallible, i.e.
// none of the [`Read`] functions will return with [`Err`].
//
// # Errors
//
// This function will panic if it turns out the 
fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[derive(Debug)]
pub struct DataInput {
    data: bool,
}
#[derive(Debug)]
pub struct DataOutput {
    data: bool,
}

pub struct ByteArrayOutput {
    data: bool,
}

pub struct ByteArrayInput {
    data: bool,
}

impl bytestream::StreamReader for DataInput {
    fn read_from<R: std::io::Read>(buffer: &mut R, order: bytestream::ByteOrder) -> Result<Self> {
        Ok(Self {
            data: bool::read_from(buffer, order)?,
        })
    }
}

impl bytestream::StreamWriter for DataOutput {
    fn write_to<W: std::io::Write>(
        &self,
        buffer: &mut W,
        order: bytestream::ByteOrder,
    ) -> Result<()> {
        self.bar.write_to(buffer, order)?;
        Ok(())
    }
}
