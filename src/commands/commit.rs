use git2::{Repository, Signature};
use std::path::Path;

pub fn commit_changes(repo_path: &str, message: &str) {
    // Open the repository
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to open repository: {}", e);
            return;
        }
    };

    // Get the index (staging area)
    let mut index = match repo.index() {
        Ok(index) => index,
        Err(e) => {
            eprintln!("Failed to get repository index: {}", e);
            return;
        }
    };

    // Write the index to the repository tree
    let tree_id = match index.write_tree() {
        Ok(tree_id) => tree_id,
        Err(e) => {
            eprintln!("Failed to write tree: {}", e);
            return;
        }
    };

    let tree = match repo.find_tree(tree_id) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("Failed to find tree: {}", e);
            return;
        }
    };

    // Get the HEAD commit (if it exists)
    let parent_commit = match repo.head() {
        Ok(head) => {
            if head.symbolic_target().is_none() {
                None
            } else {
                match head.peel_to_commit() {
                    Ok(commit) => Some(commit),
                    Err(e) => {
                        eprintln!("Failed to find HEAD commit: {}", e);
                        return;
                    }
                }
            }
        }
        Err(_) => None,
    };

    // Create a signature for the commit
    let signature = match Signature::now("Your Name", "your.email@example.com") {
        Ok(sig) => sig,
        Err(e) => {
            eprintln!("Failed to create signature: {}", e);
            return;
        }
    };

    // Create the commit
    let commit_result = if let Some(parent) = parent_commit {
        repo.commit(
            Some("HEAD"), // Update the HEAD reference
            &signature,   // Author
            &signature,   // Committer
            message,      // Commit message
            &tree,        // Tree
            &[&parent],   // Parents
        )
    } else {
        repo.commit(
            Some("HEAD"), // Update the HEAD reference
            &signature,   // Author
            &signature,   // Committer
            message,      // Commit message
            &tree,        // Tree
            &[],          // No parents (initial commit)
        )
    };

    match commit_result {
        Ok(_) => println!("Commit created successfully!"),
        Err(e) => eprintln!("Failed to create commit: {}", e),
    }
}