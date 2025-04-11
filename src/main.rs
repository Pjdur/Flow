use clap::{Command, Arg};

fn main() {
    let matches = Command::new("Sync")
        .version("1.0")
        .author("Pjdur")
        .about("A modern version control system written in Rust")
        .subcommand(
            Command::new("init")
                .about("Initialize a new repository")
        )
        .subcommand(
            Command::new("commit")
                .about("Record changes to the repository")
                .arg(Arg::new("message")
                    .short('m')
                    .long("message")
                    .value_name("MESSAGE")
                    .help("Commit message")
                    .num_args(1))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Initializing a new repository...");
            // Implement repository initialization logic here
        },
        Some(("commit", sub_m)) => {
            if let Some(message) = sub_m.get_one::<String>("message") {
                println!("Committing changes with message: {}", message);
                // Implement commit logic here
            } else {
                println!("Commit message is required");
            }
        },
        _ => println!("Unknown command"),
    }
}