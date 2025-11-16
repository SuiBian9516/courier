use std::sync::Mutex;

use logger::{Logger, transports::console_transport::ConsoleTransport};
use macros::{Deserialize, Serialize};
pub struct Server {
  logger: &'static Mutex<Logger>,
  config: ServerConfig,
}

impl Server {
  pub fn new() -> Self {
    Self { logger: Logger::init(vec![Box::new(ConsoleTransport::new(logger::level::Level::INFO))]), config: ServerConfig { port: 8080 } }
  }

  pub fn start(&self) {}

  pub fn get_logger(&self) -> &'static Mutex<Logger> {
    self.logger
  }
}

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
  #[config(default = 8080)]
  port: usize,
}
