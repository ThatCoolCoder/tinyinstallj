# tinyinstallj

A tiny installer generator for Java programs, primarily designed for use on GUI programs. It can work with command line programs but things might get a bit weird on some OS configurations (testing needed).

The installer generator is written in Python and it builds a statically linked standalone Rust executable that should run pretty much anywhere.

The generated executables are generally under 10MB in size. It can produce installers for Windows and Linux. It can probably also generate installers for Mac, but these are simply recompilations of the Linux ones and might not have full system integration (someone buy me a mac so I can test it!).

The `.jar` file (and your program's icon) are embedded in the produced executable.

When running an installer, it also creates a batch/shell script to uninstall the program.

## Usage

First, make sure you have Python >= 3.7, pip and a recent version of Rust + Cargo installed (>= 1.56).

Add this repository to your project as a submodule:
```
git submodule add -b v2 https://github.com/ThatCoolCoder/tinyinstallj
```
tiyinstallj uses a rolling release - stable branches are of the form vN. It's recommended you read a bit about git submodules if you haven't already. When git pulling and cloning, you will need to use the `--recurse-submodules` flag to ensure you actually get the submodule.

Install required Python packages for the installer to run:
```
python3 -m pip install -r tinyinstallj/requirements.txt
```

Create a configuration file `tinyinstallj.json` in the root directory of your project. Below is an example of `tinyinstallj.json` from one of my other projects. **Read the section below titled [tinyinstallj.json](#tinyinstallj.json) for details on valid values for the fields.**
```
{
    "full_program_name": "Weather by ThatCoolCoder",
    "simple_program_name": "tccweather",
    "is_console_app": false,
    "min_java_version": 17,
    "jar_path": "target/weather-1.1.1-with-dependencies.jar",
    "icon_path": "src/main/resources/icon.ico"
}
```

Then to generate an installer, run `python3 tinyinstallj/create_installer.py`. By default this builds an optimized production build. Call with the `-d` flag to build a debug build, which takes less time. The target platform for an installer will be the same as the one generating the installer; i.e if you use a Windows machine to generate an installer, you will get a Windows executable.

Cross compilation is possible but difficult and it's probably just easier to get a VM. First install your target platform using the instructions from [https://rust-lang.github.io/rustup/cross-compilation.html](https://rust-lang.github.io/rustup/cross-compilation.html). You'll need to have a C toolchain set up to compile to that platform too. If you manage to get that working then you can call `python target/create_installer.py -p <your platform>`. Call `create_installer.py` with the `-h` flag to see a list of supported platforms

#### tinyinstallj.json

A JSON file containing information about your program. Some fields are optional.

- `full_program_name` (string). Name of your program as displayed to users. Can contain letters, numbers, spaces + `()-_=+`.
- `simple_program_name` (string). How your program is run from the command line. Can contain letters, numbers + `-_`.
- `jar_path` (string). Path (relative to `tinyinstall.json`) of your `.jar` file.
- `icon_path` (string). Path to an icon file for your program to use on the desktop etc. **This icon must be a .ico or it will not display on Windows**.
- `is_console_app` (bool, optional, defaults to false). Doesn't actually do much. I think it provides a value to the Linux desktop file generator. Just set it to false.
- `min_java_version` (int, optional, defaults to 17). Minimum major version of Java required for your program to run. For newer versions of Java (>= SE 9), this corresponds to the major version number (eg **17**.0.1). For older versions, this corresponds to the minor version number (eg 1.**7**.5). The Java release numbering is annoyingly inconsistent.

## Roadmap

#### No set date:

- Windows: Robustness if `C:\Windows\System32\WindowsPowershell\v1.0\powershell.exe` doesn't exist.
- Windows: find how to update the PATH without rebooting, or at least only tell people that they need to reboot when PATH was changed (which only occurs on the first installation of a tinyinstallj program)
- Option to add shortcut to desktop
- Windows: better uninstaller integration with desktop
- Option overrrides in `tinyinstallj.json` for different platforms.
- Proper support for OSX instead of just trying to use the Linux installer
- Add more customisability to installer experience
- Potentially add more granularity about minimum Java version - currently only allows setting of major version.
- Potentially switch to a GUI with fltk or something
- Publish to pypi?

## Info for devs

This section is not complete and will be extended later.

#### Release scheme of tinyinstallj

tinyinstallj uses a rolling release scheme. The `main` branch is for dev stuff and the branches of the form `vN` are for stable, production releases. Only increment the branch number when non-backwards compatible changes are made (equivalent of a major in semantic versioning) - minor and patch releases should use the same branch.

#### How values are passed to the installer

To pass the values of program name, `.jar` file url, etc to the executable, they're read from `tinyinstallj.json` and inserted into the template `config.rs.in` which is then saved to `config.rs`. This is the main purpose of `create_installer.py`. The rest of `create_installer.py` is just building the project and naming it suitably.

#### How the installed app is run

There are two ways to run the installed `.jar`.

To run it from the terminal and in general scripty ways, a runner script is created (`.bat` on windows, shell with no extension on unix) that calls java to run the `.jar`. That way it can be run as if it was a binary executable.

To integrate the app with the desktop environment, we also create a shortcut file (`.lnk` on Windows, `.desktop` on unix). It's surprisingly difficult to make a .lnk file as they use a weird binary format and no Rust libraries currently support writing them - instead we use a powershell-runner crate to run some powershell commands. On Windows, this file is saved to `%appdata%\Roaming\Microsoft\Windows\Start Menu\Programs\` and on unix it's saved to `/usr/share/applications/`.
