use std::fmt::Display;
use std::fmt::Formatter;

use serde_derive::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ErrorKind {
  Cyphernode,
  Input,
  Internal,
}

impl Display for ErrorKind {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match self {
      ErrorKind::Input => write!(f, "Input"),
      ErrorKind::Internal => write!(f, "Internal"),
      ErrorKind::Cyphernode => write!(f, "Cyphernode"),
    }
  }
}

/// FFI Output
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CNCError {
  pub kind: String,
  pub message: String,
}

impl CNCError {
  pub fn new(kind: ErrorKind, message: &str) -> Self {
    CNCError {
      kind: kind.to_string(),
      message: message.to_string(),
    }
  }
}
