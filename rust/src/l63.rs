
use ndarray_odeint::*;
use ensemble::V;

pub fn teo(dt: f64, step: usize, mut x: V) -> V {
    let l = |y| lorenz63(10., 28., 8.0 / 3.0, y);
    let u = |y| rk4(&l, dt, y);
    for _ in 0..step {
        x = u(x);
    }
    x
}
