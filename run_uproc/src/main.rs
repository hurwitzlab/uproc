extern crate run_uproc;
use std::process;

fn main() {
    let config= run_uproc::get_args().expect("Could not get arguments");

    println!("{:?}", config);
    if let Err(e)= run_uproc::run(config){
        println!("Error: {}", e);
        process::exit(1);
    }
}
