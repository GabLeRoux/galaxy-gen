#!/usr/bin/env bash

# setup git
git init
git config --global user.email "lynncyrin@gmail.com"
git config --global user.name "[[ BOT ]] Lynn Cyrin"
git config --global pull.rebase true
git remote remove origin
git remote add origin https://${GITHUB_API_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git

# setup deploy branch
git pull origin
git checkout deploy
git pull origin deploy

# do main work
make build-wasm

# commit and push changes
git add .
git commit -m "[[ BOT ]] wasm build :: ${TRAVIS_BUILD_NUMBER}"
git pull origin deploy
git push origin deploy:deploy
