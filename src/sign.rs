use std::{error::Error, path::Path, process::Stdio};

pub fn remove_sign(exec_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let mut cmd = std::process::Command::new("/usr/bin/codesign");
    cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());
    cmd.arg("--remove-signature");
    cmd.arg(exec_path.as_ref());
    let result = cmd.spawn().unwrap().wait().unwrap();
    if result.success() {
        Ok(())
    } else {
        Err("failed to remove signature".into())
    }
}

pub fn sign(
    exec_path: impl AsRef<Path>,
    enhancement_file: Option<impl AsRef<Path>>,
) -> Result<(), Box<dyn Error>> {
    let mut cmd = std::process::Command::new("/usr/bin/codesign");
    cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());
    if let Some(enhancement_file) = enhancement_file {
        cmd.arg("--entitlements");
        cmd.arg(enhancement_file.as_ref());
    }
    cmd.arg("-s");
    cmd.arg("-");
    cmd.arg("-f");
    cmd.arg(exec_path.as_ref());
    let result = cmd.spawn().unwrap().wait().unwrap();
    if result.success() {
        Ok(())
    } else {
        Err("failed to sign".into())
    }
}

pub fn get_enhancement_xml(exec_path: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    let mut cmd = std::process::Command::new("/usr/bin/codesign");
    cmd.stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());
    cmd.arg("-d");
    cmd.arg("--entitlements");
    cmd.arg("-");
    cmd.arg("--xml");
    cmd.arg(exec_path.as_ref());
    let output = cmd.output().unwrap();
    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err("failed to get entitlements".into())
    }
}
