use ndarray::Array2;

pub struct Grid2D {
    pub nx: usize,
    pub ny: usize,
    pub rho:  Array2<f64>,
    pub rhou: Array2<f64>,
    pub rhov: Array2<f64>,
    pub e:    Array2<f64>,
}

impl Grid2D {
    pub fn new(nx: usize, ny: usize) -> Self {
        let zeros = Array2::<f64>::zeros((ny, nx)); // (rows, cols) = (ny, nx)
        Self {
            nx, ny,
            rho:  zeros.clone(),
            rhou: zeros.clone(),
            rhov: zeros.clone(),
            e:    zeros,
        }
    }
}
