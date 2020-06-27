use std::env;
use std::process;

use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("{}", config.query);
    // println!("{}", filename);
    // println!("{:?}", args);

    // let contents =
    //    fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    // println!("the text: \n{}", contents);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

