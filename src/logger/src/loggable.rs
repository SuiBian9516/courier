use bytes::{Bytes, BytesMut};
use std::collections::{LinkedList, VecDeque};

pub trait Loggable: Send + Sync {
  fn format(&self) -> Bytes;
}

impl Loggable for &'static str {
  fn format(&self) -> Bytes {
    Bytes::from_static(self.as_bytes())
  }
}

impl Loggable for String {
  fn format(&self) -> Bytes {
    Bytes::copy_from_slice(self.as_bytes())
  }
}

impl Loggable for Box<str> {
  fn format(&self) -> Bytes {
    Bytes::copy_from_slice(self.as_bytes())
  }
}

impl Loggable for std::borrow::Cow<'_, str> {
  fn format(&self) -> Bytes {
    Bytes::copy_from_slice(self.as_bytes())
  }
}

impl Loggable for char {
  fn format(&self) -> Bytes {
    let mut buf = [0u8; 4];
    let result = self.encode_utf8(&mut buf);
    Bytes::copy_from_slice(result.as_bytes())
  }
}

macro_rules! impl_signed_int {
    ($($t:ty),*) => {
        $(
            impl Loggable for $t {
                fn format(&self) -> Bytes {
                    Bytes::from(self.to_string())
                }
            }
        )*
    };
}

impl_signed_int!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_unsigned_int {
    ($($t:ty),*) => {
        $(
            impl Loggable for $t {
                fn format(&self) -> Bytes {
                    Bytes::from(self.to_string())
                }
            }
        )*
    };
}

impl_unsigned_int!(u8, u16, u32, u64, u128, usize);

impl Loggable for f32 {
  fn format(&self) -> Bytes {
    Bytes::from(self.to_string())
  }
}

impl Loggable for f64 {
  fn format(&self) -> Bytes {
    Bytes::from(self.to_string())
  }
}

impl Loggable for bool {
  fn format(&self) -> Bytes {
    if *self { Bytes::from_static(b"true") } else { Bytes::from_static(b"false") }
  }
}

impl Loggable for () {
  fn format(&self) -> Bytes {
    Bytes::from_static(b"")
  }
}

impl<T: Loggable> Loggable for Option<T> {
  fn format(&self) -> Bytes {
    match self {
      Some(value) => {
        let mut result = BytesMut::new();
        result.extend_from_slice(b"Some(");
        result.extend_from_slice(&value.format());
        result.extend_from_slice(b")");
        result.freeze()
      },
      None => Bytes::from_static(b"None"),
    }
  }
}

impl<T: Loggable, E: Loggable> Loggable for Result<T, E> {
  fn format(&self) -> Bytes {
    match self {
      Ok(value) => {
        let mut result = BytesMut::new();
        result.extend_from_slice(b"Ok(");
        result.extend_from_slice(&value.format());
        result.extend_from_slice(b")");
        result.freeze()
      },
      Err(error) => {
        let mut result = BytesMut::new();
        result.extend_from_slice(b"Err(");
        result.extend_from_slice(&error.format());
        result.extend_from_slice(b")");
        result.freeze()
      },
    }
  }
}

impl<T: Loggable> Loggable for Vec<T> {
  fn format(&self) -> Bytes {
    let mut result = BytesMut::new();
    result.extend_from_slice(b"[");

    for (i, item) in self.iter().enumerate() {
      if i > 0 {
        result.extend_from_slice(b", ");
      }
      result.extend_from_slice(&item.format());
    }

    result.extend_from_slice(b"]");
    result.freeze()
  }
}

impl<T: Loggable, const N: usize> Loggable for [T; N] {
  fn format(&self) -> Bytes {
    let mut result = BytesMut::new();
    result.extend_from_slice(b"[");

    for (i, item) in self.iter().enumerate() {
      if i > 0 {
        result.extend_from_slice(b", ");
      }
      result.extend_from_slice(&item.format());
    }

    result.extend_from_slice(b"]");
    result.freeze()
  }
}

impl<T: Loggable> Loggable for VecDeque<T> {
  fn format(&self) -> Bytes {
    let mut result = BytesMut::new();
    result.extend_from_slice(b"[");

    for (i, item) in self.iter().enumerate() {
      if i > 0 {
        result.extend_from_slice(b", ");
      }
      result.extend_from_slice(&item.format());
    }

    result.extend_from_slice(b"]");
    result.freeze()
  }
}

impl<T: Loggable> Loggable for LinkedList<T> {
  fn format(&self) -> Bytes {
    let mut result = BytesMut::new();
    result.extend_from_slice(b"[");

    for (i, item) in self.iter().enumerate() {
      if i > 0 {
        result.extend_from_slice(b", ");
      }
      result.extend_from_slice(&item.format());
    }

    result.extend_from_slice(b"]");
    result.freeze()
  }
}

impl<T: Loggable + ?Sized> Loggable for &T {
  fn format(&self) -> Bytes {
    (**self).format()
  }
}

impl<T: Loggable + ?Sized> Loggable for &mut T {
  fn format(&self) -> Bytes {
    (**self).format()
  }
}

impl<T: Loggable> Loggable for Box<T> {
  fn format(&self) -> Bytes {
    (**self).format()
  }
}

impl<T: Loggable> Loggable for [T] {
  fn format(&self) -> Bytes {
    let mut result = BytesMut::new();
    result.extend_from_slice(b"[");

    for (i, item) in self.iter().enumerate() {
      if i > 0 {
        result.extend_from_slice(b", ");
      }
      result.extend_from_slice(&item.format());
    }

    result.extend_from_slice(b"]");
    result.freeze()
  }
}

impl Loggable for Bytes {
  fn format(&self) -> Bytes {
    self.clone()
  }
}

impl Loggable for std::path::Path {
  fn format(&self) -> Bytes {
    match self.to_str() {
      Some(s) => Bytes::copy_from_slice(s.as_bytes()),
      None => Bytes::from_static(b"<invalid_path>"),
    }
  }
}

impl Loggable for std::path::PathBuf {
  fn format(&self) -> Bytes {
    self.as_path().format()
  }
}
