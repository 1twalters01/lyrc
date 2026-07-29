use clap::Parser;

mod args;
mod run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();
    run::run(args).await
}
