use std::{
  sync::{
    Arc, Mutex, OnceLock,
    atomic::{AtomicBool, Ordering},
    mpsc::{self, RecvTimeoutError, SyncSender},
  },
  thread::{self, JoinHandle},
  time::Duration,
};

use crate::{level::Level, loggable::Loggable, record::Record, transport::Transport};

pub struct Logger {
  sender: SyncSender<Record>,
  handle: JoinHandle<()>,
  is_finished: Arc<AtomicBool>,
}

static INSTANCE: OnceLock<Mutex<Logger>> = OnceLock::new();

impl Logger {
  pub fn init(transports: Vec<Box<dyn Transport>>) -> &'static Mutex<Self> {
    const CAPACITY: usize = 1000;
    let (sender, receiver) = mpsc::sync_channel::<Record>(CAPACITY);
    let state = Arc::new(AtomicBool::new(false));

    let running_state = state.clone();

    let handle = thread::Builder::new()
      .name("logger".into())
      .spawn(move || {
        let transports = transports;

        let state = running_state;

        while !state.load(Ordering::SeqCst) {
          match receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(msg) => {
              for transport in transports.iter() {
                transport.writeln(&msg);
              }
            },
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => break,
          }
        }

        if let Ok(msg) = receiver.try_recv() {
          for transport in transports.iter() {
            transport.writeln(&msg);
          }
        }
      })
      .expect("Failed to start logger thread");

    let logger = Logger { sender, handle, is_finished: state.clone() };

    INSTANCE.get_or_init(|| Mutex::new(logger))
  }

  pub fn get_instance() -> &'static Mutex<Self> {
    INSTANCE.get().expect("Logger not initialized")
  }

  fn log(&self, record: Record) -> bool {
    match self.sender.try_send(record) {
      Ok(_) => true,
      Err(_) => false,
    }
  }

  pub fn info(&self, message: impl Loggable, namespace: impl Loggable) -> bool {
    self.log(Record::new(Level::INFO, message, namespace))
  }

  pub fn warn(&self, message: impl Loggable, namespace: impl Loggable) -> bool {
    self.log(Record::new(Level::WARN, message, namespace))
  }

  pub fn error(&self, message: impl Loggable, namespace: impl Loggable) -> bool {
    self.log(Record::new(Level::ERROR, message, namespace))
  }

  pub fn debug(&self, message: impl Loggable, namespace: impl Loggable) -> bool {
    self.log(Record::new(Level::DEBUG, message, namespace))
  }

  pub fn trace(&self, message: impl Loggable, namespace: impl Loggable) -> bool {
    self.log(Record::new(Level::TRACE, message, namespace))
  }

  pub fn fatal(&self, message: impl Loggable, namespace: impl Loggable) -> bool {
    self.log(Record::new(Level::FATAL, message, namespace))
  }

  pub fn shutdown(self) {
    self.is_finished.store(true, Ordering::SeqCst);

    self.handle.join().expect("Failed to join logger thread");
  }
}
