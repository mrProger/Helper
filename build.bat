@echo off
rustc helper.rs -C lto -C panic=abort -C opt-level=s