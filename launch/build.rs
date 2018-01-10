fn main() {
  println!("cargo:rustc-link-search=native=/home/lilydjwg/workspace/remotebrowser");
  println!("cargo:rustc-link-lib=static=res");
}
