use std::process::Command;
use std::env;

pub fn update_flow() {
    if cfg!(target_os = "windows") {
        let status = Command::new("powershell")
            .args(&[
                "-c",
                "irm https://pjdur.github.io/Flow/docs/install.ps1 | iex",
            ])
            .status();

        match status {
            Ok(s) if s.success() => println!("Flow updated successfully."),
            Ok(s) => eprintln!("Flow update failed with exit code: {}", s),
            Err(e) => eprintln!("Failed to execute PowerShell: {}", e),
        }
    } else {
        println!("Flow update is currently only supported on Windows.");
    }
}