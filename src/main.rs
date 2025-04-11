mod commands;

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Sync")
        .version("1.0")
        .author("Pjdur")
        .about("A modern version control system written in Rust")
        .subcommand(
            Command::new("init")
                .about("Initialize a new repository")
                .arg(
                    Arg::new("template")
                        .short('t')
                        .long("template")
                        .value_name("TEMPLATE")
                        .help("Specify a template (c, rust, python, html, js)")
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("commit")
                .about("Record changes to the repository")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .value_name("MESSAGE")
                        .help("Commit message")
                        .num_args(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_m)) => {
            let template = sub_m.get_one::<String>("template").map(|s| s.as_str()).unwrap_or("none");
            commands::init::initialize_repository(template);
        }
        Some(("commit", sub_m)) => {
            if let Some(message) = sub_m.get_one::<String>("message") {
                println!("Committing changes with message: {}", message);
                // Implement commit logic here
            } else {
                println!("Commit message is required");
            }
        }
        _ => {
            println!("Sync v{}", "1.0");
            Command::new("Sync")
                .version("1.0")
                .about("A modern version control system written in Rust")
                .print_help()
                .expect("Failed to print help");
            println!();
        }
    }
}