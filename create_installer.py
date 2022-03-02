import json
import os
import stat
import argparse
from dataclasses import dataclass

from dataclasses_json import dataclass_json

JSON_CONFIG_FILE = 'tinyinstallj.json'
RUST_CONFIG_IN_FILE = 'src/tinyinstallj/config.rs.in'
RUST_CONFIG_OUT_FILE = 'src/tinyinstallj/config.rs'

@dataclass_json
@dataclass
class Config:
    full_program_name: str # Full name of the program, used for display. EG David's Fantastic Frobnicator
    simple_program_name: str # Name of the program without any spaces or punctuation, used for filenames. EG davids_fantastic_frobnicator
    jar_path: str
    icon_path: str
    is_console_app: bool = False
    min_java_version: int = 17

def read_json_config():
    with open(JSON_CONFIG_FILE, 'r') as f:
        file_content = f.read()
        config = Config.from_json(file_content)
        config.jar_path = os.path.relpath(config.jar_path, os.path.dirname(RUST_CONFIG_IN_FILE)).replace('\\', '\\\\')
        config.icon_path = os.path.relpath(config.icon_path, os.path.dirname(RUST_CONFIG_IN_FILE)).replace('\\', '\\\\')
        return config

def create_rust_config(config: Config, base_directory: str):
    with open(os.path.join(base_directory, RUST_CONFIG_IN_FILE), 'r') as f:
        config_template = f.read()
    
    with open(os.path.join(base_directory, RUST_CONFIG_OUT_FILE), 'w+') as f:
        # Have to pass is_console_app separately as python auto bool-to-string
        # results in first letter capitalized, which breaks rust.
        # Because python doesn't support complex expressions in f-strings,
        # we therefore have to do the conversion here.
        f.write(config_template.format(config=config,
            is_console_app=str(config.is_console_app).lower()))

def build_installer(config: Config, debug: bool, base_directory: str):
    os.chdir(base_directory)

    if debug:
        exit_status = os.system('cargo build')
    else:
        exit_status = os.system('cargo build --release')
    if exit_status:
        print('Failed to build cargo')
        rust_diagnostics()

    # Rename output file
    if debug:
        output_dir = os.path.join('target', 'debug')
    else:
        output_dir = os.path.join('target', 'release')
    old_cwd = os.getcwd()
    os.chdir(output_dir)

    if os.path.exists('tinyinstallj'):
        output_name = f'{config.simple_program_name}-installer'
        os.replace('tinyinstallj', output_name)
    elif os.path.exists('tinyinstallj.exe'):
        output_name = f'{config.simple_program_name}-installer.exe'
        os.replace('tinyinstallj.exe', output_name)
    os.chdir(old_cwd)

    return os.path.join(output_dir, output_name)

def rust_diagnostics():
    print('This is probably an error related to Rust.')
    print('Make sure you have a recent Rust version installed (>= 1.50)')
    print('Try deleting everything in the build directory.')
    print('If that doesn\'t work, then create an issue at https://github.com/ThatCoolCoder/tinyinstallj/issues')
    quit(1)

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Create an installer for a Java program')
    parser.add_argument('-d', '--debug', help='Enable debug mode', action='store_true')
    args = parser.parse_args()

    base_directory, _ = os.path.split(__file__)
    old_cwd = os.getcwd()

    print(f'-- Reading config from {JSON_CONFIG_FILE}')
    config = read_json_config()
    print(f'-- Writing config to {RUST_CONFIG_OUT_FILE}')
    create_rust_config(config, base_directory)
    print(f'-- Building installer')
    output_path = build_installer(config, args.debug, base_directory)
    output_path = os.path.relpath(os.path.abspath(output_path), old_cwd)
    print(f'-- Built installer to {output_path}')