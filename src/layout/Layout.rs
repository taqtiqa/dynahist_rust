/*
 * Copyright 2020-2021 Dynatrace LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::dynatrace::dynahist::layout;

/**
 * A histogram bin layout, which defines the bins for a {@link Histogram}.
 *
 * <p>All implementations must be immutable.
 */
pub trait Layout {

    /**
   * Maps a given value to a histogram bin index.
   *
   * <p>This function must be monotonically increasing. {@link Double#NaN} must always either return
   * an index that is smaller than or equal to {@link #getUnderflowBinIndex()} or an index that is
   * larger than or equal to {@link #getOverflowBinIndex()}.
   *
   * @param value a {@code Double} value
   * @return the index of the histogram bin to which the given value is mapped to
   */
    fn  map_to_bin_index(&self,  value: f64) -> i32 ;

    /**
   * Returns the maximum index that is associated with the underflow bin of the histogram.
   *
   * <p>Note: {@link #getUnderflowBinIndex()} &lt; {@link #getOverflowBinIndex()} always holds.
   *
   * @return the maximum index that is associated with the underflow bin of the histogram
   */
    fn  get_underflow_bin_index(&self) -> i32 ;

    /**
   * Returns the minimum index that is associated with the overflow bin of the histogram.
   *
   * <p>Note: {@link #getUnderflowBinIndex()} &lt; {@link #getOverflowBinIndex()} always holds.
   *
   * @return the minimum index that is associated with the overflow bin of the histogram
   */
    fn  get_overflow_bin_index(&self) -> i32 ;

    /**
   * Returns the smallest value that is mapped to the bin with given bin index.
   *
   * <p>This method is defined for all integer values. The returned value is equal to {@link
   * Double#NEGATIVE_INFINITY} for all indices smaller than or equal to {@link
   * #getUnderflowBinIndex()}. For all indices greater than or equal to {@link
   * #getOverflowBinIndex()} the same value is returned.
   *
   * @param binIndex the bin index
   * @return the lower bound of the bin
   */
    fn default  get_bin_lower_bound(&self,  bin_index: i32) -> f64  {
        if bin_index <= self.get_underflow_bin_index() {
            return Double::NEGATIVE_INFINITY;
        }
         let effective_bin_index: i32 = Math::min(&self.get_overflow_bin_index(), bin_index);
        return map_long_to_double(&find_first( l: & -> self.map_to_bin_index(&Algorithms::map_long_to_double(l)) >= effective_bin_index, NEGATIVE_INFINITY_MAPPED_TO_LONG, POSITIVE_INFINITY_MAPPED_TO_LONG));
    }

    /**
   * Returns the largest value that is mapped to the bin with given bin index.
   *
   * <p>This method is defined for all integer values. The returned value is equal to {@link
   * Double#POSITIVE_INFINITY} for all indices greater than or equal to {@link
   * #getOverflowBinIndex()}. For all indices smaller than or equal to {@link
   * #getUnderflowBinIndex()} the same value is returned.
   *
   * @param binIndex the bin index
   * @return the lower bound of the bin
   */
    fn default  get_bin_upper_bound(&self,  bin_index: i32) -> f64  {
        if bin_index >= self.get_overflow_bin_index() {
            return Double::POSITIVE_INFINITY;
        }
         let effective_bin_index: i32 = Math::max(&self.get_underflow_bin_index(), bin_index);
        return map_long_to_double(~Algorithms::find_first( l: & -> self.map_to_bin_index(&map_long_to_double(~l)) <= effective_bin_index, ~POSITIVE_INFINITY_MAPPED_TO_LONG, ~NEGATIVE_INFINITY_MAPPED_TO_LONG));
    }

    /**
   * Writes a {@link Layout} object.
   *
   * <p>Important: Write methods for specific implementations must be registered in {@code
   * LayoutSerialization}.
   *
   * @param dataOutput a {@link DataOutput}
   * @throws IOException if an I/O error occurs
   */
    fn default  write_with_type_info(&self,  data_output: &DataOutput)  -> /*  throws IOException */Result<Void, Rc<Exception>>   {
        LayoutSerialization::write(self, &data_output);
    }

    /**
   * Reads a {@link Layout} object.
   *
   * <p>Important: Read methods for specific implementations must be registered in {@code
   * LayoutSerialization}.
   *
   * @param dataInput a {@link DataInput}
   * @return the read layout
   * @throws IOException if an I/O error occurs
   */
    fn  read_with_type_info( data_input: &DataInput) -> /*  throws IOException */Result<Layout, Rc<Exception>>   {
        return Ok(LayoutSerialization::read(&data_input));
    }

    /**
   * Returns the smallest value which can be mapped into a regular bin.
   *
   * @return the smallest value which can be mapped into a regular bin
   */
    fn default  get_normal_range_lower_bound(&self) -> f64  {
        return self.get_bin_lower_bound(self.get_underflow_bin_index() + 1);
    }

    /**
   * Returns the largest value which can be mapped into a regular bin.
   *
   * @return the largest value which can be mapped into a regular bin
   */
    fn default  get_normal_range_upper_bound(&self) -> f64  {
        return self.get_bin_upper_bound(self.get_overflow_bin_index() - 1);
    }

    /**
   * Defines the serialization of a new layout that can then be registered using {@link
   * #register(LayoutSerializationDefinition...)}.
   *
   * @param <T> a {@code Layout} type
   * @param serialVersion a unique serial version (choose some long constant that has been generated
   *     randomly)
   * @param clazz the type of the layout
   * @param writer defines the serialization of the layout
   * @param reader defines the deserialization of the layout
   * @return a new @link {@link LayoutSerializationDefinition}
   */
    fn <T extends Layout>  define_serialization( serial_version: i64,  clazz: &Class<T>,  writer: &SerializationWriter<T>,  reader: &SerializationReader<T>) -> LayoutSerializationDefinition  {
        return LayoutSerializationDefinition::new(serial_version, &clazz, writer, reader);
    }

    /**
   * Registers the given layout serialization definitions such that they are known by subsequent
   * calls of {@link #writeWithTypeInfo(DataOutput)} and {@link #readWithTypeInfo(DataInput)}.
   *
   * @param definitions the layout serializations to register
   */
    fn  register( definitions: &LayoutSerializationDefinition)   {
        LayoutSerialization::register(definitions);
    }
}

