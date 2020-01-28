#!/bin/bash

git checkout gh-pages
git merge master
yarn install && parcel build index.html --public-url ./rstyping/dist
git add dist/
git commit -m "deployed on $(date)"
git push origin gh-pages
git checkout dev
