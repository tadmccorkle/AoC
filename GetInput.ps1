param(
	[Parameter(Mandatory)]
	[int]$Year,
	[Parameter(Mandatory)]
	[int]$Day
)

$exit = 0
Push-Location $PSScriptRoot

if (-not (Test-Path -Path ".\$Year\input")) {
	Write-Error "Invalid year provided."
	$exit = 1
} elseif ($Day -lt 1 -or $Day -gt 25) {
	Write-Error "Invalid day provided."
	$exit = 1
} elseif (-not (Test-Path -Path ".\session")) {
	Write-Error "No session ID file (.\session)."
	$exit = 1
} else {
	$session = Get-Content -Path .\session
	curl `
		--cookie "session=$session" "https://adventofcode.com/$year/day/$day/input" `
		> ".\$year\input\day$($Day.ToString("D2")).txt"
}

Pop-Location
exit $exit
