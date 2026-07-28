use crate::args::{Args, Command, Frontend};

pub async fn run(args: Args) {
    let command = args.command.unwrap_or(Command::App {
        frontend: Frontend::Tui,
    });

    println!("Initialisation code");

    match command {
        Command::Daemon => println!("daemon"),
        Command::App { frontend } => {
            println!("joint frontend stuff");
            
            match frontend {
                Frontend::Tui => println!("Tui"),
                Frontend::Cli => println!("Cli"),
            }
        }
    }
}
