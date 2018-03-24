use ::connection::TdsConnection;

pub struct TdsTcpConnection;

impl TdsConnection for TdsTcpConnection {
  // TODO
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
