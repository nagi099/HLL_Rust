use anyhow::Result;
use clap::Parser;
use hll_core::{Config, Floors, Solver};
use tracing::info;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = 256)] nx: usize,
    #[arg(long, default_value_t = 256)] ny: usize,
    #[arg(long, default_value_t = 5.0/3.0)] gamma: f64,
    #[arg(long, default_value_t = 0.5)] cfl: f64,
    #[arg(long, default_value_t = 1.0)] dx: f64,
    #[arg(long, default_value_t = 1.0)] dy: f64,
    #[arg(long, default_value_t = 0.2)] t_end: f64,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let args = Args::parse();

    let cfg = Config {
        gamma: args.gamma,
        cfl: args.cfl,
        dx: args.dx,
        dy: args.dy,
        floors: Floors { rho: 1e-12, p: 1e-12 },
    };

    let nghost = 1;

    let mut solver = Solver::new(args.nx, args.ny, nghost, cfg)?;
    solver.init_sod_x(); // or init_vortex()

    let mut t = 0.0;
    while t < args.t_end {
        let dt = solver.compute_dt()?;
        solver.step(dt)?;
        t += dt;
        if (solver.step_index() % 50) == 0 {
            info!("t={:.5}, step={}", t, solver.step_index());
            // solver.write_npy("out/...", /* fields */)?;
        }
    }
    Ok(())
}
