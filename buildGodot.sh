#!/bin/zsh

cargo build -p vnsl_godot
cargo build --release -p vnsl_godot

mkdir -p ./integrations/godot/addons/godot_vnsl/bin/debug/
mkdir -p ./integrations/godot/addons/godot_vnsl/bin/release/

cp ./target/debug/libvnsl_godot.dylib ./integrations/godot/addons/godot_vnsl/bin/debug/libvnsl_godot.dylib
cp ./target/release/libvnsl_godot.dylib ./integrations/godot/addons/godot_vnsl/bin/release/libvnsl_godot.dylib

echo Done
