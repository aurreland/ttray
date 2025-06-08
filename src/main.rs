use std::fs::File;

use clap::Parser;
use log::info;
use system_tray::client::Client;

use crate::app::App;

pub mod app;
pub mod event;
pub mod ui;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Prints debug information to app.log file
    #[arg(short, long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    if args.debug {
        simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
            log::LevelFilter::Debug,
            simplelog::Config::default(),
            File::create("app.log")?,
        )])?;
    }

    let terminal = ratatui::init();

    info!("Starting SystemTray Client");
    let client = Client::new().await?;

    info!("Starting Application");
    let result = App::new(client).run(terminal).await;

    ratatui::restore();

    result
}
