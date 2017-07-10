
#[macro_use]
extern crate ndarray_linalg;
extern crate aics_da;

use aics_da::mpf::*;
use aics_da::stat::*;
use aics_da::weight::*;

use ndarray_linalg::*;

#[test]
fn merge_resampling() {
    let n = 2;
    let k = 10000;
    let xs: Vec<_> = (0..k).map(|_| generate::random(n)).collect();

    let w = Weight::random(k);
    let (xm, pm) = w.stat2(&xs);

    let mr = MergeResampler::default();
    let xs_mr = mr.resampling(&w, &xs);
    let (xmm, pmm) = stat2(&xs_mr);
    println!("weighted mean  = \n{:?}", xm);
    println!("weighted covar = \n{:?}", pm);
    println!("m-resampled mean  = \n{:?}", xmm);
    println!("m-resampled covar = \n{:?}", pmm);
    assert_close_max!(&xmm, &xm, 0.1);
    assert_close_max!(&pmm, &pm, 0.1);
}
