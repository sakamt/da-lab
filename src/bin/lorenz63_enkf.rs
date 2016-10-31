
extern crate ndarray;
extern crate ndarray_odeint;
extern crate rustc_serialize;
extern crate aics_da;

use std::fs;
use ndarray::prelude::*;
use ndarray_odeint::*;
use aics_da::*;
use aics_da::ensemble::V;

#[derive(RustcDecodable)]
struct Setting {
    count: usize,
    k: usize,
    tau: usize,
    save_count: usize,
    dt: f64,
    r: f64,
}

fn teo(setting: &Setting, mut x: V) -> V {
    let dt = setting.dt;
    let step = setting.tau;
    let l = |y| lorenz63(10., 28., 8.0 / 3.0, y);
    let u = |y| rk4(&l, dt, y);
    for _ in 0..step {
        x = u(x);
    }
    x
}

fn main() {
    let setting: Setting = io::read_json("setting.json");
    fs::create_dir_all("data").unwrap();

    // observation settings
    let h = Array::<f64, _>::eye(3);
    let rs = setting.r.sqrt() * Array::<f64, _>::eye(3);

    let ts = TimeSeries {
        teo: |x| teo(&setting, x),
        state: arr1(&[1.0, 0.0, 0.0]),
    };
    let x_tl: Vec<V> = ts.skip(setting.count / 2).take(setting.count).collect();
    let y_tl: Vec<V> = x_tl.iter().map(|x| da::noise(&rs) + h.dot(x)).collect();
    let xs = ensemble::replica(&x_tl[0], setting.r.sqrt(), setting.k);

    let enkf = da::EnKF::new(h, rs, xs, |x| teo(&setting, x), y_tl.iter());

    println!("time,rmse");
    for (t, ((xs_b, xs_a), (x, y))) in enkf.zip(x_tl.iter().zip(y_tl.iter())).enumerate() {
        let time = (t * setting.tau) as f64 * setting.dt;
        let xm_a = ensemble::mean(&xs_a);
        let rmse = da::rmse(x, &xm_a);
        println!("{:.05},{:.05e}", time, rmse);
        if t % setting.save_count == 0 {
            let tt = t / setting.save_count;
            let xs_fname = format!("data/pre{:05}.msg", tt);
            io::save_as_msg(&xs_b, xs_fname);
            let xs_fname = format!("data/post{:05}.msg", tt);
            io::save_as_msg(&xs_a, xs_fname);
            let xs_fname = format!("data/truth{:05}.msg", tt);
            io::save_as_msg(&x, xs_fname);
            let xs_fname = format!("data/obs{:05}.msg", tt);
            io::save_as_msg(&y, xs_fname);
        }
    }
}
