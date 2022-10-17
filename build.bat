@echo off
rustc helper.rs -o ./windows/helper.exe -C lto -C panic=abort -C opt-level=s
