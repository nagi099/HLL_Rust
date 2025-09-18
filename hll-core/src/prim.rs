use crate::types::{Cons, Prim, Floors, Result, SolverError};

pub fn cons_to_prim_unchecked(cons: Cons, gamma: f64) -> Prim {
    let mut prim = Prim::default();

    prim.u = cons.rhou / cons.rho;
    prim.v = cons.rhov / cons.rho;
    prim.p = (gamma - 1.0) * (cons.e - cons.rho * (prim.u * prim.u + prim.v * prim.v) / 2.0);
    prim.a = (gamma * prim.p / cons.rho).sqrt();

    prim
}

pub fn prim_to_cons_unchecked(prim: Prim, gamma: f64) -> Cons {
    let mut cons = Cons::default();

    cons.rho = gamma * prim.p / (prim.a * prim.a);
    cons.rhou = cons.rho * prim.u;
    cons.rhov = cons.rho * prim.v;
    cons.e = prim.p / (gamma - 1.0) + cons.rho * (prim.u * prim.u + prim.v * prim.v) / 2.0;
    
    cons
}

pub fn cons_to_prim_checked(cons: Cons, gamma: f64, floors: Floors, i: usize, j: usize) -> Result<Prim> {
    let mut prim = Prim::default();

    if cons.rho < 0.0 {
        return Err(SolverError::NonPhysical { i: i, j: j, why: "rho <= 0".to_string() });
    }

    prim.u = cons.rhou / cons.rho;
    prim.v = cons.rhov / cons.rho;
    prim.p = (gamma - 1.0) * (cons.e - cons.rho * (prim.u * prim.u + prim.v * prim.v) / 2.0);
    prim.a = (gamma * prim.p / cons.rho).sqrt();

    if prim.p < floors.p {
        prim.p = floors.p;
    }

    if prim.u.is_infinite() || prim.v.is_infinite() || prim.p.is_infinite() || prim.a.is_infinite() {
        return Err(SolverError::NonPhysical { i: i, j: j, why: "NaN/Inf".to_string() });
    }

    Ok(prim)
}

pub fn prim_to_cons_checked(prim: Prim, gamma: f64, floors: Floors, i: usize, j: usize) -> Result<Cons> {
    let mut cons = Cons::default();

    if prim.p < 0.0 {
        return Err(SolverError::NonPhysical { i: i, j: j, why: "p <= 0".to_string() });
    }

    cons.rho = gamma * prim.p / (prim.a * prim.a);
    cons.rhou = cons.rho * prim.u;
    cons.rhov = cons.rho * prim.v;
    cons.e = prim.p / (gamma - 1.0) + cons.rho * (prim.u * prim.u + prim.v * prim.v) / 2.0;

    if cons.rho < floors.rho {
        cons.rho = floors.rho;
    }

    if cons.rho.is_infinite() || cons.rhou.is_infinite() || cons.rhov.is_infinite() || cons.e.is_infinite() {
        return Err(SolverError::NonPhysical { i: i, j: j, why: "NaN/Inf".to_string() });
    }
    
    Ok(cons)
}