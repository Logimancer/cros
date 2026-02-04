cross build --target=aarch64-unknown-none
c:\msys64\ucrt64\bin\qemu-system-aarch64 `
    -M raspi4b `
    -kernel target\aarch64-unknown-none\debug\cros `
    -serial telnet:localhost:4321,server,nowait