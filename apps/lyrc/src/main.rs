use clap::Parser;
use pyo3::PyResult;

mod args;
mod run;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args = args::Args::parse();
//     run::run(args).await
// }

#[pyo3_async_runtimes::tokio::main]
async fn main() -> PyResult<()> {
    let args = args::Args::parse();
    run::run(args)
        .await
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    Ok(())
}
