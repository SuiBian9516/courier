use std::ffi::OsString;

use clap::Args;
use logger::info;
use server::Server;

use crate::cli::commands::CommandExecutor;

#[derive(Args)]
pub struct NewArgs {
  /// Server name
  name: String,

  /// Path to config file
  #[arg(long, short = 'c', value_name = "FILE")]
  config: Option<OsString>,
}

impl CommandExecutor for NewArgs {
  fn execute(&self) {
    let server = Server::new();
    server.start();
  }
}
