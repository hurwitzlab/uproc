extern crate run_uproc;
use std::process;

fn main() {
    let config = match run_uproc::get_args() {
        Ok(c) => c,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    println!("{:?}", config);
    if let Err(e) = run_uproc::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
