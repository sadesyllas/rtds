pub mod tcp;

pub struct TdsConnectionOptions {
  // TODO
}

pub trait TdsConnection {
  fn connect(&mut Self, &TdsConnectionOptions);
  fn disconnect(&mut Self);
  fn read() -> Vec<u8>;
  fn write(&[u8]);
}
