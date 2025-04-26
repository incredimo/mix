param (
    [Parameter(Mandatory=$false)]
    [ValidateSet("major", "minor", "patch")]
    [string]$VersionType = "patch",

    [Parameter(Mandatory=$false)]
    [switch]$WhatIf
)

# Function to parse and increment semantic version
function Increment-Version {
    param (
        [string]$Version,
        [string]$Type
    )

    $parts = $Version -split '\.'
    $major = [int]$parts[0]
    $minor = [int]$parts[1]
    $patch = [int]$parts[2]

    switch ($Type) {
        "major" {
            $major += 1
            $minor = 0
            $patch = 0
        }
        "minor" {
            $minor += 1
            $patch = 0
        }
        "patch" {
            $patch += 1
        }
    }

    return "$major.$minor.$patch"
}

# Ensure we're in the git repository root
$gitRoot = git rev-parse --show-toplevel
if ($LASTEXITCODE -ne 0) {
    Write-Error "Not in a git repository!"
    exit 1
}
Set-Location $gitRoot

# Ensure we're on the main branch
$currentBranch = git rev-parse --abbrev-ref HEAD
if ($currentBranch -ne "main") {
    Write-Error "Not on main branch! Current branch: $currentBranch"
    exit 1
}

# Check for uncommitted changes
$status = git status --porcelain
if ($status) {
    Write-Error "There are uncommitted changes in the repository!"
    Write-Host "Please commit or stash your changes before running this script."
    exit 1
}

# Pull latest changes
Write-Host "📥 Pulling latest changes..."
if (-not $WhatIf) {
    git pull --rebase
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to pull latest changes!"
        exit 1
    }
} else {
    Write-Host "[WhatIf] Would pull latest changes with rebase"
}

# Read current version from Cargo.toml
$cargoContent = Get-Content "Cargo.toml" -Raw
if ($cargoContent -match "version\s*=\s*\"([0-9]+\.[0-9]+\.[0-9]+)\"") {
    $currentVersion = $matches[1]
    $newVersion = Increment-Version -Version $currentVersion -Type $VersionType

    Write-Host "🏷️ Current version: $currentVersion"
    Write-Host "🆕 New version: $newVersion"

    # Update version in Cargo.toml
    $updatedContent = $cargoContent -replace "version\s*=\s*\"$currentVersion\"", "version = \"$newVersion\""

    if (-not $WhatIf) {
        # Ask for confirmation
        $confirmation = Read-Host "Are you sure you want to bump version from $currentVersion to $newVersion? (y/n)"
        if ($confirmation -ne "y") {
            Write-Host "Version bump cancelled."
            exit 0
        }
        Set-Content -Path "Cargo.toml" -Value $updatedContent -NoNewline
    } else {
        Write-Host "[WhatIf] Would update Cargo.toml with new version: $newVersion"
    }

    # Run cargo check to update Cargo.lock
    Write-Host "🔍 Running cargo check..."
    if (-not $WhatIf) {
        cargo check
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Cargo check failed!"
            # Revert changes
            git checkout -- Cargo.toml
            exit 1
        }
    } else {
        Write-Host "[WhatIf] Would run 'cargo check'"
    }

    # Git operations
    Write-Host "📝 Committing changes..."
    if (-not $WhatIf) {
        git add Cargo.toml Cargo.lock
        git commit -m "chore: bump version to $newVersion"

        Write-Host "🏷️ Creating tag..."
        git tag -a "v$newVersion" -m "Version $newVersion"

        Write-Host "⬆️ Pushing changes..."
        git push origin main
        git push origin "v$newVersion"
    } else {
        Write-Host "[WhatIf] Would commit and push version bump to $newVersion"
        Write-Host "[WhatIf] Would create and push tag v$newVersion"
    }

    if (-not $WhatIf) {
        Write-Host "✅ Successfully bumped version to $newVersion and pushed changes"
    }

    # Publish to crates.io
    Write-Host "📦 Publishing to crates.io..."
    if (-not $WhatIf) {
        cargo publish --allow-dirty
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Failed to publish to crates.io!"
            exit 1
        }
    } else {
        Write-Host "[WhatIf] Would publish version $newVersion to crates.io"
    }

    if (-not $WhatIf) {
        Write-Host "✅ Successfully published to crates.io"
    }
} else {
    Write-Error "Could not find version in Cargo.toml"
    exit 1
}