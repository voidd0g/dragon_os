Param([Bool]$Debug)
if ($Debug) {
	cargo build --target x86_64-unknown-uefi
} else {
	cargo build --release --target x86_64-unknown-uefi
}

$C_CUR_PATH = Split-Path $MyInvocation.MyCommand.Path
$C_VDISK_FILE = [IO.Path]::GetFullPath( (Join-Path $C_CUR_PATH ".\dragon_os.vhdx") )

Mount-DiskImage $C_VDISK_FILE -PassThru

$driveLetter = (Get-DiskImage $C_VDISK_FILE | Get-Disk | Get-Partition | Get-Volume).DriveLetter

if (!(Test-Path ($driveLetter + ":\EFI"))) {
	New-Item -Path ($driveLetter + ":\") -Name "EFI" -ItemType "directory"
}
if (!(Test-Path ($driveLetter + ":\EFI\BOOT"))) {
	New-Item -Path ($driveLetter + ":\EFI") -Name "BOOT" -ItemType "directory"
}
if ($Debug) {
	Copy-Item ".\target\x86_64-unknown-uefi\debug\dragon_os.efi" -Recurse -Destination ($driveLetter + ":\EFI\BOOT\BOOTX64.EFI")
} else {
	Copy-Item ".\target\x86_64-unknown-uefi\release\dragon_os.efi" -Recurse -Destination ($driveLetter + ":\EFI\BOOT\BOOTX64.EFI")
}

DisMount-DiskImage $C_VDISK_FILE