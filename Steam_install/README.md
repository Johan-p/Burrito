# Install Guild Wars 2 on steam.

 > If using non-steam account use the following in your steam launch options to get the option to add your account details
```
add %command% -provider Portal.
```



# Launch a game, close it and locate a proton prefix.

```
    echo /home/$(whoami)/.local/share/Steam/steamapps/compatdata/1284210/pfx
```

# Download Burrito and extract
> ``` cd /home/$USER/ ```

> ``` Mkdir Executables ```

>  move files to Executables folder and extract them

# Edit the run_burrito_link.sh file and Make it executable. 
> ```chmod u+x un_burrito_link.sh```

> Example below is with ge-proton

https://github.com/GloriousEggroll/proton-ge-custom

> Example included in files is for proton 8

https://github.com/ValveSoftware/Proton


```
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

```


# Start Guild Wars 2

Run compositor(if not already running) to allow overlay transparency. I'm using empty config picom.
Run run_burrito_link.sh and move window over Guild Wars 2 if it doesn't get attached on multi monitor setup.
Load parsed markers from .json file. I'm using i3wm therefore Guild Wars 2 window gets tiled and its in windowed mode, Burrito window is set to floating and automatically resizes to Guild Wars 2 window resolution. In case of Guild Wars 2 window freezing adjust i3wm gap ±1-3px

# Thanks to fmazu for these awesome instructions and figuring it all out
