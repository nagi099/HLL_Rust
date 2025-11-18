use crate::grid::Grid2D;
use crate::types::{Cons, Prim};
use crate::prim::cons_to_prim_unchecked;

pub fn compute_dt(grid: Grid2D, gamma: f64, cfl: f64, dx: f64, dy: f64) -> f64 {
    let mut lambda_x_max: f64 = 0.0;
    let mut lambda_y_max: f64 = 0.0;

    for i in 0..grid.nx {
        for j in 0..grid.ny {
            let cons = Cons {
                rho: grid.rho[[i, j]], 
                rhou: grid.rhou[[i, j]], 
                rhov: grid.rhov[[i, j]], 
                e: grid.e[[i, j]], 
            };

            let prim: Prim = cons_to_prim_unchecked(cons, gamma);

            lambda_x_max = lambda_x_max.max(prim.u.abs() + prim.a);
            lambda_y_max = lambda_y_max.max(prim.v.abs() + prim.a);
        }
    }

    let dt = cfl / (lambda_x_max / dx).max(lambda_y_max / dy);

    dt
}