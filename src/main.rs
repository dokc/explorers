use clap::{Arg, ArgMatches, Command};
mod cli {
    pub mod initializer;
}

pub fn configurator() -> [Command<'static>; 1] {
    return [Command::new("config")
        .short_flag('c')
        .about("Helps you configure Express-Cassandra")
        .args(&[Arg::new("construct_new_connector")
            .short('n')
            .help("Construct a new connection with the new tag. Cassey -c -n new")
            .takes_value(true)])];
}

fn main() {
    let cli:ArgMatches = Command::new("Cassey CLI")
        .version("0.1.0")
        .about("This program lets you configure, work with Cassandra and Express-Cassandra without hassle.\nThis will configure your cassandra cluster for your express cassandra project.")
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .subcommands(configurator())
        .get_matches();
    println!(
        "{:#?}",
        cli.subcommand_matches("config")
            .unwrap()
            .value_of("construct_new_connector")
            .unwrap()
    );
    if cli
        .subcommand_matches("config")
        .unwrap()
        .value_of("construct_new_connector")
        .unwrap()
        == "new"
    {
        cli::initializer::create();
    }
}
