#![allow(dead_code)]

use bytes::Bytes;

pub const FULL_PROGRAM_NAME: &str = "Weather by ThatCoolCoder";
pub const SIMPLE_PROGRAM_NAME: &str = "tccweather";
pub const IS_CONSOLE_APP: bool = false;
pub const MIN_JAVA_VERSION: i32 = 17;
pub const JAR_BYTES: Bytes = Bytes::from_static(include_bytes!("..\\..\\test_data\\weather.jar"));
pub const ICON_BYTES: Bytes = Bytes::from_static(include_bytes!("..\\..\\test_data\\icon.ico"));