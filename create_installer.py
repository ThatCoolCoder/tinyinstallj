import json
import os
import stat
import argparse
from dataclasses import dataclass

from dataclasses_json import dataclass_json

DEFAULT_JSON_CONFIG_FILE = 'tinyinstallj.json'
RUST_CONFIG_IN_FILE = 'src/tinyinstallj/config.rs.in'
RUST_CONFIG_OUT_FILE = 'src/tinyinstallj/config.rs'
RUST_KEYBOARD_INTERRUPT_ERROR_CODE = 2

@dataclass_json
@dataclass
class Config:
    # see README.md for information about these fields
    full_program_name: str 
    icon_path: str
    jar_path: str
    simple_program_name: str

    is_console_app: bool = False
    create_desktop_shortcut: bool = True
    jvm_arguments: str = ""
    min_java_version: int = 17
    welcome_text: str = ""

def read_json_config(config_file_path: str = DEFAULT_JSON_CONFIG_FILE):
    with open(config_file_path, 'r') as f:
        file_content = f.read()
        config = Config.from_json(file_content)
        config.jar_path = os.path.relpath(
            os.path.relpath(config.jar_path, os.path.dirname(config_file_path)),
            os.path.dirname(RUST_CONFIG_OUT_FILE))
        config.icon_path = os.path.relpath(
            os.path.relpath(config.icon_path, os.path.dirname(config_file_path)),
            os.path.dirname(RUST_CONFIG_OUT_FILE))
        return config

def create_rust_config(config: Config, base_directory: str):
    with open(os.path.join(base_directory, RUST_CONFIG_IN_FILE), 'r') as f:
        config_template = f.read()
    
    with open(os.path.join(base_directory, RUST_CONFIG_OUT_FILE), 'w+') as f:
        # Have to pass these bools separately as python auto bool-to-string
        # results in first letter capitalized, which breaks rust.
        # Because python doesn't support complex expressions in f-strings,
        # we therefore have to do the conversion here.
        f.write(config_template.format(config=config,
            is_console_app=str(config.is_console_app).lower(),
            create_desktop_shortcut=str(config.create_desktop_shortcut).lower()))

def build_installer(config: Config, base_directory: str, debug: bool = False, target: str = None ):
    os.chdir(base_directory)

    command = 'cargo build'
    if not debug:
        command += ' --release'
    if target is not None:
        command += f' --target={target}'
    exit_status = os.system(command)

    if exit_status == RUST_KEYBOARD_INTERRUPT_ERROR_CODE:
        print('\n-- Build cancelled')
        quit()
    elif exit_status > 0:
        print('Failed building executable.')
        rust_diagnostics()

    output_dir = 'target'
    if target is not None:
        output_dir = os.path.join(output_dir, target)
        
    if debug:
        output_dir = os.path.join(output_dir, 'debug')
    else:
        output_dir = os.path.join(output_dir, 'release')

    old_cwd = os.getcwd()
    os.chdir(output_dir)

    if os.path.exists('tinyinstallj'):
        # Linux executables of course don't need an extension but it's good to put one
        # so that confused people can search it up and know what to do.
        # This is especially important since launching an executable directly from a browser isn't allowed in many distros
        output_name = f'{config.simple_program_name}-installer.x86_64'
        os.replace('tinyinstallj', output_name)
    elif os.path.exists('tinyinstallj.exe'):
        output_name = f'{config.simple_program_name}-installer.exe'
        os.replace('tinyinstallj.exe', output_name)
    os.chdir(old_cwd)

    return os.path.join(output_dir, output_name)

def rust_diagnostics():
    print('Make sure you have a recent Rust version installed (>= 1.50)')
    print('Try deleting everything in the build directory.')
    print('If that doesn\'t work, then create an issue at https://github.com/ThatCoolCoder/tinyinstallj/issues')
    quit(1)

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Create an installer for a Java program')
    parser.add_argument('-d', '--debug', help='Enable debug mode', action='store_true')
    parser.add_argument('-t', '--target', help='Cross-compilation target', action='store', default=None)
    parser.add_argument('-c', '--config_path', help='Specify a custom configuration file path',
        action='store', default=DEFAULT_JSON_CONFIG_FILE)
    args = parser.parse_args()

    base_directory, _ = os.path.split(__file__)
    old_cwd = os.getcwd()

    print(f'-- Reading config from {args.config_path}')
    config = read_json_config(args.config_path)
    print(f'-- Writing config to {RUST_CONFIG_OUT_FILE}')
    create_rust_config(config, base_directory)
    print(f'-- Building installer')
    output_path = build_installer(config, base_directory, args.debug, args.target)
    output_path = os.path.relpath(os.path.abspath(output_path), old_cwd)
    print(f'-- Built installer to {output_path}')