use git2::{Repository, RemoteCallbacks};
use std::env;

#[allow(dead_code)]
pub async fn fetch_changes(remote: &str) {
    let current_dir = env::current_dir().unwrap();
    let repo = match Repository::open(&current_dir) {
        Ok(repo) => repo,
        Err(e) => {
            println!("Failed to open repository: {}", e);
            return;
        },
    };

    let mut remote = match repo.find_remote(remote) {
        Ok(remote) => remote,
        Err(_) => match repo.remote_anonymous(remote) {
            Ok(remote) => remote,
            Err(e) => {
                println!("Failed to find remote: {}", e);
                return;
            },
        },
    };

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        git2::Cred::ssh_key_from_agent("git")
    });

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    match remote.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut fetch_options), None) {
        Ok(_) => println!("Successfully fetched changes from remote"),
        Err(e) => println!("Failed to fetch changes: {}", e),
    }
}