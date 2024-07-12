use clap::{Arg, Command};

pub fn build_cli() -> Command {
    let pull_command = Command::new("pull").arg(
        // dak run xxx
        Arg::new("image")
            .long("image")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Name of the image to pull"),
    );

    let run_command = Command::new("run").arg(
        // dak run --image xxx
        Arg::new("image")
            .long("image")
            .value_parser(clap::value_parser!(String))
            .required(true)
            .help("Name of the image to pull"),
    );

    Command::new("dak")
        .bin_name("dak")
        .subcommand_required(true)
        .subcommands(vec![pull_command, run_command])
}
