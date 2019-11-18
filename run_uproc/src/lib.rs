extern crate clap;
extern crate csv;
extern crate regex;
extern crate walkdir;

use clap::{App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{
    env,
    fs::{self, DirBuilder, File},
    io::Write,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Config {
    query: Vec<String>,
    othresh: Option<u32>,
    pthresh: Option<u32>,
    read_length: ReadLength,
    uproc_db_dir: PathBuf,
    uproc_model_dir: PathBuf,
    annotation_dir: PathBuf,
    out_dir: PathBuf,
    num_concurrent_jobs: Option<u32>,
    num_halt: Option<u32>,
}

#[derive(Debug, PartialEq)]
enum ReadLength {
    LongReads,
    ShortReads,
}

type MyResult<T> = Result<T, Box<dyn Error>>;
type Record = HashMap<String, String>;
type RecordLookup = HashMap<String, Record>;

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("run_uproc")
        .version("0.1.0")
        .author("Alise Ponsero, Ken Youens-Clark")
        .about("Runs UProc")
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
        .arg(
            Arg::with_name("annotation_dir")
                .short("a")
                .long("annotation_dir")
                .value_name("DIR")
                .help("Directory with annotation files"),
        )
        .arg(
            Arg::with_name("num_concurrent_jobs")
                .short("n")
                .long("num_concurrent_jobs")
                .value_name("INT")
                .default_value("8")
                .help("Number of concurrent jobs for parallel"),
        )
        .arg(
            Arg::with_name("num_halt")
                .short("H")
                .long("num_halt")
                .value_name("INT")
                .default_value("0")
                .help("Halt after this many failing jobs"),
        )
        .get_matches();

    let othresh = matches
        .value_of("othresh")
        .and_then(|x| x.trim().parse::<u32>().ok());

    let pthresh = matches
        .value_of("pthresh")
        .and_then(|x| x.trim().parse::<u32>().ok());

    let num_concurrent_jobs = matches
        .value_of("num_concurrent_jobs")
        .and_then(|x| x.trim().parse::<u32>().ok());

    let num_halt = matches
        .value_of("num_halt")
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

    let annotation_dir = match matches.value_of("annotation_dir") {
        Some(x) => PathBuf::from(x),
        _ => {
            return Err(From::from("Must have --annotation_dir"));
        }
    };

    let out_dir = match matches.value_of("out_dir") {
        Some(x) => PathBuf::from(x),
        _ => {
            let cwd = env::current_dir()?;
            cwd.join(PathBuf::from("uproc-out"))
        }
    };

    Ok(Config {
        query: matches.values_of_lossy("query").unwrap(),
        othresh,
        pthresh,
        read_length,
        uproc_db_dir,
        uproc_model_dir,
        annotation_dir,
        out_dir,
        num_concurrent_jobs,
        num_halt,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.query)?;

    if files.is_empty() {
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

    run_uproc_dna(&config, &files)?;
    let split_files = split_uproc_output(&config)?;
    annotate_uproc(&config, &split_files)?;

    println!("Done, see output in \"{:?}\"", &config.out_dir);

    Ok(())
}

// --------------------------------------------------
fn find_files(paths: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
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

    if files.is_empty() {
        return Err(From::from("No input files"));
    }

    Ok(files)
}

// --------------------------------------------------
fn find_dirs(base_dir: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
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

    if dirs.is_empty() {
        return Err(From::from(format!("No directories in {:?}", base_dir)));
    }

    Ok(dirs)
}

// --------------------------------------------------
fn run_uproc_dna(config: &Config, files: &[String]) -> MyResult<()> {
    let mut args: Vec<String> = vec![
        "--counts".to_string(),
        "--preds".to_string(),
        "--stats".to_string(),
    ];

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

    let uproc_dbs = find_dirs(&config.uproc_db_dir)?;

    let mut jobs: Vec<String> = vec![];
    for file in files.iter() {
        for db_dir in uproc_dbs.iter() {
            let db_name = &db_dir.split('/').last().unwrap();

            if let Some(basename) = Path::new(file).file_name() {
                let out_file = &config.out_dir.join(format!(
                    "{}.{}",
                    basename.to_string_lossy(),
                    db_name
                ));

                if !Path::new(&out_file).exists() {
                    jobs.push(format!(
                        "uproc-dna {} -o {} {} {} {}",
                        args.join(" "),
                        out_file.display(),
                        &db_dir,
                        &config.uproc_model_dir.to_string_lossy(),
                        file,
                    ));
                }
            }
        }
    }

    if !jobs.is_empty() {
        run_jobs(
            &jobs,
            "Running uproc-dna",
            config.num_concurrent_jobs.unwrap_or(8),
            config.num_halt.unwrap_or(1),
        )?;
    } else {
        println!("No jobs to run, skipping this step");
    }

    Ok(())
}

// --------------------------------------------------
fn run_jobs(
    jobs: &[String],
    msg: &str,
    num_concurrent_jobs: u32,
    num_halt: u32,
) -> MyResult<()> {
    let num_jobs = jobs.len();

    if num_jobs > 0 {
        println!(
            "{} (# {} job{} @ {})",
            msg,
            num_jobs,
            if num_jobs == 1 { "" } else { "s" },
            num_concurrent_jobs,
        );

        let mut process = Command::new("parallel")
            .arg("-j")
            .arg(num_concurrent_jobs.to_string())
            .arg("--halt")
            .arg(format!("soon,fail={}", num_halt.to_string()))
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;

        {
            let stdin = process.stdin.as_mut().expect("Failed to open stdin");
            stdin
                .write_all(jobs.join("\n").as_bytes())
                .expect("Failed to write to stdin");
        }

        let result = process.wait()?;
        if !result.success() {
            return Err(From::from("Failed to run jobs in parallel"));
        }
    }

    Ok(())
}

// --------------------------------------------------
fn split_uproc_output(config: &Config) -> MyResult<Vec<String>> {
    let re = Regex::new(r"\.(kegg|pfam\d+)$").unwrap();
    let files: Vec<String> = WalkDir::new(&config.out_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().display().to_string())
        .filter(|name| re.is_match(name))
        .collect();

    let mut results = vec![];
    for file in files {
        let f = File::open(&file)?;
        let f = BufReader::new(f);

        let stats_out = format!("{}.stats", file);
        let preds_out = format!("{}.preds", file);
        let counts_out = format!("{}.counts", file);

        let stats_fh = File::create(&stats_out)?;
        let preds_fh = File::create(&preds_out)?;
        let counts_fh = File::create(&counts_out)?;

        for line in f.lines() {
            let line = line?;
            let flds: Vec<&str> = line.split(',').collect();
            if flds.len() == 2 {
                writeln!(&counts_fh, "{}", &line)?;
            } else if flds.len() == 3 {
                writeln!(&stats_fh, "{}", &line)?;
            } else {
                writeln!(&preds_fh, "{}", &line)?;
            }
        }

        results.push(counts_out);
        fs::remove_file(file)?;
    }

    Ok(results)
}

// --------------------------------------------------
fn annotate_uproc(config: &Config, files: &[String]) -> MyResult<()> {
    let kegg_db = read_annotation_file(
        &config.annotation_dir.join("kegg_annotation.tab"),
        "kegg_annotation_id",
    )?;

    let pfam_db = read_annotation_file(
        &config.annotation_dir.join("pfam_annotation.tab"),
        "accession",
    )?;

    let pfam_hdrs = vec!["pfam_id", "count", "identifier", "name"];
    let kegg_hdrs = vec![
        "kegg_id",
        "count",
        "name",
        "definition",
        "pathway",
        "module",
    ];

    for file in files {
        let f = File::open(&file)?;
        let f = BufReader::new(f);

        let file_type = if file.ends_with(".pfam28.counts") {
            "pfam"
        } else if file.ends_with(".kegg.counts") {
            "kegg"
        } else {
            let msg = format!("Unexpected file: {}", file);
            return Err(From::from(msg));
        };

        let out = format!("{}.annotated", file);
        let fh = File::create(&out)?;
        let (hdrs, db) = if file_type == "pfam" {
            (&pfam_hdrs, &pfam_db)
        } else {
            (&kegg_hdrs, &kegg_db)
        };

        writeln!(&fh, "{}", hdrs.join("\t"))?;

        for line in f.lines() {
            let line = line?;
            let vals: Vec<&str> = line.split(',').collect();
            if vals.len() == 2 {
                let id = vals[0];
                let count = vals[1];
                if let Some(rec) = db.get(id) {
                    let annots = if file_type == "kegg" {
                        vec![
                            id,
                            count,
                            rec.get("name").unwrap(),
                            rec.get("definition").unwrap(),
                            rec.get("pathway").unwrap(),
                            rec.get("module").unwrap(),
                        ]
                    } else {
                        vec![
                            id,
                            count,
                            rec.get("identifier").unwrap(),
                            rec.get("name").unwrap(),
                        ]
                    };
                    writeln!(&fh, "{}", annots.join("\t"))?;
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn read_annotation_file(file: &PathBuf, key: &str) -> MyResult<RecordLookup> {
    let f = File::open(&file)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(f);

    let mut lookup: RecordLookup = HashMap::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        if let Some(id) = &record.get(key) {
            lookup.insert(id.to_string(), record.clone());
        }
    }

    Ok(lookup)
}
