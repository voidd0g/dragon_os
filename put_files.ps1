Param([Bool]$Debug, [String]$PathToRoot)

if ((Test-Path $PathToRoot) -and ((Get-Item $PathToRoot).PsIsContainer)) {
    $PathToEFI = Join-Path $PathToRoot "\EFI"
    if (!((Test-Path $PathToEFI) -and ((Get-Item $PathToEFI).PsIsContainer))) {
        New-Item -Path $PathToRoot -Name "EFI" -ItemType "directory"
    }
    $PathToBOOT = Join-Path $PathToEFI "\BOOT"
    if (!((Test-Path $PathToBOOT) -and ((Get-Item $PathToBOOT).PsIsContainer))) {
        New-Item -Path $PathToEFI -Name "BOOT" -ItemType "directory"
    }
    if ($Debug) {
        Copy-Item ".\boot_loader\target\rust_target_dragon_boot_loader\debug\dragon_os_boot_loader.efi" -Recurse -Destination (Join-Path $PathToBOOT "\BOOTX64.EFI")

        Copy-Item ".\kernel\target\rust_target_dragon_kernel\debug\dragon_os_kernel.elf" -Recurse -Destination (Join-Path $PathToRoot "\KERNEL.ELF")
    } else {
        Copy-Item ".\boot_loader\target\rust_target_dragon_boot_loader\release\dragon_os_boot_loader.efi" -Recurse -Destination (Join-Path $PathToBOOT "\BOOTX64.EFI")

        Copy-Item ".\kernel\target\rust_target_dragon_kernel\release\dragon_os_kernel.elf" -Recurse -Destination (Join-Path $PathToRoot "\KERNEL.ELF")
    }
    exit 0
} else {
    Write-Output "Directory of given path was not found."
    exit 1
}