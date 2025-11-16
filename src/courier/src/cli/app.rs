use clap::Parser;

use crate::cli::commands::Commands;

#[derive(Parser)]
#[command(
  author,
  version,
  about,
  long_about = None,
  help_template = r#"{before-help}{name} {version}
{about}

{usage-heading} {usage}

{all-args}{after-help}"#,
  disable_version_flag = true,
  disable_help_flag = true
)]
pub struct App {
  #[command(subcommand)]
  pub command: Commands,
}
