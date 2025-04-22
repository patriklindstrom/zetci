use clap::{crate_version, Arg, Command, arg};
pub fn cli() -> Command {
    Command::new("set_operations")
        .version(crate_version!())
        .author("Patrik Lindström <patrik.lindstrom@lcube.se>")
        .about("Performs set operations on csv files")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("outfile")
                .help("Sets an output file, stdout if not present")
                .global(true),
        )
        .arg(
            Arg::new("files")
                .short('f')
                .long("files")
                .value_name("file")
                .help("Sets the input files to use")
                .required(false)
                .value_delimiter(',')
                .num_args(1..)
                .global(true),
        )
        .subcommand(
            Command::new("union")
                .about("Performs union operation on csv files")
                .version("1.0")
                .author("Patrik Lindström <patrik.lindstrom@lcube.se>"),
        )
        .subcommand(
            Command::new("intersect")
                .about("Performs intersection operation on csv files")
                .version("1.0")
                .author("Patrik Lindström <patrik.lindstrom@lcube.se>")
        )
        .subcommand(
            Command::new("diffa")
                .about("Performs difference operation on csv files")
                .version("1.0")
                .author("Patrik Lindström <patrik.lindstrom@lcube.se>"),
        )
        .subcommand(
            Command::new("xor")
                .about("Performs an -exclusive or- operation on csv files")
                .version("1.0")
                .author("Patrik Lindström <patrik.lindstrom@lcube.se>")
                .arg(
                    Arg::new("value-strategy")
                        .short('s')
                        .long("value-strategy")
                        .required(true)
                        .help("How to handle values for duplicated keys: first, last, concat")
                        .default_value("first")
                        .value_parser(["first", "last", "concat"])
                )
        )
        .subcommand(
            Command::new("about")
                .about("Displays the logo")
                .version("1.0")
                .author("Patrik Lindström <patrik.lindstrom@lcube.se>"),
        )
}
