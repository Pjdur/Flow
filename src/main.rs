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
                        .required(true)
                        .index(1),
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
            Command::new("merge")
                .about("Merge a branch into the current branch")
                .arg(
                    Arg::new("branch")
                        .value_name("BRANCH")
                        .help("Name of the branch to merge")
                        .required(true)
                        .index(1),
                ),
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
                commands::commit::commit_changes(".", message);
            } else {
                println!("Commit message is required");
            }
        }
        Some(("merge", sub_m)) => {
            if let Some(branch) = sub_m.get_one::<String>("branch") {
                match commands::merge::merge_branches(".", branch) {
                    Ok(_) => println!("Successfully merged branch '{}'", branch),
                    Err(e) => eprintln!("Failed to merge branch '{}': {}", branch, e),
                }
            } else {
                println!("Branch name is required");
            }
        }
        _ => {
            println!("Flow v{}", "1.0.4");
            Command::new("Flow")
                .version("1.0.4")
                .about("A modern version control system written in Rust")
                .print_help()
                .expect("Failed to print help");
            println!();
        }
    }
}