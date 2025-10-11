use crate::types::{Cons, Prim, Floors, result, solverError};

pub fn flux_x_hll(ul: Cons, ur: Cons, gamma: f64, diag: bool) -> Cons {
    let prim_l = cons_to_prim_unchecked(ul, gamma);
    let prim_r = cons_to_prim_unchecked(ur, gamma);

    let a_l = prim_l.a;
    let a_r = prim_r.a;

    let s_l = (prim_l.u - a_l).min(prim_r.u - a_r);
    let s_r = (prim_l.u + a_l).max(prim_r.u + a_r);

    if s_l >= 0.0 {
        // Flux from left state
        return flux_x(prim_l, ul, gamma);
    } else if s_r <= 0.0 {
        // Flux from right state
        return flux_x(prim_r, ur, gamma);
    }

    let denom = s_r - s_l;
    if denom.abs() < 1e-12 {
        return Cons::default(); // Avoid division by zero
    }

    let flux_l = flux_x(prim_l, ul, gamma);
    let flux_r = flux_x(prim_r, ur, gamma);

    return Cons {
        rho: (s_r * flux_l.rho - s_l * flux_r.rho + s_l * s_r * (ur.rho - ul.rho)) / denom,
        rhou: (s_r * flux_l.rhou - s_l * flux_r.rhou + s_l * s_r * (ur.rhou - ul.rhou)) / denom,
        rhov: (s_r * flux_l.rhov - s_l * flux_r.rhov + s_l * s_r * (ur.rhov - ul.rhov)) / denom,
        e: (s_r * flux_l.e - s_l * flux_r.e + s_l * s_r * (ur.e - ul.e)) / denom,
    };
    
}

fn flux_x(prim: Prim, cons: Cons, gamma: f64) -> Cons {
    let rho = cons.rho;
    let u = prim.u;
    let v = prim.v;
    let p = prim.p;

    Cons {
        rho: rho * u,
        rhou: rho * u * u + p,
        rhov: rho * u * v,
        e: (cons.e + p) * u,
    }
}

pub fn flux_y_hll(ul: Cons, ur: Cons, gamma: f64, diag: bool) -> Cons {
    let prim_l = cons_to_prim_unchecked(ul, gamma);
    let prim_r = cons_to_prim_unchecked(ur, gamma);

    let a_l = prim_l.a;
    let a_r = prim_r.a;

    let s_l = (prim_l.v - a_l).min(prim_r.v - a_r);
    let s_r = (prim_l.v + a_l).max(prim_r.v + a_r);

    if s_l >= 0.0 {
        // Flux from left state
        return flux_y(prim_l, ul, gamma);
    } else if s_r <= 0.0 {
        // Flux from right state
        return flux_y(prim_r, ur, gamma);
    }

    let denom = s_r - s_l;
    if denom.abs() < 1e-12 {
        return Cons::default(); // Avoid division by zero
    }

    let flux_l = flux_y(prim_l, ul, gamma);
    let flux_r = flux_y(prim_r, ur, gamma);

    return Cons {
        rho: (s_r * flux_l.rho - s_l * flux_r.rho + s_l * s_r * (ur.rho - ul.rho)) / denom,
        rhou: (s_r * flux_l.rhou - s_l * flux_r.rhou + s_l * s_r * (ur.rhou - ul.rhou)) / denom,
        rhov: (s_r * flux_l.rhov - s_l * flux_r.rhov + s_l * s_r * (ur.rhov - ul.rhov)) / denom,
        e: (s_r * flux_l.e - s_l * flux_r.e + s_l * s_r * (ur.e - ul.e)) / denom,
    };
    
}

fn flux_y(prim: Prim, cons: Cons, gamma: f64) -> Cons {
    let rho = cons.rho;
    let u = prim.u;
    let v = prim.v;
    let p = prim.p;

    Cons {
        rho: rho * v,
        rhou: rho * u * v,
        rhov: rho * v * v + p,
        e: (cons.e + p) * v,
    }
}