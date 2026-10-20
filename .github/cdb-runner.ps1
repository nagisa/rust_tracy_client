# Cargo `runner` that executes a test binary under cdb, so that crashes that only reproduce in CI
# print stack traces of the faulting thread and of all other threads.
#
# Usage: CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUNNER="pwsh -NoProfile -File <path>\cdb-runner.ps1"
#
# Exits with the debuggee's exit code, or 1 if it crashed. A full dump of a crashed process is
# written to `$env:RUNNER_TEMP\dumps` (or `$env:TEMP\dumps` outside of GitHub Actions).

$ErrorActionPreference = 'Stop'

$exe = $args[0]
# Must be an array: splatting a plain string would pass it character by character.
[string[]]$testArgs = @($args | Select-Object -Skip 1)
$cdb = "${env:ProgramFiles(x86)}\Windows Kits\10\Debuggers\x64\cdb.exe"
if (-not (Test-Path $cdb)) {
    Write-Host "cdb-runner: cdb.exe not found at $cdb, running $exe directly"
    & $exe @testArgs
    exit $LASTEXITCODE
}

# Forward slashes, because cdb processes backslash escapes in quoted command strings (`D:\a\...`
# would get a BEL character in it).
$base = $(if ($env:RUNNER_TEMP) { $env:RUNNER_TEMP } else { $env:TEMP }) -replace '\\', '/'
$dumps = "$base/dumps"
$symbols = "$base/symbols"
$work = "$base/cdb-$([guid]::NewGuid())"
New-Item -ItemType Directory -Force -Path $dumps, $work | Out-Null

$startScript = "$work/start.txt"
$crashScript = "$work/crash.txt"
$exitScript = "$work/exit.txt"
$crashLog = "$work/crash.log"
$exitLog = "$work/exit.log"

# Runs at the initial breakpoint. First-chance exceptions are ignored, as they may well be handled;
# only second-chance (i.e. unhandled) exceptions run the crash script.
@"
.lines -e
sxd -c2 "`$`$<$crashScript" *
sxd -c2 "`$`$<$crashScript" av
sxd -c2 "`$`$<$crashScript" sov
sxd -c2 "`$`$<$crashScript" eh
sxe -c "`$`$<$exitScript" epr
g
"@ | Set-Content -Encoding ascii $startScript

# `.symfix+` adds Microsoft's symbol server, so that frames inside system DLLs (dbghelp.dll,
# ntdll.dll, ...) get names too. Our own symbols come from the PDBs next to the binaries.
@"
.logopen $crashLog
.echo crashed
.logclose
.echo ======================== cdb-runner: unhandled exception ========================
.lastevent
.symfix+ $symbols
.reload
.echo ------------------------ faulting thread ------------------------
.ecxr
kpn 100
.echo ------------------------ !analyze -v ------------------------
!analyze -v
.echo ------------------------ all threads ------------------------
~*kn 50
.dump /ma /u $dumps/crash.dmp
.echo =================================================================================
q
"@ | Set-Content -Encoding ascii $crashScript

# On a normal process exit, record the exit code (`.lastevent` prints `Exit process ..., code N`).
@"
.logopen $exitLog
.lastevent
.logclose
q
"@ | Set-Content -Encoding ascii $exitScript

& $cdb -lines -cf $startScript $exe @testArgs
$cdbExit = $LASTEXITCODE

if (Test-Path $crashLog) {
    exit 1
}
if ((Test-Path $exitLog) -and ((Get-Content -Raw $exitLog) -match 'Exit process [0-9a-fA-F]+:[0-9a-fA-F]+, code ([0-9a-fA-F]+)')) {
    $code = [Convert]::ToUInt32($Matches[1], 16)
    exit [BitConverter]::ToInt32([BitConverter]::GetBytes($code), 0)
}
Write-Host "cdb-runner: could not determine the exit code of $exe (cdb exited with $cdbExit)"
exit $(if ($cdbExit -ne 0) { $cdbExit } else { 1 })
