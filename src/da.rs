
extern crate ndarray_linalg;

use self::ndarray_linalg::SquareMatrix;
use ensemble::*;

pub fn forcast(teo: &Fn(V) -> V, xs: Ensemble) -> Ensemble {
    xs.into_iter().map(teo).collect()
}

pub fn enkf(xs: Ensemble, y: &V, h: &M, r: &M) -> Ensemble {
    let (_, p) = stat2(&xs);
    let v = h.dot(&p).dot(&h.t()) + r;
    let vinv = v.inv().unwrap();
    let k = p.dot(&h.t()).dot(&vinv);
    xs.into_iter()
        .map(|x| {
            let err = y - &h.dot(&x);
            x + k.dot(&err)
        })
        .collect()
}
