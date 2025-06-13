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
# Clear the console output
Clear-Host

# Kill all running flow.exe processes
Get-Process -Name "flow" -ErrorAction SilentlyContinue | Stop-Process -Force

# Wait a moment to ensure processes are stopped
Start-Sleep -Seconds 2

# Download new executable with progress bar
$dest = "$env:USERPROFILE\.flow\bin\flow.exe"
$exeUrl = "https://raw.githubusercontent.com/Pjdur/Flow/refs/heads/main/bin/flow.exe"

try {
    $response = Invoke-WebRequest -Uri $exeUrl -UseBasicParsing -Method Head
    $total = [int64]$response.Headers["Content-Length"]
    $blockSize = 8192
    $downloaded = 0
    $barLength = 20

    $req = [System.Net.HttpWebRequest]::Create($exeUrl)
    $res = $req.GetResponse()
    $stream = $res.GetResponseStream()
    $fs = [System.IO.File]::Open($dest, [System.IO.FileMode]::Create)

    $buffer = New-Object byte[] $blockSize
    while (($read = $stream.Read($buffer, 0, $blockSize)) -gt 0) {
        $fs.Write($buffer, 0, $read)
        $downloaded += $read
        $percent = [math]::Min([math]::Max($downloaded / $total, 0), 1)
        $filled = [int]($percent * $barLength)
        $bar = ('#' * $filled).PadRight($barLength, '-')
        Write-Host -NoNewline "`r[$bar] $([int]($percent*100))% "
    }
    Write-Host ""
    $fs.Close()
    $stream.Close()
    $res.Close()
} catch {
    Write-Host "Download failed: $_"
    exit 1
}

# Restart flow
Start-Process $dest
"#;
        if let Err(e) = fs::write(&updater_path, script) {
            // Clear output before error
            print!("\x1B[2J\x1B[1;1H");
            eprintln!("Failed to write updater script: {e}");
            return;
        }

        // Launch updater.ps1 in a new process and exit
        let _ = Command::new("powershell")
            .args(&["-WindowStyle", "Hidden", "-File", updater_path.to_str().unwrap()])
            .spawn();

        // Clear output before exit
        print!("\x1B[2J\x1B[1;1H");
        println!("Flow will now update and restart.");
        std::process::exit(0);
    } else {
        // Clear output before message
        print!("\x1B[2J\x1B[1;1H");
        println!("Flow update is currently only supported on Windows.");
    }
}