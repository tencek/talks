$baseUrl = "http://localhost:12345/hashdos"
$paramFile = Join-Path -Path $PSScriptRoot -ChildPath "test-data\request-10000-ok.txt"
$params = Get-Content -Path $paramFile -Raw
$url = "$baseUrl`?$params"
$jobCount = 1

$jobs = for ($i = 1; $i -le $jobCount; $i++) {
    Start-Job -ScriptBlock {
        param($u, $index)
        Write-Host "Starting request $index"
        $response = Invoke-RestMethod -Uri $u -Method GET
        return @{ Index = $index; Response = $response }
    } -ArgumentList $url, $i
}

# Collect and print results
$results = $jobs | ForEach-Object {
    $_ | Wait-Job
    $result = Receive-Job $_
    Remove-Job $_
    $result
}

# Display all responses
$results | ForEach-Object {
    Write-Output "Request $($_.Index):"
    Write-Output $($_.Response)
    Write-Output "----------------------"
}
