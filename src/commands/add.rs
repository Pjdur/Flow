use git2::Repository;
use std::path::Path;

pub fn add_files(repo_path: &str, files: &[String]) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;
    let mut index = repo.index()?;

    for file in files {
        let path = Path::new(file);
        if path.exists() {
            index.add_path(path)?;
        } else {
            eprintln!("Warning: file '{}' does not exist and will be skipped.", file);
        }
    }

    index.write()?;
    Ok(())
}