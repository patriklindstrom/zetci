// To test this run
// cargo run -- -o "test.csv" union -f "foo.csv" "fum.csv"
extern crate clap;
use clap::{App, Arg};

fn main() {
    println!("Hello, world!");
    let matches = App::new("zet-cmder")
        .version("0.1")
        .author("Patrik Lindstrom. <patrik.lindstrom.dev>")
        .about("Extracts data from several textfile with set based logic")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("CONF")
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::new("OUTPUT")
                .about("Sets the a outpuft file for the result")
                .required(false)
                .takes_value(true)
                .short('o')
                .long("output"),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .takes_value(true)
                .about("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("union")
                .about("union set operator between files")
                .version("0.1")
                .author("Patrik Lindstrom. <patrik.lindstrom.dev>")
                .arg(
                    Arg::new("FILES")
                        .short('f')
                        .long("files")
                        .takes_value(true)
                        .required(true)
                        .multiple(true)
                        .min_values(1)
                        .about("all files that should be in union"),
                ),
        )
        .get_matches();
    // You can check the value provided by positional arguments, or option arguments
    if let Some(i) = matches.value_of("OUTPUT") {
        println!("Value for output: {}", i);
    }
    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }
    match matches.subcommand_name() {
        Some("union") => println!("'zet-cmder union' was used"),
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }

    if let Some(ref matches) = matches.subcommand_matches("union") {
        // "$ zet-cmder union" was run
        println!("Subcommand union wasrun");
        if matches.is_present("FILES") {
            // "$ myapp union -f filename1.csv filename2.csv" was run
            let files: Vec<_> = matches.values_of("FILES").unwrap().collect();
            for f in files {
                println!("file : {}", f);
            }
        }
    }
}
