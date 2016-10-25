
extern crate rand;
extern crate ndarray;
extern crate ndarray_linalg;
extern crate ndarray_rand;

use self::ndarray_linalg::SquareMatrix;
use self::ndarray::prelude::*;
use self::rand::distributions::*;
use self::ndarray_rand::RandomExt;
use ensemble::*;

pub fn forcast(teo: &Fn(V) -> V, xs: Ensemble) -> Ensemble {
    xs.into_iter().map(teo).collect()
}

pub fn random(n: usize) -> V {
    let dist = Normal::new(0., 1.0);
    Array::random(n, dist)
}

pub fn enkf(xs: Ensemble, y: &V, h: &M, r: &M) -> Ensemble {
    let (_, p) = stat2(&xs);
    let v = h.dot(&p).dot(&h.t()) + r;
    let vinv = v.inv().unwrap();
    let k = p.dot(&h.t()).dot(&vinv);
    let rs = r.clone().ssqrt().unwrap();
    let n = y.len();
    xs.into_iter()
        .map(|x| {
            let err = y - &h.dot(&x) + rs.dot(&random(n));
            x + k.dot(&err)
        })
        .collect()
}
