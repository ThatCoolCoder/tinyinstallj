import json
import os
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

def build_installer(config: Config):

    # Actually build it
    exit_status = os.system('cargo build')
    if exit_status:
        print('Failed to build cargo')
        rust_diagnostics()

    # Rename output file
    # os.chdir('bin')
    # if os.path.exists('tinyinstallj'):
    #     os.rename('tinyinstallj', f'{config.simple_program_name}-installer')
    # elif os.path.exists('tinyinstallj.exe'):
    #     os.rename('tinyinstallj.exe', f'{config.simple_program_name}-installer.exe')

def rust_diagnostics():
    print('This is probably an error related to Rust.')
    print('Make sure you have a recent Rust version installed (>= 1.50)')
    print('Try deleting everything in the build directory.')
    print('If that doesn\'t work, then contact ThatCoolCoder')
    quit(1)

if __name__ == '__main__':
    config = read_json_config()
    create_rust_config(config)
    build_installer(config)
    print('Successfully built installer')