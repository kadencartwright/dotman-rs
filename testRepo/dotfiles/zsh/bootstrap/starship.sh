#!/bin/sh
BOOTSTRAP_DIR=~/.config/zsh/bootstrap
package_name='starship-git'

if $BOOTSTRAP_DIR/ensure_installed.sh $package_name; then
    exit 0
else
    $BOOTSTRAP_DIR/install_silent.sh $package_name
fi

