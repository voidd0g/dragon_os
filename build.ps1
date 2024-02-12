Param([Bool]$Debug)

Set-Location .\boot_loader

if ($Debug) {
	cargo build           --target=.\rust_target_dragon_boot_loader.json
} else {
	cargo build --release --target=.\rust_target_dragon_boot_loader.json
}

Set-Location ..\

Set-Location .\kernel

if ($Debug) {
	cargo build           --target=.\rust_target_dragon_kernel.json
} else {
	cargo build --release --target=.\rust_target_dragon_kernel.json
}

Set-Location ..\