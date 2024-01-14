#!/bin/bash

# Get the current version from the latest Git tag
current_version=$(git describe --abbrev=0 --tags)

# Parse the version components.
major=$(echo "$current_version" | cut -d. -f1 | sed 's/v//g')
minor=$(echo "$current_version" | cut -d. -f2)
patch=$(echo "$current_version" | cut -d. -f3)

# Determine the release type based on command-line arguments
if [[ $# -eq 0 ]]; then
    release_type="patch"
elif [[ $1 == "minor" ]]; then
    release_type="minor"
elif [[ $1 == "major" ]]; then
    release_type="major"
else
    echo "Invalid release type. Usage: release.sh [patch|minor|major]"
    exit 1
fi

# Increment the version based on the release type
if [[ $release_type == "patch" ]]; then
    patch=$((patch + 1))
elif [[ $release_type == "minor" ]]; then
    minor=$((minor + 1))
    patch=0
elif [[ $release_type == "major" ]]; then
    major=$((major + 1))
    minor=0
    patch=0
fi

# Create the new version tag
new_version="v$major.$minor.$patch"
git tag "$new_version"
git push origin "$new_version"
