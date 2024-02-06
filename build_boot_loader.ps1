Param([Bool]$Debug)
if ($Debug) {
	cargo build -p dragon_os_boot_loader
} else {
	cargo build -p dragon_os_boot_loader --release
}

$C_CUR_PATH = Split-Path $MyInvocation.MyCommand.Path
$C_VDISK_FILE = [IO.Path]::GetFullPath( (Join-Path $C_CUR_PATH ".\dragon_os.vhdx") )

Mount-DiskImage $C_VDISK_FILE

$driveLetter = (Get-DiskImage $C_VDISK_FILE | Get-Disk | Get-Partition | Get-Volume).DriveLetter

if (!(Test-Path ($driveLetter + ":\EFI"))) {
	New-Item -Path ($driveLetter + ":\") -Name "EFI" -ItemType "directory"
}
if (!(Test-Path ($driveLetter + ":\EFI\BOOT"))) {
	New-Item -Path ($driveLetter + ":\EFI") -Name "BOOT" -ItemType "directory"
}
if ($Debug) {
	Copy-Item ".\target\x86_64_dragon-uefi\debug\dragon_os_boot_loader.efi" -Recurse -Destination ($driveLetter + ":\EFI\BOOT\BOOTX64.EFI")
} else {
	Copy-Item ".\target\x86_64_dragon-uefi\release\dragon_os_boot_loader.efi" -Recurse -Destination ($driveLetter + ":\EFI\BOOT\BOOTX64.EFI")
}

DisMount-DiskImage $C_VDISK_FILE