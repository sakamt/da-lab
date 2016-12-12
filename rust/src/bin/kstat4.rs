
extern crate ndarray;
extern crate aics_da;

use ndarray::prelude::*;
use aics_da::*;

fn main() {
    let x = arr1(&[1.0]);
    println!("k,k2,k3,k4");
    for n in 5..25 {
        let k = (2 as usize).pow(n);
        let xs = ensemble::replica(&x, 1.0, k);
        let (k2, k3, k4) = ensemble::kstat4(&xs);
        println!("{},{:.05e},{:.05e},{:.05e}", k, k2[0], k3[0], k4[0]);
    }
}
