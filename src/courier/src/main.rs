#![cfg(not(doc))]

use clap::Parser;

use crate::cli::commands::{CommandExecutor, Commands};

mod cli;

fn main() {
  match cli::App::parse().command {
    Commands::Version(handler) => handler.execute(),
    Commands::New(handler) => handler.execute(),
  }
}
