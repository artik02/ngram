#!/bin/bash

# MIT LICENSE
#
# Copyright 2024 artik02
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the “Software”), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
# of the Software, and to permit persons to whom the Software is furnished to do
# so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

set -e # Exit immediately if a command returns a non-zero value

# Get the version in the first argument
version=$1

# Verify the the version is given and if follows semantic versioning
if [[ -z "$version" || ! "$version" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
    echo "Error: The format of the version is not valid. Plase follow semantic versioning, like: v0.0.0 or v0.0.0-alpha"
    echo "Non-compliant version: '$version'"
    exit 1
fi

# Get the directory of the script
directory=$(realpath "$(dirname "$0")")

# Release directory used to get the binary files
release_dir="$directory/target/dx/ngram/release"
# Latest directory used to create the packages
latest_dir="$directory/target/latest"

# Function to clean the latest directory
clean_latest_dir() {
    echo "Cleaning the directory '$latest_dir'..."
    echo "  Deleting the directory if exists: \"rm -r '$latest_dir' 2>/dev/null || true\""
    rm -r "$latest_dir" 2>/dev/null || true  # Ignore error if not exists
    echo "  Creating the directory: \"mkdir -p '$latest_dir'\""
    mkdir -p "$latest_dir"
}

# Function to build and copy the release files for each platform
build_and_copy() {
    platform=$1
    opt_release=$2
    opt_latest=$3

    echo "  Starting compilation for the platform: '$platform'..."
    echo "      Command to compile: \"dx build --release --platform '$platform'\""
    if ! dx build --release --platform "$platform"; then
        echo "Error: The compilation failed for the platform: '$platform'"
        exit 1
    fi
    echo "  Copying the release files for the platform: '$platform' to '$latest_dir'..."
    echo "      Comando to copy: \"cp -r '$release_dir/$platform/app$opt_release' '$latest_dir/ngram-$platform-$version$opt_latest'\""
    cp -r "$release_dir/$platform/app$opt_release" "$latest_dir/ngram-$platform-$version$opt_latest"
}

# Function to compress the release files
compress_and_ext() {
    platform=$1
    command=$2
    extension=$3

    echo "  Compressing the release files for the platform: '$platform'..."
    echo "      Change to the latest directory: \"cd '$latest_dir'\""
    cd "$latest_dir"
    ls -l
    echo "      Command to compress: \"$command 'ngram-$platform-$version$extension' 'ngram-$platform-$version'\""
    $command "ngram-$platform-$version$extension" "ngram-$platform-$version"
    echo "      Deleting release files: \"rm -r 'ngram-$platform-$version'\""
    rm -r "ngram-$platform-$version"
    echo "      Returning to the previous directory: \"cd -\""
    cd -
}

# Function to generate the release for Linux
make_linux() {
    echo "Generating release for Linux..."
    build_and_copy "linux"
    compress_and_ext "linux" "tar cvf" ".tar.gz"
}

# Function to generate the release for Windows
make_windows() {
    echo "Generating release for Windows..."
    build_and_copy "windows"
    compress_and_ext "windows" "zip -r" ".zip"
}

# Function to generate the release for Android
make_android() {
    echo "Generating release for Android..."
    build_and_copy "android" "/app/build/outputs/apk/debug/app-debug.apk" ".apk"
}

# Main creation of the release
echo "--- Starting the creation of the release ---"
clean_latest_dir
# Generate a release for each platform
make_linux
make_windows
make_android
# Release creation completed
echo "Created the release $version successfully"
