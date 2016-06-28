#!/bin/bash

set -e

SOURCE_BRANCH="master"
PUBLISH_BRANCH="gh-pages"
BUILD_DIR=".travis/build_docs"
PUBLISH_SRC="target/doc"
PUBLISH_DEST="docs"

DEPLOY_KEY=".travis/deploy_key"
DEPLOY_KEY_ENC=".travis/deploy_key.enc"

BASE=`pwd`
REPO=`git config remote.origin.url`
REPO_PUSH=${REPO/https:\/\/github.com\//git@github.com:}
COMMIT=`git rev-parse --short HEAD`

if [ "$TRAVIS" != "true" ]; then
    echo "This script should only be run by Travis"
    exit 1
fi

if [ -z "ENCRYPTION_LABEL" ]; then
    echo "==> Error: Encryption label was not properly set up"
    exit 1
fi

if [ "$TRAVIS_PULL_REQUEST" != "false" -o "$TRAVIS_BRANCH" != "$SOURCE_BRANCH" ]; then
    echo "==> Not building docs for $COMMIT because it's not on $SOURCE_BRANCH"
    exit 0
fi



cargo doc --no-deps

if [ -d "./$BUILD_DIR" ]; then
    rm -rf "./$BUILD_DIR"
fi

mkdir -p "$BUILD_DIR"
git clone -b "$PUBLISH_BRANCH" "$REPO" "$BUILD_DIR"

rm -r "./$BUILD_DIR/$PUBLISH_DEST"
mv "./$PUBLISH_SRC" "./$BUILD_DIR/$PUBLISH_DEST"



cd "$BUILD_DIR"

git add .
if git -c user.name="Travis CI" -c user.email="<>" commit -m "Update docs for commit $COMMIT"; then
    NEW_DOC_COMMIT=`git rev-parse --short HEAD`
    echo "==> Updating docs for commit $COMMIT (new commit is $NEW_DOC_COMMIT)"
else
    echo "==> No changes to commit"
    exit 0
fi

cd "$BASE"

if [ -f "$DEPLOY_KEY" ]; then
    echo "==> NOTE: Decrypted deploy key was already found!"
elif [ -n "$ENCRYPTION_LABEL" -a -f "$DEPLOY_KEY_ENC" ]; then
    ENCRYPTED_KEY_VAR="encrypted_${ENCRYPTION_LABEL}_key"
    ENCRYPTED_IV_VAR="encrypted_${ENCRYPTION_LABEL}_iv"
    ENCRYPTED_KEY=${!ENCRYPTED_KEY_VAR}
    ENCRYPTED_IV=${!ENCRYPTED_IV_VAR}
    openssl aes-256-cbc -K "$ENCRYPTED_KEY" -iv "$ENCRYPTED_IV" -in "$DEPLOY_KEY_ENC" -out "$DEPLOY_KEY" -d
    chmod 600 "$DEPLOY_KEY"
else
    echo "==> Error: SSH deploy keys were not configured"
    exit 1
fi

eval `ssh-agent -s`
ssh-add "$DEPLOY_KEY"

cd "$BUILD_DIR"

git push "$REPO_PUSH" "$PUBLISH_BRANCH"
