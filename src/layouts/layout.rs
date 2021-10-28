// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::errors::DynaHistError;
use crate::utilities::Algorithms;
use crate::utilities::data::{ DataInput, DataOutput };
use crate::utilities::Preconditions;

/// A histogram bin layout, which defines the bins for a [`Histogram`].
///
/// All implementations must be immutable.
///
pub(crate) trait Layout: Preconditions + Algorithms {

    /// Map a given value to a histogram bin index, the index of the histogram
    /// bin to which the given value is mapped to.
    ///
    /// This function must be monotonically increasing.
    ///
    /// [`map_to_bin_index`] must always either return an index that is smaller
    /// than or equal to [`get_underflow_bin_index`] or an index that is
    /// larger than or equal to [`get_overflow_bin_index`].
    ///
    fn  map_to_bin_index(&self,  value: f64) -> i32 ;


    /// Return the maximum index that is associated with the underflow bin of the histogram.
    ///
    /// **Note:**
    ///
    ///  `get_underflow_bin_index < get_overflow_bin_index()` always holds.
    ///
    fn  get_underflow_bin_index(&self) -> usize ;

   /// Return the minimum index that is associated with the overflow bin of the histogram.
   ///
   /// **Note:**
   ///
   /// `get_underflow_bin_index < get_overflow_bin_index()` always holds.
   ///
    fn  get_overflow_bin_index(&self) -> usize ;

    /// Return the lower bound of the bin; the smallest value that is mapped
    /// to the bin with the given bin index.
    ///
    /// This method is defined for all integer values.
    ///
    /// For all indices smaller than or equal to [`get_underflow_bin_index`]
    /// the value [`f64::NEG_INFINITY`] is returned.
    ///
    /// For all indices greater than or equal to [`get_overflow_bin_index`]
    /// the same value is returned.
    ///
    /// # Arguments
    ///
    /// * `bin_index` - The histogram bin index
    ///
    fn get_bin_lower_bound(&self,  bin_index: usize) -> f64  {
        if bin_index <= self.get_underflow_bin_index() {
            return f64::NEG_INFINITY;
        }
        let effective_bin_index: i32 = std::cmp::min(&self.get_overflow_bin_index(), bin_index);
        // Benchmark using an enum/struct that caches results.
        // See The Rust Book example [Storing Closures Using Generic Parameters and the Fn Traits](https://doc.rust-lang.org/book/ch13-01-closures.html#storing-closures-using-generic-parameters-and-the-fn-traits)
        let predicate = |&x: i32| Self::map_to_bin_index(Self::map_long_to_double(x)) >= effective_bin_index;
        let first = Self::find_first(predicate, Self::NEGATIVE_INFINITY_MAPPED_TO_LONG, Self::POSITIVE_INFINITY_MAPPED_TO_LONG);
        return Self::map_long_to_double(first);
    }


   /// Return the largest value that is mapped to the bin with given bin index,
   /// i.e. the upper bound of the bin.
   ///
   /// This method is defined for all integer values. T
   ///
   /// For all indices greater than or equal to [`get_overflow_bin_index`],
   /// the [`f64::INFINITY`] value is returned.
   ///
   /// For all indices smaller than or equal to [`get_underflow_bin_index`],
   /// the same value is returned. (clarify)
   ///
   /// @param bin_index the bin index
   ///
    fn get_bin_upper_bound(&self,  bin_index: usize) -> f64  {
        if bin_index >= self.get_overflow_bin_index() {
            return f64::INFINITY;
        }
        let effective_bin_index: i32 = std::cmp::max(&self.get_underflow_bin_index(), bin_index);
        // Benchmark using an enum/struct that caches results.
        // See The Rust Book example [Storing Closures Using Generic Parameters and the Fn Traits](https://doc.rust-lang.org/book/ch13-01-closures.html#storing-closures-using-generic-parameters-and-the-fn-traits)
        let predicate = |&x: i64| Self::map_to_bin_index(Self::map_long_to_double(x)) <= effective_bin_index;
        let first = Self::find_first(predicate, Self::POSITIVE_INFINITY_MAPPED_TO_LONG, Self::NEGATIVE_INFINITY_MAPPED_TO_LONG);
        return Self::map_long_to_double(first);
    }


    /// Writes a [`Layout`] object.
    ///
    /// Important: Write methods for specific implementations must be registered in {@code
    /// LayoutSerialization}.
    ///
    /// # Errors
    ///
    /// [`DynaHistError::IOError`] if an I/O error occurs.
    /// If any byte cannot be read for any reason other than end of file,
    /// an [`DynaHistError::IOError`] other than EOFException is returned.
    /// In particular, an [`DynaHistError::IOError`] may result if the input
    /// stream has been closed, e.g. a network outage.
    ///
    fn write_with_type_info(&self,  data_output: &DataOutput)  -> /*  throws IOException */Result<(), std::rc::Rc<DynaHistError>>   {
        return Ok(LayoutSerialization::write(self, data_output));
    }


    /// Read and return the read [`Layout`] object.
    ///
    /// Important: Read methods for specific implementations must be registered
    /// in {@code LayoutSerialization}.
    ///
    /// # Errors
    ///
    /// [`DynaHistError::IOError`] if an I/O error occurs.
    /// If any byte cannot be read for any reason other than end of file,
    /// an [`DynaHistError::IOError`] other than EOFException is returned.
    /// In particular, an [`DynaHistError::IOError`] may result if the input
    /// stream has been closed, e.g. a network outage.
    ///
    fn  read_with_type_info( data_input: &DataInput) -> /*  throws IOException */Result<Layout, std::rc::Rc<DynaHistError>>   {
        return Ok(LayoutSerialization::read(data_input));
    }


    /// Return the smallest value which can be mapped into a regular bin.
    ///
    fn get_normal_range_lower_bound(&self) -> f64  {
        return self.get_bin_lower_bound(self.get_underflow_bin_index() + 1);
    }


    /// Return the largest value which can be mapped into a regular bin.
    ///
    fn get_normal_range_upper_bound(&self) -> f64  {
        return self.get_bin_upper_bound(self.get_overflow_bin_index() - 1);
    }


   /// Defines the serialization of a new layout that can then be registered
   /// using {@link register(LayoutSerializationDefinition...)}.
   /// Return a new @link {@link LayoutSerializationDefinition}
   ///
   /// # Examples
   ///
   /// # Arguments
   ///
   /// - `T` a [`Layout`] type
   /// - `serial_version` a unique serial version (choose some long constant
   /// that has been generated randomly)
   /// - `class` the type of the layout (links+more detail)
   /// - `writer` defines the serialization of the layout (links+more detail)
   /// - `reader` defines the deserialization of the layout (links+more detail)
   ///
    fn<T: Layout>  define_serialization( serial_version: i64,  class: &T,  writer: &SerializationWriter<T>,  reader: &SerializationReader<T>) -> LayoutSerializationDefinition  {
        return LayoutSerializationDefinition::new(serial_version, &class, writer, reader);
    }


   /// Register the given layout serialization definitions such that they are
   /// known by subsequent calls of [`write_with_type_info`] and
   /// [`read_with_type_info`].
   ///
   /// # Arguments
   ///
   /// - `definitions` are the layout serializations to register.
   ///
    fn  register( definitions: &LayoutSerializationDefinition)   {
        LayoutSerialization::register(definitions);
    }
}
