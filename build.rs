use std::path::Path;
extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn main() {
  let mut bundler: Bundler =
    Bundler::new(Path::new("src/main.rs"), Path::new("src/bin/singlefile.rs"));
  bundler.crate_name("spring-challenge-2021");
  bundler.run();
}
