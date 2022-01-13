// Check if the user is admin. Only does something on *nix systems, as checking for windows systems is covered by a manifest in build.rs

#[cfg(target_family = "unix")]
extern crate nix;

#[cfg(target_family = "unix")]
use nix::unistd::Uid;

#[cfg(target_family = "unix")]
pub fn is_admin() -> bool {
    // (Returns true if logged in as root or using sudo)
    return Uid::effective().is_root();
}

#[cfg(target_family = "windows")]
pub fn is_admin() -> bool {
    // We can assume the user is admin as the manifest forces it to be run with admin priveliges
    return true;
}