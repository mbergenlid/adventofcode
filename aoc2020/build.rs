extern crate lalrpop;
use std::env;

fn main() {
    println!("OUT DIR: {:?}", env::var_os("OUT_DIR").unwrap_or_default());
    lalrpop::Configuration::new()
        .set_in_dir("./src")
        .process()
        .unwrap();
}
