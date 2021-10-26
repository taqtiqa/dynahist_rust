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
// package com::dynatrace::dynahist::quantile;


 const NUM_P_VALUES: i32 = 10000;

 const P_VALUES: Vec<f64> = IntStream::range(0, NUM_P_VALUES + 1)::map_to_double( i: & -> i / NUM_P_VALUES as f64)::to_array();

 const QUANTILE_ESTIMATORS: Collection<QuantileEstimator> = Arrays::as_list(&SciPyQuantileEstimator::create(0.5, 0.5), &SciPyQuantileEstimator::create(0.0, 0.5), &SciPyQuantileEstimator::create(0.5, 0.0), &SciPyQuantileEstimator::create(0.4, 0.3), &SciPyQuantileEstimator::create(0.5, 1), &SciPyQuantileEstimator::create(1, 0.5), &SciPyQuantileEstimator::create(1, 0.0), &SciPyQuantileEstimator::create(0.0, 1.0), &SciPyQuantileEstimator::create(1.0, 0.0), &SciPyQuantileEstimator::create(1, 1.0), &SciPyQuantileEstimator::create());
pub struct SciPyQuantileEstimatorTest {
}

impl SciPyQuantileEstimatorTest {

    #[test]
    pub fn  test(&self)   {
         let values: vec![Vec<f64>; 11] = vec![6.0, 47.0, 49.0, 15.0, 42.0, 41.0, 7.0, 39.0, 43.0, 40.0, 36.0, ]
        ;
        Arrays::sort(&values);
         let quantile_estimator: QuantileEstimator = SciPyQuantileEstimator::create(0.4, 0.4);
        assert_equals(19.200000000000003, &quantile_estimator.estimate_quantile(0.25, &values), 0);
        assert_equals(40, &quantile_estimator.estimate_quantile(0.5, &values), 0);
        assert_equals(42.8, &quantile_estimator.estimate_quantile(0.75, &values), 0);
    }

    #[test]
    pub fn  test2(&self)   {
         let values: vec![Vec<f64>; 2] = vec![3.0, 5.0, ]
        ;
        Arrays::sort(&values);
         let quantile_estimator: QuantileEstimator = SciPyQuantileEstimator::create(0.4, 0.4);
        assert_equals(3, &quantile_estimator.estimate_quantile(0, &values), 0);
        assert_equals(3, &quantile_estimator.estimate_quantile(0.25, &values), 0);
        assert_equals(4, &quantile_estimator.estimate_quantile(0.5, &values), 0);
        assert_equals(5, &quantile_estimator.estimate_quantile(0.75, &values), 0);
        assert_equals(5, &quantile_estimator.estimate_quantile(1, &values), 0);
    }

    #[test]
    pub fn  test3(&self)   {
         let values: vec![Vec<f64>; 2] = vec![3.0, 5.0, ]
        ;
        Arrays::sort(&values);
         let quantile_estimator: QuantileEstimator = SciPyQuantileEstimator::create(0.5, 0.5);
        assert_equals(3, &quantile_estimator.estimate_quantile(0, &values), 0);
        assert_equals(3, &quantile_estimator.estimate_quantile(0.25, &values), 0);
        assert_equals(4, &quantile_estimator.estimate_quantile(0.5, &values), 0);
        assert_equals(5, &quantile_estimator.estimate_quantile(0.75, &values), 0);
        assert_equals(5, &quantile_estimator.estimate_quantile(1, &values), 0);
    }

    #[test]
    pub fn  test_single_value(&self)   {
         let value: f64 = 5;
         let values: vec![Vec<f64>; 1] = vec![value, ]
        ;
        for  let quantile_estimator: QuantileEstimator in QUANTILE_ESTIMATORS {
            for  let p: f64 in P_VALUES {
                assert_equals(value, &quantile_estimator.estimate_quantile(p, &values), 0);
            }
        }
    }

    #[test]
    pub fn  test_no_values(&self)   {
         let values;
        for  let quantile_estimator: QuantileEstimator in QUANTILE_ESTIMATORS {
            for  let p: f64 in P_VALUES {
                assert_equals(Double::NaN, &quantile_estimator.estimate_quantile(p, &values), 0);
            }
        }
    }

    #[test]
    pub fn  test_median(&self)   {
         let values: Vec<f64> = IntStream::range(0, 20)::map_to_double( i: & -> i)::to_array();
         let true_median: f64 = 9.5;
         let alphap_values: vec![Vec<f64>; 11] = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, ]
        ;
         let betap_values: vec![Vec<f64>; 11] = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, ]
        ;
        for  let alphap: f64 in alphap_values {
            for  let betap: f64 in betap_values {
                 let estimator: QuantileEstimator = SciPyQuantileEstimator::create(alphap, betap);
                assert_equals(true_median + (alphap - betap) * 0.5, &estimator.estimate_quantile(0.5, &values), 1e-10);
            }
        }
    }

    #[test]
    pub fn  test_sci_py_quantile_estimator_constructor(&self)   {
        assert_throws(IllegalArgumentException.class, () -> SciPyQuantileEstimator::create(-1, 1));
        assert_throws(IllegalArgumentException.class, () -> SciPyQuantileEstimator::create(2, 1));
        assert_throws(IllegalArgumentException.class, () -> SciPyQuantileEstimator::create(1, -1));
        assert_throws(IllegalArgumentException.class, () -> SciPyQuantileEstimator::create(1, 2));
    }

    #[test]
    pub fn  test_to_string(&self)   {
         let alphap: f64 = 0.5;
         let betap: f64 = 0.7;
        assert_equals("SciPyQuantileEstimator [alphap=0.5, betap=0.7]", &SciPyQuantileEstimator::create(alphap, betap)::to_string());
    }
}

