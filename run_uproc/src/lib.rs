extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::{
    env, fs::{self, DirBuilder}, path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Config {
    query: Vec<String>,
    counts: bool,
    stats: bool,
    preds: bool,
    othresh: Option<u32>,
    pthresh: Option<u32>,
    read_length: ReadLength,
    uproc_db_dir: PathBuf,
    uproc_model_dir: PathBuf,
    out_dir: PathBuf,
}

#[derive(Debug, PartialEq)]
enum ReadLength {
    LongReads,
    ShortReads,
}

type MyResult<T> = Result<T, Box<Error>>;

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("run_uproc")
        .version("0.1.0")
        .author("alise ponsero")
        .about("runs uproc")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .value_name("FILE_OR_DIR")
                .help("File input or directory")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("preds")
                .short("p")
                .long("preds")
                .help("Print all classifications"),
        )
        .arg(
            Arg::with_name("stats")
                .short("f")
                .long("stats")
                .help("Print 'CLASSIFIED,UNCLASSIFIED,TOTAL' sequence counts"),
        )
        .arg(
            Arg::with_name("counts")
                .short("c")
                .long("counts")
                .help("Print 'FAMILY,COUNT'"),
        )
        .arg(
            Arg::with_name("read_length")
                .short("r")
                .long("read_length")
                .value_name("STR")
                .default_value("long")
                .help("long or short"),
        )
        .arg(
            Arg::with_name("pthresh")
                .short("P")
                .long("pthresh")
                .value_name("INT")
                .default_value("3")
                .help("Protein threshold level"),
        )
        .arg(
            Arg::with_name("othresh")
                .short("O")
                .long("othresh")
                .value_name("INT")
                .default_value("2")
                .help("ORF translation threshold level"),
        )
        .arg(
            Arg::with_name("uproc_db_dir")
                .short("d")
                .long("uproc_db_dir")
                .value_name("STR")
                .help("Directory of UProc dbs"),
        )
        .arg(
            Arg::with_name("uproc_model_dir")
                .short("m")
                .long("uproc_model_dir")
                .value_name("STR")
                .help("Directory of UProc models"),
        )
        .arg(
            Arg::with_name("out_dir")
                .short("o")
                .long("out_dir")
                .value_name("DIR")
                .help("Output directory"),
        )
        .get_matches();

    let othresh = matches
        .value_of("othresh")
        .and_then(|x| x.trim().parse::<u32>().ok());

    let pthresh = matches
        .value_of("pthresh")
        .and_then(|x| x.trim().parse::<u32>().ok());

    let read_length = match matches.value_of("read_length") {
        Some("short") => ReadLength::ShortReads,
        _ => ReadLength::LongReads,
    };

    let uproc_db_dir = match matches.value_of("uproc_db_dir") {
        Some(x) => PathBuf::from(x),
        _ => {
            return Err(From::from("Must have --uproc_db_dir"));
        }
    };

    let uproc_model_dir = match matches.value_of("uproc_model_dir") {
        Some(x) => PathBuf::from(x),
        _ => {
            return Err(From::from("Must have --uproc_model_dir"));
        }
    };

    let out_dir = match matches.value_of("out_dir") {
        Some(x) => PathBuf::from(x),
        _ => {
            let cwd = env::current_dir()?;
            cwd.join(PathBuf::from("uproc-out"))
        }
    };

    //Err(From::from("foo"))
    Ok(Config {
        query: matches.values_of_lossy("query").unwrap(),
        counts: matches.is_present("counts"),
        stats: matches.is_present("stats"),
        preds: matches.is_present("preds"),
        othresh: othresh,
        pthresh: pthresh,
        read_length: read_length,
        uproc_db_dir: uproc_db_dir,
        uproc_model_dir: uproc_model_dir,
        out_dir: out_dir,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.query)?;

    if files.len() == 0 {
        let msg = format!("No input files from query \"{:?}\"", &config.query);
        return Err(From::from(msg));
    }

    println!(
        "Will process {} file{}",
        files.len(),
        if files.len() == 1 { "" } else { "s" }
    );

    let out_dir = &config.out_dir;
    if !out_dir.is_dir() {
        DirBuilder::new().recursive(true).create(&out_dir)?;
    }

    let uproc_dir = run_uproc_dna(&config, &files)?;

    println!("Done, see output in \"{:?}\"", uproc_dir);

    Ok(())
}

// --------------------------------------------------
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

// --------------------------------------------------
fn find_dirs(base_dir: &PathBuf) -> Result<Vec<String>, Box<Error>> {
    if !base_dir.is_dir() {
        let msg = format!("base_dir \"{:?}\" is not a directory", base_dir);
        return Err(From::from(msg));
    };

    let mut dirs = vec![];
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_dir() {
            dirs.push(entry.path().display().to_string());
        }
    }

    if dirs.len() == 0 {
        return Err(From::from(format!("No directories in {:?}", base_dir)));
    }

    Ok(dirs)
}

// --------------------------------------------------
fn run_uproc_dna(config: &Config, files: &Vec<String>) -> MyResult<PathBuf> {
    let uproc_dir = config.out_dir.join(PathBuf::from("uproc"));
    if !uproc_dir.is_dir() {
        DirBuilder::new().recursive(true).create(&uproc_dir)?;
    }

    let mut args: Vec<String> = vec![];

    if config.counts {
        args.push("--counts".to_string());
    }

    if config.stats {
        args.push("--stats".to_string());
    }

    if config.preds {
        args.push("--preds".to_string());
    }

    if let Some(othresh) = config.othresh {
        args.push(format!("--othresh {}", othresh));
    }

    if let Some(pthresh) = config.pthresh {
        args.push(format!("--pthresh {}", pthresh));
    }

    args.push(if config.read_length == ReadLength::LongReads {
        "--long".to_string()
    } else {
        "--short".to_string()
    });

    let uproc_dbs = find_dirs(&config.uproc_db_dir);

    println!("uproc_dbs = {:?}", uproc_dbs);

    let mut jobs: Vec<String> = vec![];
    for file in files.iter() {
        println!("file = {:?}", file);
        for db_dir in uproc_dbs.iter() {
            println!("db = {:?}", &db_dir);
            //println!("dbsplit = {:?}", &db_dir.split("/"));

            //let Some(db_name) = match db_dir.split("/").last() {
            //    Some(x) => x,
            //    _ => {
            //        let msg = format!("Can't split {}", db_dir);
            //        return Err(From::from(msg));
            //    }
            //};

            //if let Some(basename) = Path::new(file).file_name() {
            //    let out_file =
            //        uproc_dir.join(format!("{}.{}", basename, db_name));

            //    if !Path::new(&out_file).exists() {
            //        jobs.push(format!(
            //            "hulk sketch {} -o {} -f {}",
            //            args.join(" "),
            //            out_file.display(),
            //            file,
            //        ));
            //    }
            //}
        }
    }

    Ok(uproc_dir)
}
