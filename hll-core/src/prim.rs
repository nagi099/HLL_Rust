use crate::types::{Cons, Prim};

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