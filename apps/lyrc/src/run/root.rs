use crate::{
    args::{Args, Command, Frontend},
    config::Config,
    run::{daemon::run_daemon, gui::run_gui, tui::run_tui},
};

use mpris::client::MprisClient;

pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();

    let command = args.command.unwrap_or(Command::App {
        frontend: Frontend::Tui,
    });

    let player = &MprisClient::choose_player(&config.targets_in_priority_order).await?;

    match command {
        Command::Daemon => run_daemon(config).await,
        Command::App { frontend } => match frontend {
            Frontend::Tui => run_tui(player, config).await,
            Frontend::Gui => run_gui(player, config).await,
        },
    }
}
