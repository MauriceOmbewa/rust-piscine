#!/bin/bash

# Check for commit message argument
if [ -z "$1" ]; then
  echo "Usage: ./git_push.sh \"Your commit message here\""
  exit 1
fi

# Stage all changes
git add .

# Commit with the provided message
git commit -m "$1"

# Push to origin
echo "Pushing to origin..."
git push origin main

# Push to github
echo "Pushing to github..."
git push github main
