use crate::types::Floors;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub gamma: f64,
    pub cfl: f64,
    pub dx: f64,
    pub dy: f64,
    pub floors: Floors,
}