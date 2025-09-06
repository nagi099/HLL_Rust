use thiserror::Error;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cons { pub rho: f64, pub rhou: f64, pub rhov: f64, pub e: f64 }

#[derive(Clone, Copy, Debug, Default)]
pub struct Prim { pub u: f64, pub v: f64, pub p: f64, pub a: f64 }

#[derive(Clone, Copy, Debug)]
pub struct Floors { pub rho: f64, pub p: f64 }

#[derive(Default, Clone, Debug)]
pub struct Diagnostics {
    pub floors_rho: u64,
    pub floors_p: u64,
    pub hll_to_llf_fallbacks: u64,
    pub downgraded_to_first_order: u64,
    pub nonfinite_encounters: u64,
}

#[derive(Error, Debug)]
pub enum SolverError {
    #[error("invalid config: {0}")]
    Config(String),
    #[error("non-physical state at ({i},{j}): {why}")]
    NonPhysical { i: usize, j: usize, why: String },
    #[error("non-finite encountered at {where_}")]
    NonFinite { where_: &'static str },
    #[error("time step error: {0}")]
    TimeStep(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SolverError>;
