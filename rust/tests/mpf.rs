
extern crate aics_da;
extern crate ndarray;
extern crate ndarray_rand;
extern crate ndarray_numtest;

use aics_da::mpf::*;
use aics_da::weight::*;
use aics_da::ensemble::*;
use ndarray::prelude::*;
use ndarray_rand::*;
use ndarray_numtest::prelude::*;

#[test]
fn merge_resampling() {
    let n = 2;
    let k = 10000;
    let dist = RealNormal::<f64>::new(5.0, 1.0);
    let xs: Vec<_> = (0..k)
        .map(|_| Array::random(n, dist))
        .collect();

    let w = Weight::random(k);
    let (xm, pm) = w.stat2(&xs);

    let mr = MergeResampler::default();
    let xs_mr = mr.resampling(&w, &xs);
    let (xmm, pmm) = stat2(&xs_mr);
    println!("weighted mean  = \n{:?}", xm);
    println!("weighted covar = \n{:?}", pm);
    println!("m-resampled mean  = \n{:?}", xmm);
    println!("m-resampled covar = \n{:?}", pmm);
    xmm.assert_allclose_l2(&xm, 0.1);
    pmm.assert_allclose_l2(&pm, 0.1);
}
