#!/bin/bash

APP_NAME="kek"
INSTALL_DIR="/usr/local/bin"

echo "Building the application..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi

echo "Installing the application..."
sudo cp "target/release/$APP_NAME" "$INSTALL_DIR"

if [ $? -eq 0 ]; then
    echo "$APP_NAME has been installed successfully."
else
    echo "Failed to install $APP_NAME."
    exit 1
fi
