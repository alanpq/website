use std::io::{Error, ErrorKind};

pub fn yaml_err<S: AsRef<str>, E>(opt: Option<E>, msg: S) -> Result<E, std::io::Error> {
  opt.ok_or(Error::new(ErrorKind::InvalidData, msg.as_ref()))
}