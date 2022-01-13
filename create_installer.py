import json
import os
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
    min_java_version: int
    jar_file_url: str

def read_json_config():
    with open(JSON_CONFIG_FILE, 'r') as f:
        file_content = f.read()
        config = Config.from_json(file_content)
        return config

def create_rust_config(config: Config):
    with open(RUST_CONFIG_IN_FILE, 'r') as f:
        config_template = f.read()
    
    with open(RUST_CONFIG_OUT_FILE, 'w+') as f:
        f.write(config_template.format(config=config))

def build_installer(config: Config, debug: bool):
    if debug:
        exit_status = os.system('cargo build')
    else:
        exit_status = os.system('cargo build --release')
    if exit_status:
        print('Failed to build cargo')
        rust_diagnostics()

    # Rename output file
    if debug:
        output_dir = 'target/debug'
    else:
        output_dir = 'target/release'
    os.chdir(output_dir)

    if os.path.exists('tinyinstallj'):
        output_name = f'{config.simple_program_name}-installer'
        os.rename('tinyinstallj', output_name)
    elif os.path.exists('tinyinstallj.exe'):
        output_name = f'{config.simple_program_name}-installer.exe'
        os.rename('tinyinstallj.exe', output_name)

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

    print(f'-- Reading config from {JSON_CONFIG_FILE}')
    config = read_json_config()
    print(f'-- Writing config to {RUST_CONFIG_OUT_FILE}')
    create_rust_config(config)
    print(f'-- Building installer')
    output_path = build_installer(config, args.debug)
    print(f'-- Built installer to {output_path}')