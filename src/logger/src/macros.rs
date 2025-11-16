#[macro_export]
macro_rules! log {
  ($level:ident, $message:expr, $namespace:expr) => {
    $crate::Logger::get_instance().lock().unwrap().$level($message, $namespace);
  };
}

#[macro_export]
macro_rules! info {
  ($message:expr, $namespace:expr) => {
    $crate::log!(info, $message, $namespace);
  };
}

#[macro_export]
macro_rules! warn {
  ($message:expr, $namespace:expr) => {
    $crate::log!(warn, $message, $namespace);
  };
}

#[macro_export]
macro_rules! error {
  ($message:expr, $namespace:expr) => {
    $crate::log!(error, $message, $namespace);
  };
}

#[macro_export]
macro_rules! fatal {
  ($message:expr, $namespace:expr) => {
    $crate::log!(fatal, $message, $namespace);
  };
}

#[macro_export]
macro_rules! debug {
  ($message:expr, $namespace:expr) => {
    $crate::log!(debug, $message, $namespace);
  };
}

#[macro_export]
macro_rules! trace {
  ($message:expr, $namespace:expr) => {
    $crate::log!(trace, $message, $namespace);
  };
}
