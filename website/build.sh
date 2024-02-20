#!/bin/bash
# cp -r ./src/static dist
# cp -r ../spc-book/ais-spc-book-0.0.1/book dist 

# Expected directory from which the script should run
EXPECTED_DIR="/home/eab480/Code/adaptiveitsolution-organization/ais-spc-spark/website"

# Function to check the current working directory
check_directory() {
  CURRENT_DIR=$(pwd)
  if [ "$CURRENT_DIR" != "$EXPECTED_DIR" ]; then
    echo "Error: Script must be run from $EXPECTED_DIR"
    echo "Current directory is $CURRENT_DIR"
    exit 1
  fi
}

# Function to copy src/static to dist and spc-book to dist/book
copy_resources() {
  # Copy src/static to dist
  cp -r ./src/static dist
  if [ $? -ne 0 ]; then
    echo "Failed to copy src/static to dist"
    return 1
  fi

  # Copy spc-book to dist/book
  cp -r ../spc-book/ais-spc-book-0.0.1/book dist
  if [ $? -ne 0 ]; then
    echo "Failed to copy spc-book to dist/book"
    return 1
  fi

  echo "Resources copied successfully."
  return 0
}

# Check if the script is executed from the correct directory
check_directory

# Execute the function
copy_resources

# Check if the function succeeded
if [ $? -eq 0 ]; then
  echo "Operation completed successfully."
else
  echo "Operation failed."
fi

