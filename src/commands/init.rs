use std::fs;
use std::path::Path;
use fs_extra::dir::{copy, CopyOptions};

pub fn initialize_repository(template: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let repo_path = current_dir.join("my_repo"); // You can customize the repository name

    if repo_path.exists() {
        println!("Error: Repository already exists in the current directory.");
        return;
    }

    fs::create_dir(&repo_path).expect("Failed to create repository directory");
    println!("Initialized empty repository in {:?}", repo_path);

    if template != "none" {
        let template_path = format!("src/templates/{}", template);
        if Path::new(&template_path).exists() {
            let options = CopyOptions::new();
            copy(&template_path, &repo_path, &options).expect("Failed to copy template files");
            println!("Initialized repository with {} template", template);
        } else {
            println!("Error: Template '{}' does not exist", template);
        }
    }
}