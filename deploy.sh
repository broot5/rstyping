#!/bin/bash

git checkout master
cargo web deploy --release
mv target/deploy/* ./
git add index.html style.css rstyping.js rstyping.wasm
git commit -m "updated on $(date)"
git checkout dev
