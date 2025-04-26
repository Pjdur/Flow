use git2::Repository;
use std::path::Path;

#[allow(dead_code)]
pub async fn clone_repository(url: &str) {
    let repo_name = url.split('/').last().unwrap_or("repo").replace(".git", "");
    let repo_path = Path::new(&repo_name);

    match Repository::clone(url, repo_path) {
        Ok(_) => println!("Successfully cloned repository to {:?}", repo_path),
        Err(e) => println!("Failed to clone repository: {}", e),
    }
}