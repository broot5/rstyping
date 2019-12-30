#!/bin/bash
git checkout gh-pages
git merge master
cargo web deploy --release
mv target/deploy/* ./
git add index.html style.css rstyping.js rstyping.wasm
git commit -m "updated on $(date)"
git push origin gh-pages
git checkout dev
