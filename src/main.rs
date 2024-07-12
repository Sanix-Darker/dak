mod cgroups;
mod cli;
mod container;
mod filesystem;
mod namespaces;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        Some(("run", matches)) => {
            if let Some(image) = matches.get_one::<String>("image") {
                println!("> Running image : {:?}", image);
                // run the image << containers -> cgroups -> namespaces -> networking ->
                // filesystem
                container::start_container(image)
            }
        }
        Some(("pull", matches)) => {
            if let Some(image) = matches.get_one::<String>("image") {
                println!("> Pulling image : {:?}", image);
                // download of the image
            }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
