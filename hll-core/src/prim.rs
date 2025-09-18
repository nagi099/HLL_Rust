use crate::types::{Cons, Prim, Floors, Result, SolverError};

pub fn cons_to_prim_unchecked(cons: Cons, gamma: f64) -> Prim {
    let u = cons.rhou / cons.rho;
    let v = cons.rhov / cons.rho;
    let p = (gamma - 1.0) * (cons.e - cons.rho * (u * u + v * v) / 2.0);
    let a = (gamma * p / cons.rho).sqrt();

    Prim { 
        u: u, 
        v: v, 
        p: p, 
        a: a, 
    }
}

pub fn prim_to_cons_unchecked(prim: Prim, gamma: f64) -> Cons {
    let rho = gamma * prim.p / (prim.a * prim.a);
    let rhou = rho * prim.u;
    let rhov = rho * prim.v;
    let e = prim.p / (gamma - 1.0) + rho * (prim.u * prim.u + prim.v * prim.v) / 2.0;

    Cons { 
        rho: rho, 
        rhou: rhou, 
        rhov: rhov, 
        e: e, 
    }
}

pub fn cons_to_prim_checked(cons: Cons, gamma: f64, floors: Floors, i: usize, j: usize) -> Result<Prim> {
    if cons.rho < 0.0 {
        return Err(SolverError::NonPhysical { i: i, j: j, why: "rho <= 0".to_string() });
    }

    let u = cons.rhou / cons.rho;
    let v = cons.rhov / cons.rho;
    let mut p = (gamma - 1.0) * (cons.e - cons.rho * (u * u + v * v) / 2.0);
    let a = (gamma * p / cons.rho).sqrt();

    if p < floors.p {
        p = floors.p;
    }

    if u.is_infinite() || v.is_infinite() || p.is_infinite() || a.is_infinite() {
        return Err(SolverError::NonPhysical { i: i, j: j, why: "NaN/Inf".to_string() });
    }

    Ok(Prim {
        u: u,
        v: v,
        p: p,
        a: a,
    })
}

pub fn prim_to_cons_checked(prim: Prim, gamma: f64, floors: Floors, i: usize, j: usize) -> Result<Cons> {
    if prim.p < 0.0 {
        return Err(SolverError::NonPhysical { i: i, j: j, why: "p <= 0".to_string() });
    }

    let mut rho = gamma * prim.p / (prim.a * prim.a);
    let rhou = rho * prim.u;
    let rhov = rho * prim.v;
    let e = prim.p / (gamma - 1.0) + rho * (prim.u * prim.u + prim.v * prim.v) / 2.0;

    if rho < floors.rho {
        rho = floors.rho;
    }

    if rho.is_infinite() || rhou.is_infinite() || rhov.is_infinite() || e.is_infinite() {
        return Err(SolverError::NonPhysical { i: i, j: j, why: "NaN/Inf".to_string() });
    }

    Ok(Cons { 
        rho: rho, 
        rhou: rhou, 
        rhov: rhov, 
        e: e 
    })
}