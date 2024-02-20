Param([Bool]$Debug)

$C_CUR_PATH = Split-Path $MyInvocation.MyCommand.Path
$C_VDISK_FILE = [IO.Path]::GetFullPath( (Join-Path $C_CUR_PATH "..\dragon_os_emulator\dragon_os_emulator.vhd") )

Mount-DiskImage $C_VDISK_FILE

$driveLetter = (Get-DiskImage $C_VDISK_FILE | Get-Disk | Get-Partition | Get-Volume).DriveLetter

if (!(Test-Path ($driveLetter + ":\EFI"))) {
	New-Item -Path ($driveLetter + ":\") -Name "EFI" -ItemType "directory"
}
if (!(Test-Path ($driveLetter + ":\EFI\BOOT"))) {
	New-Item -Path ($driveLetter + ":\EFI") -Name "BOOT" -ItemType "directory"
}
if ($Debug) {
	Copy-Item ".\boot_loader\target\rust_target_dragon_boot_loader\debug\dragon_os_boot_loader.efi" -Recurse -Destination ($driveLetter + ":\EFI\BOOT\BOOTX64.EFI")
} else {
	Copy-Item ".\boot_loader\target\rust_target_dragon_boot_loader\release\dragon_os_boot_loader.efi" -Recurse -Destination ($driveLetter + ":\EFI\BOOT\BOOTX64.EFI")
}
if ($Debug) {
	Copy-Item ".\kernel\target\rust_target_dragon_kernel\debug\dragon_os_kernel.elf" -Recurse -Destination ($driveLetter + ":\KERNEL.ELF")
} else {
	Copy-Item ".\kernel\target\rust_target_dragon_kernel\release\dragon_os_kernel.elf" -Recurse -Destination ($driveLetter + ":\KERNEL.ELF")
}

DisMount-DiskImage $C_VDISK_FILE