# tinyinstallj

A tiny installer generator for Java programs, primarily designed for use on GUI programs. It can work with command line programs but things can get a bit weird on some OS configurations.

The installer generator is written in Python and it builds a statically linked standalone Rust executable.

The generated executables are generally under 10MB in size. It can produce installers for Windows and Linux, and the Linux installers probably work on Mac.

The `.jar` file is not embedded in the installer, the installer instead is only provided with the URL of a jar file to download and install.

When running an installer, it also creates a batch/shell script to uninstall the program.

## This program is not ready for production yet

It's still under initial development.

## Usage (planned)

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

Create a configuration file `tinyinstallj.json` in the root directory of your project. Below is an example of `tinyinstallj.json` from one of my other projects. **Note that there are quite specific requirements for each field so please read the subsection [tinyinstallj.json](#tinyinstallj.json) for details.**
```
{
    "full_program_name": "Weather by ThatCoolCoder",
    "simple_program_name": "tccweather",
    "is_console_app": false,
    "min_java_version": 17,
    "jar_url": "https://github.com/ThatCoolCoder/weather/releases/download/v1.1.1/weather-1.1.1`.jar`",
    "icon_url": "https://raw.githubusercontent.com/ThatCoolCoder/weather/main/src/main/resources/icon.ico",
}
```


Then to generate an installer, run `python3 tinyinstallj/create_installer.py`. By default this builds an optimized production build. Call with the `-d` flag to build a debug build, which takes less time. The target platform for an installer will be the same as the one generating the installer; i.e if you use a Windows machine to generate an installer, you will get a Windows executable. Cross compilation might be possible in the future, this depends if Rust/Cargo supports it.

#### tinyinstallj.json

A JSON file containing information about your program. All of the fields below are required; the installer generator will break if you miss some. The requirements for the fields below are not actually enforced by the generator, but **putting invalid values will prevent your installer from running correctly on some or all systems.**

- `full_program_name` (string). Name of your program as displayed to users. Can contain letters, numbers, spaces + `()-_=+`.
- `simple_program_name` (string). How your program is run from the command line. Can contain letters, numbers + `-_`.
- `is_console_app` (bool). Doesn't actually do much. I think it provides a value to the Linux desktop file generator. Just set it to false.
- `min_java_version` (int). Minimum major version of Java required for your program to run. For newer versions of Java (>= SE 9), this corresponds to the major version number (eg **17**.0.1). For older versions, this corresponds to the minor version number (eg 1.**7**.5). The Java release numbering is annoyingly inconsistent.
- `jar_url` (string). URL of the `.jar` file to download when installing your app. For a point release scheme, you can attach the `.jar` as an asset to a Github release or whatever than then you can modify this template `https://github.com/{user}/{repo}/releases/download/{tag}/{program}.jar`. For a rolling release, you could do something like `https://github.com/{user}/{repo}/blob/{distribution_branch}/program.jar`.
- `icon_url` (string). URL of the icon of your program. **This icon must be a .ico or it will not display on Windows**.

## Roadmap

#### Further in future:

- Windows: Robustness if `C:\Windows\System32\WindowsPowershell\v1.0\powershell.exe` doesn't exist
- On Windows, find how to update the PATH without rebooting, or at least only tell people that they need to reboot when PATH was changed (which only occurs on the first installation of a tinyinstallj program)
- Option to add shortcut to desktop
- On Windows, better uninstaller integration with desktop
- Proper support for OSX instead of just trying to use the Linux installer
- Add more customisability to installer experience
- Potentially add more granularity about minimum Java version - currently only allows setting of major version.
- Potentially switch to a GUI with fltk or something

## Info for devs

This section is not complete and will be extended later.

#### How values are passed to the installer

To pass the values of program name, `.jar` file url, etc to the executable, they're read from `tinyinstallj.json` and inserted into the template `config.rs.in` which is then saved to `config.rs`. This is the main purpose of `create_installer.py`.

#### How the installed app is run

There are two ways to run the installed `.jar`.

To run it from the terminal and in general scripty ways, a runner script is created (`.bat` on windows, shell with no extension on unix) that calls java to run the `.jar`. That way it can be run as if it was a binary executable.

To integrate the app with the desktop environment, we also create a shortcut file (`.lnk` on Windows, `.desktop` on unix). It's surprisingly difficult to make a .lnk file as they use a weird binary format and no Rust libraries currently support writing them - instead we use a powershell-runner crate to run some powershell commands. On Windows, this file is saved to `%appdata%\Roaming\Microsoft\Windows\Start Menu\Programs\` and on unix it's saved to `/usr/share/applications/`.