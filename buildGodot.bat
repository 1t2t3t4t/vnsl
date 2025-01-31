@echo off

cargo build -p vnsl_godot
cargo build --release -p vnsl_godot

setlocal enabledelayedexpansion
set ENVS=release debug

for %%E in (%ENVS%) do (
    mkdir "integrations\godot\addons\godot_vnsl\bin\%%E" 2>nul
    set DEST=integrations\godot\addons\godot_vnsl\bin\%%E\vnsl_godot.dll
    set SRC=target\%%E\vnsl_godot.dll
    if exist !DEST! del !DEST!
    copy !SRC! !DEST!
)

echo Done
