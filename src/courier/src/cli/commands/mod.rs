mod new;
mod version;

pub use clap::Subcommand;

use crate::cli::commands::{new::NewArgs, version::VersionArgs};

#[derive(Subcommand)]
pub enum Commands {
  /// Get version
  Version(VersionArgs),

  /// Create a new server instance
  New(NewArgs),
}

pub trait CommandExecutor {
  fn execute(&self);
}
