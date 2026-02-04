cross build --release --target=aarch64-unknown-none
Write-Output '****** Press CTRL-A x to exit ******'
c:\msys64\ucrt64\bin\qemu-system-aarch64 `
    -nographic `
    -M raspi4b `
    -kernel target\aarch64-unknown-none\release\cros `
    2> $null