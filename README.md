# tinyinstallj

A tiny installer generator for Java programs, primarily designed for use on GUI programs (on Windows it has some odd behaviors with console programs).

The installer generator is written in Python and it builds a statically linked standalone Rust executable.

The generated executables are generally under 10MB in size. It can produce installers for Windows and Linux, and the Linux installers probably work on Mac.

The `.jar` file is not embedded in the installer, the installer instead is only provided with the URL of a jar file to download and install.

When running an installer, it also creates a batch/shell script to uninstall the program.

## Usage

#### The program is not usable yet but here is the planned usage:

First, make sure you have Python >= 3.7, pip and a recent version of Rust + Cargo installed (say >= 1.50).

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
    "min_java_version": "17",
    "jar_file_url": "https://github.com/ThatCoolCoder/weather/releases/download/v1.1.1/weather-1.1.1.jar"
}
```
This example is for a standard/point release scheme, but it could easily be adapted to a rolling release. 

Then to generate an installer, run `python3 tinyinstallj/create_installer.py`. By default this builds an optimized production build. Call with the `-d` flag to build a debug build, which takes less time. The target platform for an installer will be the same as the one generating the installer; i.e if you use a Windows machine to generate an installer, you will get a Windows executable. Cross compilation might be possible in the future, this depends if Rust/Cargo supports it.

## Roadmap

#### Before 1st release:

- Create .desktop files on Linux and .lnk files on windows
- Include icon in installation
- On Linux, detect if it's being run not in a terminal and open it in a terminal

#### Further in future:

- Add more customisability to installer experience
- (maybe) more granularity about minimum Java version - currently only allows setting of major version

## Info for devs

This section is not complete and will be extended later.

#### How values are passed to the installer

To pass the values of program name, .jar file url, etc to the executable, they're read from `tinyinstallj.json` and inserted into the template `config.rs.in` which is then saved to `config.rs`. This is the main purpose of `create_installer.py`.