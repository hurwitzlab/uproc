extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::{
    env, fs::{self, DirBuilder, File}, io::Write, path::{Path, PathBuf},
};
#[derive(Debug)]
pub struct Config{
    query: Vec<String>,
}

type MyResult<T>= Result<T, Box<Error>>;


pub fn get_args() -> MyResult<Config>{
   let matches = App::new("run_uproc")
                      .version("0.1.0")
                      .author("alise ponsero")
                      .about("runs uproc")
                      .arg(
                            Arg::with_name("query")
                            .short("q")
                            .long("query")
                            .value_name("FILE_OR_DIR")
                            .help("File imput or directory")
                            .required(true)
                            .min_values(1),
                        )
                        .get_matches();
    println!("{:?}", matches);
    //Err(From::from("foo"))
    Ok( Config {
        query: matches.values_of_lossy("query").unwrap(),
    })
}   

pub fn run(config: Config) -> MyResult<()> {
    let files= find_files(&config.query)?;
    println!(
        "Will process {} file{}", files.len(), 
        if files.len()==1 {""} else {"s"}
        );

        //let out_dir =&config.out_dir;
        
    Ok(())
}

fn find_files(paths: &Vec<String>) -> Result<Vec<String>, Box<Error>> {
    let mut files = vec![];
    for path in paths {
        let meta = fs::metadata(path)?;
        if meta.is_file() {
            files.push(path.to_owned());
        } else {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let meta = entry.metadata()?;
                if meta.is_file() {
                    files.push(entry.path().display().to_string());
                }
            }
        };
    }

    if files.len() == 0 {
        return Err(From::from("No input files"));
    }

    Ok(files)
}
