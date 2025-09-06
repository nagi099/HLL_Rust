use crate::{config::Config, grid::Grid2D, types::{Result, SolverError}};

pub struct Solver {
    pub grid: Grid2D,
    pub cfg: Config,
    step: usize,
}

impl Solver {
    pub fn new(nx: usize, ny: usize, cfg: Config) -> Result<Self> {
        if nx == 0 || ny == 0 {
            return Err(SolverError::Config("nx, ny must be > 0".into()));
        }
        if cfg.gamma <= 1.0 {
            return Err(SolverError::Config("gamma must be > 1".into()));
        }
        Ok(Self { grid: Grid2D::new(nx, ny), cfg, step: 0 })
    }

    pub fn step_index(&self) -> usize { self.step }

    pub fn compute_dt(&self) -> Result<f64> {
        // とりあえずの仮 dt（あとでCFL計算に差し替え）
        Ok(self.cfg.cfl.min(1.0) * 0.1)
    }

    pub fn step(&mut self, _dt: f64) -> Result<()> {
        // ここに境界→流束→更新の本実装を入れる
        self.step += 1;
        Ok(())
    }

    pub fn init_sod_x(&mut self) {
        let nx = self.grid.nx;
        let ny = self.grid.ny;
        let mid = nx / 2;
        for j in 0..ny {
            for i in 0..nx {
                let left = i < mid;
                let (rho, u, v, p) = if left { (1.0, 0.0, 0.0, 1.0) } else { (0.125, 0.0, 0.0, 0.1) };
                let e = p/(self.cfg.gamma - 1.0) + 0.5 * rho * (u*u + v*v);
                self.grid.rho[(j,i)]  = rho;
                self.grid.rhou[(j,i)] = rho * u;
                self.grid.rhov[(j,i)] = rho * v;
                self.grid.e[(j,i)]    = e;
            }
        }
    }
}
