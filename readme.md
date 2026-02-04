# CHRYOS

### Documentation
[UART](
https://github.com/raspberrypi/documentation/blob/master/documentation/asciidoc/computers/configuration/uart.adoc)

#### To Debug
```
build+run_gdb.ps1
util\gdb-multiarch\gdb-multiarch.exe .\target\aarch64-unknown-none\debug\chryos
```
#### To get rid of Rust Analyzer's false positive on the panic handler:
create "./vscode/settings.json" with contents:
```
{
    "rust-analyzer.checkOnSave.allTargets": false,
    "rust-analyzer.cargo.target": "aarch64-unknown-none"
}
```