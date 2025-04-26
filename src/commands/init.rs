use std::fs;
use std::path::Path;
use git2::Repository;

pub fn initialize_repository(template: &str, repo_name: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let repo_path = current_dir.join(repo_name);

    if repo_path.exists() {
        println!("Error: Repository already exists in the current directory.");
        return;
    }

    fs::create_dir(&repo_path).expect("Failed to create repository directory");

    // Initialize a Git repository using git2
    match Repository::init(&repo_path) {
        Ok(_) => println!("Initialized empty Git repository in {:?}", repo_path),
        Err(e) => {
            println!("Error: Failed to initialize Git repository: {}", e);
            return;
        }
    }

    if template != "none" {
        let template_path = format!("src/templates/{}", template);
        if Path::new(&template_path).exists() {
            let template_files = fs::read_dir(&template_path)
                .expect("Failed to read template directory")
                .map(|entry| entry.expect("Failed to read directory entry").path())
                .collect::<Vec<_>>();

            for file in template_files {
                if file.is_file() {
                    let file_name = file.file_name().unwrap();
                    let destination = repo_path.join(file_name);
                    fs::copy(&file, &destination).expect("Failed to copy template file");
                }
            }
            println!("Initialized repository with {} template", template);
        } else {
            println!("Error: Template '{}' does not exist", template);
        }
    }
}