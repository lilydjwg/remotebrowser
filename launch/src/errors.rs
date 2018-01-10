use std;

error_chain! {
  foreign_links {
    IoError(std::io::Error);
  }
}
