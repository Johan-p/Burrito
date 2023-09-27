#!/bin/bash

USER=$(whoami)
PROTON="GE-Proton7-55"
PROTON_PATH="/home/$USER/.local/share/Steam/compatibilitytools.d/$PROTON/files/bin/wine"
GUILD_WARS_PATH="/home/$USER/.local/share/Steam/steamapps/common/Guild Wars 2"
GUILD_WARS_PREFIX_PATH="/home/$USER/.local/share/Steam/steamapps/compatdata/1284210/pfx"
# Environment variables
export DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1="1"
export __GL_SHADER_DISK_CACHE="1"
export __GL_SHADER_DISK_CACHE_PATH=$GUILD_WARS_PATH
export WINEDEBUG="-all"
export DXVK_LOG_LEVEL="none"
export WINEARCH="win64"
export WINE=$PROTON_PATH
export WINEPREFIX=$GUILD_WARS_PREFIX_PATH
export WINEESYNC="1"
export WINEFSYNC="1"
export DXVK_NVAPIHACK="0"
export DXVK_ENABLE_NVAPI="1"
export WINEDLLOVERRIDES="winemenubuilder="
export WINE_LARGE_ADDRESS_AWARE="1"
export TERM="xterm"

# Command
gamemoderun $PROTON_PATH "/home/$USER/Executables/Burrito_Linux/burrito_link.exe"
