use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

pub fn commit_changes(message: &str) {
    // Generate a unique hash for the commit
    let hash = generate_commit_hash();

    // Get the current date in DD/MM/YY format
    let date = get_current_date();

    // Format the commit entry
    let commit_entry = format!("[{}] {} <{}>", message, date, hash);

    // Write the commit entry to commits.txt
    if let Err(e) = save_commit_to_file(&commit_entry) {
        eprintln!("Failed to save commit: {}", e);
    } else {
        println!("Commit saved successfully!");
    }
}

// Generate a unique hash for the commit
fn generate_commit_hash() -> String {
    let system_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp = system_time.as_secs();
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_string());
    format!("{:x}", hasher.finalize())
}

// Get the current date in DD/MM/YY format
fn get_current_date() -> String {
    let now = chrono::Utc::now();
    now.format("%d/%m/%y").to_string()
}

// Save the commit entry to commits.txt
fn save_commit_to_file(commit_entry: &str) -> io::Result<()> {
    let file_path = Path::new(".sync/commits.txt");
    let mut file = if file_path.exists() {
        OpenOptions::new().append(true).open(file_path)?
    } else {
        fs::File::create(file_path)?
    };

    writeln!(file, "{}", commit_entry)?;
    Ok(())
}