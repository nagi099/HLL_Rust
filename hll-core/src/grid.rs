use ndarray::Array2;

pub struct Grid2D {
    pub nx: usize,
    pub ny: usize,
    pub nghost: usize,
    pub rho:  Array2<f64>,
    pub rhou: Array2<f64>,
    pub rhov: Array2<f64>,
    pub e:    Array2<f64>,
}

impl Grid2D {
    pub fn new(nx: usize, ny: usize, nghost: usize) -> Self {
        let zeros = Array2::<f64>::zeros((ny + nghost*2, nx + nghost*2)); 
        Self {
            nx, ny, 
            nghost: nghost,
            rho:  zeros.clone(),
            rhou: zeros.clone(),
            rhov: zeros.clone(),
            e:    zeros,
        }
    }

    pub fn idx_with_ghost(&self, i: usize, j: usize) -> (usize, usize) {
        (j + self.nghost, i + self.nghost)
    }
}
