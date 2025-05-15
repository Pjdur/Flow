use git2::{Repository, Error, MergeOptions};

pub fn merge_branches(repo_path: &str, branch_to_merge: &str) -> Result<(), Error> {
    // Open the repository
    let repo = Repository::open(repo_path)?;

    // Get the current branch
    let head_ref = repo.head()?;
    let head_commit = head_ref.peel_to_commit()?;
    let head_branch = head_ref.shorthand().unwrap_or("HEAD");

    // Find the branch to merge
    let branch_ref = format!("refs/heads/{}", branch_to_merge);
    let branch = repo.find_reference(&branch_ref)?;
    let branch_commit = branch.peel_to_commit()?;

    // Perform the merge
    let mut index = repo.merge_commits(&head_commit, &branch_commit, Some(&MergeOptions::new()))?;

    if index.has_conflicts() {
        println!("Merge conflicts detected. Please resolve them manually.");
        return Ok(());
    }

    // Write the merge result to the repository
    let tree_oid = index.write_tree_to(&repo)?;
    let tree = repo.find_tree(tree_oid)?;

    let signature = repo.signature()?;
    let merge_message = format!("Merge branch '{}' into '{}'", branch_to_merge, head_branch);
    repo.commit(Some("HEAD"), &signature, &signature, &merge_message, &tree, &[&head_commit, &branch_commit])?;

    println!("Successfully merged branch '{}' into '{}'", branch_to_merge, head_branch);
    Ok(())
}