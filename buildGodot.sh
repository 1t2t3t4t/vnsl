#!/bin/zsh

cargo build -p vnsl_godot
cargo build --release -p vnsl_godot

ENVS=(release debug)

for env in "${ENVS[@]}"; do
    mkdir -p ./integrations/godot/addons/godot_vnsl/bin/$env/

    DEST=./integrations/godot/addons/godot_vnsl/bin/$env/libvnsl_godot.dylib
    SRC=./target/$env/libvnsl_godot.dylib
    rm $DEST
    cp $SRC $DEST
done

echo Done
