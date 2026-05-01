use std::process::Command;
use std::str;

//I assume someone trying to replace rm with this would also have findmnt from coreutils installed
//If not uh... loop through /proc/mounts to find the mount root
pub fn find_mount_root(path: &str) -> Option<String> {
    let output = Command::new("findmnt")
        .arg("--output")
        .arg("TARGET")
        .arg("--noheadings")
        .arg("--target")
        .arg(format!("{}", path))
        .output()
        .expect("failed to find mount root");
    match str::from_utf8(&output.stdout) {
        Ok(s) => Some(s.trim().to_string()),
        Err(_) => Some("/".to_string()), //assume root if findmnt fails
    }
}
