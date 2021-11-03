// Copyright 2021 Mark van de Vyver
// Copyright 2020-2021 Dynatrace LLC
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub struct AlgorithmsTest {
}

impl Preconditions for AlgorithmsTest { }

impl Algorithms for AlgorithmsTest {

    #[test]
    fn test_interpolate(&self) {
        assert_eq!(4.5, interpolate(3.5, 3, 4, 4, 5), 0.0);
        assert_eq!(4.5, interpolate(3, 3, 4, 3, 5), 0.0);
        assert_eq!(4.5, interpolate(2, 3, 4, 3, 5), 0.0);
        assert_eq!(4, interpolate(2, 3, 4, 4, 5), 0.0);
        assert_eq!(5, interpolate(6, 3, 4, 4, 5), 0.0);
        assert_eq!(f64::INFINITY, interpolate(2, 3, f64::INFINITY, 4, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(3.5, 3, f64::INFINITY, 4, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(5, 3, f64::INFINITY, 4, f64::INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(2, 3, f64::NEG_INFINITY, 4, f64::INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(3.5, 3, f64::NEG_INFINITY, 4, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(5, 3, f64::NEG_INFINITY, 4, f64::INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(2, 3, f64::NEG_INFINITY, 4, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(3.5, 3, f64::NEG_INFINITY, 4, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(5, 3, f64::NEG_INFINITY, 4, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(2, 3, f64::INFINITY, 4, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(3.5, 3, f64::INFINITY, 4, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(5, 3, f64::INFINITY, 4, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(-1, 0, f64::INFINITY, 0, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(0, 0, f64::INFINITY, 0, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(1, 0, f64::INFINITY, 0, f64::INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(-1, 0, f64::NEG_INFINITY, 0, f64::INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(0, 0, f64::NEG_INFINITY, 0, f64::INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(1, 0, f64::NEG_INFINITY, 0, f64::INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(-1, 0, f64::INFINITY, 0, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(0, 0, f64::INFINITY, 0, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(1, 0, f64::INFINITY, 0, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(-1, 0, f64::NEG_INFINITY, 0, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(0, 0, f64::NEG_INFINITY, 0, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(1, 0, f64::NEG_INFINITY, 0, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(-1, 0, f64::INFINITY, 0, 0), 0.0);
        assert_eq!(f64::INFINITY, interpolate(0, 0, f64::INFINITY, 0, 0), 0.0);
        assert_eq!(f64::INFINITY, interpolate(1, 0, f64::INFINITY, 0, 0), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(-1, 0, f64::NEG_INFINITY, 0, 0), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(0, 0, f64::NEG_INFINITY, 0, 0), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(1, 0, f64::NEG_INFINITY, 0, 0), 0.0);
        assert_eq!(f64::INFINITY, interpolate(-1, 0, 0, 0, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(0, 0, 0, 0, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(1, 0, 0, 0, f64::INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(-1, 0, 0, 0, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(0, 0, 0, 0, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(1, 0, 0, 0, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(-1, -1, f64::INFINITY, 1, 0), 0.0);
        assert_eq!(f64::INFINITY, interpolate(0, -1, f64::INFINITY, 1, 0), 0.0);
        assert_eq!(0, interpolate(1, -1, f64::INFINITY, 1, 0), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(-1, -1, f64::NEG_INFINITY, 1, 0), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(0, -1, f64::NEG_INFINITY, 1, 0), 0.0);
        assert_eq!(0, interpolate(1, -1, f64::NEG_INFINITY, 1, 0), 0.0);
        assert_eq!(0, interpolate(-1, -1, 0, 1, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(0, -1, 0, 1, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(1, -1, 0, 1, f64::INFINITY), 0.0);
        assert_eq!(0, interpolate(-1, -1, 0, 1, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(0, -1, 0, 1, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(1, -1, 0, 1, f64::NEG_INFINITY), 0.0);
        assert_that(interpolate(next_up(1e30), 1e30, 1e30, 1e300, 1e300)).is_close_to(&Math::next_up(1e30), &Percentage::with_percentage(1e-6));
        assert_that(interpolate(&Math::next_up(1e30), 1e300, 1e300, 1e30, 1e30)).is_close_to(&Math::next_up(1e30), &Percentage::with_percentage(1e-6));
        assert_that(interpolate(&Math::next_down(-1e30), -1e30, -1e30, -1e300, -1e300)).is_close_to(&Math::next_down(-1e30), &Percentage::with_percentage(1e-6));
        assert_that(interpolate(&Math::next_down(-1e30), -1e300, -1e300, -1e30, -1e30)).is_close_to(&Math::next_down(-1e30), &Percentage::with_percentage(1e-6));
    }

    #[test]
    fn test_interpolate_symmetry(&self) {
         let random: Random = Random::new(0);
         let num_test_cycles: i32 = 1000;
        {
             let mut i: i32 = 0;
            while i < num_test_cycles {
               {
                     let x_vals: vec![Vec<f64>; 3] = vec![random.next_double(), random.next_double(), random.next_double(), ]
                    ;
                    Arrays::sort(&x_vals);
                     let x1: f64 = x_vals[0];
                     let x: f64 = x_vals[1];
                     let x2: f64 = x_vals[2];
                     let y1: f64 = random.next_double();
                     let y2: f64 = random.next_double();
                     let interpolated_value1: f64 = Algorithms::interpolate(x, x1, y1, x2, y2);
                     let interpolated_value2: f64 = Algorithms::interpolate(x, x2, y2, x1, y1);
                    assert_eq!(0, &Double::compare(interpolated_value1, interpolated_value2));
                }
                i += 1;
             }
         }

    }

    #[test]
    fn test_interpolate_monotonicity(&self) {
         let random: Random = Random::new(0);
         let num_test_cycles: i32 = 1000;
        {
             let mut i: i32 = 0;
            while i < num_test_cycles {
               {
                     let x_vals: vec![Vec<f64>; 3] = vec![random.next_double(), random.next_double(), random.next_double(), ]
                    ;
                    Arrays::sort(&x_vals);
                     let x1: f64 = x_vals[0];
                     let x: f64 = x_vals[1];
                     let x2: f64 = x_vals[2];
                     let y1: f64 = random.next_double();
                     let y2: f64 = random.next_double();
                     let interpolated_value_left: f64 = Algorithms::interpolate(&Math::next_down(x), x1, y1, x2, y2);
                     let interpolated_value_mid: f64 = Algorithms::interpolate(x, x1, y1, x2, y2);
                     let interpolated_value_right: f64 = Algorithms::interpolate(&Math::next_up(x), x1, y1, x2, y2);
                    if y1 <= y2 {
                        assert_true(interpolated_value_left <= interpolated_value_mid);
                        assert_true(interpolated_value_mid <= interpolated_value_right);
                    } else {
                        assert_true(interpolated_value_left >= interpolated_value_mid);
                        assert_true(interpolated_value_mid >= interpolated_value_right);
                    }
                }
                i += 1;
             }
         }

    }

    #[test]
    fn test_interpolate_na_n(&self) {
        assert_eq!(f64::NAN, interpolate(f64::NAN, 3, 4, 4, 5), 0.0);
        assert_eq!(f64::NAN, interpolate(3.5, f64::NAN, 4, 4, 5), 0.0);
        assert_eq!(f64::NAN, interpolate(3.5, 3, 4, f64::NAN, 5), 0.0);
        assert_eq!(f64::NAN, interpolate(3.5, f64::NEG_INFINITY, 4, f64::INFINITY, 5), 0.0);
        assert_eq!(f64::NAN, interpolate(3.5, 2, f64::NEG_INFINITY, 4, f64::INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(3.5, f64::NEG_INFINITY, 3, f64::INFINITY, 4), 0.0);
        assert_eq!(3, interpolate(3, f64::NEG_INFINITY, 3, f64::INFINITY, 3), 0.0);
        assert_eq!(f64::NAN, interpolate(0, f64::NEG_INFINITY, 0, f64::INFINITY, 1), 0.0);
        assert_eq!(f64::NAN, interpolate(0, f64::NEG_INFINITY, 0, f64::INFINITY, -1), 0.0);
        assert_eq!(f64::NAN, interpolate(0, f64::NEG_INFINITY, 1, f64::INFINITY, 0), 0.0);
        assert_eq!(f64::NAN, interpolate(0, f64::NEG_INFINITY, -1, f64::INFINITY, 0), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(0, 1, f64::NEG_INFINITY, 2, f64::INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(1, 1, f64::NEG_INFINITY, 2, f64::INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(1.5, 1, f64::NEG_INFINITY, 2, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(2, 1, f64::NEG_INFINITY, 2, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(3, 1, f64::NEG_INFINITY, 2, f64::INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(0, 1, f64::INFINITY, 2, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::INFINITY, interpolate(1, 1, f64::INFINITY, 2, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NAN, interpolate(1.5, 1, f64::INFINITY, 2, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(2, 1, f64::INFINITY, 2, f64::NEG_INFINITY), 0.0);
        assert_eq!(f64::NEG_INFINITY, interpolate(3, 1, f64::INFINITY, 2, f64::NEG_INFINITY), 0.0);
    }

    #[test]
    fn test_calculate_midpoint(&self) {
        assert_eq!(-1, &Algorithms::calculate_midpoint(i64::MIN, i64::MAX));
        assert_eq!(-1, &Algorithms::calculate_midpoint(i64::MIN, i64::MAX - 1));
        assert_eq!(-2, &Algorithms::calculate_midpoint(i64::MIN, i64::MAX - 2));
        assert_eq!(-2, &Algorithms::calculate_midpoint(i64::MIN, i64::MAX - 3));
        assert_eq!(0, &Algorithms::calculate_midpoint(i64::MIN + 1, i64::MAX));
        assert_eq!(0, &Algorithms::calculate_midpoint(i64::MIN + 2, i64::MAX));
        assert_eq!(i64::MAX, &Algorithms::calculate_midpoint(i64::MAX, i64::MAX));
        assert_eq!(i64::MAX - 1, &Algorithms::calculate_midpoint(i64::MAX - 1, i64::MAX));
        assert_eq!(i64::MAX - 1, &Algorithms::calculate_midpoint(i64::MAX - 2, i64::MAX));
        assert_eq!(i64::MIN, &Algorithms::calculate_midpoint(i64::MIN, i64::MIN));
        assert_eq!(i64::MIN, &Algorithms::calculate_midpoint(i64::MIN + 1, i64::MIN));
        assert_eq!(i64::MIN + 1, &Algorithms::calculate_midpoint(i64::MIN + 2, i64::MIN));
        assert_eq!(i64::MIN + 1, &Algorithms::calculate_midpoint(i64::MIN + 3, i64::MIN));
        assert_eq!(i64::MIN / 2, &Algorithms::calculate_midpoint(0, i64::MIN));
        assert_eq!(i64::MAX / 2, &Algorithms::calculate_midpoint(0, i64::MAX));
        assert_eq!(-5, &Algorithms::calculate_midpoint(-4, -6));
        assert_eq!(-6, &Algorithms::calculate_midpoint(-4, -7));
        assert_eq!(-6, &Algorithms::calculate_midpoint(-5, -7));
        assert_eq!(-6, &Algorithms::calculate_midpoint(-4, -8));
        assert_eq!(5, &Algorithms::calculate_midpoint(4, 6));
        assert_eq!(5, &Algorithms::calculate_midpoint(4, 7));
        assert_eq!(6, &Algorithms::calculate_midpoint(5, 7));
        assert_eq!(6, &Algorithms::calculate_midpoint(4, 8));
        assert_eq!(0, &Algorithms::calculate_midpoint(-2, 3));
        assert_eq!(1, &Algorithms::calculate_midpoint(-1, 4));
        assert_eq!(-1, &Algorithms::calculate_midpoint(-3, 2));
    }

    #[test]
    fn test_map_double_to_long(&self) {
        assert_eq!(0, &Algorithms::map_double_to_long(0));
        assert_eq!(1, &Algorithms::map_double_to_long(f64::MIN));
        assert_eq!(0x7fefffffffffffff, &Algorithms::map_double_to_long(f64::MAX));
        assert_eq!(0x7ff0000000000000, &Algorithms::map_double_to_long(f64::INFINITY));
        assert_eq!(-1, &Algorithms::map_double_to_long(-0.0));
        assert_eq!(-2, &Algorithms::map_double_to_long(-f64::MIN));
        assert_eq!(0x8010000000000000, &Algorithms::map_double_to_long(-f64::MAX));
        assert_eq!(0x800fffffffffffff, &Algorithms::map_double_to_long(f64::NEG_INFINITY));
        assert_eq!(0x7ff8000000000000, &Algorithms::map_double_to_long(f64::NAN));
    }

    #[test]
    fn test_long_to_double(&self) {
        assert_eq!(0, &Algorithms::map_long_to_double(0), 0.0);
        assert_eq!(f64::MIN, &Algorithms::map_long_to_double(1), 0.0);
        assert_eq!(f64::MAX, &Algorithms::map_long_to_double(0x7fefffffffffffff), 0.0);
        assert_eq!(f64::INFINITY, &Algorithms::map_long_to_double(0x7ff0000000000000), 0.0);
        assert_eq!(-0.0, &Algorithms::map_long_to_double(-1), 0.0);
        assert_eq!(-f64::MIN, &Algorithms::map_long_to_double(-2), 0.0);
        assert_eq!(-f64::MAX, &Algorithms::map_long_to_double(-1 - 0x7fefffffffffffff), 0.0);
        assert_eq!(f64::NEG_INFINITY, &Algorithms::map_long_to_double(-1 - 0x7ff0000000000000), 0.0);
        assert_eq!(f64::NAN, &Algorithms::map_long_to_double(0x7ff8000000000000), 0.0);
        assert_eq!(f64::NAN, &Algorithms::map_long_to_double(i64::MAX), 0.0);
        assert_eq!(f64::NAN, &Algorithms::map_long_to_double(0x7ff0000000000000 + 1), 0.0);
        assert_eq!(f64::NAN, &Algorithms::map_long_to_double(0x800fffffffffffff - 1), 0.0);
        assert_eq!(f64::NAN, &Algorithms::map_long_to_double(-2 - 0x7ff0000000000000), 0.0);
        assert_eq!(f64::NAN, &Algorithms::map_long_to_double(i64::MIN), 0.0);
    }

    #[test]
    fn test_double_mapping(&self) {
         let cycles: i32 = 100000;
         let rnd: Random = Random::new(0);
        {
             let mut i: i32 = 0;
            while i < cycles {
               {
                     let d: f64 = rnd.next_double() * 2 - 1;
                    assert_eq!(d, &Algorithms::map_long_to_double(&Algorithms::map_double_to_long(d)), 0.0);
                }
                i += 1;
             }
         }

    }

    #[test]
    fn test_find_first(&self) {
         let max_num_evaluations: i32 = 65;
        self.test_find_first(3, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(i64::MAX, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(i64::MAX - 1, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(i64::MIN, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(i64::MIN + 1, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(0, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(0, -1, i64::MAX, max_num_evaluations);
        self.test_find_first(0, i64::MIN, 1, max_num_evaluations);
        self.test_find_first(0, -1, 1, max_num_evaluations);
        self.test_find_first(0, -1, 0, max_num_evaluations);
        self.test_find_first(0, 0, 1, max_num_evaluations);
        self.test_find_first(0, 0, 0, max_num_evaluations);
        self.test_find_first(1, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(10, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(100, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(i64::MAX - 2, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(i64::MAX - 1, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(i64::MAX, i64::MIN, i64::MAX, max_num_evaluations);
        self.test_find_first(0, 0, i64::MAX, max_num_evaluations);
        assert_throws(DynaHist::IllegalArgumentError.class, () -> Algorithms::find_first( l: & -> false, 0, i64::MAX));
    // assertThrows(DynaHist::IllegalArgumentError.class, () -> Algorithms.findFirst(l -> false, 1, -1));
    // assertThrows(DynaHist::IllegalArgumentError.class, () -> Algorithms.findFirst(l -> false, -1, 1, 2));
    // assertThrows(DynaHist::IllegalArgumentError.class, () -> Algorithms.findFirst(l -> false, -1, 1, -2));
    }

    fn test_find_first_with_initial_guess( first_true_index: i64,  min: i64,  max: i64,  initial_guess: i64,  max_num_evaluations: i32) {
         let evaluated_values: Set<Long> = HashSet<>::new();
         let predicate: LongPredicate =  value: & -> {
            assert_true(&evaluated_values.add(value));
            return value >= first_true_index;
        };
        assert_eq!(first_true_index, &Algorithms::find_first_guess(&predicate, min, max, initial_guess));
        assert_that(&evaluated_values.size()).is_less_than_or_equal_to(max_num_evaluations);
    }

    fn test_find_first( first_true_index: i64,  min: i64,  max: i64,  max_num_evaluations: i32) {
         let evaluated_values: Set<Long> = HashSet<>::new();
         let predicate: LongPredicate =  value: & -> {
            assert_true(&evaluated_values.add(value));
            return value >= first_true_index;
        };
        assert_eq!(first_true_index, &Algorithms::find_first(&predicate, min, max));
        assert_that(&evaluated_values.size()).is_less_than_or_equal_to(max_num_evaluations);
    }

    #[test]
    fn test_find_first_with_initial_guess(&self) {
         let max_num_evaluations: i32 = 128;
        Self::test_find_first_with_initial_guess(i64::MAX, i64::MIN, i64::MAX, i64::MIN, max_num_evaluations);
        Self::test_find_first_with_initial_guess(i64::MIN, i64::MIN, i64::MAX, i64::MAX, max_num_evaluations);
        Self::test_find_first_with_initial_guess(i64::MAX, i64::MIN, i64::MAX, i64::MIN + 1, max_num_evaluations);
        Self::test_find_first_with_initial_guess(i64::MIN, i64::MIN, i64::MAX, i64::MAX - 1, max_num_evaluations);
        Self::test_find_first_with_initial_guess(i64::MAX, i64::MIN, i64::MAX, i64::MAX, max_num_evaluations);
        Self::test_find_first_with_initial_guess(i64::MIN, i64::MIN, i64::MAX, i64::MIN, max_num_evaluations);
        Self::test_find_first_with_initial_guess(i64::MAX, i64::MIN, i64::MAX, i64::MAX - 1, max_num_evaluations);
        Self::test_find_first_with_initial_guess(i64::MIN, i64::MIN, i64::MAX, i64::MIN + 1, max_num_evaluations);
        assert_throws(DynaHist::IllegalArgumentError.class, () -> Algorithms::find_first_guess( l: & -> false, i64::MIN, i64::MAX, 0));
    }

    #[test]
    fn test_find_first_with_initial_guess2(&self) {
         let max_num_evaluations_with_initial_guess: i32 = 128;
         let max_num_evaluations: i32 = 65;
        {
             let mut j: i32 = 0;
            while j < 100 {
               {
                     let first_true_index: i64 = i64::MAX - j;
                    {
                         let mut i: i32 = 0;
                        while i < 100 {
                           {
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i64::MAX - i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, -1 - i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i64::MIN + i, max_num_evaluations_with_initial_guess);
                            }
                            i += 1;
                         }
                     }

                    self.test_find_first(first_true_index, i64::MIN, i64::MAX, max_num_evaluations);
                }
                j += 1;
             }
         }

        {
             let mut j: i32 = 0;
            while j < 100 {
               {
                     let first_true_index: i64 = i64::MIN + j;
                    {
                         let mut i: i32 = 0;
                        while i < 100 {
                           {
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i64::MAX - i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, -1 - i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i64::MIN + i, max_num_evaluations_with_initial_guess);
                            }
                            i += 1;
                         }
                     }

                    self.test_find_first(first_true_index, i64::MIN, i64::MAX, max_num_evaluations);
                }
                j += 1;
             }
         }

        {
             let mut j: i32 = 0;
            while j < 100 {
               {
                     let first_true_index: i64 = j;
                    {
                         let mut i: i32 = 0;
                        while i < 100 {
                           {
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i64::MAX - i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, -1 - i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i64::MIN + i, max_num_evaluations_with_initial_guess);
                            }
                            i += 1;
                         }
                     }

                    self.test_find_first(first_true_index, i64::MIN, i64::MAX, max_num_evaluations);
                }
                j += 1;
             }
         }

        {
             let mut j: i32 = 0;
            while j < 100 {
               {
                     let first_true_index: i64 = -j - 1;
                    {
                         let mut i: i32 = 0;
                        while i < 100 {
                           {
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i64::MAX - i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, -1 - i, max_num_evaluations_with_initial_guess);
                                Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MAX, i64::MIN + i, max_num_evaluations_with_initial_guess);
                            }
                            i += 1;
                         }
                     }

                    self.test_find_first(first_true_index, i64::MIN, i64::MAX, max_num_evaluations);
                }
                j += 1;
             }
         }

        {
             let mut j: i32 = 0;
            while j < 20 {
               {
                    {
                         let mut i: i32 = 0;
                        while i <= j {
                           {
                                 let first_true_index: i64 = i64::MAX - i;
                                {
                                     let mut k: i32 = 0;
                                    while k <= j {
                                       {
                                            Self::test_find_first_with_initial_guess(first_true_index, i64::MAX - j, i64::MAX, i64::MAX - k, max_num_evaluations_with_initial_guess);
                                        }
                                        k += 1;
                                     }
                                 }

                            }
                            i += 1;
                         }
                     }

                }
                j += 1;
             }
         }

        {
             let mut j: i32 = 0;
            while j < 20 {
               {
                    {
                         let mut i: i32 = 0;
                        while i <= j {
                           {
                                 let first_true_index: i64 = i64::MIN + i;
                                {
                                     let mut k: i32 = 0;
                                    while k <= j {
                                       {
                                            Self::test_find_first_with_initial_guess(first_true_index, i64::MIN, i64::MIN + j, i64::MIN + k, max_num_evaluations_with_initial_guess);
                                        }
                                        k += 1;
                                     }
                                 }

                            }
                            i += 1;
                         }
                     }

                }
                j += 1;
             }
         }

    }

    #[test]
    fn test_find_first_with_initial_guess3(&self) {
        Self::test_find_first_with_initial_guess(1, i64::MIN, i64::MAX, 0, 2);
        Self::test_find_first_with_initial_guess(134325, i64::MIN, i64::MAX, 134324, 2);
        Self::test_find_first_with_initial_guess(0, i64::MIN, i64::MAX, 1, 4);
        Self::test_find_first_with_initial_guess(134324, i64::MIN, i64::MAX, 134325, 4);
        Self::test_find_first_with_initial_guess(2, i64::MIN, i64::MAX, 0, 4);
        Self::test_find_first_with_initial_guess(3, i64::MIN, i64::MAX, 0, 4);
    }

    #[test]
    fn test_interpolate_random(&self) {
         let random: SplittableRandom = SplittableRandom::new(0);
         let num_iterations: i32 = 1000;
         let a: i64 = 10;
         let maxb: i64 = 100;
         let c: i64 = 10;
        {
             let mut i: i32 = 0;
            while i < num_iterations {
               {
                     let b: i64 = random.next_long(maxb);
                     let l: i64 = random.next_long(Algorithms::NEGATIVE_INFINITY_MAPPED_TO_LONG + 1 + a, Algorithms::POSITIVE_INFINITY_MAPPED_TO_LONG - b - c);
                     let mut x1: f64 = Algorithms::map_long_to_double(l);
                     let mut x2: f64 = Algorithms::map_long_to_double(l + b);
                    if random.next_boolean() {
                         let t: f64 = x1;
                        x1 = x2;
                        x2 = t;
                    }
                     let y1: f64 = random.next_double(-1, 1);
                     let y2: f64 = random.next_double(-1, 1);
                     let previous_y: f64 = f64::NAN;
                    {
                         let mut j: i64 = 0;
                        while j < a + b + c {
                           {
                                 let x: f64 = Algorithms::map_long_to_double(l + j - a);
                                 let y: f64 = Algorithms::interpolate(x, x1, y1, x2, y2);
                                assert_that(y).is_between(&std::cmp::min(y1, y2), &std::cmp::max(y1, y2));
                                if !previous_y.is_nan() {
                                    if (y1 <= y2 && x1 <= x2) || (y1 >= y2 && x1 >= x2) {
                                        assert_that(y).is_greater_than_or_equal_to(previous_y);
                                    }
                                    if (y1 <= y2 && x1 >= x2) || (y1 >= y2 && x1 <= x2) {
                                        assert_that(y).is_less_than_or_equal_to(previous_y);
                                    }
                                }
                                previous_y = y;
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

    }

    #[test]
    fn test_interpolate_equal_x(&self) {
         let random: SplittableRandom = SplittableRandom::new(0);
         let num_iterations: i32 = 1000;
        {
             let mut i: i32 = 0;
            while i < num_iterations {
               {
                     let x: f64 = random.next_double(-10, 10);
                     let y1: f64 = random.next_double(-10, 10);
                     let y2: f64 = random.next_double(-10, 10);
                     let expected_y: f64 = 0.5 * y1 + 0.5 * y2;
                    assert_eq!(expected_y, interpolate(&Math::next_down(x), x, y1, x, y2), 0.0);
                    assert_eq!(expected_y, interpolate(x, x, y1, x, y2), 0.0);
                    assert_eq!(expected_y, interpolate(&Math::next_up(x), x, y1, x, y2), 0.0);
                }
                i += 1;
             }
         }

    }

    #[test]
    fn test_interpolate_negative_zero(&self) {
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(-0.0 * 5.0));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(-0.0, -1.0, -0.0, -0.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(-0.0, -0.0, -0.0, -0.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(f64::INFINITY, -0.0, -0.0, -0.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(f64::NEG_INFINITY, -0.0, -0.0, -0.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(f64::NEG_INFINITY, -10.0, -0.0, 10.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(-100.0, -10.0, -0.0, 10.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(-10.0, -10.0, -0.0, 10.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(-1.0, -10.0, -0.0, 10.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(0.0, -10.0, -0.0, 10.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(1.0, -10.0, -0.0, 10.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(10.0, -10.0, -0.0, 10.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(100.0, -10.0, -0.0, 10.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(f64::INFINITY, -10.0, -0.0, 10.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(f64::NEG_INFINITY, -10.0, 0.0, 10.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(-100.0, -10.0, 0.0, 10.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(-10.0, -10.0, 0.0, 10.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(-1.0, -10.0, 0.0, 10.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(0.0, -10.0, 0.0, 10.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(1.0, -10.0, 0.0, 10.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(10.0, -10.0, 0.0, 10.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(100.0, -10.0, 0.0, 10.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(-0.0), &Algorithms::map_double_to_long(interpolate(f64::INFINITY, -10.0, 0.0, 10.0, -0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(5.0, 5.0, -0.0, 5.0, 0.0)));
        assert_eq!(&Algorithms::map_double_to_long(0.0), &Algorithms::map_double_to_long(interpolate(5.0, 5.0, 0.0, 5.0, -0.0)));
    }

    #[test]
    fn test_clip(&self) {
        assert_eq!(-3, &Algorithms::clip(1, -3, -3));
        assert_eq!(3, &Algorithms::clip(1, 3, 7));
        assert_eq!(5, &Algorithms::clip(5, 3, 7));
        assert_eq!(7, &Algorithms::clip(8, 3, 7));
        assert_throws(DynaHist::IllegalArgumentError.class, () -> Algorithms::clip(10, 5, 4));
    }
}
