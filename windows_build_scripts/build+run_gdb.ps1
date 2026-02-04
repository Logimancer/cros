cross build --target=aarch64-unknown-none
Write-Output '****** Press CTRL-A x to exit ******'
c:\msys64\ucrt64\bin\qemu-system-aarch64 `
    -M virt `
    -S `
    -gdb tcp::1234 `
    -M raspi4b `
    -kernel target\aarch64-unknown-none\debug\cros `
    -nographic `
    -smp 4 `
    2> $null