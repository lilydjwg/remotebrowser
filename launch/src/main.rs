#[macro_use] extern crate error_chain;

use std::process::Command;
use std::net::TcpStream;
use std::io::prelude::*;
use std::env::args;
use std::io::stdin;

const SERVER_ADDR: &str = "10.0.2.2:4543";
const LOCAL_BROWSER: &str = r"C:\Program Files\Mozilla Firefox\firefox.exe";

mod errors;

use errors::*;

fn open_local(url: &str) -> Result<()> {
  Command::new(LOCAL_BROWSER)
    .arg(url)
    .status()?;
  Ok(())
}

fn open_remote(url: &str) -> Result<()> {
  let mut stream = TcpStream::connect(SERVER_ADDR)?;
  stream.write_all(url.as_bytes())?;
  Ok(())
}

fn main2() -> Result<()> {
  let url = args().nth(1).ok_or("no url given")?;
  if let Err(e) = open_remote(&url) {
    eprintln!("open remotely failed: {:?}", e);
    println!("Press Enter to open locally.");
    stdin().read_line(&mut String::new())?;
    open_local(&url)
  } else {
    Ok(())
  }
}

fn main() {
  if let Err(e) = main2() {
    eprintln!("Error: {:?}", e);
    println!("Press Enter to exit.");
    stdin().read_line(&mut String::new()).unwrap();
  }
}
