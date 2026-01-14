# Validate all Terraform modules in the workspace
# Usage: ./validate_all.ps1

$root = Get-Location
$searchPath = if (Test-Path "iac") { "iac" } else { "." }
$modules = Get-ChildItem -Path $searchPath -Recurse -Filter "main.tf" | 
Where-Object { $_.DirectoryName -notmatch "\\.terraform" } |
Select-Object -ExpandProperty DirectoryName | Get-Unique

$failed = @()
$passed = 0

foreach ($module in $modules) {
    Write-Host "Validating module: $module" -ForegroundColor Cyan
    Set-Location $module
    
    # Initialize implementation (backend=false to avoid remote state config)
    terraform init -backend=false -no-color > $null 2>&1
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  -> Init Failed" -ForegroundColor Red
        $failed += $module
        continue
    }
    
    # Validate
    terraform validate -no-color
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  -> Valid" -ForegroundColor Green
        $passed++
    }
    else {
        Write-Host "  -> Invalid" -ForegroundColor Red
        $failed += $module
    }
}

Set-Location $root

Write-Host "`nSummary:" -ForegroundColor White
Write-Host "Passed: $passed" -ForegroundColor Green
Write-Host "Failed: $($failed.Count)" -ForegroundColor Red

if ($failed.Count -gt 0) {
    Write-Host "`nFailed Modules:" -ForegroundColor Red
    $failed | ForEach-Object { Write-Host "- $_" }
    exit 1
}
else {
    exit 0
}
