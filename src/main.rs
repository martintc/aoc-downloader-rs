use crate::{config::write_config, request::{create_request_url, make_request}};

mod config;
mod request;

const HELP: &str = "\
aoc-downloader

USAGE:
  aoc-downloader [OPTIONS]

FLAGS:
  -h, --help  Prints help information

OPTIONS:
  --day       Day number of puzzle to retrieve input file for [REQUIRED 1-25]
  --year      Year number of puzzle to retrieve input file for [REQUIRED]
  --output    Directory to place puzzle input file [REQUIRED]
  --api       Api key for advent of code
";

#[derive(Debug)]
struct Args {
    year: u32,
    day: u16,
    output: String,
    api_key: Option<String>
}

fn parse_args() -> anyhow::Result<Args, anyhow::Error> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
	print!("{}", HELP);
	std::process::exit(0);
    }

    let args = Args {
	day: args.value_from_str("--day")?,
	year: args.value_from_str("--year")?,
	output: args.value_from_str("--output")?,
	api_key: args.opt_value_from_str("--api")?
    };

    Ok(args)
}

fn check_directory_exists(path: &str) -> bool {
    let metadata = match std::fs::metadata(path) {
	Ok(metadata) => metadata,
	Err(_) => return false,
    };

    if metadata.is_dir() {
	return true;
    }

    return false;
}


fn main() -> anyhow::Result<(), anyhow::Error> {
    // get arguments
    let args = match parse_args() {
	Ok(args) => args,
	Err(_) => {
	    print!("{}", HELP);
	    std::process::exit(0);
	}
    };

    if args.day > 25 || args.day == 0 {
	println!("Must be a day between 1 and 25.");
	std::process::exit(0);
    }

    if !check_directory_exists(&args.output) {
	println!("Directory for output does not exist.");
	std::process::exit(0);
    }
    
    // get config
    let mut config = match config::get_config() {
	Ok(config) => config,
	Err(e) => panic!("Error: {}", e)
    };

    if let Some(api_key) = args.api_key {
	config.api_key = api_key;
	write_config(&config)?;
    }

    println!("{:#?}", config);

    let request_url = create_request_url(args.day, args.year);
    println!("{}", request_url);

    make_request(&request_url, &config.api_key)?;
    
    Ok(())
}
