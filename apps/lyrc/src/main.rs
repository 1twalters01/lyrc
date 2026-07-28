use clap::Parser;

mod args;
mod run;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    run::run(args).await;
}
