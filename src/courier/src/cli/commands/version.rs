use clap::Args;

use crate::cli::commands::CommandExecutor;

#[derive(Args)]
pub struct VersionArgs {}

impl CommandExecutor for VersionArgs {
  fn execute(&self) {
    println!("{} {} (build {}, branch {})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("COMMIT_ID"), env!("BRANCH"),);
  }
}
