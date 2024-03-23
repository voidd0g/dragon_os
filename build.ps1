Param([Bool]$Debug)

Set-Location .\boot_loader

if ($Debug) {
	cargo build           --target=.\rust_target_dragon_boot_loader.json
} else {
	cargo build --release --target=.\rust_target_dragon_boot_loader.json
}

if (!$?) {
    Write-Output ("Building boot loader failed with exit code " + $LASTEXITCODE.ToString() + ".")
	Set-Location ..\
	exit 1
}

Set-Location ..\

Set-Location .\kernel

if ($Debug) {
	cargo build           --target=.\rust_target_dragon_kernel.json
} else {
	cargo build --release --target=.\rust_target_dragon_kernel.json
}

if (!$?) {
    Write-Output ("Building kernel failed with exit code " + $LASTEXITCODE.ToString() + ".")
	Set-Location ..\
	exit 2
}

Set-Location ..\
exit 0