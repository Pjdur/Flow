mod commands;

use clap::{Arg, Command};

#[tokio::main]
async fn main() {
    let matches = Command::new("Flow")
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
                )
                .arg(
                    Arg::new("repo_name")
                        .value_name("REPO_NAME")
                        .help("Name of the repository to initialize")
                        .required(true) // Make this argument mandatory
                        .index(1), // Positional argument
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
        .subcommand(
            Command::new("fetch")
                .about("Fetch changes from a remote repository"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_m)) => {
            let template = sub_m.get_one::<String>("template").map(|s| s.as_str()).unwrap_or("none");
            if let Some(repo_name) = sub_m.get_one::<String>("repo_name") {
                commands::init::initialize_repository(template, repo_name);
            } else {
                println!("Repository name is required");
            }
        }
        Some(("commit", sub_m)) => {
            if let Some(message) = sub_m.get_one::<String>("message") {
                println!("Committing changes with message: {}", message);
                commands::commit::commit_changes(message, "default_arg");
            } else {
                println!("Commit message is required");
            }
        }
        Some(("fetch", sub_m)) => {
            println!("Fetching changes from remote repository...");
            if let Some(remote) = sub_m.get_one::<String>("remote") {
                commands::fetch::fetch_changes(remote).await;
            } else {
                println!("Remote repository name is required");
            }
        }
        _ => {
            println!("Flow v{}", "1.0");
            Command::new("Flow")
                .version("1.0")
                .about("A modern version control system written in Rust")
                .print_help()
                .expect("Failed to print help");
            println!();
        }
    }
}