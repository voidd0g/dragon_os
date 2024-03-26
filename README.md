# dragon_os
An implementation of [MikanOS](https://github.com/uchan-nos/mikanos-build) in Rust language with some extra functions.

## Build
For windows user, execute build.ps1 like below.
The argument is to determine debug/release build.
```
.\build.ps1 $False
```

## Put Fies
Put the kernel output at '/'(root) and rename it to 'KERNEL.ELF'
Put the boot_loader output at '/EFI/BOOT/' and rename it to 'BOOTX64.EFI'
