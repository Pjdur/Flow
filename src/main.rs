use clap::{App, Arg};

fn main() {
    let matches = App::new("Sync")
        .version("1.0")
        .author("Pjdur")
        .about("A modern version control system written in Rust")
        .subcommand(
            App::new("init")
                .about("Initialize a new repository")
        )
        .subcommand(
            App::new("commit")
                .about("Record changes to the repository")
                .arg(Arg::new("message")
                    .short('m')
                    .long("message")
                    .value_name("MESSAGE")
                    .about("Commit message")
                    .takes_value(true))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Initializing a new repository...");
            // Implement repository initialization logic here
        },
        Some(("commit", sub_m)) => {
            if let Some(message) = sub_m.value_of("message") {
                println!("Committing changes with message: {}", message);
                // Implement commit logic here
            } else {
                println!("Commit message is required");
            }
        },
        _ => println!("Unknown command"),
    }
}