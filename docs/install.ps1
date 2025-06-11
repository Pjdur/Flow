# Set variables
$homeDir = "$env:USERPROFILE\.flow"
$exeUrl = "https://github.com/Pjdur/Flow/blob/main/bin/flow.exe?raw=true"
$destFile = Join-Path -Path $homeDir -ChildPath "bin\flow.exe"
$binDir = Split-Path -Path $destFile -Parent
$checksumUrl = "https://raw.githubusercontent.com/Pjdur/Flow/refs/heads/main/checksum.txt"

# Fetch expected checksum from GitHub
try {
    Write-Host "Fetching expected checksum..."
    $expectedChecksum = (Invoke-WebRequest -Uri $checksumUrl -UseBasicParsing).Content.Trim()
} catch {
    Write-Error "Failed to retrieve checksum from '$checksumUrl'"
    exit 1
}

# Create home directory path if it doesn't exist
if (-not (Test-Path -Path $homeDir)) {
    try {
        $null = New-Item -Path $homeDir -ItemType Directory -Force
    } catch {
        Write-Error "Failed to install: unable to create home directory '$homeDir'"
        exit 1
    }
}

# Create bin directory if it doesn't exist
if (-not (Test-Path -Path $binDir)) {
    try {
        $null = New-Item -Path $binDir -ItemType Directory -Force
    } catch {
        Write-Error "Failed to create bin directory '$binDir'"
        exit 1
    }
}

try {
    # Download the executable with progress indication
    Write-Host "Downloading executable..."
    if (Test-Path $destFile) { [System.IO.File]::Delete($destFile) }

    $webClient = New-Object System.Net.WebClient
    $stream = [System.IO.File]::Create($destFile)
    $webStream = $webClient.OpenRead($exeUrl)
    $bufferSize = 8192
    $buffer = New-Object byte[] $bufferSize
    $downloadedBytes = 0
    $totalBytes = $webClient.DownloadData($exeUrl).Length

    try {
        while (($bytesRead = $webStream.Read($buffer, 0, $bufferSize)) -gt 0) {
            $stream.Write($buffer, 0, $bytesRead)
            $downloadedBytes += $bytesRead

            # Calculate percentage complete
            $percentComplete = [math]::Floor(($downloadedBytes / $totalBytes) * 100)
            $progressBar = ("#" * ($percentComplete / 2.5)) + ("-" * (40 - [math]::Floor($percentComplete / 2.5)))
            Write-Host -NoNewline "`r[$progressBar] $percentComplete% "
        }
    } finally {
        $stream.Close()
        $webStream.Close()
    }

    # Verify checksum
    Write-Host "`nVerifying checksum..."
    $calculatedChecksum = Get-FileHash -Path $destFile -Algorithm SHA256 | Select-Object -ExpandProperty Hash
    if ($calculatedChecksum -ne $expectedChecksum) {
        Write-Error "Checksum verification failed. Expected: $expectedChecksum, but got: $calculatedChecksum. Report this security issue to https://github.com/Pjdur/Flow/security/advisories"
        Remove-Item -Path $destFile -Force
        exit 1
    }

    # Add directory to PATH for the current user
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if (-not ($userPath -like "*$binDir*")) {
        [Environment]::SetEnvironmentVariable("Path", $userPath + ";$binDir", "User")
        $env:Path += ";$binDir"
    }

    Write-Host "`nSuccessfully installed Flow to $homeDir"
    Write-Host "Flow has been successfully installed."
} catch {
    Write-Error "Failed to download or install the executable: $_"
    exit 1
}
