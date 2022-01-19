# tinyinstallj

A tiny installer generator for Java programs, primarily designed for use on GUI programs (on Windows it has some odd behaviors with console programs).

The installer generator is written in Python and it builds a statically linked standalone Rust executable.

The generated executables are generally under 10MB in size. It can produce installers for Windows and Linux, and the Linux installers probably work on Mac.

The `.jar` file is not embedded in the installer, the installer instead is only provided with the URL of a jar file to download and install.

When running an installer, it also creates a batch/shell script to uninstall the program.

## This program is not ready for production yet

It's still under initial development and the first stable version is still some way off. 

## Usage (planned usage)

First, make sure you have Python >= 3.7, pip and a recent version of Rust + Cargo installed (>= 1.56).

Add this repository to your project as a submodule:
```
git submodule add xxx (todo: figure out url & release scheme)
```
It's recommended you read a bit about git submodules if you haven't already. When git pulling and cloning, you will need to use the `--recurse-submodules` flag to ensure you actually get the submodule.

Install required Python packages for the installer to run:
```
python3 -m pip install -r tinyinstallj/requirements.txt
```

Create a configuration file `tinyinstallj.json` in the root directory of your project. Here is an example of `tinyinstallj.json` from one of my other projects:
```
{
    "full_program_name": "Weather by ThatCoolCoder",
    "simple_program_name": "tccweather",
    "is_console_app": false,
    "min_java_version": "17",
    "jar_file_url": "https://github.com/ThatCoolCoder/weather/releases/download/v1.1.1/weather-1.1.1.jar",
    "icon_url": "https://raw.githubusercontent.com/ThatCoolCoder/weather/main/src/main/resources/icon.png",
    "icon_file_extension": ".png"
}
```
This example is for a standard/point release scheme, but it could easily be adapted to a rolling release.

Then to generate an installer, run `python3 tinyinstallj/create_installer.py`. By default this builds an optimized production build. Call with the `-d` flag to build a debug build, which takes less time. The target platform for an installer will be the same as the one generating the installer; i.e if you use a Windows machine to generate an installer, you will get a Windows executable. Cross compilation might be possible in the future, this depends if Rust/Cargo supports it.

## Roadmap

#### Before 1st release:

- Create .lnk files on windows
- On Linux, detect if it's being run not in a terminal and open it in a terminal (just needs debugging)
- On Linux, make pin to taskbar work - currently pins java installation which is not right

#### Further in future:

- On Windows, find how to update the PATH without rebooting, or at least only tell people that they need to reboot when PATH was changed (which only occurs on the first installation of a tinyinstallj program)
- Option to add to desktop
- Add more customisability to installer experience
- (maybe) more granularity about minimum Java version - currently only allows setting of major version
- Potentially switch to a GUI with fltk or something

## Info for devs

This section is not complete and will be extended later.

#### How values are passed to the installer

To pass the values of program name, .jar file url, etc to the executable, they're read from `tinyinstallj.json` and inserted into the template `config.rs.in` which is then saved to `config.rs`. This is the main purpose of `create_installer.py`.

#### How the installed app is run

The ideal would arguably to have just the .jar file, which would be placed on PATH. However, this doesn't work because:
- You probably can't set the icon of a .jar so it will look ugly and sad
- Some systems might not be configured to run a .jar with Java when double clicked
- It won't be runnable from the command line, unless on Unix you use a shebang, but that can't be added to a binary file, I believe.
- Unless you literally place the .jar on the user's desktop, it won't be runnable without searching through folders.

To make installed apps usable from both the GUI and the command line, we create 2 files:
- a runner script (batch on Windows, shell with no extension on Unix)
- a desktop file (.lnk on Windows, .desktop on Unix, both of which are WIPs)

The runner script is located in the same dir as the .jar (although it will work from anywhere) and because that dir is on PATH, it can be run from anywhere just using the program name. On Windows, because it's a batch file, a terminal opens when the file is run, which is bad. To combat this, we use `START /B javaw` to run a windowless JRE outside of the current session, so the window closes very quickly and is not an issue. On Linux this isn't an issue because it's a far superior and better thought out OS.

The desktop file points to the runner script. I don't think it can point to the .jar as you would need to pass the args `-jar filenamehere` to Java and .lnk files are probably too limited for that. This file is then moved to the relevant location.