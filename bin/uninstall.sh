#!/bin/bash

APP_NAME="kek"
INSTALL_DIR="/usr/local/bin"

if [ -f "$INSTALL_DIR/$APP_NAME" ]; then
    echo "Uninstalling the application..."
    sudo rm "$INSTALL_DIR/$APP_NAME"

    if [ $? -eq 0 ]; then
        echo "$APP_NAME has been uninstalled successfully."
    else
        echo "Failed to uninstall $APP_NAME."
        exit 1
    fi
else
    echo "$APP_NAME is not installed in $INSTALL_DIR."
fi
