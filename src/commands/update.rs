use std::env;
use std::fs;
use std::process::Command;

pub fn update_flow() {
    if cfg!(target_os = "windows") {
        // Prepare updater.ps1 path
        let temp_dir = env::temp_dir();
        let updater_path = temp_dir.join("updater.ps1");

        // Write the updater script
        let script = r#"
# Kill all running flow.exe processes
Get-Process -Name "flow" -ErrorAction SilentlyContinue | Stop-Process -Force

# Wait a moment to ensure processes are stopped
Start-Sleep -Seconds 2

# Download new executable
$dest = "$env:USERPROFILE\.flow\bin\flow.exe"
$exeUrl = "https://raw.githubusercontent.com/Pjdur/Flow/refs/heads/main/flow.exe"
Invoke-WebRequest -Uri $exeUrl -OutFile $dest -UseBasicParsing

# Restart flow
Start-Process $dest
"#;
        if let Err(e) = fs::write(&updater_path, script) {
            eprintln!("Failed to write updater script: {e}");
            return;
        }

        // Launch updater.ps1 in a new process and exit
        let _ = Command::new("powershell")
            .args(&["-WindowStyle", "Hidden", "-File", updater_path.to_str().unwrap()])
            .spawn();

        println!("Flow will now update and restart.");
        std::process::exit(0);
    } else {
        println!("Flow update is currently only supported on Windows.");
    }
}